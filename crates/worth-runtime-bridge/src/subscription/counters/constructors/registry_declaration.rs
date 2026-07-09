use super::super::BridgeSubscriptionCounters;

impl BridgeSubscriptionCounters {
    pub fn from_frozen_registry(
        family_count: usize,
        family_supported_slice_kind_count: usize,
    ) -> Self {
        Self::new(
            1,
            family_count,
            family_supported_slice_kind_count,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        )
    }

    pub fn from_declaration(
        declaration_input_slice_intent_count: usize,
        declaration_normalized_slice_intent_count: usize,
        declaration_deduplicated_slice_intent_count: usize,
    ) -> Self {
        Self::new(
            0,
            0,
            0,
            1,
            1,
            declaration_input_slice_intent_count,
            declaration_normalized_slice_intent_count,
            declaration_deduplicated_slice_intent_count,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        )
    }

    pub fn from_rejection(
        declaration_input_slice_intent_count: usize,
        declaration_normalized_slice_intent_count: usize,
        declaration_deduplicated_slice_intent_count: usize,
    ) -> Self {
        Self::new(
            0,
            0,
            0,
            1,
            0,
            declaration_input_slice_intent_count,
            declaration_normalized_slice_intent_count,
            declaration_deduplicated_slice_intent_count,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        )
    }
}
