use std::sync::Arc;

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCounters {
    family_registry_freeze_count: usize,
    family_count: usize,
    family_supported_slice_kind_count: usize,
    family_lookup_count: usize,
    declaration_count: usize,
    declaration_input_slice_intent_count: usize,
    declaration_normalized_slice_intent_count: usize,
    declaration_deduplicated_slice_intent_count: usize,
    declaration_rejection_count: usize,
    basis_request_count: usize,
    basis_binding_count: usize,
    basis_rejection_count: usize,
    signal_strategy_selection_count: usize,
    signal_strategy_rejection_count: usize,
    admitted_subscription_count: usize,
    lifecycle_record_count: usize,
    replay_reconstruction_count: usize,
    replay_mismatch_count: usize,
    diagnostics_bundle_count: usize,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCounters {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        family_registry_freeze_count: usize,
        family_count: usize,
        family_supported_slice_kind_count: usize,
        family_lookup_count: usize,
        declaration_count: usize,
        declaration_input_slice_intent_count: usize,
        declaration_normalized_slice_intent_count: usize,
        declaration_deduplicated_slice_intent_count: usize,
        declaration_rejection_count: usize,
        basis_request_count: usize,
        basis_binding_count: usize,
        basis_rejection_count: usize,
        signal_strategy_selection_count: usize,
        signal_strategy_rejection_count: usize,
        admitted_subscription_count: usize,
        lifecycle_record_count: usize,
        replay_reconstruction_count: usize,
        replay_mismatch_count: usize,
        diagnostics_bundle_count: usize,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            concat!(
                "bridge-subscription-counters|family-registry-freeze-count:{}|",
                "family-count:{}|family-supported-slice-kind-count:{}|family-lookup-count:{}|",
                "declaration-count:{}|declaration-input-slice-intent-count:{}|",
                "declaration-normalized-slice-intent-count:{}|",
                "declaration-deduplicated-slice-intent-count:{}|",
                "declaration-rejection-count:{}|basis-request-count:{}|basis-binding-count:{}|",
                "basis-rejection-count:{}|signal-strategy-selection-count:{}|",
                "signal-strategy-rejection-count:{}|admitted-subscription-count:{}|",
                "lifecycle-record-count:{}|replay-reconstruction-count:{}|",
                "replay-mismatch-count:{}|diagnostics-bundle-count:{}"
            ),
            family_registry_freeze_count,
            family_count,
            family_supported_slice_kind_count,
            family_lookup_count,
            declaration_count,
            declaration_input_slice_intent_count,
            declaration_normalized_slice_intent_count,
            declaration_deduplicated_slice_intent_count,
            declaration_rejection_count,
            basis_request_count,
            basis_binding_count,
            basis_rejection_count,
            signal_strategy_selection_count,
            signal_strategy_rejection_count,
            admitted_subscription_count,
            lifecycle_record_count,
            replay_reconstruction_count,
            replay_mismatch_count,
            diagnostics_bundle_count,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            family_registry_freeze_count,
            family_count,
            family_supported_slice_kind_count,
            family_lookup_count,
            declaration_count,
            declaration_input_slice_intent_count,
            declaration_normalized_slice_intent_count,
            declaration_deduplicated_slice_intent_count,
            declaration_rejection_count,
            basis_request_count,
            basis_binding_count,
            basis_rejection_count,
            signal_strategy_selection_count,
            signal_strategy_rejection_count,
            admitted_subscription_count,
            lifecycle_record_count,
            replay_reconstruction_count,
            replay_mismatch_count,
            diagnostics_bundle_count,
            canonical_basis,
            digest: Arc::from(format!("bridge-subscription-counters:sha256:{digest:x}")),
        }
    }

    pub fn family_registry_freeze_count(&self) -> usize {
        self.family_registry_freeze_count
    }

    pub fn family_count(&self) -> usize {
        self.family_count
    }

    pub fn family_supported_slice_kind_count(&self) -> usize {
        self.family_supported_slice_kind_count
    }

    pub fn family_lookup_count(&self) -> usize {
        self.family_lookup_count
    }

    pub fn declaration_count(&self) -> usize {
        self.declaration_count
    }

    pub fn declaration_input_slice_intent_count(&self) -> usize {
        self.declaration_input_slice_intent_count
    }

    pub fn declaration_normalized_slice_intent_count(&self) -> usize {
        self.declaration_normalized_slice_intent_count
    }

    pub fn declaration_deduplicated_slice_intent_count(&self) -> usize {
        self.declaration_deduplicated_slice_intent_count
    }

    pub fn declaration_rejection_count(&self) -> usize {
        self.declaration_rejection_count
    }

    pub fn basis_request_count(&self) -> usize {
        self.basis_request_count
    }

    pub fn basis_binding_count(&self) -> usize {
        self.basis_binding_count
    }

    pub fn basis_rejection_count(&self) -> usize {
        self.basis_rejection_count
    }

    pub fn signal_strategy_selection_count(&self) -> usize {
        self.signal_strategy_selection_count
    }

    pub fn signal_strategy_rejection_count(&self) -> usize {
        self.signal_strategy_rejection_count
    }

    pub fn admitted_subscription_count(&self) -> usize {
        self.admitted_subscription_count
    }

    pub fn lifecycle_record_count(&self) -> usize {
        self.lifecycle_record_count
    }

    pub fn replay_reconstruction_count(&self) -> usize {
        self.replay_reconstruction_count
    }

    pub fn replay_mismatch_count(&self) -> usize {
        self.replay_mismatch_count
    }

    pub fn diagnostics_bundle_count(&self) -> usize {
        self.diagnostics_bundle_count
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    pub fn from_frozen_registry(family_count: usize, family_supported_slice_kind_count: usize) -> Self {
        Self::new(1, family_count, family_supported_slice_kind_count, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    }

    pub fn from_declaration(
        declaration_input_slice_intent_count: usize,
        declaration_normalized_slice_intent_count: usize,
        declaration_deduplicated_slice_intent_count: usize,
    ) -> Self {
        Self::new(0, 0, 0, 1, 1, declaration_input_slice_intent_count, declaration_normalized_slice_intent_count, declaration_deduplicated_slice_intent_count, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    }

    pub fn from_rejection(
        declaration_input_slice_intent_count: usize,
        declaration_normalized_slice_intent_count: usize,
        declaration_deduplicated_slice_intent_count: usize,
    ) -> Self {
        Self::new(0, 0, 0, 1, 0, declaration_input_slice_intent_count, declaration_normalized_slice_intent_count, declaration_deduplicated_slice_intent_count, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    }

    pub fn from_basis_binding() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
    }

    pub fn from_incompatible_basis_kind_rejection() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
    }

    pub fn from_basis_resolution_rejection() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
    }

    pub fn from_signal_strategy_descriptor() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
    }

    pub fn from_admitted_subscription() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0)
    }

    pub fn from_lifecycle_record() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
    }

    pub fn from_diagnostics_bundle() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
    }

    pub fn from_replay_reconstruction() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
    }

    pub fn from_replay_mismatch() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::BridgeSubscriptionCounters;

    #[test]
    fn incompatible_basis_rejection_counters_match_actual_work() {
        let counters = BridgeSubscriptionCounters::from_incompatible_basis_kind_rejection();

        assert_eq!(counters.declaration_rejection_count(), 0);
        assert_eq!(counters.basis_request_count(), 1);
        assert_eq!(counters.basis_binding_count(), 0);
        assert_eq!(counters.basis_rejection_count(), 1);
        assert_eq!(counters.signal_strategy_selection_count(), 0);
        assert_eq!(counters.signal_strategy_rejection_count(), 0);
    }
}
