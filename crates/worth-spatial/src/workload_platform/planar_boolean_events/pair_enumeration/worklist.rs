use super::counters::PlanarBooleanSegmentPairEnumerationCounters;
use super::denial::{
    PlanarBooleanSegmentPairEnumerationDenial, PlanarBooleanSegmentPairEnumerationDenialKind,
};
use super::identity::{canonical_segment_set_identity, pair_enumeration_identity};
use super::query_index::query_candidate_index_product;
use super::receipt::PlanarBooleanSegmentPairEnumerationReceipt;
use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_events::segment_identity::PlanarBooleanCanonicalSegment;
use std::cmp::Ordering;

pub(crate) fn enumerate_segment_pairs(
    left: &[PlanarBooleanCanonicalSegment],
    right: &[PlanarBooleanCanonicalSegment],
) -> Result<PlanarBooleanSegmentPairEnumerationReceipt, PlanarBooleanSegmentPairEnumerationDenial> {
    let canonical_segment_set_identity = canonical_segment_set_identity(left, right);
    let ordered_left = sorted_segment_pair_worklist(left);
    let ordered_right = sorted_segment_pair_worklist(right);
    let planned_counters = planned_segment_pair_enumeration_counters(&ordered_left, &ordered_right);
    reject_unrepresentable_pair_breadth(&canonical_segment_set_identity, planned_counters)?;
    reject_operand_side_mismatches(
        &canonical_segment_set_identity,
        &ordered_left,
        &ordered_right,
        planned_counters,
    )?;

    let candidate_index_product = query_candidate_index_product(
        &canonical_segment_set_identity,
        &ordered_left,
        &ordered_right,
        planned_counters,
    )?;
    let counters = candidate_index_product.counters();
    certify_indexed_pair_breadth(&canonical_segment_set_identity, counters)?;
    let work_items = candidate_index_product
        .rows()
        .iter()
        .map(|row| row.to_work_item())
        .collect::<Vec<_>>();

    let identity = pair_enumeration_identity(
        &canonical_segment_set_identity,
        candidate_index_product.product_identity(),
        candidate_index_product.declaration_digest(),
        candidate_index_product.plan_digest(),
        candidate_index_product.envelope_digest(),
        counters,
        &work_items,
    );
    Ok(PlanarBooleanSegmentPairEnumerationReceipt::new(
        identity,
        candidate_index_product,
    ))
}

fn reject_operand_side_mismatches(
    canonical_segment_set_identity: &str,
    ordered_left: &[&PlanarBooleanCanonicalSegment],
    ordered_right: &[&PlanarBooleanCanonicalSegment],
    planned_counters: PlanarBooleanSegmentPairEnumerationCounters,
) -> Result<(), PlanarBooleanSegmentPairEnumerationDenial> {
    if ordered_left
        .iter()
        .any(|segment| segment.operand_side() != PlanarBooleanCommonPlaneOperandSide::Left)
        || ordered_right
            .iter()
            .any(|segment| segment.operand_side() != PlanarBooleanCommonPlaneOperandSide::Right)
    {
        return Err(PlanarBooleanSegmentPairEnumerationDenial::new(
            PlanarBooleanSegmentPairEnumerationDenialKind::OperandSideMismatch,
            canonical_segment_set_identity,
            PlanarBooleanSegmentPairEnumerationCounters::new(
                planned_counters.left_segment_count(),
                planned_counters.right_segment_count(),
                0,
                planned_counters.expected_pair_breadth(),
            ),
            "indexed segment-pair planning requires operands to be side-partitioned before broad-phase culling",
        ));
    }
    Ok(())
}

fn sorted_segment_pair_worklist(
    segments: &[PlanarBooleanCanonicalSegment],
) -> Vec<&PlanarBooleanCanonicalSegment> {
    let mut ordered_segments = segments.iter().collect::<Vec<_>>();
    ordered_segments.sort_by(compare_segment_pair_worklist_order);
    ordered_segments
}

fn planned_segment_pair_enumeration_counters(
    ordered_left: &[&PlanarBooleanCanonicalSegment],
    ordered_right: &[&PlanarBooleanCanonicalSegment],
) -> PlanarBooleanSegmentPairEnumerationCounters {
    PlanarBooleanSegmentPairEnumerationCounters::new(ordered_left.len(), ordered_right.len(), 0, 0)
}

fn reject_unrepresentable_pair_breadth(
    canonical_segment_set_identity: &str,
    planned_counters: PlanarBooleanSegmentPairEnumerationCounters,
) -> Result<(), PlanarBooleanSegmentPairEnumerationDenial> {
    if planned_counters.expected_pair_breadth_overflowed() {
        return Err(PlanarBooleanSegmentPairEnumerationDenial::new(
            PlanarBooleanSegmentPairEnumerationDenialKind::PairBreadthOverflow,
            canonical_segment_set_identity,
            planned_counters,
            "segment-pair enumeration breadth exceeds representable in-memory worklist capacity",
        ));
    }
    Ok(())
}

fn certify_indexed_pair_breadth(
    canonical_segment_set_identity: &str,
    counters: PlanarBooleanSegmentPairEnumerationCounters,
) -> Result<(), PlanarBooleanSegmentPairEnumerationDenial> {
    if counters
        .emitted_pair_breadth()
        .saturating_add(counters.skipped_pair_count())
        != counters.expected_pair_breadth()
        || counters.query_index_candidate_count() != counters.emitted_pair_breadth()
        || counters.query_index_culled_pair_count() != counters.skipped_pair_count()
    {
        return Err(PlanarBooleanSegmentPairEnumerationDenial::new(
            PlanarBooleanSegmentPairEnumerationDenialKind::EmittedPairBreadthMismatch,
            canonical_segment_set_identity,
            counters,
            "indexed segment-pair enumeration must account for emitted and culled pair breadth",
        ));
    }
    Ok(())
}

fn compare_segment_pair_worklist_order(
    left: &&PlanarBooleanCanonicalSegment,
    right: &&PlanarBooleanCanonicalSegment,
) -> Ordering {
    left.canonical_segment_identity()
        .cmp(right.canonical_segment_identity())
        .then_with(|| left.carrier_identity().cmp(right.carrier_identity()))
        .then_with(|| {
            left.source_face_identity()
                .cmp(right.source_face_identity())
        })
        .then_with(|| {
            left.source_loop_identity()
                .cmp(right.source_loop_identity())
        })
        .then_with(|| {
            left.source_edge_identity()
                .cmp(right.source_edge_identity())
        })
        .then_with(|| {
            left.projection_stage_identity()
                .cmp(right.projection_stage_identity())
        })
        .then_with(|| {
            left.local_frame_identity()
                .cmp(right.local_frame_identity())
        })
        .then_with(|| {
            left.precision_basis_identity()
                .cmp(right.precision_basis_identity())
        })
}
