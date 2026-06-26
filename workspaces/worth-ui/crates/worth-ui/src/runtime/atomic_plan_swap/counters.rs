#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiAtomicPlanSwapCounters {
    prior_valid_capture_count: usize,
    activation_gate_count: usize,
    next_active_state_build_count: usize,
    active_state_mutation_count: usize,
    rollback_restore_count: usize,
    source_reparse_count: usize,
    registry_rebuild_count: usize,
    semantic_replanning_count: usize,
    query_replanning_count: usize,
    handle_allocation_count: usize,
    denial_count: usize,
}

impl WorthUiAtomicPlanSwapCounters {
    pub(crate) fn new() -> Self {
        Self {
            prior_valid_capture_count: 0,
            activation_gate_count: 0,
            next_active_state_build_count: 0,
            active_state_mutation_count: 0,
            rollback_restore_count: 0,
            source_reparse_count: 0,
            registry_rebuild_count: 0,
            semantic_replanning_count: 0,
            query_replanning_count: 0,
            handle_allocation_count: 0,
            denial_count: 0,
        }
    }

    pub(crate) fn record_prior_valid_capture(&mut self) {
        self.prior_valid_capture_count += 1;
    }

    pub(crate) fn record_activation_gate(&mut self) {
        self.activation_gate_count += 1;
    }

    pub(crate) fn record_next_active_state_build(&mut self) {
        self.next_active_state_build_count += 1;
    }

    pub(crate) fn record_active_state_mutation(&mut self) {
        self.active_state_mutation_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_rollback_restore(&mut self) {
        self.rollback_restore_count += 1;
    }

    pub(crate) fn record_denial(&mut self) {
        self.denial_count += 1;
    }

    pub fn prior_valid_capture_count(self) -> usize {
        self.prior_valid_capture_count
    }

    pub fn activation_gate_count(self) -> usize {
        self.activation_gate_count
    }

    pub fn next_active_state_build_count(self) -> usize {
        self.next_active_state_build_count
    }

    pub fn active_state_mutation_count(self) -> usize {
        self.active_state_mutation_count
    }

    pub fn rollback_restore_count(self) -> usize {
        self.rollback_restore_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn registry_rebuild_count(self) -> usize {
        self.registry_rebuild_count
    }

    pub fn semantic_replanning_count(self) -> usize {
        self.semantic_replanning_count
    }

    pub fn query_replanning_count(self) -> usize {
        self.query_replanning_count
    }

    pub fn handle_allocation_count(self) -> usize {
        self.handle_allocation_count
    }

    pub fn denial_count(self) -> usize {
        self.denial_count
    }
}
