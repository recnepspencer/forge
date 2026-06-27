use crate::{
    DirtyPageCounterSnapshot, DirtyPageIdentity, DirtyPageState, DirtyPublicationPlan,
    DirtyPublicationReceipt, DirtyShutdownPosture, DirtyShutdownReport, ResidentFrameDenial,
    ResidentFrameDenialKind, ResidentFrameIdentity, ResidentFrameLoadRequest, ResidentFrameTable,
    ResidentFrameToken,
};

use crate::resident_frame_record::{
    ResidentFrameDirtyMarkTransition, ResidentFrameWriteScheduleTransition,
};

impl ResidentFrameTable {
    pub const fn dirty_counters(&self) -> DirtyPageCounterSnapshot {
        self.dirty_counters
    }

    pub fn mark_dirty(
        &mut self,
        token: ResidentFrameToken,
    ) -> Result<DirtyPageState, ResidentFrameDenial> {
        self.dirty_counters = self.dirty_counters.with_dirty_mark_attempt();
        let identity = self.validate_resident_token_for_dirty(token)?;
        self.reject_dirty_mutation_behind_record_view(identity)?;
        let request = self.record_at_slot(identity.slot())?.request();
        let transition = self
            .record_at_slot(identity.slot())?
            .dirty_mark_transition();
        match transition {
            ResidentFrameDirtyMarkTransition::AlreadyResidentDirty => {
                self.dirty_counters = self.dirty_counters.with_already_dirty();
                self.publish_dirty_counters();
                return Ok(self.dirty_state_report(identity, request));
            }
            ResidentFrameDirtyMarkTransition::NewlyDirty
            | ResidentFrameDirtyMarkTransition::NewlyDirtyBehindScheduledWrite => {
                self.reject_dirty_budget_overflow()?;
            }
        }
        let actual_transition = self.record_at_slot_mut(identity.slot())?.mark_dirty();
        debug_assert_eq!(actual_transition, transition);
        self.dirty_counters = match transition {
            ResidentFrameDirtyMarkTransition::NewlyDirty => self
                .dirty_counters
                .with_newly_dirty(request.frame_size().as_bytes()),
            ResidentFrameDirtyMarkTransition::NewlyDirtyBehindScheduledWrite => self
                .dirty_counters
                .with_newly_dirty_behind_scheduled_write(request.frame_size().as_bytes()),
            ResidentFrameDirtyMarkTransition::AlreadyResidentDirty => self.dirty_counters,
        };
        self.publish_dirty_counters();
        Ok(self.dirty_state_report(identity, request))
    }

    pub fn plan_dirty_publication(
        &mut self,
        dirty_identity: DirtyPageIdentity,
    ) -> Result<DirtyPublicationPlan, ResidentFrameDenial> {
        self.dirty_counters = self.dirty_counters.with_publication_plan_attempt();
        let identity = dirty_identity.resident_frame_identity();
        let record = self.record_at_slot(identity.slot())?;
        if record.identity() != identity {
            return self
                .deny_dirty_publication_plan(ResidentFrameDenialKind::DirtyPublicationPlanStale);
        }
        if !record.has_resident_dirty_delta() {
            return self
                .deny_dirty_publication_plan(ResidentFrameDenialKind::ResidentFrameNotDirty);
        }
        if record.has_active_pin() {
            self.record_publication_conflict_behind_record_view();
            self.record_protected_mutation_denial();
            return self.deny_dirty_publication_plan(
                ResidentFrameDenialKind::DirtyPublicationBehindActiveLease,
            );
        }
        let request = record.request();
        let lease_epoch = self.lease_epochs[identity.slot().index() as usize];
        let dirty_epoch = record.dirty_publication_epoch();
        self.publish_dirty_counters();
        Ok(DirtyPublicationPlan::new(
            dirty_identity,
            request,
            lease_epoch,
            dirty_epoch,
            self.dirty_counters,
        ))
    }

