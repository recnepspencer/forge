use super::WorthQueryWorkspace;

impl WorthQueryWorkspace {
    pub fn install_owned_bridge_async_declaration(
        &mut self,
        declaration: super::super::WorthQueryOwnedAsyncRequestDeclaration,
    ) -> Result<
        super::super::WorthQueryInstalledOwnedAsyncDeclaration,
        super::super::WorthQueryOwnedAsyncRuntimeDenial,
    > {
        self.runtime
            .install_owned_bridge_async_declaration(declaration)
    }

    pub fn admit_owned_bridge_async_request(
        &mut self,
        declaration: &super::super::WorthQueryInstalledOwnedAsyncDeclaration,
        truth_basis: worth_runtime_bridge::facade::BridgeAsyncRequestTruthViewBasis,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeOwnedAsyncRequestAdmission,
        super::super::WorthQueryOwnedAsyncRuntimeDenial,
    > {
        self.runtime
            .admit_owned_bridge_async_request(declaration, truth_basis)
    }

    pub fn retire_owned_bridge_async_request(
        &mut self,
        request: &worth_runtime_bridge::facade::AdmittedBridgeAsyncRequestIdentity,
    ) -> Result<(), super::super::WorthQueryOwnedAsyncRuntimeDenial> {
        self.runtime.retire_owned_bridge_async_request(request)
    }

    pub fn admit_owned_bridge_async_completion(
        &mut self,
        request: &worth_runtime_bridge::facade::AdmittedBridgeAsyncRequestIdentity,
        raw: worth_signal::facade::RawCompletionEnvelope,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeAsyncCompletionAdmissionReport,
        super::super::WorthQueryOwnedAsyncRuntimeDenial,
    > {
        self.runtime
            .admit_owned_bridge_async_completion(request, raw)
    }

    pub fn admit_owned_bridge_async_effects_indeterminate(
        &mut self,
        observation: worth_runtime_bridge::facade::BridgeAsyncEffectsIndeterminateObservation,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeAsyncCompletionAdmissionReport,
        super::super::WorthQueryOwnedAsyncRuntimeDenial,
    > {
        self.runtime
            .admit_owned_bridge_async_effects_indeterminate(observation)
    }

    pub fn order_owned_bridge_async_completion(
        &self,
        report: &worth_runtime_bridge::facade::BridgeAsyncCompletionAdmissionReport,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeMixedCauseOrdering,
        super::super::WorthQueryOwnedAsyncRuntimeDenial,
    > {
        self.runtime.order_owned_bridge_async_completion(report)
    }

    pub fn owned_async_runtime_topology(
        &self,
    ) -> Option<super::super::WorthQueryOwnedAsyncRuntimeTopology> {
        self.runtime.owned_async_runtime_topology()
    }

    pub fn supersede_owned_bridge_async_live_view<T>(
        &mut self,
        view: &super::super::WorthQueryLiveView<T>,
        prior: &super::super::WorthQueryInstalledOwnedAsyncDeclaration,
        displacing: &super::super::WorthQueryInstalledOwnedAsyncDeclaration,
    ) -> Result<
        super::super::WorthQueryAsyncResultTransitionBatch,
        super::super::WorthQueryAsyncSourceBindingError,
    > {
        self.runtime
            .supersede_owned_bridge_async_live_view(view, prior, displacing)
    }

    pub fn deny_owned_bridge_async_live_view<T>(
        &mut self,
        view: &super::super::WorthQueryLiveView<T>,
        declaration: &super::super::WorthQueryInstalledOwnedAsyncDeclaration,
    ) -> Result<
        super::super::WorthQueryAsyncResultTransitionBatch,
        super::super::WorthQueryAsyncSourceBindingError,
    > {
        self.runtime
            .deny_owned_bridge_async_live_view(view, declaration)
    }

    pub fn cancel_owned_bridge_async_live_view<T>(
        &mut self,
        view: &super::super::WorthQueryLiveView<T>,
        declaration: &super::super::WorthQueryInstalledOwnedAsyncDeclaration,
    ) -> Result<
        super::super::WorthQueryAsyncResultTransitionBatch,
        super::super::WorthQueryAsyncSourceBindingError,
    > {
        self.runtime
            .cancel_owned_bridge_async_live_view(view, declaration)
    }
}
