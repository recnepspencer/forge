#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiActivationGateCounters {
    boundary_check_count: usize,
    readiness_check_count: usize,
    digest_check_count: usize,
    query_rebind_entry_check_count: usize,
    lane_parity_check_count: usize,
    active_state_mutation_count: usize,
    semantic_replanning_count: usize,
    query_replanning_count: usize,
    handle_allocation_count: usize,
    denial_count: usize,
}

impl WorthUiActivationGateCounters {
    pub(crate) fn record_boundary_check(&mut self) {
        self.boundary_check_count += 1;
    }

    pub(crate) fn record_readiness_check(&mut self) {
        self.readiness_check_count += 1;
    }

    pub(crate) fn record_digest_check(&mut self) {
        self.digest_check_count += 1;
    }

    pub(crate) fn record_query_rebind_entry_checks(&mut self, count: usize) {
        self.query_rebind_entry_check_count += count;
    }

    pub(crate) fn record_lane_parity_check(&mut self) {
        self.lane_parity_check_count += 1;
    }

    pub(crate) fn record_denial(&mut self) {
        self.denial_count += 1;
    }

    pub fn boundary_check_count(self) -> usize {
        self.boundary_check_count
    }

    pub fn readiness_check_count(self) -> usize {
        self.readiness_check_count
    }

    pub fn digest_check_count(self) -> usize {
        self.digest_check_count
    }

    pub fn query_rebind_entry_check_count(self) -> usize {
        self.query_rebind_entry_check_count
    }

    pub fn lane_parity_check_count(self) -> usize {
        self.lane_parity_check_count
    }

    pub fn active_state_mutation_count(self) -> usize {
        self.active_state_mutation_count
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
