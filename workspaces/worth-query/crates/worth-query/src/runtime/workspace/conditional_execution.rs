use super::WorthQueryWorkspace;

impl WorthQueryWorkspace {
    pub fn deliver_conditional_authoritative_change<D: 'static, O: 'static, F: 'static>(
        &mut self,
        _domain: D,
        _operation: O,
        _family: F,
        location: &worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        dependency_ordinal: usize,
        request: worth_runtime_bridge::facade::RelationalCommittedPatchRequest,
    ) -> Result<
        worth_runtime_bridge::facade::CorrespondenceDeliveryOutcome,
        crate::domain_installation::WorthQueryConditionalDeliveryDenial,
    > {
        let node = self
            .runtime
            .conditional_nodes::<D, O, F>()
            .into_iter()
            .find(|node| node.lowering.location() == location)
            .ok_or(
                crate::domain_installation::WorthQueryConditionalDeliveryDenial::NodeNotInstalled,
            )?;
        self.runtime.deliver_conditional_authoritative_change(
            node.as_ref(),
            dependency_ordinal,
            request,
        )
    }

    pub(crate) fn execute_installed_conditional(
        &mut self,
        lowering: &std::sync::Arc<worth_runtime_bridge::facade::BridgeInstalledConditionalLowering>,
        query_binding_identity: &str,
        query_capability_identity: u64,
        snapshot_identity: &str,
        bridge_snapshot_identity: Option<&worth_runtime_bridge::facade::TruthSnapshotIdentity>,
        execution_identity: &str,
        attempt: u64,
        context: &mut dyn std::any::Any,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeConditionalDecisionEvidence,
        (
            worth_runtime_bridge::facade::BridgeConditionalDenialKind,
            String,
            worth_signal::facade::SignalConditionalDecisionCounters,
            usize,
        ),
    > {
        self.runtime.execute_conditional(
            lowering,
            query_binding_identity,
            query_capability_identity,
            snapshot_identity,
            bridge_snapshot_identity,
            execution_identity,
            attempt,
            context,
        )
    }
}
