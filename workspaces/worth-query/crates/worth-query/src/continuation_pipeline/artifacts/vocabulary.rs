use crate::application::{
    WorthQueryDeclarationBridgeContinuationMode, WorthQueryDeclarationBridgeTruthContext,
};
use crate::basis_lifecycle::BasisFamily;

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
    truth_context: WorthQueryDeclarationBridgeTruthContext,
) -> WorthQueryContinuationTruthContext {
    match truth_context {
        WorthQueryDeclarationBridgeTruthContext::Current => {
            WorthQueryContinuationTruthContext::Current
        }
        WorthQueryDeclarationBridgeTruthContext::Historical => {
            WorthQueryContinuationTruthContext::Historical
        }
        WorthQueryDeclarationBridgeTruthContext::Preview => {
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
