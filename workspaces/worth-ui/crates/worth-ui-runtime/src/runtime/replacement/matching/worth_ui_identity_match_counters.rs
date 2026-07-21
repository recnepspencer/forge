#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiIdentityMatchCounters {
    active_nodes_indexed: usize,
    candidate_nodes_indexed: usize,
    stable_seed_lookups: usize,
    duplicate_active_identity_count: usize,
    duplicate_candidate_identity_count: usize,
    identity_kind_mismatch_count: usize,
    matches_emitted: usize,
    unmatched_active_count: usize,
    unmatched_candidate_count: usize,
}

impl WorthUiIdentityMatchCounters {
    pub(crate) fn record_active_node_indexed(&mut self) {
        self.active_nodes_indexed += 1;
    }

    pub(crate) fn record_candidate_node_indexed(&mut self) {
        self.candidate_nodes_indexed += 1;
    }

    pub(crate) fn record_stable_seed_lookup(&mut self) {
        self.stable_seed_lookups += 1;
    }

    pub(crate) fn record_duplicate_active_identity(&mut self) {
        self.duplicate_active_identity_count += 1;
    }

    pub(crate) fn record_duplicate_candidate_identity(&mut self) {
        self.duplicate_candidate_identity_count += 1;
    }

    pub(crate) fn record_identity_kind_mismatch(&mut self) {
        self.identity_kind_mismatch_count += 1;
    }

    pub(crate) fn record_match_emitted(&mut self) {
        self.matches_emitted += 1;
    }

    pub(crate) fn record_unmatched_active(&mut self) {
        self.unmatched_active_count += 1;
    }

    pub(crate) fn record_unmatched_candidate(&mut self) {
        self.unmatched_candidate_count += 1;
    }

    pub fn active_nodes_indexed(&self) -> usize {
        self.active_nodes_indexed
    }

    pub fn candidate_nodes_indexed(&self) -> usize {
        self.candidate_nodes_indexed
    }

    pub fn stable_seed_lookups(&self) -> usize {
        self.stable_seed_lookups
    }

    pub fn duplicate_active_identity_count(&self) -> usize {
        self.duplicate_active_identity_count
    }

    pub fn duplicate_candidate_identity_count(&self) -> usize {
        self.duplicate_candidate_identity_count
    }

    pub fn identity_kind_mismatch_count(&self) -> usize {
        self.identity_kind_mismatch_count
    }

    pub fn matches_emitted(&self) -> usize {
        self.matches_emitted
    }

    pub fn unmatched_active_count(&self) -> usize {
        self.unmatched_active_count
    }

    pub fn unmatched_candidate_count(&self) -> usize {
        self.unmatched_candidate_count
    }
}
