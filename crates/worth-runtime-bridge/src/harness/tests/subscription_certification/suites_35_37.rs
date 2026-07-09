mod declared_lane_admission;
mod family_strategy_distinction;
mod lane_matrix_audit;
mod phase_17_sufficiency;

use super::support::*;
use crate::facade::{
    BridgeRuntimePolicy, BridgeSubscriptionCertificationComparisonOutcome,
    BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionReferenceWorkloadComponentIdSet,
    BridgeSubscriptionReferenceWorkloadFamilyKind,
    BridgeSubscriptionReferenceWorkloadLaneCoverageRole,
    BridgeSubscriptionReferenceWorkloadLaneIdSet, BridgeSubscriptionReferenceWorkloadLaneKind,
    BridgeSubscriptionReferenceWorkloadLaneRequest,
    BridgeSubscriptionReferenceWorkloadProductIdSet,
    BridgeSubscriptionReferenceWorkloadRejectionKind,
    BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet,
};

fn product_ids() -> BridgeSubscriptionReferenceWorkloadProductIdSet {
    BridgeSubscriptionReferenceWorkloadProductIdSet::from_declared_product_labels(
        (0..128).map(|slot| format!("product-{slot:03}")),
    )
}

fn component_ids() -> BridgeSubscriptionReferenceWorkloadComponentIdSet {
    BridgeSubscriptionReferenceWorkloadComponentIdSet::from_declared_component_labels([
        "steel", "rubber", "copper", "glass", "labor",
    ])
}

fn lane_ids() -> BridgeSubscriptionReferenceWorkloadLaneIdSet {
    BridgeSubscriptionReferenceWorkloadLaneIdSet::from_declared_lane_labels([
        "authoritative-live",
        "time-only-routing",
        "historical-replay",
        "historical-basis-replay",
        "branch-local",
        "shared-fanout",
        "divergent-sharing-rejection",
        "stale-checkpoint-rejection",
        "restart-resume",
        "continuation",
        "denied-continuation",
        "preview-discard",
        "preview-promotion",
        "hostile-adapter-variation",
        "diagnostics-tier-variation",
        "canonical-ordering-hostility",
        "strategy-lowering-provenance",
        "bundle-insufficiency",
    ])
}

fn all_lane_requests() -> Vec<BridgeSubscriptionReferenceWorkloadLaneRequest> {
    vec![
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::HostileAdapterVariation,
            BridgeSubscriptionReferenceWorkloadFamilyKind::CollectionMembership,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::TimeOnlyRouting,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::HistoricalReplay,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::HistoricalBasisReplay,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::BranchLocal,
            BridgeSubscriptionReferenceWorkloadFamilyKind::CollectionMembership,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::SharedFanout,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::DivergentSharingRejection,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::StaleCheckpointRejection,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::RestartResume,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::Continuation,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::DeniedContinuation,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::PreviewDiscard,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::PreviewPromotion,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::DiagnosticsTierVariation,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::CanonicalOrderingHostility,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::StrategyLoweringProvenance,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::BundleInsufficiency,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
    ]
}
