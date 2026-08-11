mod digests;
mod row_validation;

use digests::{bundle_digest_parts, coverage_digest_parts};

use super::super::certification::{
    digest_parts, CanonicalCertificationRow, CertificationMatrix, RejectionCertificationRow,
};
use super::super::profiles::CertificationProfile;
use crate::live::LivePolicyCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveBundleFamily {
    Detail,
    OrderedCollection,
    BoundedMaterialization,
}

impl LiveBundleFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Detail => "detail",
            Self::OrderedCollection => "ordered_collection",
            Self::BoundedMaterialization => "bounded_materialization",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveOutcomeKind {
    Patch,
    Suppressed,
    Refresh,
    ProgressAdvance,
    CoalescedDelivery,
    StreamLoweredDelivery,
}

impl LiveOutcomeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Patch => "patch",
            Self::Suppressed => "suppressed",
            Self::Refresh => "refresh",
            Self::ProgressAdvance => "progress_advance",
            Self::CoalescedDelivery => "coalesced_delivery",
            Self::StreamLoweredDelivery => "stream_lowered_delivery",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveFailureClass {
    ForbiddenWidthBudgetOverflowBehavior,
    ForbiddenRefreshEscapeHatch,
    ForbiddenCoalescingClass,
    NonMonotonicChangeSequence,
    GapfulChangeSequence,
    InvalidLiveBasisPromotion,
    UnsupportedPatchFamily,
    UnsupportedLiveFamily,
    RawCdcLeakageForbidden,
    UnsupportedLocalityFamily,
    UnsupportedLocalityPredicate,
    UnsupportedStreamConsumerContract,
    RawPartitionEventLeakageForbidden,
    RawStreamMemberForbidden,
    RawStreamMemberLeakageForbidden,
    ForbiddenLocalityWidening,
    ForbiddenBroadSuccessLane,
    ForbiddenStreamWindowOverflowSuccess,
    BridgeSliceIncompatibilityDenied,
}

