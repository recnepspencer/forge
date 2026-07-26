use worth_query_declaration::facade::domain_computation::{
    WorthQueryPartialEffectPosture, WorthQueryRetainedProgressPosture,
    WorthQueryYieldedStatePosture,
};

use super::WorthQueryExecutionResourceEnvelope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledYieldContract {
    retained_bytes_ceiling: u64,
    partial_effect_posture: WorthQueryPartialEffectPosture,
    retained_progress_posture: WorthQueryRetainedProgressPosture,
}

impl WorthQueryInstalledYieldContract {
    pub(super) fn derive(envelope: &WorthQueryExecutionResourceEnvelope) -> Option<Self> {
        (envelope.yielded_state_posture() == WorthQueryYieldedStatePosture::ProviderCheckpoint)
            .then(|| Self {
                retained_bytes_ceiling: envelope.resource_ceiling(
                    worth_query_declaration::facade::domain_computation::WorthQueryResourceDimension::RetainedBytes,
                ),
                partial_effect_posture: envelope.partial_effect_posture(),
                retained_progress_posture: envelope.retained_progress_posture(),
            })
    }

    pub const fn retained_bytes_ceiling(&self) -> u64 {
        self.retained_bytes_ceiling
    }

    pub const fn partial_effect_posture(&self) -> WorthQueryPartialEffectPosture {
        self.partial_effect_posture
    }

    pub const fn partial_effects_may_remain(&self) -> bool {
        matches!(
            self.partial_effect_posture,
            WorthQueryPartialEffectPosture::PartialEffectsMayRemain
        )
    }

    pub const fn retained_progress_posture(&self) -> WorthQueryRetainedProgressPosture {
        self.retained_progress_posture
    }
}

impl WorthQueryExecutionResourceEnvelope {
    pub fn yield_contract(&self) -> Option<WorthQueryInstalledYieldContract> {
        WorthQueryInstalledYieldContract::derive(self)
    }
}

#[cfg(test)]
mod tests {
    use worth_query_declaration::facade::domain_computation::{
        WorthQueryCancellationSafePointFamily, WorthQueryExecutionMode,
        WorthQueryRetainedProgressPosture, WorthQueryYieldedStatePosture,
    };

    use super::super::{
        WorthQueryExecutionAccessProductFamily, WorthQueryExecutionAllocatorFamily,
        WorthQueryExecutionProviderFamily, WorthQueryExecutionProviderRequirements,
        WorthQueryExecutionResourceContract, WorthQueryExecutionResourceEnvelope,
        WorthQueryExecutionStrategyContract, WorthQueryExecutionStrategyName,
    };

    #[test]
    fn provider_checkpoint_requires_retained_attempt_capacity() {
        let envelope = WorthQueryExecutionResourceEnvelope::bounded(
            8,
            8,
            WorthQueryExecutionMode::Asynchronous,
            WorthQueryCancellationSafePointFamily::new("yield-step").unwrap(),
        )
        .with_yielded_state_posture(WorthQueryYieldedStatePosture::ProviderCheckpoint);

        assert_eq!(
            WorthQueryExecutionResourceContract::declared([strategy(envelope)]),
            Err("provider-checkpoint-requires-retained-attempt-capacity")
        );
    }

    #[test]
    fn installed_yield_contract_carries_exact_retention_and_effect_posture() {
        let envelope = WorthQueryExecutionResourceEnvelope::bounded(
            8,
            13,
            WorthQueryExecutionMode::Asynchronous,
            WorthQueryCancellationSafePointFamily::new("yield-step").unwrap(),
        )
        .with_yielded_state_posture(WorthQueryYieldedStatePosture::ProviderCheckpoint)
        .with_retained_progress_posture(WorthQueryRetainedProgressPosture::RetainAttemptCapacity);

        let contract = envelope.yield_contract().unwrap();
        assert_eq!(contract.retained_bytes_ceiling(), 13);
        assert_eq!(
            contract.retained_progress_posture(),
            WorthQueryRetainedProgressPosture::RetainAttemptCapacity
        );
        assert!(!contract.partial_effects_may_remain());
    }

    fn strategy(
        envelope: WorthQueryExecutionResourceEnvelope,
    ) -> WorthQueryExecutionStrategyContract {
        WorthQueryExecutionStrategyContract::new(
            WorthQueryExecutionStrategyName::new("yield").unwrap(),
            envelope,
            WorthQueryExecutionProviderRequirements::new(
                WorthQueryExecutionProviderFamily::new("provider").unwrap(),
                WorthQueryExecutionAccessProductFamily::new("access").unwrap(),
                WorthQueryExecutionAllocatorFamily::new("allocator").unwrap(),
            ),
        )
    }
}
