#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanLoopReplayParityCounters {
    compared_loop_evidence_receipts: usize,
    compared_reconstructed_loops: usize,
    compared_born_loops: usize,
    compared_island_partitions: usize,
    compared_split_attributions: usize,
    compared_role_outcomes: usize,
    compared_degenerate_outcomes: usize,
    compared_decision_logs: usize,
    compared_loop_ledgers: usize,
    compared_checkpoints: usize,
    rejected_replay_mismatches: usize,
}

impl PlanarBooleanLoopReplayParityCounters {
    pub(crate) fn compared_loop_evidence_receipts(&mut self) {
        self.compared_loop_evidence_receipts += 1;
    }

    pub(crate) fn compared_reconstructed_loops(&mut self) {
        self.compared_reconstructed_loops += 1;
    }

    pub(crate) fn compared_born_loops(&mut self) {
        self.compared_born_loops += 1;
    }

    pub(crate) fn compared_island_partitions(&mut self) {
        self.compared_island_partitions += 1;
    }

    pub(crate) fn compared_split_attributions(&mut self) {
        self.compared_split_attributions += 1;
    }

    pub(crate) fn compared_role_outcomes(&mut self) {
        self.compared_role_outcomes += 1;
    }

    pub(crate) fn compared_degenerate_outcomes(&mut self) {
        self.compared_degenerate_outcomes += 1;
    }

    pub(crate) fn compared_decision_logs(&mut self) {
        self.compared_decision_logs += 1;
    }

    pub(crate) fn compared_loop_ledgers(&mut self) {
        self.compared_loop_ledgers += 1;
    }

    pub(crate) fn compared_checkpoints(&mut self) {
        self.compared_checkpoints += 1;
    }

    pub(crate) fn rejected_replay_mismatch(&mut self) {
        self.rejected_replay_mismatches += 1;
    }

    pub fn compared_loop_evidence_receipts_count(self) -> usize {
        self.compared_loop_evidence_receipts
    }

    pub fn compared_reconstructed_loops_count(self) -> usize {
        self.compared_reconstructed_loops
    }

    pub fn compared_born_loops_count(self) -> usize {
        self.compared_born_loops
    }

    pub fn compared_island_partitions_count(self) -> usize {
        self.compared_island_partitions
    }

    pub fn compared_split_attributions_count(self) -> usize {
        self.compared_split_attributions
    }

    pub fn compared_role_outcomes_count(self) -> usize {
        self.compared_role_outcomes
    }

    pub fn compared_degenerate_outcomes_count(self) -> usize {
        self.compared_degenerate_outcomes
    }

    pub fn compared_decision_logs_count(self) -> usize {
        self.compared_decision_logs
    }

    pub fn compared_loop_ledgers_count(self) -> usize {
        self.compared_loop_ledgers
    }

    pub fn compared_checkpoints_count(self) -> usize {
        self.compared_checkpoints
    }

    pub fn rejected_replay_mismatches_count(self) -> usize {
        self.rejected_replay_mismatches
    }
}
