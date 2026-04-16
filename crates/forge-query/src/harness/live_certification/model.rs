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
    ForbiddenStreamWidthOverflowSuccess,
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
            Self::ForbiddenStreamWidthOverflowSuccess => "forbidden-stream-width-overflow-success",
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

impl LiveCertificationRow {
    pub fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.hostile_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
    }

    pub fn has_hostile_coverage(&self) -> bool {
        match self.hostile_expectation {
            LiveHostileExpectation::EquivalentToControl => {
                self.control_lane.query_digest == self.hostile_lane.query_digest
                    && self.control_lane.result_digest == self.hostile_lane.result_digest
                    && self.control_lane.delivery_digest == self.hostile_lane.delivery_digest
                    && self.control_lane.replay_digest == self.hostile_lane.replay_digest
                    && self.control_lane.family == self.hostile_lane.family
                    && self.control_lane.outcome_kind == self.hostile_lane.outcome_kind
                    && self.control_lane.outcome_digest == self.hostile_lane.outcome_digest
                    && self.control_lane.basis_digest == self.hostile_lane.basis_digest
                    && self.control_lane.subscription_digest
                        == self.hostile_lane.subscription_digest
                    && self.control_lane.query_digest == self.parity_lane.query_digest
                    && self.control_lane.result_digest == self.parity_lane.result_digest
                    && self.control_lane.delivery_digest == self.parity_lane.delivery_digest
                    && self.control_lane.replay_digest == self.parity_lane.replay_digest
                    && self.control_lane.family == self.parity_lane.family
                    && self.control_lane.outcome_kind == self.parity_lane.outcome_kind
                    && self.control_lane.outcome_digest == self.parity_lane.outcome_digest
                    && self.control_lane.basis_digest == self.parity_lane.basis_digest
                    && self.control_lane.subscription_digest == self.parity_lane.subscription_digest
            }
            LiveHostileExpectation::ReplayEndStateEquivalent => {
                self.control_lane.query_digest == self.hostile_lane.query_digest
                    && self.control_lane.result_digest == self.hostile_lane.result_digest
                    && self.control_lane.delivery_digest == self.hostile_lane.delivery_digest
                    && self.control_lane.family == self.hostile_lane.family
                    && self.control_lane.outcome_kind == self.hostile_lane.outcome_kind
                    && self.control_lane.outcome_digest == self.hostile_lane.outcome_digest
                    && self.control_lane.basis_digest == self.hostile_lane.basis_digest
                    && self.control_lane.query_digest == self.parity_lane.query_digest
                    && self.control_lane.result_digest == self.parity_lane.result_digest
                    && self.control_lane.delivery_digest == self.parity_lane.delivery_digest
                    && self.control_lane.family == self.parity_lane.family
                    && self.control_lane.outcome_kind == self.parity_lane.outcome_kind
                    && self.control_lane.outcome_digest == self.parity_lane.outcome_digest
                    && self.control_lane.basis_digest == self.parity_lane.basis_digest
            }
            LiveHostileExpectation::ReplayStepwiseEquivalent => {
                !self.control_lane.replay_step_delivery_digests.is_empty()
                    && self.control_lane.replay_step_delivery_digests
                        == self.hostile_lane.replay_step_delivery_digests
                    && self.control_lane.replay_step_delivery_digests
                        == self.parity_lane.replay_step_delivery_digests
                    && self.control_lane.query_digest == self.hostile_lane.query_digest
                    && self.control_lane.result_digest == self.hostile_lane.result_digest
                    && self.control_lane.delivery_digest == self.hostile_lane.delivery_digest
                    && self.control_lane.query_digest == self.parity_lane.query_digest
                    && self.control_lane.result_digest == self.parity_lane.result_digest
                    && self.control_lane.delivery_digest == self.parity_lane.delivery_digest
            }
        }
    }
}

