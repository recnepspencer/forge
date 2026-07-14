#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiCommittedAllocationActivationCounters {
    ledger_predecessor_checks: u16,
    readiness_checks: u16,
    graph_predecessor_checks: u16,
    scroll_binding_checks: u16,
    portal_binding_checks: u16,
    frame_replacement_checks: u16,
    frame_boundary_checks: u16,
    active_successor_builds: u16,
    denial_count: u16,
    live_mutation_count: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiCommittedAllocationActivationCounterExhaustion {
    LedgerPredecessorChecks,
    ReadinessChecks,
    GraphPredecessorChecks,
    ScrollBindingChecks,
    PortalBindingChecks,
    FrameReplacementChecks,
    FrameBoundaryChecks,
    ActiveSuccessorBuilds,
    DenialCount,
    LiveMutationCount,
}

impl UiCommittedAllocationActivationCounters {
    pub(crate) fn record_readiness_checks(
        &mut self,
        count: usize,
    ) -> Result<(), UiCommittedAllocationActivationCounterExhaustion> {
        add(
            &mut self.readiness_checks,
            count,
            UiCommittedAllocationActivationCounterExhaustion::ReadinessChecks,
        )
    }

    pub(crate) fn record_denial(
        &mut self,
    ) -> Result<(), UiCommittedAllocationActivationCounterExhaustion> {
        add(
            &mut self.denial_count,
            1,
            UiCommittedAllocationActivationCounterExhaustion::DenialCount,
        )
    }

    pub(crate) fn record_ledger_predecessor_check(
        &mut self,
    ) -> Result<(), UiCommittedAllocationActivationCounterExhaustion> {
        add(
            &mut self.ledger_predecessor_checks,
            1,
            UiCommittedAllocationActivationCounterExhaustion::LedgerPredecessorChecks,
        )
    }

    pub(crate) fn record_graph_predecessor_check(
        &mut self,
    ) -> Result<(), UiCommittedAllocationActivationCounterExhaustion> {
        add(
            &mut self.graph_predecessor_checks,
            1,
            UiCommittedAllocationActivationCounterExhaustion::GraphPredecessorChecks,
        )
    }

    pub(crate) fn record_scroll_binding_check(
        &mut self,
    ) -> Result<(), UiCommittedAllocationActivationCounterExhaustion> {
        add(
            &mut self.scroll_binding_checks,
            1,
            UiCommittedAllocationActivationCounterExhaustion::ScrollBindingChecks,
        )
    }

    pub(crate) fn record_portal_binding_check(
        &mut self,
    ) -> Result<(), UiCommittedAllocationActivationCounterExhaustion> {
        add(
            &mut self.portal_binding_checks,
            1,
            UiCommittedAllocationActivationCounterExhaustion::PortalBindingChecks,
        )
    }

    pub(crate) fn record_frame_replacement_check(
        &mut self,
    ) -> Result<(), UiCommittedAllocationActivationCounterExhaustion> {
        add(
            &mut self.frame_replacement_checks,
            1,
            UiCommittedAllocationActivationCounterExhaustion::FrameReplacementChecks,
        )
    }

    pub(crate) fn record_frame_boundary_check(
        &mut self,
    ) -> Result<(), UiCommittedAllocationActivationCounterExhaustion> {
        add(
            &mut self.frame_boundary_checks,
            1,
            UiCommittedAllocationActivationCounterExhaustion::FrameBoundaryChecks,
        )
    }

    pub(crate) fn record_active_successor_build(
        &mut self,
    ) -> Result<(), UiCommittedAllocationActivationCounterExhaustion> {
        add(
            &mut self.active_successor_builds,
            1,
            UiCommittedAllocationActivationCounterExhaustion::ActiveSuccessorBuilds,
        )
    }

    pub(crate) fn record_live_mutation(
        &mut self,
    ) -> Result<(), UiCommittedAllocationActivationCounterExhaustion> {
        add(
            &mut self.live_mutation_count,
            1,
            UiCommittedAllocationActivationCounterExhaustion::LiveMutationCount,
        )
    }

    pub fn ledger_predecessor_checks(self) -> u16 {
        self.ledger_predecessor_checks
    }
    pub fn readiness_checks(self) -> u16 {
        self.readiness_checks
    }
    pub fn graph_predecessor_checks(self) -> u16 {
        self.graph_predecessor_checks
    }
    pub fn scroll_binding_checks(self) -> u16 {
        self.scroll_binding_checks
    }
    pub fn portal_binding_checks(self) -> u16 {
        self.portal_binding_checks
    }
    pub fn frame_replacement_checks(self) -> u16 {
        self.frame_replacement_checks
    }
    pub fn frame_boundary_checks(self) -> u16 {
        self.frame_boundary_checks
    }
    pub fn active_successor_builds(self) -> u16 {
        self.active_successor_builds
    }
    pub fn denial_count(self) -> u16 {
        self.denial_count
    }
    pub fn live_mutation_count(self) -> u16 {
        self.live_mutation_count
    }
}

fn add(
    target: &mut u16,
    count: usize,
    exhaustion: UiCommittedAllocationActivationCounterExhaustion,
) -> Result<(), UiCommittedAllocationActivationCounterExhaustion> {
    let count = u16::try_from(count).map_err(|_| exhaustion)?;
    *target = target.checked_add(count).ok_or(exhaustion)?;
    Ok(())
}
