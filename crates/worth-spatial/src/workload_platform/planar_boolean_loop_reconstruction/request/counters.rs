#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionRequestCounters {
    split_consumption_products_consumed: usize,
    split_chain_rows_bound: usize,
    missing_authority_rejected: usize,
}

impl PlanarBooleanLoopReconstructionRequestCounters {
    pub(crate) fn consumed_split_consumption_product(&mut self) {
        self.split_consumption_products_consumed += 1;
    }

    pub(crate) fn consumed_split_chain_rows(&mut self, count: usize) {
        self.split_chain_rows_bound += count;
    }

    pub(crate) fn rejected_missing_authority(&mut self) {
        self.missing_authority_rejected += 1;
    }

    pub fn split_consumption_products_consumed(self) -> usize {
        self.split_consumption_products_consumed
    }

    pub fn split_chain_rows_bound(self) -> usize {
        self.split_chain_rows_bound
    }

    pub fn missing_authority_rejected(self) -> usize {
        self.missing_authority_rejected
    }
}
