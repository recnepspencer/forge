#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanLoopDecisionLogCounters {
    continuation_rows_consumed: usize,
    walk_outcomes_consumed: usize,
    loop_candidates_consumed: usize,
    denied_loop_candidates_consumed: usize,
    reconstructed_loops_consumed: usize,
    born_loops_consumed: usize,
    island_rows_consumed: usize,
    split_attribution_rows_consumed: usize,
    role_rows_consumed: usize,
    degenerate_rows_consumed: usize,
    identity_rows_consumed: usize,
    propagated_name_rows_consumed: usize,
    propagated_signature_rows_consumed: usize,
    decision_rows_emitted: usize,
    lookup_index_entries: usize,
    duplicate_decision_identity_denials: usize,
    request_identity_mismatch_denials: usize,
}

impl PlanarBooleanLoopDecisionLogCounters {
    pub(crate) fn consumed_continuation_row(&mut self) {
        self.continuation_rows_consumed += 1;
    }

    pub(crate) fn consumed_walk_outcome(&mut self) {
        self.walk_outcomes_consumed += 1;
    }

    pub(crate) fn consumed_loop_candidate(&mut self) {
        self.loop_candidates_consumed += 1;
    }

    pub(crate) fn consumed_denied_loop_candidate(&mut self) {
        self.denied_loop_candidates_consumed += 1;
    }

    pub(crate) fn consumed_reconstructed_loop(&mut self) {
        self.reconstructed_loops_consumed += 1;
    }

    pub(crate) fn consumed_born_loop(&mut self) {
        self.born_loops_consumed += 1;
    }

    pub(crate) fn consumed_island_row(&mut self) {
        self.island_rows_consumed += 1;
    }

    pub(crate) fn consumed_split_attribution_row(&mut self) {
        self.split_attribution_rows_consumed += 1;
    }

    pub(crate) fn consumed_role_row(&mut self) {
        self.role_rows_consumed += 1;
    }

    pub(crate) fn consumed_degenerate_row(&mut self) {
        self.degenerate_rows_consumed += 1;
    }

    pub(crate) fn consumed_identity_row(&mut self) {
        self.identity_rows_consumed += 1;
    }

    pub(crate) fn consumed_propagated_name_row(&mut self) {
        self.propagated_name_rows_consumed += 1;
    }

    pub(crate) fn consumed_propagated_signature_row(&mut self) {
        self.propagated_signature_rows_consumed += 1;
    }

    pub(crate) fn emitted_decision_row(&mut self) {
        self.decision_rows_emitted += 1;
    }

    pub(crate) fn indexed_lookup_entries(&mut self, count: usize) {
        self.lookup_index_entries += count;
    }

    pub(crate) fn denied_duplicate_decision_identity(&mut self) {
        self.duplicate_decision_identity_denials += 1;
    }

    pub(crate) fn denied_request_identity_mismatch(&mut self) {
        self.request_identity_mismatch_denials += 1;
    }

    pub fn continuation_rows_consumed(self) -> usize {
        self.continuation_rows_consumed
    }

    pub fn walk_outcomes_consumed(self) -> usize {
        self.walk_outcomes_consumed
    }

    pub fn loop_candidates_consumed(self) -> usize {
        self.loop_candidates_consumed
    }

    pub fn denied_loop_candidates_consumed(self) -> usize {
        self.denied_loop_candidates_consumed
    }

    pub fn reconstructed_loops_consumed(self) -> usize {
        self.reconstructed_loops_consumed
    }

    pub fn born_loops_consumed(self) -> usize {
        self.born_loops_consumed
    }

    pub fn island_rows_consumed(self) -> usize {
        self.island_rows_consumed
    }

    pub fn split_attribution_rows_consumed(self) -> usize {
        self.split_attribution_rows_consumed
    }

    pub fn role_rows_consumed(self) -> usize {
        self.role_rows_consumed
    }

    pub fn degenerate_rows_consumed(self) -> usize {
        self.degenerate_rows_consumed
    }

    pub fn identity_rows_consumed(self) -> usize {
        self.identity_rows_consumed
    }

    pub fn propagated_name_rows_consumed(self) -> usize {
        self.propagated_name_rows_consumed
    }

    pub fn propagated_signature_rows_consumed(self) -> usize {
        self.propagated_signature_rows_consumed
    }

    pub fn decision_rows_emitted(self) -> usize {
        self.decision_rows_emitted
    }

    pub fn lookup_index_entries(self) -> usize {
        self.lookup_index_entries
    }

    pub fn duplicate_decision_identity_denials(self) -> usize {
        self.duplicate_decision_identity_denials
    }

    pub fn request_identity_mismatch_denials(self) -> usize {
        self.request_identity_mismatch_denials
    }
}
