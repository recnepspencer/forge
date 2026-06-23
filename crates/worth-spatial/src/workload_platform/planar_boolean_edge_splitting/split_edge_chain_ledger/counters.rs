#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanSplitEdgeChainLedgerCounters {
    ledger_chains_emitted: usize,
    fragment_rows_consumed: usize,
    overlap_chains_consumed: usize,
    persistent_name_rows_bound: usize,
    decision_rows_bound: usize,
    validation_receipts_consumed: usize,
    foreign_product_denials: usize,
    missing_validation_denials: usize,
    missing_persistent_name_denials: usize,
    missing_decision_log_denials: usize,
    duplicate_chain_identity_denials: usize,
    downstream_identities_emitted: usize,
}

impl PlanarBooleanSplitEdgeChainLedgerCounters {
    pub(crate) fn emitted_chain(&mut self) {
        self.ledger_chains_emitted += 1;
    }
    pub(crate) fn consumed_fragment(&mut self) {
        self.fragment_rows_consumed += 1;
    }
    pub(crate) fn consumed_overlap_chain(&mut self) {
        self.overlap_chains_consumed += 1;
    }
    pub(crate) fn bound_persistent_name(&mut self) {
        self.persistent_name_rows_bound += 1;
    }
    pub(crate) fn bound_decision(&mut self) {
        self.decision_rows_bound += 1;
    }
    pub(crate) fn consumed_validation_receipt(&mut self) {
        self.validation_receipts_consumed += 1;
    }
    pub(crate) fn rejected_foreign_product(&mut self) {
        self.foreign_product_denials += 1;
    }
    pub(crate) fn rejected_missing_validation(&mut self) {
        self.missing_validation_denials += 1;
    }
    pub(crate) fn rejected_missing_persistent_name(&mut self) {
        self.missing_persistent_name_denials += 1;
    }
    pub(crate) fn rejected_missing_decision_log(&mut self) {
        self.missing_decision_log_denials += 1;
    }
    pub(crate) fn rejected_duplicate_chain_identity(&mut self) {
        self.duplicate_chain_identity_denials += 1;
    }
    pub(crate) fn emitted_downstream_identity(&mut self) {
        self.downstream_identities_emitted += 1;
    }

    pub fn ledger_chains_emitted(self) -> usize {
        self.ledger_chains_emitted
    }
    pub fn fragment_rows_consumed(self) -> usize {
        self.fragment_rows_consumed
    }
    pub fn overlap_chains_consumed(self) -> usize {
        self.overlap_chains_consumed
    }
    pub fn persistent_name_rows_bound(self) -> usize {
        self.persistent_name_rows_bound
    }
    pub fn decision_rows_bound(self) -> usize {
        self.decision_rows_bound
    }
    pub fn validation_receipts_consumed(self) -> usize {
        self.validation_receipts_consumed
    }
    pub fn foreign_product_denials(self) -> usize {
        self.foreign_product_denials
    }
    pub fn missing_validation_denials(self) -> usize {
        self.missing_validation_denials
    }
    pub fn missing_persistent_name_denials(self) -> usize {
        self.missing_persistent_name_denials
    }
    pub fn missing_decision_log_denials(self) -> usize {
        self.missing_decision_log_denials
    }
    pub fn duplicate_chain_identity_denials(self) -> usize {
        self.duplicate_chain_identity_denials
    }
    pub fn downstream_identities_emitted(self) -> usize {
        self.downstream_identities_emitted
    }
}
