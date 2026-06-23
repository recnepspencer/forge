#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitReplayParityCounters {
    ledger_identity_rows_compared: usize,
    split_request_rows_compared: usize,
    decision_log_rows_compared: usize,
    operational_truth_rows_compared: usize,
    fragment_identity_rows_compared: usize,
    overlap_chain_rows_compared: usize,
    persistent_naming_rows_compared: usize,
    checkpoint_rows_compared: usize,
    orientation_rows_compared: usize,
    replay_closure_rows_compared: usize,
    closeout_rows_read: usize,
    retained_replay_rows_read: usize,
    replay_rows_emitted: usize,
    event_extraction_reexecutions: usize,
    candidate_index_reexecutions: usize,
    replay_mismatches_rejected: usize,
    checkpoint_mismatches_rejected: usize,
    orientation_mismatches_rejected: usize,
}

impl PlanarBooleanEdgeSplitReplayParityCounters {
    pub(crate) fn compared_ledger_identities(&mut self) {
        self.ledger_identity_rows_compared += 1;
    }

    pub(crate) fn compared_split_request(&mut self) {
        self.split_request_rows_compared += 1;
    }

    pub(crate) fn compared_decision_log_identities(&mut self) {
        self.decision_log_rows_compared += 1;
    }

    pub(crate) fn compared_operational_truth(&mut self) {
        self.operational_truth_rows_compared += 1;
    }

    pub(crate) fn compared_fragments(&mut self) {
        self.fragment_identity_rows_compared += 1;
    }

    pub(crate) fn compared_overlap_chains(&mut self) {
        self.overlap_chain_rows_compared += 1;
    }

    pub(crate) fn compared_persistent_naming(&mut self) {
        self.persistent_naming_rows_compared += 1;
    }

    pub(crate) fn compared_checkpoint(&mut self) {
        self.checkpoint_rows_compared += 1;
    }

    pub(crate) fn compared_orientation(&mut self) {
        self.orientation_rows_compared += 1;
    }

    pub(crate) fn compared_replay_closure_rows(&mut self, rows: usize) {
        self.replay_closure_rows_compared += rows;
    }

    pub(crate) fn consumed_query_replay_product(
        &mut self,
        counters: super::super::replay_execution::PlanarBooleanEdgeSplitReplayProductCounters,
    ) {
        self.closeout_rows_read += counters.closeout_rows_read();
        self.retained_replay_rows_read += counters.retained_replay_rows_read();
        self.replay_rows_emitted += counters.replay_rows_emitted();
        self.event_extraction_reexecutions += counters.event_extraction_reexecutions();
        self.candidate_index_reexecutions += counters.candidate_index_reexecutions();
    }

    pub(crate) fn rejected_replay_mismatch(&mut self) {
        self.replay_mismatches_rejected += 1;
    }

    pub fn ledger_identity_rows_compared(self) -> usize {
        self.ledger_identity_rows_compared
    }

    pub fn split_request_rows_compared(self) -> usize {
        self.split_request_rows_compared
    }

    pub fn decision_log_rows_compared(self) -> usize {
        self.decision_log_rows_compared
    }

    pub fn operational_truth_rows_compared(self) -> usize {
        self.operational_truth_rows_compared
    }

    pub fn fragment_identity_rows_compared(self) -> usize {
        self.fragment_identity_rows_compared
    }

    pub fn overlap_chain_rows_compared(self) -> usize {
        self.overlap_chain_rows_compared
    }

    pub fn persistent_naming_rows_compared(self) -> usize {
        self.persistent_naming_rows_compared
    }

    pub fn checkpoint_rows_compared(self) -> usize {
        self.checkpoint_rows_compared
    }

    pub fn orientation_rows_compared(self) -> usize {
        self.orientation_rows_compared
    }

    pub fn replay_closure_rows_compared(self) -> usize {
        self.replay_closure_rows_compared
    }

    pub fn closeout_rows_read(self) -> usize {
        self.closeout_rows_read
    }

    pub fn retained_replay_rows_read(self) -> usize {
        self.retained_replay_rows_read
    }

    pub fn replay_rows_emitted(self) -> usize {
        self.replay_rows_emitted
    }

    pub fn event_extraction_reexecutions(self) -> usize {
        self.event_extraction_reexecutions
    }

    pub fn candidate_index_reexecutions(self) -> usize {
        self.candidate_index_reexecutions
    }

    pub fn replay_mismatches_rejected(self) -> usize {
        self.replay_mismatches_rejected
    }

    pub fn checkpoint_mismatches_rejected(self) -> usize {
        self.checkpoint_mismatches_rejected
    }

    pub fn orientation_mismatches_rejected(self) -> usize {
        self.orientation_mismatches_rejected
    }
}
