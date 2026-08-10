use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryDeclarationBridgeRouting,
    WorthQueryDeclarationFutureProjection, WorthQueryDeclarationInput,
    WorthQueryDeclarationSignalExecutionFamily, WorthQueryDomainEntryMarker,
};
use crate::basis_lifecycle::BasisFamily;

use super::super::readmission::WorthQueryPreparedContinuationExecutionReadmission;
use super::vocabulary::{
    WorthQueryContinuationBasisPosture, WorthQueryContinuationRuntimeContract,
    WorthQueryContinuationTruthContext, WorthQueryContinuationWorkspaceContract,
    WorthQueryPreparedContinuationExecutionMode, WorthQueryPreparedContinuationFamily,
    WorthQueryPreparedContinuationSignalPosture,
};

pub struct WorthQueryPreparedContinuation<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    family: WorthQueryPreparedContinuationFamily,
    truth_context: WorthQueryContinuationTruthContext,
    basis_posture: WorthQueryContinuationBasisPosture,
    workspace_contract: WorthQueryContinuationWorkspaceContract,
    runtime_contract: WorthQueryContinuationRuntimeContract,
    execution_mode: WorthQueryPreparedContinuationExecutionMode,
    required_basis_families: Vec<BasisFamily>,
    execution_readmission: WorthQueryPreparedContinuationExecutionReadmission,
    bridge_routing: WorthQueryDeclarationBridgeRouting<D, I>,
    signal_posture: WorthQueryPreparedContinuationSignalPosture,
    signal_execution_family: Option<WorthQueryDeclarationSignalExecutionFamily>,
    signal_compatibility_digest: Option<String>,
    prepared_digest: String,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryPreparedContinuation<D, I>
{
    pub(crate) fn new(
        family: WorthQueryPreparedContinuationFamily,
        truth_context: WorthQueryContinuationTruthContext,
        basis_posture: WorthQueryContinuationBasisPosture,
        workspace_contract: WorthQueryContinuationWorkspaceContract,
        runtime_contract: WorthQueryContinuationRuntimeContract,
        execution_mode: WorthQueryPreparedContinuationExecutionMode,
        required_basis_families: Vec<BasisFamily>,
        execution_readmission: WorthQueryPreparedContinuationExecutionReadmission,
        bridge_routing: WorthQueryDeclarationBridgeRouting<D, I>,
        signal_posture: WorthQueryPreparedContinuationSignalPosture,
        signal_execution_family: Option<WorthQueryDeclarationSignalExecutionFamily>,
        signal_compatibility_digest: Option<String>,
        prepared_digest: String,
    ) -> Self {
        Self {
            family,
            truth_context,
            basis_posture,
            workspace_contract,
            runtime_contract,
            execution_mode,
            required_basis_families,
            execution_readmission,
            bridge_routing,
            signal_posture,
            signal_execution_family,
            signal_compatibility_digest,
            prepared_digest,
        }
    }

    pub fn family(&self) -> WorthQueryPreparedContinuationFamily {
        self.family
    }

    pub fn truth_context(&self) -> WorthQueryContinuationTruthContext {
        self.truth_context
    }

    pub fn basis_posture(&self) -> WorthQueryContinuationBasisPosture {
        self.basis_posture
    }

    pub fn workspace_contract(&self) -> WorthQueryContinuationWorkspaceContract {
        self.workspace_contract
    }

    pub fn runtime_contract(&self) -> WorthQueryContinuationRuntimeContract {
        self.runtime_contract
    }

    pub fn execution_mode(&self) -> WorthQueryPreparedContinuationExecutionMode {
        self.execution_mode
    }

    pub fn required_basis_families(&self) -> &[BasisFamily] {
        &self.required_basis_families
    }

    pub fn required_capability_families(&self) -> &[WorthQueryCapabilityFamily] {
        self.execution_readmission.required_capability_families()
    }

    pub fn execution_readmission(&self) -> &WorthQueryPreparedContinuationExecutionReadmission {
        &self.execution_readmission
    }

    pub fn bridge_routing(&self) -> &WorthQueryDeclarationBridgeRouting<D, I> {
        &self.bridge_routing
    }

    pub fn signal_posture(&self) -> WorthQueryPreparedContinuationSignalPosture {
        self.signal_posture
    }

    pub fn signal_execution_family(&self) -> Option<WorthQueryDeclarationSignalExecutionFamily> {
        self.signal_execution_family
    }

    pub fn signal_compatibility_digest(&self) -> Option<&str> {
        self.signal_compatibility_digest.as_deref()
    }

    pub fn future_projection(&self) -> &WorthQueryDeclarationFutureProjection {
        self.bridge_routing.future_projection()
    }

    pub fn basis_lifecycle_support_digest(&self) -> &str {
        self.bridge_routing.basis_lifecycle_support_digest()
    }

    pub fn prepared_digest(&self) -> &str {
        &self.prepared_digest
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.bridge_routing.handle_identity_digest()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.bridge_routing.operating_context_identity_digest()
    }

    pub fn declaration_digest(&self) -> &str {
        self.bridge_routing.declaration_digest()
    }

    pub fn progression_digest(&self) -> Option<&str> {
        self.bridge_routing.progression_digest()
    }

    pub fn route_plan_digest(&self) -> Option<&str> {
        self.bridge_routing.route_plan_digest()
    }

    pub fn receipt_digest(&self) -> &worth_foundational::facade::CanonicalDerivedDigest {
        self.bridge_routing.receipt_digest()
    }

    pub fn envelope_digest(&self) -> &worth_foundational::facade::CanonicalDerivedDigest {
        self.bridge_routing.envelope_digest()
    }
}
