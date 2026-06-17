use std::cmp::Ordering;

use crate::workload_platform::planar_boolean_events::segment_identity::PlanarBooleanCanonicalSegment;

use super::counters::PlanarBooleanSegmentPairEnumerationCounters;
use super::denial::{
    PlanarBooleanSegmentPairEnumerationDenial, PlanarBooleanSegmentPairEnumerationDenialKind,
};
use super::product::{
    PlanarBooleanCandidateBroadPhaseReason, PlanarBooleanCandidateEnvelopeBasis,
    PlanarBooleanSegmentCandidateRowReceipt,
};

pub(crate) struct CandidateIndexExecution {
    rows: Vec<PlanarBooleanSegmentCandidateRowReceipt>,
    broad_phase_comparison_count: usize,
}

impl CandidateIndexExecution {
    pub(crate) fn into_rows(self) -> Vec<PlanarBooleanSegmentCandidateRowReceipt> {
        self.rows
    }

    pub(crate) fn broad_phase_comparison_count(&self) -> usize {
        self.broad_phase_comparison_count
    }
}

pub(crate) fn execute_aabb_sweep_candidate_index(
    canonical_segment_set_identity: &str,
    ordered_left: &[&PlanarBooleanCanonicalSegment],
    ordered_right: &[&PlanarBooleanCanonicalSegment],
    planned_counters: PlanarBooleanSegmentPairEnumerationCounters,
) -> Result<CandidateIndexExecution, PlanarBooleanSegmentPairEnumerationDenial> {
    let mut indexed_left = ordered_left
        .iter()
        .copied()
        .map(IndexedSegmentEnvelope::try_from_segment)
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| {
            degenerate_envelope_denial(canonical_segment_set_identity, planned_counters)
        })?;
    indexed_left.sort_by(compare_right_index_order);

    let mut indexed_right = ordered_right
        .iter()
        .copied()
        .map(IndexedSegmentEnvelope::try_from_segment)
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| {
            degenerate_envelope_denial(canonical_segment_set_identity, planned_counters)
        })?;
    indexed_right.sort_by(compare_right_index_order);

    let mut candidate_pairs = Vec::new();
    let mut broad_phase_comparison_count = 0;
    let mut right_cursor = 0;
    let mut active_right = Vec::new();
    for left_envelope in indexed_left {
        while right_cursor < indexed_right.len()
            && indexed_right[right_cursor].envelope.min_x <= left_envelope.envelope.max_x
        {
            active_right.push(indexed_right[right_cursor]);
            right_cursor += 1;
        }
        active_right
            .retain(|right_envelope| right_envelope.envelope.max_x >= left_envelope.envelope.min_x);
        append_left_segment_candidates(
            canonical_segment_set_identity,
            &mut candidate_pairs,
            left_envelope.segment,
            &active_right,
            planned_counters,
            &mut broad_phase_comparison_count,
        )?;
    }
    candidate_pairs
        .sort_by(|left, right| left.candidate_identity().cmp(right.candidate_identity()));
    candidate_pairs.dedup_by(|left, right| left.candidate_identity() == right.candidate_identity());
    Ok(CandidateIndexExecution {
        rows: candidate_pairs,
        broad_phase_comparison_count,
    })
}

