use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionCertificationBundleSealed, BridgeSubscriptionCertificationCounterSnapshot,
    BridgeSubscriptionSourceArtifactIndex,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionReferenceWorkloadLaneKind {
    AuthoritativeLive,
    DiagnosticsTierVariation,
    HostileAdapterVariation,
    HistoricalReplay,
    HistoricalBasisReplay,
    BranchLocal,
    SharedFanout,
    DivergentSharingRejection,
    StaleCheckpointRejection,
    RestartResume,
    Continuation,
    DeniedContinuation,
    PreviewDiscard,
    PreviewPromotion,
    CanonicalOrderingHostility,
    StrategyLoweringProvenance,
    BundleInsufficiency,
}

impl BridgeSubscriptionReferenceWorkloadLaneKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative-live",
            Self::DiagnosticsTierVariation => "diagnostics-tier-variation",
            Self::HostileAdapterVariation => "hostile-adapter-variation",
            Self::HistoricalReplay => "historical-replay",
            Self::HistoricalBasisReplay => "historical-basis-replay",
            Self::BranchLocal => "branch-local",
            Self::SharedFanout => "shared-fanout",
            Self::DivergentSharingRejection => "divergent-sharing-rejection",
            Self::StaleCheckpointRejection => "stale-checkpoint-rejection",
            Self::RestartResume => "restart-resume",
            Self::Continuation => "continuation",
            Self::DeniedContinuation => "denied-continuation",
            Self::PreviewDiscard => "preview-discard",
            Self::PreviewPromotion => "preview-promotion",
            Self::CanonicalOrderingHostility => "canonical-ordering-hostility",
            Self::StrategyLoweringProvenance => "strategy-lowering-provenance",
            Self::BundleInsufficiency => "bundle-insufficiency",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionReferenceWorkloadFamilyKind {
    DetailExact,
    CollectionMembership,
}

impl BridgeSubscriptionReferenceWorkloadFamilyKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DetailExact => "detail-exact",
            Self::CollectionMembership => "collection-membership",
        }
    }
}

const FIRST_SHIP_LANE_MATRIX: [BridgeSubscriptionReferenceWorkloadLaneKind; 17] = [
    BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive,
    BridgeSubscriptionReferenceWorkloadLaneKind::DiagnosticsTierVariation,
    BridgeSubscriptionReferenceWorkloadLaneKind::HostileAdapterVariation,
    BridgeSubscriptionReferenceWorkloadLaneKind::HistoricalReplay,
    BridgeSubscriptionReferenceWorkloadLaneKind::HistoricalBasisReplay,
    BridgeSubscriptionReferenceWorkloadLaneKind::BranchLocal,
    BridgeSubscriptionReferenceWorkloadLaneKind::SharedFanout,
    BridgeSubscriptionReferenceWorkloadLaneKind::DivergentSharingRejection,
    BridgeSubscriptionReferenceWorkloadLaneKind::StaleCheckpointRejection,
    BridgeSubscriptionReferenceWorkloadLaneKind::RestartResume,
    BridgeSubscriptionReferenceWorkloadLaneKind::Continuation,
    BridgeSubscriptionReferenceWorkloadLaneKind::DeniedContinuation,
    BridgeSubscriptionReferenceWorkloadLaneKind::PreviewDiscard,
    BridgeSubscriptionReferenceWorkloadLaneKind::PreviewPromotion,
    BridgeSubscriptionReferenceWorkloadLaneKind::CanonicalOrderingHostility,
    BridgeSubscriptionReferenceWorkloadLaneKind::StrategyLoweringProvenance,
    BridgeSubscriptionReferenceWorkloadLaneKind::BundleInsufficiency,
];

impl BridgeSubscriptionReferenceWorkloadLaneKind {
    pub const fn first_ship_matrix() -> &'static [Self] {
        &FIRST_SHIP_LANE_MATRIX
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadLaneRequest {
    lane_kind: BridgeSubscriptionReferenceWorkloadLaneKind,
    family_kind: BridgeSubscriptionReferenceWorkloadFamilyKind,
}

impl BridgeSubscriptionReferenceWorkloadLaneRequest {
    pub fn new(
        lane_kind: BridgeSubscriptionReferenceWorkloadLaneKind,
        family_kind: BridgeSubscriptionReferenceWorkloadFamilyKind,
    ) -> Self {
        Self {
            lane_kind,
            family_kind,
        }
    }

    pub fn lane_kind(&self) -> BridgeSubscriptionReferenceWorkloadLaneKind {
        self.lane_kind
    }

    pub fn family_kind(&self) -> BridgeSubscriptionReferenceWorkloadFamilyKind {
        self.family_kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadLaneReport {
    lane_kind: BridgeSubscriptionReferenceWorkloadLaneKind,
    family_kind: BridgeSubscriptionReferenceWorkloadFamilyKind,
    source_artifact_index_digest: Arc<str>,
    certification_bundle_digest: Arc<str>,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionReferenceWorkloadLaneReport {
    pub(crate) fn from_bundle(
        request: BridgeSubscriptionReferenceWorkloadLaneRequest,
        source_artifact_index: &BridgeSubscriptionSourceArtifactIndex,
        bundle: &BridgeSubscriptionCertificationBundleSealed,
    ) -> Self {
        let counters = BridgeSubscriptionCertificationCounterSnapshot::combine([
            *source_artifact_index.counters(),
            *bundle.counters(),
        ]);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-reference-workload-lane-report|lane={}|family={}|source-index={}|bundle={}|counters={}",
            request.lane_kind().as_str(),
            request.family_kind().as_str(),
            source_artifact_index.digest(),
            bundle.digest(),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            lane_kind: request.lane_kind(),
            family_kind: request.family_kind(),
            source_artifact_index_digest: Arc::from(source_artifact_index.digest()),
            certification_bundle_digest: Arc::from(bundle.digest()),
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-reference-workload-lane-report:sha256:{digest:x}"
            )),
        }
    }

    pub fn lane_kind(&self) -> BridgeSubscriptionReferenceWorkloadLaneKind {
        self.lane_kind
    }

    pub fn family_kind(&self) -> BridgeSubscriptionReferenceWorkloadFamilyKind {
        self.family_kind
    }

    pub fn source_artifact_index_digest(&self) -> &str {
        self.source_artifact_index_digest.as_ref()
    }

    pub fn certification_bundle_digest(&self) -> &str {
        self.certification_bundle_digest.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
