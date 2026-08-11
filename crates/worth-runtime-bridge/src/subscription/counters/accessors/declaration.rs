use super::super::BridgeSubscriptionCounters;

impl BridgeSubscriptionCounters {
    pub fn family_registry_freeze_count(&self) -> usize {
        self.values.family_registry_freeze_count
    }

    pub fn family_count(&self) -> usize {
        self.values.family_count
    }

    pub fn family_supported_slice_kind_count(&self) -> usize {
        self.values.family_supported_slice_kind_count
    }

    pub fn family_lookup_count(&self) -> usize {
        self.values.family_lookup_count
    }

    pub fn declaration_count(&self) -> usize {
        self.values.declaration_count
    }

    pub fn declaration_input_slice_intent_count(&self) -> usize {
        self.values.declaration_input_slice_intent_count
    }

    pub fn declaration_normalized_slice_intent_count(&self) -> usize {
        self.values.declaration_normalized_slice_intent_count
    }

    pub fn declaration_deduplicated_slice_intent_count(&self) -> usize {
        self.values.declaration_deduplicated_slice_intent_count
    }

    pub fn declaration_rejection_count(&self) -> usize {
        self.values.declaration_rejection_count
    }

    pub fn basis_request_count(&self) -> usize {
        self.values.basis_request_count
    }

    pub fn basis_binding_count(&self) -> usize {
        self.values.basis_binding_count
    }

    pub fn basis_rejection_count(&self) -> usize {
        self.values.basis_rejection_count
    }

    pub fn signal_strategy_selection_count(&self) -> usize {
        self.values.signal_strategy_selection_count
    }

    pub fn signal_strategy_rejection_count(&self) -> usize {
        self.values.signal_strategy_rejection_count
    }
}
