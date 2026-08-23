use worth_runtime_bridge::facade::{
    AdmittedBridgeAsyncRequestIdentity, BridgeAsyncCompletionAdmissionReport,
    BridgeAsyncCompletionRejectionKind, BridgeAsyncRequestAdmissionRequest,
    BridgeAsyncRequestIdentityRejectionKind, BridgeAsyncRequestTruthViewBasis,
    BridgeAsyncSourceDeclarationRejectionKind, BridgeOwnedAsyncRequestResponseDeclaration,
};

use super::WorthQueryRuntime;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOwnedAsyncRequestDeclaration {
    identity: crate::application::WorthQueryAsyncResourceRequestIdentity,
    clause: crate::application::WorthQueryAsyncDeclarationClause,
    payload_contract: u64,
    max_payload_bytes: u64,
    retry_max_attempts: u32,
}

impl WorthQueryOwnedAsyncRequestDeclaration {
    pub fn from_async_resource_identity(
        identity: crate::application::WorthQueryAsyncResourceRequestIdentity,
        payload_contract: u64,
        max_payload_bytes: u64,
        retry_max_attempts: u32,
    ) -> Self {
        let clause = crate::application::WorthQueryAsyncDeclarationClause::resource_request(
            identity.source_family(),
            identity.loading_posture(),
            identity.failure_posture(),
            identity.request_identity().to_vec(),
        );
        Self {
            identity,
            clause,
            payload_contract,
            max_payload_bytes,
            retry_max_attempts,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledOwnedAsyncDeclaration {
    runtime_provenance: super::WorthQueryRuntimeProvenance,
    signal_graph_instance: u64,
    identity: crate::application::WorthQueryAsyncResourceRequestIdentity,
    clause: crate::application::WorthQueryAsyncDeclarationClause,
    lowered: worth_runtime_bridge::facade::LoweredBridgeAsyncSourceDeclaration,
}

impl WorthQueryInstalledOwnedAsyncDeclaration {
    pub fn identity(&self) -> &crate::application::WorthQueryAsyncResourceRequestIdentity {
        &self.identity
    }

    pub fn clause(&self) -> &crate::application::WorthQueryAsyncDeclarationClause {
        &self.clause
    }

    pub fn runtime_provenance(&self) -> super::WorthQueryRuntimeProvenance {
        self.runtime_provenance
    }

    pub(super) const fn signal_graph_instance(&self) -> u64 {
        self.signal_graph_instance
    }

    pub(super) fn lowered_declaration_identity(
        &self,
    ) -> &worth_runtime_bridge::facade::BridgeAsyncSourceDeclarationIdentity {
        self.lowered.declaration_identity()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOwnedAsyncRuntimeDenial {
    ConditionalRuntimeUnavailable,
    ForeignRuntime,
    SuccessorRuntime,
    Declaration(BridgeAsyncSourceDeclarationRejectionKind),
    Request(BridgeAsyncRequestIdentityRejectionKind),
    Completion(BridgeAsyncCompletionRejectionKind),
    RequestRetirementFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryOwnedAsyncRuntimeTopology {
    signal_graph_instance: u64,
    installed_conditional_nodes: usize,
    installed_async_declarations: usize,
    active_signal_nodes: usize,
}

impl WorthQueryOwnedAsyncRuntimeTopology {
    pub const fn signal_graph_instance(self) -> u64 {
        self.signal_graph_instance
    }

    pub const fn installed_conditional_nodes(self) -> usize {
        self.installed_conditional_nodes
    }

    pub const fn installed_async_declarations(self) -> usize {
        self.installed_async_declarations
    }

    pub const fn active_signal_nodes(self) -> usize {
        self.active_signal_nodes
    }
}

impl WorthQueryRuntime {
    pub fn install_owned_bridge_async_declaration(
        &mut self,
        declaration: WorthQueryOwnedAsyncRequestDeclaration,
    ) -> Result<WorthQueryInstalledOwnedAsyncDeclaration, WorthQueryOwnedAsyncRuntimeDenial> {
        let runtime_provenance = self.runtime_provenance();
        let runtime = self
            .conditional_signal_runtime
            .as_mut()
            .ok_or(WorthQueryOwnedAsyncRuntimeDenial::ConditionalRuntimeUnavailable)?;
        let lowered = runtime
            .install_owned_async_request_response(BridgeOwnedAsyncRequestResponseDeclaration::new(
                declaration.identity.canonical_identity(),
                format!(
                    "query-owned-async:legacy:{}",
                    declaration.identity.canonical_identity()
                ),
                declaration.payload_contract,
                declaration.max_payload_bytes,
                declaration.retry_max_attempts,
            ))
            .map_err(|denial| WorthQueryOwnedAsyncRuntimeDenial::Declaration(denial.kind()))?;
        Ok(WorthQueryInstalledOwnedAsyncDeclaration {
            runtime_provenance,
            signal_graph_instance: runtime.owned_signal_graph_instance_id(),
            identity: declaration.identity,
            clause: declaration.clause,
            lowered,
        })
    }

    pub fn admit_owned_bridge_async_request(
        &mut self,
        declaration: &WorthQueryInstalledOwnedAsyncDeclaration,
        truth_basis: BridgeAsyncRequestTruthViewBasis,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeOwnedAsyncRequestAdmission,
        WorthQueryOwnedAsyncRuntimeDenial,
    > {
        if declaration.runtime_provenance != self.runtime_provenance() {
            return Err(WorthQueryOwnedAsyncRuntimeDenial::ForeignRuntime);
        }
        let runtime = self
            .conditional_signal_runtime
            .as_mut()
            .ok_or(WorthQueryOwnedAsyncRuntimeDenial::ConditionalRuntimeUnavailable)?;
        if declaration.signal_graph_instance != runtime.owned_signal_graph_instance_id() {
            return Err(WorthQueryOwnedAsyncRuntimeDenial::SuccessorRuntime);
        }
        let binding = runtime.bind_owned_async_request_basis(&declaration.lowered, truth_basis);
        let request =
            BridgeAsyncRequestAdmissionRequest::request_response(&declaration.lowered, &binding)
                .map_err(|denial| WorthQueryOwnedAsyncRuntimeDenial::Request(denial.kind()))?;
        let admission = runtime
            .admit_owned_async_request_identity(request)
            .map_err(|denial| WorthQueryOwnedAsyncRuntimeDenial::Request(denial.kind()))?;
        Ok(admission)
    }

    pub fn admit_owned_bridge_async_completion(
        &mut self,
        request: &AdmittedBridgeAsyncRequestIdentity,
        raw: worth_signal::facade::RawCompletionEnvelope,
    ) -> Result<BridgeAsyncCompletionAdmissionReport, WorthQueryOwnedAsyncRuntimeDenial> {
        let runtime = self
            .conditional_signal_runtime
            .as_mut()
            .ok_or(WorthQueryOwnedAsyncRuntimeDenial::ConditionalRuntimeUnavailable)?;
        let validated = runtime
            .validate_owned_async_completion_envelope(request, raw)
            .map_err(|denial| WorthQueryOwnedAsyncRuntimeDenial::Completion(denial.kind()))?;
        runtime
            .admit_owned_async_completion(request, &validated)
            .map_err(|denial| WorthQueryOwnedAsyncRuntimeDenial::Completion(denial.kind()))
    }

    pub fn admit_owned_bridge_async_effects_indeterminate(
        &mut self,
        observation: worth_runtime_bridge::facade::BridgeAsyncEffectsIndeterminateObservation,
    ) -> Result<BridgeAsyncCompletionAdmissionReport, WorthQueryOwnedAsyncRuntimeDenial> {
        let runtime = self
            .conditional_signal_runtime
            .as_mut()
            .ok_or(WorthQueryOwnedAsyncRuntimeDenial::ConditionalRuntimeUnavailable)?;
        runtime
            .admit_owned_async_effects_indeterminate(observation)
            .map_err(|denial| WorthQueryOwnedAsyncRuntimeDenial::Completion(denial.kind()))
    }

    pub fn retire_owned_bridge_async_request(
        &mut self,
        request: &AdmittedBridgeAsyncRequestIdentity,
    ) -> Result<(), WorthQueryOwnedAsyncRuntimeDenial> {
        let runtime = self
            .conditional_signal_runtime
            .as_mut()
            .ok_or(WorthQueryOwnedAsyncRuntimeDenial::ConditionalRuntimeUnavailable)?;
        match runtime.retire_owned_async_request(request) {
            Ok(true) => Ok(()),
            Ok(false) => Ok(()),
            Err(_) => Err(WorthQueryOwnedAsyncRuntimeDenial::RequestRetirementFailed),
        }
    }

    pub fn order_owned_bridge_async_completion(
        &self,
        report: &BridgeAsyncCompletionAdmissionReport,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeMixedCauseOrdering,
        WorthQueryOwnedAsyncRuntimeDenial,
    > {
        self.conditional_signal_runtime
            .as_ref()
            .ok_or(WorthQueryOwnedAsyncRuntimeDenial::ConditionalRuntimeUnavailable)
            .map(|runtime| runtime.order_owned_async_completion_report(report))
    }

    pub fn owned_async_runtime_topology(&self) -> Option<WorthQueryOwnedAsyncRuntimeTopology> {
        self.conditional_signal_runtime.as_ref().map(|runtime| {
            WorthQueryOwnedAsyncRuntimeTopology {
                signal_graph_instance: runtime.owned_signal_graph_instance_id(),
                installed_conditional_nodes: self.conditional_execution_registry.len(),
                installed_async_declarations: runtime.owned_async_declaration_count(),
                active_signal_nodes: runtime.owned_signal_active_node_count(),
            }
        })
    }
}
