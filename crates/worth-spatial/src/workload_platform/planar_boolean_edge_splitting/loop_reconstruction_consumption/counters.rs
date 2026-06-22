#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionSplitConsumptionCounters {
    downstream_gate_consumed: usize,
    receipts_consumed: usize,
    stage_index_rows_consumed: usize,
    missing_authority_rejected: usize,
}

impl PlanarBooleanLoopReconstructionSplitConsumptionCounters {
    pub(crate) fn consumed_downstream_gate(&mut self) {
        self.downstream_gate_consumed += 1;
    }

    pub(crate) fn consumed_receipts(&mut self, count: usize) {
        self.receipts_consumed += count;
    }

    pub(crate) fn consumed_stage_index_rows(&mut self, count: usize) {
        self.stage_index_rows_consumed += count;
    }

    pub(crate) fn rejected_missing_authority(&mut self) {
        self.missing_authority_rejected += 1;
    }

    pub fn downstream_gate_consumed(self) -> usize {
        self.downstream_gate_consumed
    }

    pub fn receipts_consumed(self) -> usize {
        self.receipts_consumed
    }

    pub fn stage_index_rows_consumed(self) -> usize {
        self.stage_index_rows_consumed
    }

    pub fn missing_authority_rejected(self) -> usize {
        self.missing_authority_rejected
    }
}
