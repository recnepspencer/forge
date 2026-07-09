use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryDeclarationBridgeContinuationMode,
    WorthQueryDeclarationBridgeRouting, WorthQueryDeclarationFutureProjection,
    WorthQueryDeclarationInput, WorthQueryDeclarationSignalExecutionFamily,
    WorthQueryDomainEntryMarker,
};
use crate::basis_lifecycle::BasisFamily;

use super::readmission::WorthQueryPreparedContinuationExecutionReadmission;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPreparedContinuationFamily {
    BridgeRuntimeRoute,
    BridgeTruthView,
    BridgePreviewSession,
    BridgePreviewPromotion,
    BridgeSubscriptionPreparation,
    BridgeWritebackPreparation,
}

impl WorthQueryPreparedContinuationFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BridgeRuntimeRoute => "bridge-runtime-route",
            Self::BridgeTruthView => "bridge-truth-view",
            Self::BridgePreviewSession => "bridge-preview-session",
            Self::BridgePreviewPromotion => "bridge-preview-promotion",
            Self::BridgeSubscriptionPreparation => "bridge-subscription-preparation",
            Self::BridgeWritebackPreparation => "bridge-writeback-preparation",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryContinuationTruthContext {
    Current,
    Historical,
    Preview,
}

impl WorthQueryContinuationTruthContext {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Historical => "historical",
            Self::Preview => "preview",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryContinuationBasisPosture {
    CurrentHead,
    HistoricalSnapshot,
    PreviewDerived,
    Mixed,
}

impl WorthQueryContinuationBasisPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CurrentHead => "current-head",
            Self::HistoricalSnapshot => "historical-snapshot",
            Self::PreviewDerived => "preview-derived",
            Self::Mixed => "mixed",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryContinuationWorkspaceContract {
    RuntimeWorkspace,
    TruthViewWorkspace,
    PreviewWorkspace,
    SubscriptionWorkspace,
    WritebackWorkspace,
}

impl WorthQueryContinuationWorkspaceContract {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeWorkspace => "runtime-workspace",
            Self::TruthViewWorkspace => "truth-view-workspace",
            Self::PreviewWorkspace => "preview-workspace",
            Self::SubscriptionWorkspace => "subscription-workspace",
            Self::WritebackWorkspace => "writeback-workspace",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryContinuationRuntimeContract {
    RuntimeRoute,
    TruthView,
    PreviewSession,
    PreviewPromotion,
    SubscriptionPreparation,
    WritebackPreparation,
}

impl WorthQueryContinuationRuntimeContract {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeRoute => "runtime-route",
            Self::TruthView => "truth-view",
            Self::PreviewSession => "preview-session",
            Self::PreviewPromotion => "preview-promotion",
            Self::SubscriptionPreparation => "subscription-preparation",
            Self::WritebackPreparation => "writeback-preparation",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPreparedContinuationExecutionMode {
    ExplicitBridgeLowering,
}

impl WorthQueryPreparedContinuationExecutionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitBridgeLowering => "explicit-bridge-lowering",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPreparedContinuationSignalPosture {
    Compatible,
    Deferred,
    Denied,
    Failed,
}

impl WorthQueryPreparedContinuationSignalPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::Deferred => "deferred",
            Self::Denied => "denied",
            Self::Failed => "failed",
        }
    }
}

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

pub(crate) fn family_for_mode(
    mode: WorthQueryDeclarationBridgeContinuationMode,
) -> WorthQueryPreparedContinuationFamily {
    match mode {
        WorthQueryDeclarationBridgeContinuationMode::RuntimeRoute => {
            WorthQueryPreparedContinuationFamily::BridgeRuntimeRoute
        }
        WorthQueryDeclarationBridgeContinuationMode::TruthView => {
            WorthQueryPreparedContinuationFamily::BridgeTruthView
        }
        WorthQueryDeclarationBridgeContinuationMode::PreviewSession => {
            WorthQueryPreparedContinuationFamily::BridgePreviewSession
        }
        WorthQueryDeclarationBridgeContinuationMode::PreviewPromotion => {
            WorthQueryPreparedContinuationFamily::BridgePreviewPromotion
        }
        WorthQueryDeclarationBridgeContinuationMode::SubscriptionPreparation => {
            WorthQueryPreparedContinuationFamily::BridgeSubscriptionPreparation
        }
        WorthQueryDeclarationBridgeContinuationMode::WritebackPreparation => {
            WorthQueryPreparedContinuationFamily::BridgeWritebackPreparation
        }
    }
}

