use crate::domain_installation::{
    WorthQueryDomainExecutionIndexRebuildReport, WorthQueryDomainHandleDenial,
    WorthQueryDomainInstallationLookupCounters, WorthQueryDomainInstallationReceipt,
    WorthQueryDomainRebindDenial, WorthQueryDomainRebindReceipt, WorthQueryDomainRebindRequest,
    WorthQueryInstalledDomainAuthorityWitness, WorthQueryInstalledDomainHandle,
    WorthQueryInstalledDomainOperation, WorthQueryInstalledDomainOperationLookupDenial,
    WorthQueryReboundDomainHandle,
};

use super::WorthQueryRuntime;

impl WorthQueryRuntime {
    pub(crate) fn deliver_conditional_authoritative_change(
        &mut self,
        node: &crate::domain_installation::WorthQueryInstalledConditionalNode,
        dependency_ordinal: usize,
        request: worth_runtime_bridge::facade::RelationalCommittedPatchRequest,
    ) -> Result<
        worth_runtime_bridge::facade::CorrespondenceDeliveryOutcome,
        crate::domain_installation::WorthQueryConditionalDeliveryDenial,
    > {
        let runtime = self.conditional_signal_runtime.as_mut().ok_or(
            crate::domain_installation::WorthQueryConditionalDeliveryDenial::NodeNotInstalled,
        )?;
        runtime
            .deliver_authoritative_change(&node.lowering, dependency_ordinal, request)
            .map_err(crate::domain_installation::WorthQueryConditionalDeliveryDenial::bridge)
    }