impl LiveFailureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ForbiddenWidthBudgetOverflowBehavior => {
                "forbidden-width-budget-overflow-behavior"
            }
            Self::ForbiddenRefreshEscapeHatch => "forbidden-refresh-escape-hatch",
            Self::ForbiddenCoalescingClass => "forbidden-coalescing-class",
            Self::NonMonotonicChangeSequence => "non-monotonic-change-sequence",
            Self::GapfulChangeSequence => "gapful-change-sequence",
            Self::InvalidLiveBasisPromotion => "invalid-live-basis-promotion",
            Self::UnsupportedPatchFamily => "unsupported-patch-family",
            Self::UnsupportedLiveFamily => "unsupported-live-family",
            Self::RawCdcLeakageForbidden => "raw-cdc-leakage-forbidden",
            Self::UnsupportedLocalityFamily => "unsupported-locality-family",
            Self::UnsupportedLocalityPredicate => "unsupported-locality-predicate",
            Self::UnsupportedStreamConsumerContract => "unsupported-stream-consumer-contract",
            Self::RawPartitionEventLeakageForbidden => "raw-partition-event-leakage-forbidden",
            Self::RawStreamMemberForbidden => "raw-stream-member-forbidden",
            Self::RawStreamMemberLeakageForbidden => "raw-stream-member-leakage-forbidden",
            Self::ForbiddenLocalityWidening => "forbidden-locality-widening",
            Self::ForbiddenBroadSuccessLane => "forbidden-broad-success-lane",
            Self::ForbiddenStreamWindowOverflowSuccess => {
                "forbidden-stream-window-overflow-success"
            }
            Self::BridgeSliceIncompatibilityDenied => "bridge-slice-incompatibility-denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveCertificationBundle {
    pub profile: CertificationProfile,
    pub query_digest: String,
    pub result_digest: String,
    pub delivery_digest: String,
    pub replay_digest: String,
    pub replay_step_delivery_digests: Vec<String>,
    pub family: LiveBundleFamily,
    pub outcome_kind: LiveOutcomeKind,
    pub outcome_digest: String,
    pub basis_digest: String,
    pub subscription_digest: String,
    pub counter_snapshot: LivePolicyCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveRejectionBundle {
    pub profile: CertificationProfile,
    pub failure_class: LiveFailureClass,
    pub failure_digest: String,
    pub counter_snapshot: LivePolicyCounters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum LivePerturbationClass {
    DetailPatchParity,
    DetailReplayEndStateParity,
    DetailReplayStepwiseParity,
    OrderedCollectionPatchParity,
    OrderedCollectionReplayEndStateParity,
    OrderedCollectionReplayStepwiseParity,
    BoundedMaterializationPatchParity,
    BoundedMaterializationReplayEndStateParity,
    BoundedMaterializationReplayStepwiseParity,
    SuppressionParity,
    ProgressBasisParity,
    RefreshFallbackParity,
    CoalescingParity,
    WidthOverflowRejection,
    RefreshRejection,
    CoalescingRejection,
    NonMonotonicSequenceRejection,
    SequenceGapRejection,
    InvalidLivePromotionRejection,
    UnsupportedPatchFamilyRejection,
    UnsupportedLiveFamilyRejection,
    RawCdcLeakageRejection,
    RegionScopedConvergenceParity,
    CollectionPartitionParity,
    BoundedMaterializationRegionParity,
    OffRegionSuppressionParity,
    BroadVsRegionParity,
    StreamContractParity,
    CdcStreamLoweredParity,
    LocalityWideningAdmissionParity,
    LocalityBreadthBudgetEnforcement,
    StreamMemberWidthBudgetEnforcement,
    LocalityWorkAvoidedParity,
    UnsupportedLocalityFamilyRejection,
    UnsupportedLocalityPredicateRejection,
    UnsupportedStreamConsumerRejection,
    RawPartitionLeakageRejection,
    RawStreamMemberForbiddenRejection,
    RawStreamMemberLeakageRejection,
    ForbiddenLocalityWideningRejection,
    ForbiddenBroadSuccessLaneRejection,
    ForbiddenStreamWidthOverflowSuccessRejection,
    ForbiddenStreamWindowOverflowSuccessRejection,
    BridgeSliceIncompatibilityRejection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveHostileExpectation {
    EquivalentToControl,
    ReplayEndStateEquivalent,
    ReplayStepwiseEquivalent,
}
pub type LiveCertificationRow = CanonicalCertificationRow<
    LivePerturbationClass,
    LiveCertificationBundle,
    LiveHostileExpectation,
>;
pub type LiveRejectionRow =
    RejectionCertificationRow<LivePerturbationClass, LiveCertificationBundle, LiveRejectionBundle>;
pub type LiveCertificationMatrix = CertificationMatrix<
    LivePerturbationClass,
    LiveCertificationBundle,
    LiveRejectionBundle,
    LiveHostileExpectation,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneFiveLiveCertificationArtifact {
    pub suite_name: &'static str,
    pub certification_bundle_digest: String,
    pub coverage_matrix_digest: String,
    pub counter_snapshot: LivePolicyCounters,
    pub matrix: LiveCertificationMatrix,
}

impl LiveCertificationMatrix {
    pub fn into_milestone_five_artifact(self) -> MilestoneFiveLiveCertificationArtifact {
        let certification_bundle_digest = digest_parts(&bundle_digest_parts(&self));
        let coverage_matrix_digest = digest_parts(&coverage_digest_parts(&self));
        let counter_snapshot = self.aggregate_counters();

        MilestoneFiveLiveCertificationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest,
            coverage_matrix_digest,
            counter_snapshot,
            matrix: self,
        }
    }

    fn aggregate_counters(&self) -> LivePolicyCounters {
        let mut aggregate = self
            .rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .fold(LivePolicyCounters::default(), |mut aggregate, lane| {
                aggregate.absorb(&lane.counter_snapshot);
                aggregate
            });
        for row in &self.rejection_rows {
            aggregate.absorb(&row.control_lane.counter_snapshot);
            aggregate.absorb(&row.hostile_lane.counter_snapshot);
            aggregate.absorb(&row.parity_lane.counter_snapshot);
        }
        aggregate
    }
}

impl LiveCertificationBundle {
    pub fn has_required_outputs(&self) -> bool {
        !self.query_digest.is_empty()
            && !self.result_digest.is_empty()
            && !self.delivery_digest.is_empty()
            && !self.replay_digest.is_empty()
            && !self.outcome_digest.is_empty()
            && !self.basis_digest.is_empty()
            && !self.subscription_digest.is_empty()
            && self.counter_snapshot.has_activity()
    }
}