    pub fn record_dirty_write_scheduled(
        &mut self,
        plan: DirtyPublicationPlan,
    ) -> Result<DirtyPublicationReceipt, ResidentFrameDenial> {
        let identity = plan.dirty_identity().resident_frame_identity();
        let transition = {
            let record = self.record_at_slot(identity.slot())?;
            if record.identity() != identity {
                return self.deny_stale_dirty_publication_schedule();
            }
            if self.lease_epochs[identity.slot().index() as usize] != plan.lease_epoch() {
                return self.deny_stale_dirty_publication_schedule();
            }
            if record.dirty_publication_epoch() != plan.dirty_publication_epoch() {
                return self.deny_stale_dirty_publication_schedule();
            }
            if !record.has_resident_dirty_delta() {
                return Err(ResidentFrameDenial::new(
                    ResidentFrameDenialKind::ResidentFrameNotDirty,
                ));
            }
            if record.has_active_pin() {
                self.record_publication_conflict_behind_record_view();
                self.record_protected_mutation_denial();
                self.dirty_counters = self.dirty_counters.with_write_scheduling_denial();
                self.publish_dirty_counters();
                return Err(ResidentFrameDenial::new(
                    ResidentFrameDenialKind::DirtyPublicationBehindActiveLease,
                ));
            }
            record.write_schedule_transition().ok_or_else(|| {
                ResidentFrameDenial::new(ResidentFrameDenialKind::ResidentFrameNotDirty)
            })?
        };
        let record = self.record_at_slot_mut(identity.slot())?;
        let actual_transition = record.mark_write_scheduled_not_durable().ok_or_else(|| {
            ResidentFrameDenial::new(ResidentFrameDenialKind::ResidentFrameNotDirty)
        })?;
        debug_assert_eq!(actual_transition, transition);
        self.dirty_counters = match transition {
            ResidentFrameWriteScheduleTransition::FirstScheduledWrite => self
                .dirty_counters
                .with_first_publication_receipt(plan.frame_size_bytes()),
            ResidentFrameWriteScheduleTransition::AdditionalScheduledWriteBehindPendingWrite => {
                self.dirty_counters
                    .with_additional_publication_receipt(plan.frame_size_bytes())
            }
        };
        self.publish_dirty_counters();
        Ok(DirtyPublicationReceipt::new(
            plan.dirty_identity(),
            crate::DirtyPageCount::from_observed_pages(1),
            self.dirty_counters,
        ))
    }

    pub fn dirty_shutdown_closeout(&mut self) -> DirtyShutdownReport {
        let posture = if self.dirty_counters.has_unflushed_dirty_state() {
            self.dirty_counters = self.dirty_counters.with_dirty_shutdown_unflushed();
            self.publish_dirty_counters();
            DirtyShutdownPosture::UnflushedDirtyPagesRemain
        } else {
            DirtyShutdownPosture::CleanNoDirtyPages
        };
        DirtyShutdownReport::new(posture, self.dirty_counters)
    }

    fn reject_dirty_budget_overflow(&mut self) -> Result<(), ResidentFrameDenial> {
        let next_dirty_pages = self.dirty_counters.dirty_pages().as_pages() + 1;
        if next_dirty_pages > self.entry.admission().budget().dirty_pages().as_pages() {
            self.dirty_counters = self.dirty_counters.with_dirty_budget_denial();
            self.publish_dirty_counters();
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::DirtyPageBudgetExceeded,
            ));
        }
        Ok(())
    }

    fn validate_resident_token_for_dirty(
        &self,
        token: ResidentFrameToken,
    ) -> Result<ResidentFrameIdentity, ResidentFrameDenial> {
        self.reject_slot_out_of_range(token.slot())?;
        let record = self.record_at_slot(token.slot())?;
        if record.identity().generation() != token.resident_generation() {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::StaleResidentGeneration,
            ));
        }
        Ok(record.identity())
    }

    fn dirty_state_report(
        &self,
        identity: ResidentFrameIdentity,
        request: ResidentFrameLoadRequest,
    ) -> DirtyPageState {
        DirtyPageState::new(
            DirtyPageIdentity::new(identity),
            request,
            self.dirty_counters,
        )
    }

    fn deny_dirty_publication_plan<T>(
        &mut self,
        kind: ResidentFrameDenialKind,
    ) -> Result<T, ResidentFrameDenial> {
        self.dirty_counters = self.dirty_counters.with_publication_plan_denial();
        self.publish_dirty_counters();
        Err(ResidentFrameDenial::new(kind))
    }

    fn deny_stale_dirty_publication_schedule<T>(&mut self) -> Result<T, ResidentFrameDenial> {
        self.dirty_counters = self.dirty_counters.with_stale_publication_plan_denial();
        self.publish_dirty_counters();
        Err(ResidentFrameDenial::new(
            ResidentFrameDenialKind::DirtyPublicationPlanStale,
        ))
    }
}
