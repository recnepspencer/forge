use super::{WorthQueryAuthorityLane, WorthQueryRuntimeStateKind};
use crate::query_basis_lifecycle::{
    BasisIntentDenialKind, BasisLifecyclePosture, BasisOperationLaneRequest,
    DeniedBasisCapabilityKind, NormalizedBasisFamily,
};

pub(crate) fn authority_lane_for_basis_family(
    family: &NormalizedBasisFamily,
    lifecycle_posture: BasisLifecyclePosture,
    operation_lane: &BasisOperationLaneRequest,
) -> WorthQueryAuthorityLane {
    if lifecycle_posture == BasisLifecyclePosture::AdvisoryPreview {
        return WorthQueryAuthorityLane::PreviewTruth;
    }
    if lifecycle_posture == BasisLifecyclePosture::MutablePreparation {
        return WorthQueryAuthorityLane::PendingWriteIntent;
    }
    if lifecycle_posture == BasisLifecyclePosture::HistoricalReplay
        || operation_lane == &BasisOperationLaneRequest::Replay
    {
        return WorthQueryAuthorityLane::BridgeExternalState;
    }
    match family {
        NormalizedBasisFamily::CurrentHead | NormalizedBasisFamily::RuntimeSnapshot => {
            WorthQueryAuthorityLane::AuthoritativeTruth
        }
        NormalizedBasisFamily::BranchHead | NormalizedBasisFamily::BranchSnapshot => {
            WorthQueryAuthorityLane::BranchLocalTruth
        }
        NormalizedBasisFamily::HistoricalSnapshot | NormalizedBasisFamily::HistoricalCommit => {
            WorthQueryAuthorityLane::BridgeExternalState
        }
        NormalizedBasisFamily::Preview | NormalizedBasisFamily::PreviewDerivedHistorical => {
            WorthQueryAuthorityLane::PreviewTruth
        }
    }
}

pub(crate) fn authority_lane_for_denied_basis(
    family: &NormalizedBasisFamily,
    operation_lane: &BasisOperationLaneRequest,
) -> WorthQueryAuthorityLane {
    if operation_lane == &BasisOperationLaneRequest::MutationPreparation {
        return WorthQueryAuthorityLane::PendingWriteIntent;
    }
    if operation_lane == &BasisOperationLaneRequest::Replay {
        return WorthQueryAuthorityLane::BridgeExternalState;
    }
    match family {
        NormalizedBasisFamily::BranchHead | NormalizedBasisFamily::BranchSnapshot => {
            WorthQueryAuthorityLane::BranchLocalTruth
        }
        NormalizedBasisFamily::HistoricalSnapshot | NormalizedBasisFamily::HistoricalCommit => {
            WorthQueryAuthorityLane::BridgeExternalState
        }
        NormalizedBasisFamily::Preview | NormalizedBasisFamily::PreviewDerivedHistorical => {
            WorthQueryAuthorityLane::PreviewTruth
        }
        _ => WorthQueryAuthorityLane::AuthoritativeTruth,
    }
}

pub(crate) fn state_kind_for_basis_denial(
    kind: &DeniedBasisCapabilityKind,
) -> WorthQueryRuntimeStateKind {
    match kind {
        DeniedBasisCapabilityKind::Stale { .. }
        | DeniedBasisCapabilityKind::PreviewDrifted { .. } => WorthQueryRuntimeStateKind::Stale,
        DeniedBasisCapabilityKind::LowerRuntimeCapabilityUnsupported { .. }
        | DeniedBasisCapabilityKind::HistoricalReplayUnsupported { .. }
        | DeniedBasisCapabilityKind::DurableOverclaim { .. } => {
            WorthQueryRuntimeStateKind::Unsupported
        }
        DeniedBasisCapabilityKind::LowerRuntimeBindingMissing { .. }
        | DeniedBasisCapabilityKind::LowerRuntimeBindingMismatch { .. }
        | DeniedBasisCapabilityKind::Inaccessible { .. }
        | DeniedBasisCapabilityKind::PolicyMasked { .. }
        | DeniedBasisCapabilityKind::TenantMismatched { .. }
        | DeniedBasisCapabilityKind::SchemaIncompatible { .. }
        | DeniedBasisCapabilityKind::OperationIneligible { .. } => {
            WorthQueryRuntimeStateKind::Denied
        }
    }
}

pub(crate) fn state_kind_for_basis_intent_denial(
    kind: &BasisIntentDenialKind,
) -> WorthQueryRuntimeStateKind {
    match kind {
        BasisIntentDenialKind::MalformedIdentifier { .. } => WorthQueryRuntimeStateKind::Denied,
        BasisIntentDenialKind::UnsupportedCompatibilityFamily { .. }
        | BasisIntentDenialKind::UnsupportedFutureNeighbor { .. } => {
            WorthQueryRuntimeStateKind::Unsupported
        }
    }
}

pub(crate) fn authority_lane_for_intent_denial(
    kind: &BasisIntentDenialKind,
    operation_lane: &BasisOperationLaneRequest,
) -> WorthQueryAuthorityLane {
    match kind {
        BasisIntentDenialKind::UnsupportedFutureNeighbor { family, .. } => match family.as_str() {
            "temporal" => WorthQueryAuthorityLane::TemporalExecutionState,
            "async_resource" => WorthQueryAuthorityLane::AsyncResourceState,
            "durable_reload" | "restart_stable_envelope" | "store_backed_parity" => {
                WorthQueryAuthorityLane::BridgeExternalState
            }
            _ => WorthQueryAuthorityLane::AuthoritativeTruth,
        },
        _ if operation_lane == &BasisOperationLaneRequest::MutationPreparation => {
            WorthQueryAuthorityLane::PendingWriteIntent
        }
        _ => WorthQueryAuthorityLane::AuthoritativeTruth,
    }
}
