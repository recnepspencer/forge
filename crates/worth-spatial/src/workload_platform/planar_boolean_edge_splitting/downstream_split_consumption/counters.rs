#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanDownstreamSplitConsumptionCounters {
    receipts_consumed: usize,
    split_chains_consumed: usize,
    fragment_rows_consumed: usize,
    vertex_rows_consumed: usize,
    persistent_name_rows_consumed: usize,
    replay_parity_rows_consumed: usize,
    stage_index_rows_consumed: usize,
    foreign_receipts_rejected: usize,
    missing_receipts_rejected: usize,
    non_receipt_evidence_rejected: usize,
}

impl PlanarBooleanDownstreamSplitConsumptionCounters {
    pub(crate) fn consumed_receipt(&mut self) {
        self.receipts_consumed += 1;
    }

    pub(crate) fn consumed_split_chains(&mut self, count: usize) {
        self.split_chains_consumed += count;
    }

    pub(crate) fn consumed_fragment_rows(&mut self, count: usize) {
        self.fragment_rows_consumed += count;
    }

    pub(crate) fn consumed_vertex_rows(&mut self, count: usize) {
        self.vertex_rows_consumed += count;
    }

    pub(crate) fn consumed_persistent_name_rows(&mut self, count: usize) {
        self.persistent_name_rows_consumed += count;
    }

    pub(crate) fn consumed_replay_parity_rows(&mut self, count: usize) {
        self.replay_parity_rows_consumed += count;
    }

    pub(crate) fn consumed_stage_index_rows(&mut self, count: usize) {
        self.stage_index_rows_consumed += count;
    }

    pub(crate) fn rejected_foreign_receipt(&mut self) {
        self.foreign_receipts_rejected += 1;
    }

    pub(crate) fn rejected_missing_receipt(&mut self) {
        self.missing_receipts_rejected += 1;
    }

    pub(crate) fn rejected_non_receipt_evidence(&mut self) {
        self.non_receipt_evidence_rejected += 1;
    }

    pub fn receipts_consumed(self) -> usize {
        self.receipts_consumed
    }

    pub fn split_chains_consumed(self) -> usize {
        self.split_chains_consumed
    }

    pub fn fragment_rows_consumed(self) -> usize {
        self.fragment_rows_consumed
    }

    pub fn vertex_rows_consumed(self) -> usize {
        self.vertex_rows_consumed
    }

    pub fn persistent_name_rows_consumed(self) -> usize {
        self.persistent_name_rows_consumed
    }

    pub fn replay_parity_rows_consumed(self) -> usize {
        self.replay_parity_rows_consumed
    }

    pub fn stage_index_rows_consumed(self) -> usize {
        self.stage_index_rows_consumed
    }

    pub fn foreign_receipts_rejected(self) -> usize {
        self.foreign_receipts_rejected
    }

    pub fn missing_receipts_rejected(self) -> usize {
        self.missing_receipts_rejected
    }

    pub fn non_receipt_evidence_rejected(self) -> usize {
        self.non_receipt_evidence_rejected
    }
}
