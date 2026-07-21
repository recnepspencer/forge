#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiActivationStagingCounters {
    verified_input_count: usize,
    digest_comparison_count: usize,
    staged_reconciliation_receipt_count: usize,
    staged_query_binding_count: usize,
    rejected_missing_input_count: usize,
    rejected_mismatched_input_count: usize,
    receipt_verification_count: usize,
    active_mutation_observed_count: usize,
}

impl WorthUiActivationStagingCounters {
    pub(crate) fn record_verified_input(&mut self) {
        self.verified_input_count += 1;
    }

    pub(crate) fn record_digest_comparison(&mut self) {
        self.digest_comparison_count += 1;
    }

    pub(crate) fn record_staged_reconciliation_receipts(&mut self, count: usize) {
        self.staged_reconciliation_receipt_count += count;
    }

    pub(crate) fn record_staged_query_bindings(&mut self, count: usize) {
        self.staged_query_binding_count += count;
    }

    pub(crate) fn record_rejected_missing_input(&mut self) {
        self.rejected_missing_input_count += 1;
    }

    pub(crate) fn record_rejected_mismatched_input(&mut self) {
        self.rejected_mismatched_input_count += 1;
    }

    pub(crate) fn record_receipt_verification(&mut self) {
        self.receipt_verification_count += 1;
    }

    pub(crate) fn record_active_mutation_observed(&mut self) {
        self.active_mutation_observed_count += 1;
    }

    pub fn verified_input_count(&self) -> usize {
        self.verified_input_count
    }

    pub fn digest_comparison_count(&self) -> usize {
        self.digest_comparison_count
    }

    pub fn staged_reconciliation_receipt_count(&self) -> usize {
        self.staged_reconciliation_receipt_count
    }

    pub fn staged_query_binding_count(&self) -> usize {
        self.staged_query_binding_count
    }

    pub fn rejected_missing_input_count(&self) -> usize {
        self.rejected_missing_input_count
    }

    pub fn rejected_mismatched_input_count(&self) -> usize {
        self.rejected_mismatched_input_count
    }

    pub fn receipt_verification_count(&self) -> usize {
        self.receipt_verification_count
    }

    pub fn active_mutation_observed_count(&self) -> usize {
        self.active_mutation_observed_count
    }
}
