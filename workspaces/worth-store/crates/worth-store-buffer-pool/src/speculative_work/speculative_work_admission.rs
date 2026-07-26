use crate::{
    AllocationAdmission, AllocationGrant, AllocationReceipt, AllocationRequest, AllocationScope,
    PhysicalSpeculativeWorkKind, PrefetchAdmission, PrefetchPlan, PrefetchRequest,
    ReadAheadAdmission, ReadAheadPlan, ReadAheadRequest, SpeculativePhysicalWorkDenial,
    SpeculativePhysicalWorkDenialKind, SpeculativeWorkBudgetSnapshot,
    SpeculativeWorkCounterSnapshot, SpeculativeWorkReplayIdentity, WriteBehindAdmission,
    WriteBehindPlan, WriteBehindRequest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeculativePhysicalWorkAdmission {
    counters: SpeculativeWorkCounterSnapshot,
}

impl SpeculativePhysicalWorkAdmission {
    pub const fn new() -> Self {
        Self {
            counters: SpeculativeWorkCounterSnapshot::empty(),
        }
    }

    pub const fn counters(self) -> SpeculativeWorkCounterSnapshot {
        self.counters
    }

    pub fn lower_read_ahead(
        &mut self,
        request: ReadAheadRequest,
        budget: SpeculativeWorkBudgetSnapshot,
        allocation: &mut AllocationAdmission,
    ) -> Result<ReadAheadPlan, SpeculativePhysicalWorkDenial> {
        self.counters = self.counters.with_read_ahead_attempt();
        let resident_frames = request.window().as_resident_frames();
        self.reject_resident_pressure(
            PhysicalSpeculativeWorkKind::ReadAhead,
            resident_frames,
            budget,
        )
        .map_err(|kind| self.deny_read_ahead(kind))?;
        self.reject_pin_pressure(resident_frames, budget)
            .map_err(|kind| self.deny_read_ahead(kind))?;
        let allocation_grant = self
            .admit_allocation(request.allocation(), allocation)
            .map_err(|kind| self.deny_read_ahead(kind))?;
        let allocation_bytes = allocation_grant.as_ref().map_or(0, AllocationGrant::bytes);
        self.counters = self
            .counters
            .with_read_ahead_admitted(resident_frames)
            .with_allocation_bytes_admitted(allocation_bytes);
        Ok(ReadAheadPlan::new(
            request.window(),
            self.replay_identity(
                PhysicalSpeculativeWorkKind::ReadAhead,
                resident_frames,
                0,
                allocation_bytes,
                budget,
            ),
            allocation_grant,
            self.counters,
        ))
    }

    pub fn lower_prefetch(
        &mut self,
        request: PrefetchRequest,
        budget: SpeculativeWorkBudgetSnapshot,
        allocation: &mut AllocationAdmission,
    ) -> Result<PrefetchPlan, SpeculativePhysicalWorkDenial> {
        self.counters = self.counters.with_prefetch_attempt();
        let resident_frames = request.window().as_resident_frames();
        self.reject_resident_pressure(
            PhysicalSpeculativeWorkKind::Prefetch,
            resident_frames,
            budget,
        )
        .map_err(|kind| self.deny_prefetch(kind))?;
        self.reject_pin_pressure(resident_frames, budget)
            .map_err(|kind| self.deny_prefetch(kind))?;
        let allocation_grant = self
            .admit_allocation(request.allocation(), allocation)
            .map_err(|kind| self.deny_prefetch(kind))?;
        let allocation_bytes = allocation_grant.as_ref().map_or(0, AllocationGrant::bytes);
        self.counters = self
            .counters
            .with_prefetch_admitted(resident_frames)
            .with_allocation_bytes_admitted(allocation_bytes);
        Ok(PrefetchPlan::new(
            request.window(),
            self.replay_identity(
                PhysicalSpeculativeWorkKind::Prefetch,
                resident_frames,
                0,
                allocation_bytes,
                budget,
            ),
            allocation_grant,
            self.counters,
        ))
    }

    pub fn lower_write_behind(
        &mut self,
        request: WriteBehindRequest,
        budget: SpeculativeWorkBudgetSnapshot,
        allocation: &mut AllocationAdmission,
    ) -> Result<WriteBehindPlan, SpeculativePhysicalWorkDenial> {
        self.counters = self.counters.with_write_behind_attempt();
        let dirty_pages = request.dirty_page_count().as_pages();
        if budget.dirty_pages_exceed_budget() {
            return Err(self.deny_write_behind(
                SpeculativePhysicalWorkDenialKind::DirtyBudgetWouldBeExceeded {
                    requested_pages: dirty_pages,
                    dirty_pages_used: budget.dirty_pages_used(),
                    dirty_page_budget: budget.dirty_page_budget(),
                },
            ));
        }
        if dirty_pages > budget.dirty_pages_used() {
            return Err(self.deny_write_behind(
                SpeculativePhysicalWorkDenialKind::DirtyWorkNotResident {
                    requested_pages: dirty_pages,
                    dirty_pages_used: budget.dirty_pages_used(),
                },
            ));
        }
        let allocation_grant = self
            .admit_allocation(request.allocation(), allocation)
            .map_err(|kind| self.deny_write_behind(kind))?;
        let allocation_bytes = allocation_grant.as_ref().map_or(0, AllocationGrant::bytes);
        self.counters = self
            .counters
            .with_write_behind_admitted(dirty_pages)
            .with_allocation_bytes_admitted(allocation_bytes);
        Ok(WriteBehindPlan::new(
            self.replay_identity(
                PhysicalSpeculativeWorkKind::WriteBehind,
                0,
                dirty_pages,
                allocation_bytes,
                budget,
            ),
            allocation_grant,
            self.counters,
        ))
    }

    pub fn record_read_ahead_admitted(
        &mut self,
        plan: ReadAheadPlan,
        allocation: &mut AllocationAdmission,
    ) -> Result<ReadAheadAdmission, SpeculativePhysicalWorkDenial> {
        let (_, replay_identity, grant, counters) = plan.into_parts();
        let receipt = self
            .record_allocation(grant, allocation)
            .map_err(|kind| self.deny_read_ahead(kind))?;
        self.counters = counters;
        Ok(ReadAheadAdmission::new(replay_identity, receipt, counters))
    }

    pub fn record_prefetch_admitted(
        &mut self,
        plan: PrefetchPlan,
        allocation: &mut AllocationAdmission,
    ) -> Result<PrefetchAdmission, SpeculativePhysicalWorkDenial> {
        let (_, replay_identity, grant, counters) = plan.into_parts();
        let receipt = self
            .record_allocation(grant, allocation)
            .map_err(|kind| self.deny_prefetch(kind))?;
        self.counters = counters;
        Ok(PrefetchAdmission::new(replay_identity, receipt, counters))
    }

    pub fn record_write_behind_admitted(
        &mut self,
        plan: WriteBehindPlan,
        allocation: &mut AllocationAdmission,
    ) -> Result<WriteBehindAdmission, SpeculativePhysicalWorkDenial> {
        let (replay_identity, grant, counters) = plan.into_parts();
        let receipt = self
            .record_allocation(grant, allocation)
            .map_err(|kind| self.deny_write_behind(kind))?;
        self.counters = counters;
        Ok(WriteBehindAdmission::new(
            replay_identity,
            receipt,
            counters,
        ))
    }

    pub fn reject_unsupported_qos_claim(&mut self) -> SpeculativePhysicalWorkDenial {
        self.counters = self.counters.with_deferred();
        SpeculativePhysicalWorkDenial::new(
            SpeculativePhysicalWorkDenialKind::UnsupportedQosClaim,
            self.counters,
        )
    }

    fn reject_resident_pressure(
        &self,
        _kind: PhysicalSpeculativeWorkKind,
        requested_frames: u32,
        budget: SpeculativeWorkBudgetSnapshot,
    ) -> Result<(), SpeculativePhysicalWorkDenialKind> {
        if requested_frames <= budget.free_frame_count() {
            return Ok(());
        }
        if budget.all_resident_frames_protected() {
            return Err(
                SpeculativePhysicalWorkDenialKind::ProtectedEvictionPressure { requested_frames },
            );
        }
        Err(
            SpeculativePhysicalWorkDenialKind::ResidentBudgetWouldBeExceeded {
                requested_frames,
                free_frames: budget.free_frame_count(),
            },
        )
    }

    fn reject_pin_pressure(
        &self,
        requested_pages: u32,
        budget: SpeculativeWorkBudgetSnapshot,
    ) -> Result<(), SpeculativePhysicalWorkDenialKind> {
        if !budget.pinned_pages_exceed_budget(requested_pages) {
            return Ok(());
        }
        Err(
            SpeculativePhysicalWorkDenialKind::PinBudgetWouldBeExceeded {
                requested_pages,
                pinned_pages_used: budget.pinned_pages_used(),
                pinned_page_budget: budget.pinned_page_budget(),
            },
        )
    }

    fn admit_allocation(
        &mut self,
        request: Option<AllocationRequest>,
        allocation: &mut AllocationAdmission,
    ) -> Result<Option<AllocationGrant>, SpeculativePhysicalWorkDenialKind> {
        let Some(request) = request else {
            return Ok(None);
        };
        if request.scope() == AllocationScope::Foreground {
            return Err(
                SpeculativePhysicalWorkDenialKind::ForegroundAllocationInterference {
                    requested_bytes: request.requested_bytes().unwrap_or(0),
                },
            );
        }
        allocation
            .admit(request)
            .map(Some)
            .map_err(SpeculativePhysicalWorkDenialKind::AllocationDenied)
    }

    fn record_allocation(
        &mut self,
        grant: Option<AllocationGrant>,
        allocation: &mut AllocationAdmission,
    ) -> Result<Option<AllocationReceipt>, SpeculativePhysicalWorkDenialKind> {
        let Some(grant) = grant else {
            return Ok(None);
        };
        allocation
            .record_allocation(grant)
            .map(Some)
            .map_err(SpeculativePhysicalWorkDenialKind::AllocationDenied)
    }

    fn replay_identity(
        &self,
        kind: PhysicalSpeculativeWorkKind,
        resident_frames: u32,
        dirty_pages: u32,
        allocation_bytes: u64,
        budget: SpeculativeWorkBudgetSnapshot,
    ) -> SpeculativeWorkReplayIdentity {
        SpeculativeWorkReplayIdentity::new(
            kind,
            resident_frames,
            dirty_pages,
            allocation_bytes,
            budget.resident_frame_count(),
            budget.dirty_pages_used(),
        )
    }

    fn deny_read_ahead(
        &mut self,
        kind: SpeculativePhysicalWorkDenialKind,
    ) -> SpeculativePhysicalWorkDenial {
        self.counters = self.counters.with_read_ahead_denied();
        SpeculativePhysicalWorkDenial::new(kind, self.counters)
    }

    fn deny_prefetch(
        &mut self,
        kind: SpeculativePhysicalWorkDenialKind,
    ) -> SpeculativePhysicalWorkDenial {
        self.counters = self.counters.with_prefetch_denied();
        SpeculativePhysicalWorkDenial::new(kind, self.counters)
    }

    fn deny_write_behind(
        &mut self,
        kind: SpeculativePhysicalWorkDenialKind,
    ) -> SpeculativePhysicalWorkDenial {
        self.counters = self.counters.with_write_behind_denied();
        SpeculativePhysicalWorkDenial::new(kind, self.counters)
    }
}

impl Default for SpeculativePhysicalWorkAdmission {
    fn default() -> Self {
        Self::new()
    }
}
