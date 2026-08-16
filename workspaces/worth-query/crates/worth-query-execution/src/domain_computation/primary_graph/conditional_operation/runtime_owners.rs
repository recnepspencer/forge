use super::{
    installation::ConditionalClockLease,
    lifecycle::{
        WorthQueryConditionalOperationRegistry, WorthQueryPreparedConditionalRuntimeBinding,
    },
    publication::ConditionalRuntimeAffinity,
    signal_decision_reentry::WorthQueryConditionalTruthBasis,
    WorthQueryConditionalRuntimeInstallationDenial,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

/// Unwind-safe temporary ownership of the two mutable conditional runtime
/// roots. Drop always restores both roots unless a prepared successor Bridge
/// is explicitly committed.
pub(super) struct ConditionalRuntimeOwners<'runtime, Schema> {
    runtime: &'runtime mut WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    registry: Option<WorthQueryConditionalOperationRegistry<Schema>>,
    bridge: Option<worth_runtime_bridge::facade::BridgeOwnedSignalRuntime>,
}

impl<'runtime, Schema> ConditionalRuntimeOwners<'runtime, Schema> {
    pub(super) fn take(
        runtime: &'runtime mut WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    ) -> Self {
        let registry = std::mem::take(
            runtime
                .conditional_operations
                .get_mut()
                .unwrap_or_else(std::sync::PoisonError::into_inner),
        );
        let bridge = runtime.bridge.take_conditional();
        Self {
            runtime,
            registry: Some(registry),
            bridge: Some(bridge),
        }
    }

    pub(super) fn binding_count(&self) -> usize {
        self.registry
            .as_ref()
            .expect("registry owner is retained")
            .len()
    }

    pub(super) fn retained_resource_counts(
        &self,
    ) -> super::lifecycle::WorthQueryConditionalRetainedResourceCounts {
        self.registry
            .as_ref()
            .expect("registry owner is retained")
            .retained_resource_counts()
    }

    pub(super) fn reconstruction_work(
        &self,
    ) -> super::temporal_reconstruction::WorthQueryTemporalReconstructionWork {
        self.registry
            .as_ref()
            .expect("registry owner is retained")
            .reconstruction_work()
    }

    pub(super) fn fresh_bridge(
        &self,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeOwnedSignalRuntime,
        worth_runtime_bridge::facade::BridgeConditionalDenial,
    > {
        self.bridge
            .as_ref()
            .expect("Bridge owner is retained")
            .successor_installation_runtime()
    }

    pub(super) fn prepare_reinstallation(
        &self,
        successor: &mut worth_runtime_bridge::facade::BridgeOwnedSignalRuntime,
        affinity: &ConditionalRuntimeAffinity,
    ) -> Result<
        std::collections::BTreeMap<String, WorthQueryPreparedConditionalRuntimeBinding>,
        WorthQueryConditionalRuntimeInstallationDenial,
    > {
        self.registry
            .as_ref()
            .expect("registry owner is retained")
            .prepare_derived_runtime_reinstallation(
                self.runtime,
                successor,
                &self.runtime.primary_graph_authority,
                affinity,
            )
    }

    pub(super) fn reconcile_prepared_reinstallation(
        &mut self,
        successor: &mut worth_runtime_bridge::facade::BridgeOwnedSignalRuntime,
        prepared: &mut std::collections::BTreeMap<
            String,
            WorthQueryPreparedConditionalRuntimeBinding,
        >,
    ) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial> {
        self.registry
            .as_mut()
            .expect("registry owner is retained")
            .reconcile_prepared_runtime_reinstallation(successor, prepared)
    }

    pub(super) fn commit_reinstallation(
        &mut self,
        successor: worth_runtime_bridge::facade::BridgeOwnedSignalRuntime,
        prepared: std::collections::BTreeMap<String, WorthQueryPreparedConditionalRuntimeBinding>,
    ) {
        self.registry
            .as_mut()
            .expect("registry owner is retained")
            .apply_derived_runtime_reinstallation(prepared);
        self.registry
            .as_ref()
            .expect("registry owner is retained")
            .synchronize_commit_routes(self.runtime);
        self.bridge = Some(successor);
    }

    pub(super) fn clear_maintenance_failure(&self) {
        self.runtime
            .primary_provider
            .clear_conditional_maintenance_failure();
    }

    pub(super) fn advance_granular_invalidation_generation(&self) {
        self.runtime
            .granular_invalidation
            .advance_runtime_generation();
    }

    pub(super) fn granular_invalidation_installation(
        &self,
    ) -> super::super::WorthQueryGranularInvalidationInstallation {
        self.runtime.granular_invalidation.current()
    }

    pub(super) fn observe_clock(
        &mut self,
        identity: &str,
        lease: &std::sync::Arc<ConditionalClockLease>,
        truth: &WorthQueryConditionalTruthBasis,
    ) -> Option<super::clock_observation::ErasedClockObservationOutcome> {
        self.registry
            .as_mut()
            .expect("registry owner is retained")
            .observe_clock(
                identity,
                lease,
                self.bridge.as_mut().expect("Bridge owner is retained"),
                self.runtime,
                truth,
            )
    }
}

impl<Schema> Drop for ConditionalRuntimeOwners<'_, Schema> {
    fn drop(&mut self) {
        if let Some(bridge) = self.bridge.take() {
            self.runtime.bridge.restore_conditional(bridge);
        }
        if let Some(registry) = self.registry.take() {
            *self
                .runtime
                .conditional_operations
                .get_mut()
                .unwrap_or_else(std::sync::PoisonError::into_inner) = registry;
        }
    }
}
