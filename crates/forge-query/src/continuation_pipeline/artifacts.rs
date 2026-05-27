use crate::application::{
    ForgeQueryDeclarationBridgeContinuationMode, ForgeQueryDeclarationBridgeRouting,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationSignalExecutionFamily,
    ForgeQueryDomainEntryMarker,
};
use crate::basis_lifecycle::BasisFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreparedContinuationFamily {
    BridgeRuntimeRoute,
    BridgeTruthView,
    BridgePreviewSession,
    BridgePreviewPromotion,
    BridgeSubscriptionPreparation,
    BridgeWritebackPreparation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryContinuationTruthContext {
    Current,
    Historical,
    Preview,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryContinuationBasisPosture {
    CurrentHead,
    HistoricalSnapshot,
    PreviewDerived,
    Mixed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryContinuationWorkspaceContract {
    RuntimeWorkspace,
    TruthViewWorkspace,
    PreviewWorkspace,
    SubscriptionWorkspace,
    WritebackWorkspace,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryContinuationRuntimeContract {
    RuntimeRoute,
    TruthView,
    PreviewSession,
    PreviewPromotion,
    SubscriptionPreparation,
    WritebackPreparation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreparedContinuationExecutionMode {
    ExplicitBridgeLowering,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreparedContinuationSignalPosture {
    Compatible,
    Deferred,
    Denied,
    Failed,
}

pub struct ForgeQueryPreparedContinuation<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    family: ForgeQueryPreparedContinuationFamily,
    truth_context: ForgeQueryContinuationTruthContext,
    basis_posture: ForgeQueryContinuationBasisPosture,
    workspace_contract: ForgeQueryContinuationWorkspaceContract,
    runtime_contract: ForgeQueryContinuationRuntimeContract,
    execution_mode: ForgeQueryPreparedContinuationExecutionMode,
    required_basis_families: Vec<BasisFamily>,
    bridge_routing: ForgeQueryDeclarationBridgeRouting<D, I>,
    signal_posture: ForgeQueryPreparedContinuationSignalPosture,
    signal_execution_family: Option<ForgeQueryDeclarationSignalExecutionFamily>,
    signal_compatibility_digest: Option<String>,
    prepared_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryPreparedContinuation<D, I>
{
    pub(crate) fn new(
        family: ForgeQueryPreparedContinuationFamily,
        truth_context: ForgeQueryContinuationTruthContext,
        basis_posture: ForgeQueryContinuationBasisPosture,
        workspace_contract: ForgeQueryContinuationWorkspaceContract,
        runtime_contract: ForgeQueryContinuationRuntimeContract,
        execution_mode: ForgeQueryPreparedContinuationExecutionMode,
        required_basis_families: Vec<BasisFamily>,
        bridge_routing: ForgeQueryDeclarationBridgeRouting<D, I>,
        signal_posture: ForgeQueryPreparedContinuationSignalPosture,
        signal_execution_family: Option<ForgeQueryDeclarationSignalExecutionFamily>,
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
            bridge_routing,
            signal_posture,
            signal_execution_family,
            signal_compatibility_digest,
            prepared_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryPreparedContinuationFamily {
        self.family
    }

    pub fn truth_context(&self) -> ForgeQueryContinuationTruthContext {
        self.truth_context
    }

    pub fn basis_posture(&self) -> ForgeQueryContinuationBasisPosture {
        self.basis_posture
    }

    pub fn workspace_contract(&self) -> ForgeQueryContinuationWorkspaceContract {
        self.workspace_contract
    }

    pub fn runtime_contract(&self) -> ForgeQueryContinuationRuntimeContract {
        self.runtime_contract
    }

    pub fn execution_mode(&self) -> ForgeQueryPreparedContinuationExecutionMode {
        self.execution_mode
    }

    pub fn required_basis_families(&self) -> &[BasisFamily] {
        &self.required_basis_families
    }

    pub fn bridge_routing(&self) -> &ForgeQueryDeclarationBridgeRouting<D, I> {
        &self.bridge_routing
    }

    pub fn signal_posture(&self) -> ForgeQueryPreparedContinuationSignalPosture {
        self.signal_posture
    }

    pub fn signal_execution_family(&self) -> Option<ForgeQueryDeclarationSignalExecutionFamily> {
        self.signal_execution_family
    }

    pub fn signal_compatibility_digest(&self) -> Option<&str> {
        self.signal_compatibility_digest.as_deref()
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

    pub fn receipt_digest(&self) -> &forge_foundational::facade::CanonicalDerivedDigest {
        self.bridge_routing.receipt_digest()
    }

    pub fn envelope_digest(&self) -> &forge_foundational::facade::CanonicalDerivedDigest {
        self.bridge_routing.envelope_digest()
    }
}

pub(crate) fn family_for_mode(
    mode: ForgeQueryDeclarationBridgeContinuationMode,
) -> ForgeQueryPreparedContinuationFamily {
    match mode {
        ForgeQueryDeclarationBridgeContinuationMode::RuntimeRoute => {
            ForgeQueryPreparedContinuationFamily::BridgeRuntimeRoute
        }
        ForgeQueryDeclarationBridgeContinuationMode::TruthView => {
            ForgeQueryPreparedContinuationFamily::BridgeTruthView
        }
        ForgeQueryDeclarationBridgeContinuationMode::PreviewSession => {
            ForgeQueryPreparedContinuationFamily::BridgePreviewSession
        }
        ForgeQueryDeclarationBridgeContinuationMode::PreviewPromotion => {
            ForgeQueryPreparedContinuationFamily::BridgePreviewPromotion
        }
        ForgeQueryDeclarationBridgeContinuationMode::SubscriptionPreparation => {
            ForgeQueryPreparedContinuationFamily::BridgeSubscriptionPreparation
        }
        ForgeQueryDeclarationBridgeContinuationMode::WritebackPreparation => {
            ForgeQueryPreparedContinuationFamily::BridgeWritebackPreparation
        }
    }
}

pub(crate) fn truth_context_for_mode(
    truth_context: crate::application::ForgeQueryDeclarationBridgeTruthContext,
) -> ForgeQueryContinuationTruthContext {
    match truth_context {
        crate::application::ForgeQueryDeclarationBridgeTruthContext::Current => {
            ForgeQueryContinuationTruthContext::Current
        }
        crate::application::ForgeQueryDeclarationBridgeTruthContext::Historical => {
            ForgeQueryContinuationTruthContext::Historical
        }
        crate::application::ForgeQueryDeclarationBridgeTruthContext::Preview => {
            ForgeQueryContinuationTruthContext::Preview
        }
    }
}

pub(crate) fn basis_posture_for_families(
    basis_families: &[BasisFamily],
) -> ForgeQueryContinuationBasisPosture {
    if basis_families.iter().all(|family| {
        matches!(
            family,
            BasisFamily::CurrentHead
                | BasisFamily::BranchHead
                | BasisFamily::BranchSnapshot
                | BasisFamily::RuntimeSnapshot
        )
    }) {
        ForgeQueryContinuationBasisPosture::CurrentHead
    } else if basis_families
        .iter()
        .all(|family| matches!(family, BasisFamily::HistoricalSnapshot))
    {
        ForgeQueryContinuationBasisPosture::HistoricalSnapshot
    } else if basis_families
        .iter()
        .all(|family| matches!(family, BasisFamily::Preview | BasisFamily::PreviewDerived))
    {
        ForgeQueryContinuationBasisPosture::PreviewDerived
    } else {
        ForgeQueryContinuationBasisPosture::Mixed
    }
}

pub(crate) fn workspace_contract_for_mode(
    mode: ForgeQueryDeclarationBridgeContinuationMode,
) -> ForgeQueryContinuationWorkspaceContract {
    match mode {
        ForgeQueryDeclarationBridgeContinuationMode::RuntimeRoute => {
            ForgeQueryContinuationWorkspaceContract::RuntimeWorkspace
        }
        ForgeQueryDeclarationBridgeContinuationMode::TruthView => {
            ForgeQueryContinuationWorkspaceContract::TruthViewWorkspace
        }
        ForgeQueryDeclarationBridgeContinuationMode::PreviewSession
        | ForgeQueryDeclarationBridgeContinuationMode::PreviewPromotion => {
            ForgeQueryContinuationWorkspaceContract::PreviewWorkspace
        }
        ForgeQueryDeclarationBridgeContinuationMode::SubscriptionPreparation => {
            ForgeQueryContinuationWorkspaceContract::SubscriptionWorkspace
        }
        ForgeQueryDeclarationBridgeContinuationMode::WritebackPreparation => {
            ForgeQueryContinuationWorkspaceContract::WritebackWorkspace
        }
    }
}

pub(crate) fn runtime_contract_for_mode(
    mode: ForgeQueryDeclarationBridgeContinuationMode,
) -> ForgeQueryContinuationRuntimeContract {
    match mode {
        ForgeQueryDeclarationBridgeContinuationMode::RuntimeRoute => {
            ForgeQueryContinuationRuntimeContract::RuntimeRoute
        }
        ForgeQueryDeclarationBridgeContinuationMode::TruthView => {
            ForgeQueryContinuationRuntimeContract::TruthView
        }
        ForgeQueryDeclarationBridgeContinuationMode::PreviewSession => {
            ForgeQueryContinuationRuntimeContract::PreviewSession
        }
        ForgeQueryDeclarationBridgeContinuationMode::PreviewPromotion => {
            ForgeQueryContinuationRuntimeContract::PreviewPromotion
        }
        ForgeQueryDeclarationBridgeContinuationMode::SubscriptionPreparation => {
            ForgeQueryContinuationRuntimeContract::SubscriptionPreparation
        }
        ForgeQueryDeclarationBridgeContinuationMode::WritebackPreparation => {
            ForgeQueryContinuationRuntimeContract::WritebackPreparation
        }
    }
}