pub(crate) fn truth_context_for_mode(
    truth_context: crate::application::WorthQueryDeclarationBridgeTruthContext,
) -> WorthQueryContinuationTruthContext {
    match truth_context {
        crate::application::WorthQueryDeclarationBridgeTruthContext::Current => {
            WorthQueryContinuationTruthContext::Current
        }
        crate::application::WorthQueryDeclarationBridgeTruthContext::Historical => {
            WorthQueryContinuationTruthContext::Historical
        }
        crate::application::WorthQueryDeclarationBridgeTruthContext::Preview => {
            WorthQueryContinuationTruthContext::Preview
        }
    }
}

pub(crate) fn basis_posture_for_families(
    basis_families: &[BasisFamily],
) -> WorthQueryContinuationBasisPosture {
    if basis_families.iter().all(|family| {
        matches!(
            family,
            BasisFamily::CurrentHead
                | BasisFamily::BranchHead
                | BasisFamily::BranchSnapshot
                | BasisFamily::RuntimeSnapshot
        )
    }) {
        WorthQueryContinuationBasisPosture::CurrentHead
    } else if basis_families
        .iter()
        .all(|family| matches!(family, BasisFamily::HistoricalSnapshot))
    {
        WorthQueryContinuationBasisPosture::HistoricalSnapshot
    } else if basis_families
        .iter()
        .all(|family| matches!(family, BasisFamily::Preview | BasisFamily::PreviewDerived))
    {
        WorthQueryContinuationBasisPosture::PreviewDerived
    } else {
        WorthQueryContinuationBasisPosture::Mixed
    }
}

pub(crate) fn workspace_contract_for_mode(
    mode: WorthQueryDeclarationBridgeContinuationMode,
) -> WorthQueryContinuationWorkspaceContract {
    match mode {
        WorthQueryDeclarationBridgeContinuationMode::RuntimeRoute => {
            WorthQueryContinuationWorkspaceContract::RuntimeWorkspace
        }
        WorthQueryDeclarationBridgeContinuationMode::TruthView => {
            WorthQueryContinuationWorkspaceContract::TruthViewWorkspace
        }
        WorthQueryDeclarationBridgeContinuationMode::PreviewSession
        | WorthQueryDeclarationBridgeContinuationMode::PreviewPromotion => {
            WorthQueryContinuationWorkspaceContract::PreviewWorkspace
        }
        WorthQueryDeclarationBridgeContinuationMode::SubscriptionPreparation => {
            WorthQueryContinuationWorkspaceContract::SubscriptionWorkspace
        }
        WorthQueryDeclarationBridgeContinuationMode::WritebackPreparation => {
            WorthQueryContinuationWorkspaceContract::WritebackWorkspace
        }
    }
}

pub(crate) fn runtime_contract_for_mode(
    mode: WorthQueryDeclarationBridgeContinuationMode,
) -> WorthQueryContinuationRuntimeContract {
    match mode {
        WorthQueryDeclarationBridgeContinuationMode::RuntimeRoute => {
            WorthQueryContinuationRuntimeContract::RuntimeRoute
        }
        WorthQueryDeclarationBridgeContinuationMode::TruthView => {
            WorthQueryContinuationRuntimeContract::TruthView
        }
        WorthQueryDeclarationBridgeContinuationMode::PreviewSession => {
            WorthQueryContinuationRuntimeContract::PreviewSession
        }
        WorthQueryDeclarationBridgeContinuationMode::PreviewPromotion => {
            WorthQueryContinuationRuntimeContract::PreviewPromotion
        }
        WorthQueryDeclarationBridgeContinuationMode::SubscriptionPreparation => {
            WorthQueryContinuationRuntimeContract::SubscriptionPreparation
        }
        WorthQueryDeclarationBridgeContinuationMode::WritebackPreparation => {
            WorthQueryContinuationRuntimeContract::WritebackPreparation
        }
    }
}
