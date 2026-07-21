#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiPlanLoweringCounters {
    staged_node_input_count: usize,
    query_binding_input_count: usize,
    reconciliation_receipt_input_count: usize,
    component_hook_input_count: usize,
    rejected_component_hook_count: usize,
    readiness_verification_count: usize,
    epoch_verification_count: usize,
    source_parse_count: usize,
    registry_string_lookup_count: usize,
}

impl WorthUiPlanLoweringCounters {
    pub(crate) fn record_staged_node_input(&mut self) {
        self.staged_node_input_count += 1;
    }

    pub(crate) fn record_query_binding_input(&mut self) {
        self.query_binding_input_count += 1;
    }

    pub(crate) fn record_reconciliation_receipts(&mut self, count: usize) {
        self.reconciliation_receipt_input_count += count;
    }

    pub(crate) fn record_component_hook_input(&mut self) {
        self.component_hook_input_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_rejected_component_hook(&mut self) {
        self.rejected_component_hook_count += 1;
    }

    pub(crate) fn record_readiness_verification(&mut self) {
        self.readiness_verification_count += 1;
    }

    pub(crate) fn record_epoch_verification(&mut self) {
        self.epoch_verification_count += 1;
    }

    pub fn staged_node_input_count(self) -> usize {
        self.staged_node_input_count
    }

    pub fn query_binding_input_count(self) -> usize {
        self.query_binding_input_count
    }

    pub fn reconciliation_receipt_input_count(self) -> usize {
        self.reconciliation_receipt_input_count
    }

    pub fn component_hook_input_count(self) -> usize {
        self.component_hook_input_count
    }

    pub fn rejected_component_hook_count(self) -> usize {
        self.rejected_component_hook_count
    }

    pub fn readiness_verification_count(self) -> usize {
        self.readiness_verification_count
    }

    pub fn epoch_verification_count(self) -> usize {
        self.epoch_verification_count
    }

    pub fn source_parse_count(self) -> usize {
        self.source_parse_count
    }

    pub fn registry_string_lookup_count(self) -> usize {
        self.registry_string_lookup_count
    }
}
