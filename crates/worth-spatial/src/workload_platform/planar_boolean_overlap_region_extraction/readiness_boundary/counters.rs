#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanOverlapReadinessLoopLedgerBindingCounters {
    readiness_consumers_consumed: usize,
    loop_ledger_receipts_consumed: usize,
    provenance_mismatches_rejected: usize,
    missing_authority_rejected: usize,
}

impl PlanarBooleanOverlapReadinessLoopLedgerBindingCounters {
    pub(crate) fn consumed_readiness_consumer(&mut self) {
        self.readiness_consumers_consumed += 1;
    }

    pub(crate) fn consumed_loop_ledger_receipt(&mut self) {
        self.loop_ledger_receipts_consumed += 1;
    }

    pub(crate) fn rejected_provenance_mismatch(&mut self) {
        self.provenance_mismatches_rejected += 1;
    }

    pub(crate) fn rejected_missing_authority(&mut self) {
        self.missing_authority_rejected += 1;
    }

    pub fn readiness_consumers_consumed(self) -> usize {
        self.readiness_consumers_consumed
    }

    pub fn loop_ledger_receipts_consumed(self) -> usize {
        self.loop_ledger_receipts_consumed
    }

    pub fn provenance_mismatches_rejected(self) -> usize {
        self.provenance_mismatches_rejected
    }

    pub fn missing_authority_rejected(self) -> usize {
        self.missing_authority_rejected
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionExtractionRequestCounters {
    readiness_bindings_consumed: usize,
    loop_ledger_rows_bound: usize,
}

impl PlanarBooleanOverlapRegionExtractionRequestCounters {
    pub(crate) fn consumed_readiness_binding(&mut self) {
        self.readiness_bindings_consumed += 1;
    }

    pub(crate) fn consumed_loop_ledger_rows(&mut self, count: usize) {
        self.loop_ledger_rows_bound += count;
    }

    pub fn readiness_bindings_consumed(self) -> usize {
        self.readiness_bindings_consumed
    }

    pub fn loop_ledger_rows_bound(self) -> usize {
        self.loop_ledger_rows_bound
    }
}
