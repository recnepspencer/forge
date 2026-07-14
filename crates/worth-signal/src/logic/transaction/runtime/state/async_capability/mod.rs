mod admission;
mod declaration;
mod equivalence;
mod gate;
mod hierarchy;
mod hierarchy_history;
mod history;
mod keyed_equivalence;
mod keyed_history;

use crate::data::async_node::{
    AsyncCapableNode, AsyncKeyedNodeCapabilityBinding, AsyncNodeAdmissionClassification,
    AsyncNodeHistoricalParityDenialClass, DeniedAsyncNodeHistoricalParity,
    LoweredAsyncNodeCapabilityBundle,
};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;

use super::runtime_state::SignalRuntime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AsyncAdmissionMode {
    NewLineage,
    Refresh,
}

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn ensure_live_async_node_owner(
        &mut self,
        node: NodeId,
        action: &str,
    ) -> Result<(), SignalError> {
        if self.graph.is_alive(node) {
            return Ok(());
        }
        self.telemetry.resource.resource_non_live_owner_denial_count += 1;
        Err(SignalError::invalid_input(format!(
            "cannot {action} for non-live owner {node}"
        )))
    }

    fn with_async_classification_performance(
        &self,
        classification: AsyncNodeAdmissionClassification,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> AsyncNodeAdmissionClassification {
        AsyncNodeAdmissionClassification::new(
            classification.node(),
            classification.node_state(),
            classification.lifecycle_class(),
            classification.condition().clone(),
            classification.class(),
            classification.condition_block_class(),
            classification.dirty_aspects(),
            classification.dirty_partition_scope_count(),
            classification.contract_partition_scope_count(),
            classification.max_dependency_delta(),
            classification.previous_value_reference().cloned(),
            classification.decision_digest().clone(),
            performance,
        )
    }

    fn is_interior_async_gate(&self, node: NodeId) -> Result<bool, SignalError> {
        Ok(!self.graph.dependencies_of(node)?.is_empty()
            && !self.graph.subscribers_of(node)?.is_empty())
    }

    fn current_async_capability_bundle(
        &mut self,
        handle: &AsyncCapableNode,
        branch_restore_width: u32,
    ) -> Result<LoweredAsyncNodeCapabilityBundle, DeniedAsyncNodeHistoricalParity> {
        let performance = ResourceBoundaryPerformanceEnvelope::async_node_historical_parity(
            branch_restore_width,
            0,
            1,
        );
        if !self.graph.is_alive(handle.node()) {
            self.telemetry
                .resource
                .async_node_capability_broad_scan_denial_count += 1;
            return Err(DeniedAsyncNodeHistoricalParity::new(
                handle.node(),
                AsyncNodeHistoricalParityDenialClass::NonLiveOwner,
                handle.registry_digest().clone(),
                None,
                handle.bundle_digest().clone(),
                None,
                handle.payload_contract_digest().clone(),
                None,
                performance,
            ));
        }

        let Some(bundle) = self.async_node_capability_bundle_for_node(handle.node()) else {
            self.telemetry
                .resource
                .async_node_capability_broad_scan_denial_count += 1;
            return Err(DeniedAsyncNodeHistoricalParity::new(
                handle.node(),
                AsyncNodeHistoricalParityDenialClass::UndeclaredCapability,
                handle.registry_digest().clone(),
                None,
                handle.bundle_digest().clone(),
                None,
                handle.payload_contract_digest().clone(),
                None,
                performance,
            ));
        };

        if handle.registry_digest() != bundle.registry_digest() {
            self.telemetry
                .resource
                .async_node_capability_broad_scan_denial_count += 1;
            return Err(DeniedAsyncNodeHistoricalParity::new(
                handle.node(),
                AsyncNodeHistoricalParityDenialClass::RegistryDigestDrift,
                handle.registry_digest().clone(),
                Some(bundle.registry_digest().clone()),
                handle.bundle_digest().clone(),
                Some(bundle.bundle_digest().clone()),
                handle.payload_contract_digest().clone(),
                Some(bundle.payload_contract_digest().clone()),
                performance,
            ));
        }
        if handle.bundle_digest() != bundle.bundle_digest() {
            self.telemetry
                .resource
                .async_node_capability_broad_scan_denial_count += 1;
            return Err(DeniedAsyncNodeHistoricalParity::new(
                handle.node(),
                AsyncNodeHistoricalParityDenialClass::BundleDigestDrift,
                handle.registry_digest().clone(),
                Some(bundle.registry_digest().clone()),
                handle.bundle_digest().clone(),
                Some(bundle.bundle_digest().clone()),
                handle.payload_contract_digest().clone(),
                Some(bundle.payload_contract_digest().clone()),
                performance,
            ));
        }
        if handle.payload_contract_digest() != bundle.payload_contract_digest() {
            self.telemetry
                .resource
                .async_node_capability_broad_scan_denial_count += 1;
            return Err(DeniedAsyncNodeHistoricalParity::new(
                handle.node(),
                AsyncNodeHistoricalParityDenialClass::PayloadContractDigestDrift,
                handle.registry_digest().clone(),
                Some(bundle.registry_digest().clone()),
                handle.bundle_digest().clone(),
                Some(bundle.bundle_digest().clone()),
                handle.payload_contract_digest().clone(),
                Some(bundle.payload_contract_digest().clone()),
                performance,
            ));
        }

        Ok(bundle)
    }

    fn async_keyed_binding_matches_handle(
        binding: &AsyncKeyedNodeCapabilityBinding,
        handle: &AsyncCapableNode,
    ) -> bool {
        binding.node() == handle.node()
            && binding.registry_digest() == handle.registry_digest()
            && binding.bundle_digest() == handle.bundle_digest()
            && binding.payload_contract_digest() == handle.payload_contract_digest()
    }
}
