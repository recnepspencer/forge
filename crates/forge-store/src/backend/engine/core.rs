use crate::{
    authority::AuthoritativeExportBundle,
    failure::{StoreError, StoreErrorKind},
    layout::{AdmittedAspectLayoutReadPlan, AspectLayoutReadPlanDecision, AspectLayoutReadRequest},
    media::DurableMediaReport,
    wal::WalRecord,
};

use super::{
    StateBackedStoreBackend, StatePersistence,
    super::integrity::{verify_milestone_6_access_structures, verify_milestone_7_access_structures},
};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    fn refresh_access_structure_verifications(&mut self) {
        let media_report = self.persistence.durable_media_report();
        self.milestone_6_access_structure_verification =
            verify_milestone_6_access_structures(&self.state, media_report);
        self.milestone_7_access_structure_verification =
            verify_milestone_7_access_structures(&self.state, media_report);
    }

    pub(crate) fn state(&self) -> &super::super::records::StoreState {
        &self.state
    }

    pub(crate) fn counters(&self) -> &crate::evidence::StoreCounters {
        &self.counters
    }

    pub(crate) fn commit_replacement_state(
        &mut self,
        next: super::super::records::StoreState,
    ) -> Result<(), StoreError> {
        next.verify_integrity()?;
        let report = self.persistence.persist_state(&next)?;
        verify_durable_barrier(&mut self.counters, &report)?;
        self.state = next;
        self.refresh_access_structure_verifications();
        Ok(())
    }

    pub(super) fn require_admitted_aspect_layout_plan(
        &self,
        request: AspectLayoutReadRequest,
        operation_name: &str,
    ) -> Result<AdmittedAspectLayoutReadPlan, StoreError> {
        match self.plan_aspect_layout_read(request)? {
            AspectLayoutReadPlanDecision::Admitted(plan) => Ok(plan),
            AspectLayoutReadPlanDecision::Fallback(plan) => Err(StoreError::new(
                StoreErrorKind::AspectLayoutFallbackRequired,
                format!(
                    "{operation_name} requires an admitted Milestone 6 layout request, but planning fell back: {}",
                    plan.reason()
                ),
            )),
            AspectLayoutReadPlanDecision::Rejected(plan) => Err(StoreError::new(
                StoreErrorKind::AspectScopeUnsupported,
                format!(
                    "{operation_name} requires an admitted Milestone 6 layout request, but planning rejected the request: {}",
                    plan.reason()
                ),
            )),
        }
    }

    pub(super) fn append_wal_record_committed(&mut self, record: WalRecord) -> Result<(), StoreError> {
        let inserted_sequence = record.wal_sequence;
        self.state.append_wal_record(record)?;

        if let Err(error) = self.state.verify_wal_record_family() {
            self.state.wal_records.remove(&inserted_sequence);
            self.state.next_wal_sequence = inserted_sequence;
            return Err(error);
        }

        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state.wal_records.remove(&inserted_sequence);
                self.state.next_wal_sequence = inserted_sequence;
                return Err(error);
            }
        };

        verify_durable_barrier(&mut self.counters, &report)?;
        self.counters.record_state_delta_apply(1, 1);
        Ok(())
    }

    pub fn open_with_persistence(mut persistence: P) -> Result<Self, StoreError> {
        let state = persistence.load_state()?;
        state.verify_integrity()?;
        let milestone_6_access_structure_verification =
            verify_milestone_6_access_structures(&state, persistence.durable_media_report());
        let milestone_7_access_structure_verification =
            verify_milestone_7_access_structures(&state, persistence.durable_media_report());
        Ok(Self {
            persistence,
            state,
            milestone_6_access_structure_verification,
            milestone_7_access_structure_verification,
            milestone_6_scope_prepare_counts: std::collections::HashMap::new(),
            counters: crate::evidence::StoreCounters::default(),
        })
    }

    pub fn open_with_persistence_for_durable_recovery(
        mut persistence: P,
    ) -> Result<Self, StoreError> {
        let state = persistence.load_state()?;
        state.verify_integrity_for_durable_recovery()?;
        let milestone_6_access_structure_verification =
            verify_milestone_6_access_structures(&state, persistence.durable_media_report());
        let milestone_7_access_structure_verification =
            verify_milestone_7_access_structures(&state, persistence.durable_media_report());
        Ok(Self {
            persistence,
            state,
            milestone_6_access_structure_verification,
            milestone_7_access_structure_verification,
            milestone_6_scope_prepare_counts: std::collections::HashMap::new(),
            counters: crate::evidence::StoreCounters::default(),
        })
    }

    pub fn from_export_bundle_with_persistence(
        mut persistence: P,
        bundle: AuthoritativeExportBundle,
    ) -> Result<Self, StoreError> {
        let state = super::super::records::StoreState::from_authoritative_export_bundle(bundle)?;
        let _ = persistence.persist_state(&state)?;
        let milestone_6_access_structure_verification =
            verify_milestone_6_access_structures(&state, persistence.durable_media_report());
        let milestone_7_access_structure_verification =
            verify_milestone_7_access_structures(&state, persistence.durable_media_report());
        Ok(Self {
            persistence,
            state,
            milestone_6_access_structure_verification,
            milestone_7_access_structure_verification,
            milestone_6_scope_prepare_counts: std::collections::HashMap::new(),
            counters: crate::evidence::StoreCounters::default(),
        })
    }
}

pub(crate) fn verify_durable_barrier(
    counters: &mut crate::evidence::StoreCounters,
    report: &DurableMediaReport,
) -> Result<(), StoreError> {
    if report.content_barrier() < report.ack_required_barrier() {
        counters.record_durable_ack_barrier_violation();
        return Err(StoreError::new(
            StoreErrorKind::DurableBarrierContractViolation,
            format!(
                "backend {:?} reported content barrier {:?} below required acknowledgment barrier {:?}",
                report.backend_family(),
                report.content_barrier(),
                report.ack_required_barrier()
            ),
        ));
    }
    counters.record_durable_barrier_verified();
    Ok(())
}
