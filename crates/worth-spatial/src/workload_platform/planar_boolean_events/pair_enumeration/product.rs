use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_events::segment_identity::PlanarBooleanCanonicalSegment;

use super::counters::PlanarBooleanSegmentPairEnumerationCounters;
use super::denial::PlanarBooleanSegmentPairEnumerationDenial;
use super::identity::pair_work_item_identity;
use super::product_validation::validate_candidate_index_product_input;
use super::work_item::PlanarBooleanSegmentPairWorkItem;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCandidateIndexStrategy {
    AabbSweep,
}

impl PlanarBooleanCandidateIndexStrategy {
    pub(crate) fn query_key(self) -> &'static str {
        match self {
            Self::AabbSweep => "aabb-sweep-v1",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCandidateIndexFallbackPosture {
    NotUsed,
    FullBreadthNonProduction,
}

impl PlanarBooleanCandidateIndexFallbackPosture {
    pub(crate) fn query_key(self) -> &'static str {
        match self {
            Self::NotUsed => "not-used",
            Self::FullBreadthNonProduction => "full-breadth-non-production",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCandidateIndexLifecycleOutcome {
    Bound,
}

impl PlanarBooleanCandidateIndexLifecycleOutcome {
    pub(crate) fn query_key(self) -> &'static str {
        match self {
            Self::Bound => "bound",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCandidateBroadPhaseReason {
    AabbEnvelopeOverlap,
}

impl PlanarBooleanCandidateBroadPhaseReason {
    fn query_key(self) -> &'static str {
        match self {
            Self::AabbEnvelopeOverlap => "aabb-envelope-overlap",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanarBooleanCandidateEnvelopeBasis {
    left_min_x: f64,
    left_max_x: f64,
    left_min_y: f64,
    left_max_y: f64,
    right_min_x: f64,
    right_max_x: f64,
    right_min_y: f64,
    right_max_y: f64,
}

impl PlanarBooleanCandidateEnvelopeBasis {
    pub(crate) fn from_segments(
        left: &PlanarBooleanCanonicalSegment,
        right: &PlanarBooleanCanonicalSegment,
    ) -> Option<Self> {
        let left = segment_envelope(left)?;
        let right = segment_envelope(right)?;
        Some(Self {
            left_min_x: left[0],
            left_max_x: left[1],
            left_min_y: left[2],
            left_max_y: left[3],
            right_min_x: right[0],
            right_max_x: right[1],
            right_min_y: right[2],
            right_max_y: right[3],
        })
    }

    fn identity_parts(self) -> Vec<String> {
        [
            ("left-min-x", self.left_min_x),
            ("left-max-x", self.left_max_x),
            ("left-min-y", self.left_min_y),
            ("left-max-y", self.left_max_y),
            ("right-min-x", self.right_min_x),
            ("right-max-x", self.right_max_x),
            ("right-min-y", self.right_min_y),
            ("right-max-y", self.right_max_y),
        ]
        .into_iter()
        .map(|(name, value)| format!("{name}-bits:{}", value.to_bits()))
        .collect()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSegmentCandidateRowReceipt {
    left: PlanarBooleanCanonicalSegment,
    right: PlanarBooleanCanonicalSegment,
    broad_phase_reason: PlanarBooleanCandidateBroadPhaseReason,
    envelope_basis: PlanarBooleanCandidateEnvelopeBasis,
    candidate_identity: String,
}

impl PlanarBooleanSegmentCandidateRowReceipt {
    pub(crate) fn new(
        left: PlanarBooleanCanonicalSegment,
        right: PlanarBooleanCanonicalSegment,
        broad_phase_reason: PlanarBooleanCandidateBroadPhaseReason,
        envelope_basis: PlanarBooleanCandidateEnvelopeBasis,
    ) -> Option<Self> {
        if left.operand_side() != PlanarBooleanCommonPlaneOperandSide::Left
            || right.operand_side() != PlanarBooleanCommonPlaneOperandSide::Right
            || left.local_frame_identity() != right.local_frame_identity()
            || left.precision_basis_identity() != right.precision_basis_identity()
        {
            return None;
        }
        let row = Self {
            left,
            right,
            broad_phase_reason,
            envelope_basis,
            candidate_identity: String::new(),
        };
        Some(Self {
            candidate_identity: candidate_row_identity(&row),
            ..row
        })
    }

    pub fn left(&self) -> &PlanarBooleanCanonicalSegment {
        &self.left
    }

    pub fn right(&self) -> &PlanarBooleanCanonicalSegment {
        &self.right
    }

    pub fn candidate_identity(&self) -> &str {
        &self.candidate_identity
    }

    pub fn broad_phase_reason(&self) -> PlanarBooleanCandidateBroadPhaseReason {
        self.broad_phase_reason
    }

    pub fn envelope_basis(&self) -> PlanarBooleanCandidateEnvelopeBasis {
        self.envelope_basis
    }

    pub fn left_source_face_identity(&self) -> &str {
        self.left.source_face_identity()
    }

    pub fn left_source_loop_identity(&self) -> &str {
        self.left.source_loop_identity()
    }

    pub fn left_source_edge_identity(&self) -> &str {
        self.left.source_edge_identity()
    }

    pub fn right_source_face_identity(&self) -> &str {
        self.right.source_face_identity()
    }

    pub fn right_source_loop_identity(&self) -> &str {
        self.right.source_loop_identity()
    }

    pub fn right_source_edge_identity(&self) -> &str {
        self.right.source_edge_identity()
    }

    pub fn local_frame_identity(&self) -> &str {
        self.left.local_frame_identity()
    }

    pub fn precision_basis_identity(&self) -> &str {
        self.left.precision_basis_identity()
    }

    pub(crate) fn to_work_item(&self) -> PlanarBooleanSegmentPairWorkItem {
        PlanarBooleanSegmentPairWorkItem::from_candidate_row(self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PlanarBooleanSegmentCandidateIndexProductInput {
    pub canonical_segment_set_identity: String,
    pub declaration_digest: String,
    pub plan_digest: String,
    pub envelope_digest: String,
    pub strategy: PlanarBooleanCandidateIndexStrategy,
    pub fallback_posture: PlanarBooleanCandidateIndexFallbackPosture,
    pub lifecycle_outcome: PlanarBooleanCandidateIndexLifecycleOutcome,
    pub counters: PlanarBooleanSegmentPairEnumerationCounters,
    pub rows: Vec<PlanarBooleanSegmentCandidateRowReceipt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSegmentCandidateIndexProduct {
    product_identity: String,
    canonical_segment_set_identity: String,
    declaration_digest: String,
    plan_digest: String,
    envelope_digest: String,
    strategy: PlanarBooleanCandidateIndexStrategy,
    fallback_posture: PlanarBooleanCandidateIndexFallbackPosture,
    lifecycle_outcome: PlanarBooleanCandidateIndexLifecycleOutcome,
    counters: PlanarBooleanSegmentPairEnumerationCounters,
    rows: Vec<PlanarBooleanSegmentCandidateRowReceipt>,
}

impl PlanarBooleanSegmentCandidateIndexProduct {
    pub(crate) fn new(
        input: PlanarBooleanSegmentCandidateIndexProductInput,
    ) -> Result<Self, PlanarBooleanSegmentPairEnumerationDenial> {
        validate_candidate_index_product_input(&input)?;
        let product = Self {
            product_identity: String::new(),
            canonical_segment_set_identity: input.canonical_segment_set_identity,
            declaration_digest: input.declaration_digest,
            plan_digest: input.plan_digest,
            envelope_digest: input.envelope_digest,
            strategy: input.strategy,
            fallback_posture: input.fallback_posture,
            lifecycle_outcome: input.lifecycle_outcome,
            counters: input.counters,
            rows: input.rows,
        };
        Ok(Self {
            product_identity: candidate_index_product_identity(&product),
            ..product
        })
    }

    pub fn product_identity(&self) -> &str {
        &self.product_identity
    }

    pub fn canonical_segment_set_identity(&self) -> &str {
        &self.canonical_segment_set_identity
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn plan_digest(&self) -> &str {
        &self.plan_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn strategy(&self) -> PlanarBooleanCandidateIndexStrategy {
        self.strategy
    }

    pub fn fallback_posture(&self) -> PlanarBooleanCandidateIndexFallbackPosture {
        self.fallback_posture
    }

    pub fn lifecycle_outcome(&self) -> PlanarBooleanCandidateIndexLifecycleOutcome {
        self.lifecycle_outcome
    }

    pub fn counters(&self) -> PlanarBooleanSegmentPairEnumerationCounters {
        self.counters
    }

    pub fn rows(&self) -> &[PlanarBooleanSegmentCandidateRowReceipt] {
        &self.rows
    }

    pub(crate) fn work_items(&self) -> Vec<PlanarBooleanSegmentPairWorkItem> {
        self.rows
            .iter()
            .map(PlanarBooleanSegmentCandidateRowReceipt::to_work_item)
            .collect()
    }

    pub fn certifies_production_candidate_discovery(&self) -> bool {
        self.fallback_posture == PlanarBooleanCandidateIndexFallbackPosture::NotUsed
            && !self.counters.fallback_used()
    }
}

fn candidate_row_identity(row: &PlanarBooleanSegmentCandidateRowReceipt) -> String {
    let mut parts = vec![
        "planar-boolean-segment-candidate-row".to_string(),
        format!("left:{}", row.left.canonical_segment_identity()),
        format!("right:{}", row.right.canonical_segment_identity()),
        format!("left-carrier:{}", row.left.carrier_identity()),
        format!("right-carrier:{}", row.right.carrier_identity()),
        format!("left-face:{}", row.left.source_face_identity()),
        format!("right-face:{}", row.right.source_face_identity()),
        format!("left-loop:{}", row.left.source_loop_identity()),
        format!("right-loop:{}", row.right.source_loop_identity()),
        format!("left-edge:{}", row.left.source_edge_identity()),
        format!("right-edge:{}", row.right.source_edge_identity()),
        format!("reason:{}", row.broad_phase_reason.query_key()),
        format!("local-frame:{}", row.left.local_frame_identity()),
        format!("precision-basis:{}", row.left.precision_basis_identity()),
        format!("pair:{}", pair_work_item_identity(&row.left, &row.right)),
    ];
    parts.extend(row.envelope_basis.identity_parts());
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

fn candidate_index_product_identity(product: &PlanarBooleanSegmentCandidateIndexProduct) -> String {
    let mut parts = vec![
        "planar-boolean-segment-candidate-index-product".to_string(),
        format!(
            "canonical-segment-set:{}",
            product.canonical_segment_set_identity
        ),
        format!("declaration:{}", product.declaration_digest),
        format!("plan:{}", product.plan_digest),
        format!("envelope:{}", product.envelope_digest),
        format!("strategy:{}", product.strategy.query_key()),
        format!("fallback:{}", product.fallback_posture.query_key()),
        format!("lifecycle:{}", product.lifecycle_outcome.query_key()),
        format!("left-count:{}", product.counters.left_segment_count()),
        format!("right-count:{}", product.counters.right_segment_count()),
        format!(
            "possible-pairs:{}",
            product.counters.expected_pair_breadth()
        ),
        format!("emitted:{}", product.counters.emitted_pair_breadth()),
        format!(
            "culled:{}",
            product.counters.query_index_culled_pair_count()
        ),
        format!(
            "broad-phase-comparisons:{}",
            product.counters.broad_phase_comparison_count()
        ),
    ];
    parts.extend(
        product
            .rows
            .iter()
            .map(|row| format!("candidate:{}", row.candidate_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

fn segment_envelope(segment: &PlanarBooleanCanonicalSegment) -> Option<[f64; 4]> {
    let start = segment.source_ordered_start_endpoint().point();
    let end = segment.source_ordered_end_endpoint().point();
    [start[0], start[1], end[0], end[1]]
        .into_iter()
        .all(f64::is_finite)
        .then_some([
            start[0].min(end[0]),
            start[0].max(end[0]),
            start[1].min(end[1]),
            start[1].max(end[1]),
        ])
}