impl LiveRejectionRow {
    pub fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
            && !self.hostile_lane.failure_digest.is_empty()
            && (self
                .hostile_lane
                .counter_snapshot
                .live_refresh_denial_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .live_coalescing_denial_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .live_patch_width_overflow_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .live_non_monotonic_sequence_rejection_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .live_change_sequence_gap_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .live_invalid_promotion_rejection_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .live_unsupported_patch_family_rejection_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .locality_breadth_budget_cross_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .locality_widening_denial_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .locality_widening_budget_cross_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .locality_bridge_slice_incompatibility_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .stream_member_width_budget_cross_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .stream_contract_denial_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .locality_unsupported_family_rejection_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .locality_unsupported_predicate_rejection_count()
                > 0)
    }

    pub fn has_hostile_coverage(&self) -> bool {
        self.control_lane.query_digest == self.parity_lane.query_digest
            && self.control_lane.result_digest == self.parity_lane.result_digest
            && self.control_lane.delivery_digest == self.parity_lane.delivery_digest
            && self.control_lane.replay_digest == self.parity_lane.replay_digest
            && self.control_lane.family == self.parity_lane.family
            && self.control_lane.outcome_kind == self.parity_lane.outcome_kind
            && self.control_lane.outcome_digest == self.parity_lane.outcome_digest
            && self.control_lane.basis_digest == self.parity_lane.basis_digest
            && self.control_lane.subscription_digest == self.parity_lane.subscription_digest
    }
}

fn bundle_digest_parts(matrix: &LiveCertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    for row in &matrix.rows {
        parts.push(format!("canonical:{}", row.row_name));
        parts.extend(lane_digest_parts(&row.control_lane, "control"));
        parts.extend(lane_digest_parts(&row.hostile_lane, "hostile"));
        parts.extend(lane_digest_parts(&row.parity_lane, "parity"));
    }
    for row in &matrix.rejection_rows {
        parts.push(format!("rejection:{}", row.row_name));
        parts.extend(lane_digest_parts(&row.control_lane, "control"));
        parts.extend(rejection_digest_parts(
            &row.hostile_lane,
            "hostile_rejection",
        ));
        parts.extend(lane_digest_parts(&row.parity_lane, "parity"));
    }
    parts
}

fn coverage_digest_parts(matrix: &LiveCertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    parts.extend(
        matrix
            .rows
            .iter()
            .map(|row| format!("canonical:{}", row.row_name)),
    );
    parts.extend(
        matrix
            .rejection_rows
            .iter()
            .map(|row| format!("rejection:{}", row.row_name)),
    );
    parts
}

fn lane_digest_parts(bundle: &LiveCertificationBundle, label: &str) -> Vec<String> {
    let mut parts = vec![
        format!("{label}_profile:{:?}", bundle.profile),
        format!("{label}_query_digest:{}", bundle.query_digest),
        format!("{label}_result_digest:{}", bundle.result_digest),
        format!("{label}_delivery_digest:{}", bundle.delivery_digest),
        format!("{label}_replay_digest:{}", bundle.replay_digest),
        format!(
            "{label}_replay_step_count:{}",
            bundle.replay_step_delivery_digests.len()
        ),
        format!("{label}_family:{}", bundle.family.as_str()),
        format!("{label}_outcome_kind:{}", bundle.outcome_kind.as_str()),
        format!("{label}_outcome_digest:{}", bundle.outcome_digest),
        format!("{label}_basis:{}", bundle.basis_digest),
        format!("{label}_subscription:{}", bundle.subscription_digest),
    ];
    parts.extend(
        bundle
            .replay_step_delivery_digests
            .iter()
            .map(|digest| format!("{label}_replay_step_delivery:{digest}")),
    );
    parts.extend(bundle.counter_snapshot.digest_parts(label));
    parts
}

fn rejection_digest_parts(bundle: &LiveRejectionBundle, label: &str) -> Vec<String> {
    let mut parts = vec![
        format!("{label}_profile:{:?}", bundle.profile),
        format!("{label}_failure_class:{}", bundle.failure_class.as_str()),
        format!("{label}_failure_digest:{}", bundle.failure_digest),
    ];
    parts.extend(bundle.counter_snapshot.digest_parts(label));
    parts
}