fn append_left_segment_candidates(
    canonical_segment_set_identity: &str,
    candidate_pairs: &mut Vec<PlanarBooleanSegmentCandidateRowReceipt>,
    left_segment: &PlanarBooleanCanonicalSegment,
    indexed_right: &[IndexedSegmentEnvelope<'_>],
    planned_counters: PlanarBooleanSegmentPairEnumerationCounters,
    broad_phase_comparison_count: &mut usize,
) -> Result<(), PlanarBooleanSegmentPairEnumerationDenial> {
    let left_envelope = SegmentEnvelope::try_from_segment(left_segment).ok_or_else(|| {
        degenerate_envelope_denial(canonical_segment_set_identity, planned_counters)
    })?;
    for right_envelope in indexed_right {
        *broad_phase_comparison_count += 1;
        if !left_envelope.overlaps(&right_envelope.envelope) {
            continue;
        }
        let envelope_basis = PlanarBooleanCandidateEnvelopeBasis::from_segments(
            left_segment,
            right_envelope.segment,
        )
        .ok_or_else(|| {
            degenerate_envelope_denial(canonical_segment_set_identity, planned_counters)
        })?;
        let row = PlanarBooleanSegmentCandidateRowReceipt::new(
            left_segment.clone(),
            right_envelope.segment.clone(),
            PlanarBooleanCandidateBroadPhaseReason::AabbEnvelopeOverlap,
            envelope_basis,
        )
        .ok_or_else(|| {
            operand_side_mismatch_denial(
                canonical_segment_set_identity,
                planned_counters,
                candidate_pairs.len(),
            )
        })?;
        candidate_pairs.push(row);
    }
    Ok(())
}

fn compare_right_index_order(
    left: &IndexedSegmentEnvelope<'_>,
    right: &IndexedSegmentEnvelope<'_>,
) -> Ordering {
    left.envelope
        .min_x
        .total_cmp(&right.envelope.min_x)
        .then_with(|| left.envelope.max_x.total_cmp(&right.envelope.max_x))
        .then_with(|| {
            left.segment
                .canonical_segment_identity()
                .cmp(right.segment.canonical_segment_identity())
        })
        .then_with(|| {
            left.segment
                .carrier_identity()
                .cmp(right.segment.carrier_identity())
        })
}

fn operand_side_mismatch_denial(
    canonical_segment_set_identity: &str,
    planned_counters: PlanarBooleanSegmentPairEnumerationCounters,
    emitted_pair_breadth: usize,
) -> PlanarBooleanSegmentPairEnumerationDenial {
    PlanarBooleanSegmentPairEnumerationDenial::new(
        PlanarBooleanSegmentPairEnumerationDenialKind::OperandSideMismatch,
        canonical_segment_set_identity,
        PlanarBooleanSegmentPairEnumerationCounters::new(
            planned_counters.left_segment_count(),
            planned_counters.right_segment_count(),
            emitted_pair_breadth,
            planned_counters
                .expected_pair_breadth()
                .saturating_sub(emitted_pair_breadth),
        ),
        "indexed segment-pair planning requires left/right canonical segment operands",
    )
}

fn degenerate_envelope_denial(
    canonical_segment_set_identity: &str,
    planned_counters: PlanarBooleanSegmentPairEnumerationCounters,
) -> PlanarBooleanSegmentPairEnumerationDenial {
    PlanarBooleanSegmentPairEnumerationDenial::new(
        PlanarBooleanSegmentPairEnumerationDenialKind::CandidateEnvelopeInvalid,
        canonical_segment_set_identity,
        PlanarBooleanSegmentPairEnumerationCounters::new(
            planned_counters.left_segment_count(),
            planned_counters.right_segment_count(),
            0,
            planned_counters.expected_pair_breadth(),
        )
        .with_strategy_counts(0, 0, 1, false),
        "Query-owned segment candidate rows require finite, non-collapsed candidate envelopes",
    )
}

#[derive(Clone, Copy)]
struct IndexedSegmentEnvelope<'a> {
    segment: &'a PlanarBooleanCanonicalSegment,
    envelope: SegmentEnvelope,
}

impl<'a> IndexedSegmentEnvelope<'a> {
    fn try_from_segment(segment: &'a PlanarBooleanCanonicalSegment) -> Option<Self> {
        Some(Self {
            segment,
            envelope: SegmentEnvelope::try_from_segment(segment)?,
        })
    }
}

#[derive(Clone, Copy)]
struct SegmentEnvelope {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

impl SegmentEnvelope {
    fn try_from_segment(segment: &PlanarBooleanCanonicalSegment) -> Option<Self> {
        let start = segment.source_ordered_start_endpoint().point();
        let end = segment.source_ordered_end_endpoint().point();
        [start[0], start[1], end[0], end[1]]
            .into_iter()
            .all(f64::is_finite)
            .then_some(())?;
        if start[0] == end[0] && start[1] == end[1] {
            return None;
        }
        Some(Self {
            min_x: start[0].min(end[0]),
            max_x: start[0].max(end[0]),
            min_y: start[1].min(end[1]),
            max_y: start[1].max(end[1]),
        })
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.min_x <= other.max_x
            && other.min_x <= self.max_x
            && self.min_y <= other.max_y
            && other.min_y <= self.max_y
    }
}
