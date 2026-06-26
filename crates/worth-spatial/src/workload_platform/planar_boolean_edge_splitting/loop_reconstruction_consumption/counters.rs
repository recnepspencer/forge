#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionSplitConsumptionCounters {
    downstream_gate_consumed: usize,
    receipts_consumed: usize,
    spatial_lookup_products_consumed: usize,
    spatial_lookup_indexed_lookups: usize,
    spatial_lookup_raw_row_scans: usize,
    missing_authority_rejected: usize,
}

impl PlanarBooleanLoopReconstructionSplitConsumptionCounters {
    pub(crate) fn consumed_downstream_gate(&mut self) {
        self.downstream_gate_consumed += 1;
    }

    pub(crate) fn consumed_receipts(&mut self, count: usize) {
        self.receipts_consumed += count;
    }

    pub(crate) fn consumed_spatial_lookup_product(
        &mut self,
        indexed_lookups: usize,
        raw_row_scans: usize,
    ) {
        self.spatial_lookup_products_consumed += 1;
        self.spatial_lookup_indexed_lookups += indexed_lookups;
        self.spatial_lookup_raw_row_scans += raw_row_scans;
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

    pub fn spatial_lookup_products_consumed(self) -> usize {
        self.spatial_lookup_products_consumed
    }

    pub fn spatial_lookup_indexed_lookups(self) -> usize {
        self.spatial_lookup_indexed_lookups
    }

    pub fn spatial_lookup_raw_row_scans(self) -> usize {
        self.spatial_lookup_raw_row_scans
    }

    pub fn missing_authority_rejected(self) -> usize {
        self.missing_authority_rejected
    }
}
