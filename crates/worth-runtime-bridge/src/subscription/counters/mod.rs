use std::sync::Arc;

use sha2::{Digest, Sha256};

mod accessors;
mod canonical_basis;
mod constructors;
mod values;

use canonical_basis::subscription_counters_canonical_basis;
use values::BridgeSubscriptionCounterValues;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCounters {
    values: BridgeSubscriptionCounterValues,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCounters {
    pub fn zero() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues::default())
    }

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
        Self::from_values(BridgeSubscriptionCounterValues {
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
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    fn from_values(values: BridgeSubscriptionCounterValues) -> Self {
        let canonical_basis = subscription_counters_canonical_basis(&values);
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            values,
            canonical_basis,
            digest: Arc::from(format!("bridge-subscription-counters:sha256:{digest:x}")),
        }
    }
}

#[cfg(test)]
mod tests;
