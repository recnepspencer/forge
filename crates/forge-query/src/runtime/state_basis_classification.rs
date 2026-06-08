use super::{ForgeQueryAuthorityLane, ForgeQueryRuntimeStateKind};
use crate::query_basis_lifecycle::{
    BasisIntentDenialKind, BasisLifecyclePosture, BasisOperationLaneRequest,
    DeniedBasisCapabilityKind, NormalizedBasisFamily,
};

pub(crate) fn authority_lane_for_basis_family(
    family: &NormalizedBasisFamily,
    lifecycle_posture: BasisLifecyclePosture,
    operation_lane: &BasisOperationLaneRequest,
) -> ForgeQueryAuthorityLane {
    if lifecycle_posture == BasisLifecyclePosture::AdvisoryPreview {
        return ForgeQueryAuthorityLane::PreviewTruth;
    }
    if lifecycle_posture == BasisLifecyclePosture::MutablePreparation {
        return ForgeQueryAuthorityLane::PendingWriteIntent;
    }
    if lifecycle_posture == BasisLifecyclePosture::HistoricalReplay
        || operation_lane == &BasisOperationLaneRequest::Replay
    {
        return ForgeQueryAuthorityLane::BridgeExternalState;
    }
    match family {
        NormalizedBasisFamily::CurrentHead | NormalizedBasisFamily::RuntimeSnapshot => {
            ForgeQueryAuthorityLane::AuthoritativeTruth
        }
        NormalizedBasisFamily::BranchHead | NormalizedBasisFamily::BranchSnapshot => {
            ForgeQueryAuthorityLane::BranchLocalTruth
        }
        NormalizedBasisFamily::HistoricalSnapshot | NormalizedBasisFamily::HistoricalCommit => {
            ForgeQueryAuthorityLane::BridgeExternalState
        }
        NormalizedBasisFamily::Preview | NormalizedBasisFamily::PreviewDerivedHistorical => {
            ForgeQueryAuthorityLane::PreviewTruth
        }
    }
}

pub(crate) fn authority_lane_for_denied_basis(
    family: &NormalizedBasisFamily,
    operation_lane: &BasisOperationLaneRequest,
) -> ForgeQueryAuthorityLane {
    if operation_lane == &BasisOperationLaneRequest::MutationPreparation {
        return ForgeQueryAuthorityLane::PendingWriteIntent;
    }
    if operation_lane == &BasisOperationLaneRequest::Replay {
        return ForgeQueryAuthorityLane::BridgeExternalState;
    }
    match family {
        NormalizedBasisFamily::BranchHead | NormalizedBasisFamily::BranchSnapshot => {
            ForgeQueryAuthorityLane::BranchLocalTruth
        }
        NormalizedBasisFamily::HistoricalSnapshot | NormalizedBasisFamily::HistoricalCommit => {
            ForgeQueryAuthorityLane::BridgeExternalState
        }
        NormalizedBasisFamily::Preview | NormalizedBasisFamily::PreviewDerivedHistorical => {
            ForgeQueryAuthorityLane::PreviewTruth
        }
        _ => ForgeQueryAuthorityLane::AuthoritativeTruth,
    }
}

pub(crate) fn state_kind_for_basis_denial(
    kind: &DeniedBasisCapabilityKind,
) -> ForgeQueryRuntimeStateKind {
    match kind {
        DeniedBasisCapabilityKind::Stale { .. }
        | DeniedBasisCapabilityKind::PreviewDrifted { .. } => ForgeQueryRuntimeStateKind::Stale,
        DeniedBasisCapabilityKind::LowerRuntimeCapabilityUnsupported { .. }
        | DeniedBasisCapabilityKind::HistoricalReplayUnsupported { .. }
        | DeniedBasisCapabilityKind::DurableOverclaim { .. } => {
            ForgeQueryRuntimeStateKind::Unsupported
        }
        DeniedBasisCapabilityKind::LowerRuntimeBindingMissing { .. }
        | DeniedBasisCapabilityKind::LowerRuntimeBindingMismatch { .. }
        | DeniedBasisCapabilityKind::Inaccessible { .. }
        | DeniedBasisCapabilityKind::PolicyMasked { .. }
        | DeniedBasisCapabilityKind::TenantMismatched { .. }
        | DeniedBasisCapabilityKind::SchemaIncompatible { .. }
        | DeniedBasisCapabilityKind::OperationIneligible { .. } => {
            ForgeQueryRuntimeStateKind::Denied
        }
    }
}

pub(crate) fn state_kind_for_basis_intent_denial(
    kind: &BasisIntentDenialKind,
) -> ForgeQueryRuntimeStateKind {
    match kind {
        BasisIntentDenialKind::MalformedIdentifier { .. } => ForgeQueryRuntimeStateKind::Denied,
        BasisIntentDenialKind::UnsupportedCompatibilityFamily { .. }
        | BasisIntentDenialKind::UnsupportedFutureNeighbor { .. } => {
            ForgeQueryRuntimeStateKind::Unsupported
        }
    }
}

pub(crate) fn authority_lane_for_intent_denial(
    kind: &BasisIntentDenialKind,
    operation_lane: &BasisOperationLaneRequest,
) -> ForgeQueryAuthorityLane {
    match kind {
        BasisIntentDenialKind::UnsupportedFutureNeighbor { family, .. } => match family.as_str() {
            "temporal" => ForgeQueryAuthorityLane::TemporalExecutionState,
            "async_resource" => ForgeQueryAuthorityLane::AsyncResourceState,
            "durable_reload" | "restart_stable_envelope" | "store_backed_parity" => {
                ForgeQueryAuthorityLane::BridgeExternalState
            }
            _ => ForgeQueryAuthorityLane::AuthoritativeTruth,
        },
        _ if operation_lane == &BasisOperationLaneRequest::MutationPreparation => {
            ForgeQueryAuthorityLane::PendingWriteIntent
        }
        _ => ForgeQueryAuthorityLane::AuthoritativeTruth,
    }
}