    pub(crate) fn conditional_nodes<D: 'static, O: 'static, F: 'static>(
        &self,
    ) -> Vec<std::sync::Arc<crate::domain_installation::WorthQueryInstalledConditionalNode>> {
        self.conditional_execution_registry
            .operation_nodes::<D, O, F>()
    }

    pub fn rebuild_conditional_execution_index(
        &mut self,
    ) -> crate::domain_installation::WorthQueryConditionalExecutionIndexRebuildReport {
        self.conditional_execution_registry
            .destroy_and_rebuild_index()
    }

    pub(crate) fn execute_conditional(
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
        let runtime = self.conditional_signal_runtime.as_mut().ok_or_else(|| {
            (
                worth_runtime_bridge::facade::BridgeConditionalDenialKind::StaleLowering,
                "installed conditional runtime is unavailable".to_string(),
                worth_signal::facade::SignalConditionalDecisionCounters::default(),
                0,
            )
        })?;
        runtime
            .execute(
                worth_runtime_bridge::facade::BridgeConditionalExecutionRequest {
                    lowering,
                    query_binding_identity,
                    query_capability_identity,
                    snapshot_identity,
                    bridge_snapshot_identity,
                    execution_identity,
                    attempt,
                },
                context,
            )
            .map_err(|denial| {
                (
                    denial.kind(),
                    denial.detail().to_string(),
                    denial.signal_counters(),
                    denial.semantic_observation_reads(),
                )
            })
    }

    pub(crate) fn workflow_parallel_admission_provider<D: 'static, O: 'static, F: 'static>(
        &self,
    ) -> Option<
        std::sync::Arc<
            crate::domain_installation::WorthQueryInstalledWorkflowParallelAdmissionProvider,
        >,
    > {
        self.workflow_parallel_admission_provider_registry
            .get::<D, O, F>()
    }

    pub(crate) fn workflow_stage_executor<D: 'static, O: 'static, F: 'static>(
        &self,
    ) -> Option<std::sync::Arc<crate::domain_installation::WorthQueryInstalledWorkflowStageExecutor>>
    {
        self.workflow_stage_executor_registry.get::<D, O, F>()
    }

    pub(crate) fn domain_operation_executor<D: 'static, O: 'static, F: 'static>(
        &self,
    ) -> Option<
        std::sync::Arc<crate::domain_installation::WorthQueryInstalledDomainOperationExecutor>,
    > {
        self.domain_operation_executor_registry.get::<D, O, F>()
    }

    pub(crate) fn consumer_support_profile(
        &self,
    ) -> &crate::domain_installation::WorthQueryConsumerSupportProfile {
        &self.consumer_support_profile
    }

    pub fn graph_participation<G: 'static>(
        &self,
        _marker: G,
    ) -> Result<
        crate::domain_installation::WorthQueryInstalledGraphParticipation<G>,
        crate::domain_installation::WorthQueryGraphParticipationLookupDenial,
    > {
        self.graph_participation_registry
            .get::<G>()
            .map(crate::domain_installation::WorthQueryInstalledGraphParticipation::new)
    }

    pub fn operation<D, O, F>(
        &self,
        handle: &WorthQueryInstalledDomainHandle<D>,
        _operation: O,
        _family: F,
    ) -> Result<
        WorthQueryInstalledDomainOperation<D, O, F>,
        WorthQueryInstalledDomainOperationLookupDenial,
    >
    where
        D: 'static,
        O: 'static,
        F: 'static,
    {
        self.resolve_installed_operation::<D, O, F>(handle)
    }

    pub(crate) fn resolve_installed_operation<D: 'static, O: 'static, F: 'static>(
        &self,
        handle: &WorthQueryInstalledDomainHandle<D>,
    ) -> Result<
        WorthQueryInstalledDomainOperation<D, O, F>,
        WorthQueryInstalledDomainOperationLookupDenial,
    > {
        self.validate_installed_domain_handle(handle)
            .map_err(WorthQueryInstalledDomainOperationLookupDenial::domain)?;
        let domain_marker = std::any::TypeId::of::<D>();
        let operation_marker = std::any::TypeId::of::<O>();
        let family_marker = std::any::TypeId::of::<F>();
        let (authority, workflow_graph) = self
            .installed_domain_execution_index()
            .domain_operation_authority(domain_marker, operation_marker, family_marker)
            .ok_or_else(WorthQueryInstalledDomainOperationLookupDenial::operation_not_installed)?;
        let graph_bindings = self
            .installed_domain_execution_index()
            .domain_operation_graph_bindings(domain_marker, operation_marker, family_marker)
            .to_vec();
        Ok(WorthQueryInstalledDomainOperation::mint(
            handle.authority_arc(),
            authority,
            workflow_graph,
            graph_bindings,
        ))
    }

    pub(crate) fn installed_graph_participation(
        &self,
        marker: std::any::TypeId,
    ) -> Result<
        std::sync::Arc<crate::domain_installation::WorthQueryInstalledGraphParticipationRecord>,
        crate::domain_installation::WorthQueryGraphParticipationLookupDenial,
    > {
        self.graph_participation_registry.get_by_marker(marker)
    }

    pub fn domain<D: 'static>(
        &self,
        _marker: D,
    ) -> Result<WorthQueryInstalledDomainHandle<D>, WorthQueryDomainHandleDenial> {
        self.domain_installation_registry.domain::<D>()
    }

    pub fn domain_installation_receipt<D: 'static>(
        &self,
        _marker: D,
    ) -> Option<&WorthQueryDomainInstallationReceipt> {
        self.domain_installation_registry.receipt::<D>()
    }

    pub fn domain_installation_receipts(
        &self,
    ) -> impl ExactSizeIterator<Item = &WorthQueryDomainInstallationReceipt> {
        self.domain_installation_registry.receipts()
    }

    pub fn validate_installed_domain_handle<D: 'static>(
        &self,
        handle: &WorthQueryInstalledDomainHandle<D>,
    ) -> Result<(), WorthQueryDomainHandleDenial> {
        self.domain_installation_registry.validate(handle)
    }

    pub(crate) fn validate_installed_domain_witness<D: 'static>(
        &self,
        witness: &WorthQueryInstalledDomainAuthorityWitness,
    ) -> Result<(), WorthQueryDomainHandleDenial> {
        self.domain_installation_registry
            .validate_authority::<D>(witness.authority())
    }

    pub(crate) fn validate_installed_domain_authority(
        &self,
        witness: &WorthQueryInstalledDomainAuthorityWitness,
    ) -> Result<(), WorthQueryDomainHandleDenial> {
        self.domain_installation_registry
            .validate_erased_authority(witness.authority())
    }

    pub fn domain_installation_lookup_counters(
        &self,
    ) -> WorthQueryDomainInstallationLookupCounters {
        self.domain_installation_registry.lookup_counters()
    }

    pub fn verify_domain_execution_index_rebuild(
        &self,
    ) -> WorthQueryDomainExecutionIndexRebuildReport {
        self.domain_installation_registry
            .rebuild_execution_index_report()
    }

    pub fn rebind_domain<D: 'static>(
        &self,
        request: WorthQueryDomainRebindRequest<D>,
    ) -> Result<WorthQueryReboundDomainHandle<D>, WorthQueryDomainRebindDenial> {
        let prior = request.into_prior();
        let current = self
            .domain_installation_registry
            .domain::<D>()
            .map_err(|_| WorthQueryDomainRebindDenial::domain_not_installed(&prior))?;
        if prior.package_identity() != current.package_identity() {
            return Err(WorthQueryDomainRebindDenial::package_meaning_changed(
                &prior,
                current.authority(),
            ));
        }
        let current_witness = current.authority_witness();
        let receipt = WorthQueryDomainRebindReceipt::new(&prior, &current_witness);
        Ok(WorthQueryReboundDomainHandle::new(current, receipt))
    }

    pub(crate) fn installed_domain_execution_index(
        &self,
    ) -> &crate::domain_installation::WorthQueryInstalledDomainExecutionIndex {
        self.domain_installation_registry.execution_index()
    }

    pub(crate) fn installed_domain_authority_by_marker(
        &self,
        marker: std::any::TypeId,
    ) -> Option<std::sync::Arc<crate::domain_installation::WorthQueryInstalledDomainAuthority>>
    {
        self.domain_installation_registry
            .authority_by_marker(marker)
    }

    #[cfg(test)]
    pub(crate) fn destroy_and_rebuild_domain_execution_index(
        &mut self,
    ) -> WorthQueryDomainExecutionIndexRebuildReport {
        self.domain_installation_registry
            .destroy_and_rebuild_execution_index()
    }

    pub(crate) fn replace_domain_installation_with_successor_generation(&mut self) {
        self.domain_installation_registry
            .replace_with_successor_generation();
    }
}
