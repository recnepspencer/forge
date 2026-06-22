use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_events::endpoint_normalization::{
    normalize_endpoint_order, validate_segment_endpoint_admissibility,
};
use crate::workload_platform::planar_boolean_events::segment_carriers::{
    PlanarBooleanSegmentCarrier, PlanarBooleanSegmentCarrierEndpointFacts,
};
use crate::workload_platform::planar_boolean_events::PlanarBooleanSegmentPairEnumerationReceipt;
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanCandidateIndexFallbackPosture, PlanarBooleanCandidateIndexLifecycleOutcome,
    PlanarBooleanCandidateIndexStrategy,
};

use super::{
    PlanarBooleanCanonicalSegment, PlanarBooleanCanonicalSegmentSet,
    PlanarBooleanCanonicalSegmentSetDenial,
};

#[test]
fn segment_pair_enumeration_exposes_sweep_locality_not_hidden_full_breadth_scan() {
    let left = (0..10)
        .map(|index| {
            let x = index as f64;
            canonical_segment_from_side_edge_and_points(
                PlanarBooleanCommonPlaneOperandSide::Left,
                format!("left-edge-{index}"),
                [x, 0.0],
                [x, 1.0],
            )
            .expect("left sweep segment should canonicalize")
        })
        .collect::<Vec<_>>();
    let right = (0..10)
        .map(|index| {
            let x = index as f64;
            canonical_segment_from_side_edge_and_points(
                PlanarBooleanCommonPlaneOperandSide::Right,
                format!("right-edge-{index}"),
                [x, 0.5],
                [x, 1.5],
            )
            .expect("right sweep segment should canonicalize")
        })
        .collect::<Vec<_>>();

    let receipt = PlanarBooleanCanonicalSegmentSet::for_pair_enumeration_test(left, right)
        .segment_pair_enumeration_receipt()
        .expect("sweep-local pair enumeration should certify");
    let counters = receipt.counters();

    assert_eq!(counters.expected_pair_breadth(), 100);
    assert_eq!(counters.emitted_pair_breadth(), 10);
    assert_eq!(counters.query_index_culled_pair_count(), 90);
    assert_eq!(counters.broad_phase_comparison_count(), 10);
    assert_eq!(
        receipt.candidate_index_strategy(),
        PlanarBooleanCandidateIndexStrategy::AabbSweep
    );
    assert_eq!(
        receipt.fallback_posture(),
        PlanarBooleanCandidateIndexFallbackPosture::NotUsed
    );
    assert_eq!(
        receipt.candidate_index_lifecycle_outcome(),
        PlanarBooleanCandidateIndexLifecycleOutcome::Bound
    );
    assert!(
        receipt
            .candidate_index_product()
            .certifies_production_candidate_discovery(),
        "full-breadth or fallback candidate discovery must not certify production proof"
    );
    assert!(
        counters.broad_phase_comparison_count() < counters.expected_pair_breadth(),
        "candidate-index proof must expose actual broad-phase locality"
    );

    let mut expected_source_edge_pairs = (0..10)
        .map(|index| (format!("left-edge-{index}"), format!("right-edge-{index}")))
        .collect::<Vec<_>>();
    expected_source_edge_pairs.sort();
    assert_eq!(
        candidate_source_edge_pairs(&receipt),
        expected_source_edge_pairs,
        "candidate completeness proof must be by authored source-edge identity"
    );
}

#[test]
fn query_candidate_index_product_owns_rows_counters_culls_and_fallback_posture() {
    let receipt = separated_sweep_receipt();
    let product = receipt.candidate_index_product();
    let counters = product.counters();

    assert_eq!(
        receipt.candidate_index_product_identity(),
        product.product_identity()
    );
    assert_eq!(
        receipt.query_index_declaration_digest(),
        product.declaration_digest()
    );
    assert_eq!(receipt.query_index_plan_digest(), product.plan_digest());
    assert_eq!(
        receipt.query_index_envelope_digest(),
        product.envelope_digest()
    );
    assert_eq!(receipt.candidate_rows(), product.rows());
    assert_eq!(counters.expected_pair_breadth(), 100);
    assert_eq!(counters.query_index_candidate_count(), product.rows().len());
    assert_eq!(counters.query_index_culled_pair_count(), 90);
    assert_eq!(
        product.fallback_posture(),
        PlanarBooleanCandidateIndexFallbackPosture::NotUsed
    );
}

#[test]
fn boolean_event_pipeline_consumes_candidate_index_product_not_local_work_items() {
    let receipt = separated_sweep_receipt();

    assert_eq!(receipt.candidate_rows().len(), receipt.work_items().len());
    for (row, work_item) in receipt
        .candidate_rows()
        .iter()
        .zip(receipt.work_items().iter())
    {
        assert_eq!(
            row.left().canonical_segment_identity(),
            work_item.left().canonical_segment_identity()
        );
        assert_eq!(
            row.right().canonical_segment_identity(),
            work_item.right().canonical_segment_identity()
        );
    }
}

fn separated_sweep_receipt() -> PlanarBooleanSegmentPairEnumerationReceipt {
    let left = (0..10)
        .map(|index| {
            let x = index as f64;
            canonical_segment_from_side_edge_and_points(
                PlanarBooleanCommonPlaneOperandSide::Left,
                format!("left-edge-{index}"),
                [x, 0.0],
                [x, 1.0],
            )
            .expect("left sweep segment should canonicalize")
        })
        .collect::<Vec<_>>();
    let right = (0..10)
        .map(|index| {
            let x = index as f64;
            canonical_segment_from_side_edge_and_points(
                PlanarBooleanCommonPlaneOperandSide::Right,
                format!("right-edge-{index}"),
                [x, 0.5],
                [x, 1.5],
            )
            .expect("right sweep segment should canonicalize")
        })
        .collect::<Vec<_>>();

    PlanarBooleanCanonicalSegmentSet::for_pair_enumeration_test(left, right)
        .segment_pair_enumeration_receipt()
        .expect("sweep-local pair enumeration should certify")
}

fn canonical_segment_from_side_edge_and_points(
    side: PlanarBooleanCommonPlaneOperandSide,
    source_edge_identity: impl Into<String>,
    start: [f64; 2],
    end: [f64; 2],
) -> Result<PlanarBooleanCanonicalSegment, PlanarBooleanCanonicalSegmentSetDenial> {
    let carrier = PlanarBooleanSegmentCarrier::for_canonical_segment_test_on_side_with_source_edge(
        side,
        source_edge_identity,
        test_endpoint_for_point(start),
        test_endpoint_for_point(end),
    );
    validate_segment_endpoint_admissibility(&carrier)?;
    Ok(PlanarBooleanCanonicalSegment::from_carrier(
        &carrier,
        normalize_endpoint_order(&carrier),
    ))
}

fn candidate_source_edge_pairs(
    receipt: &PlanarBooleanSegmentPairEnumerationReceipt,
) -> Vec<(String, String)> {
    let mut pairs = receipt
        .candidate_rows()
        .iter()
        .map(|row| {
            (
                row.left_source_edge_identity().to_string(),
                row.right_source_edge_identity().to_string(),
            )
        })
        .collect::<Vec<_>>();
    pairs.sort();
    pairs
}

fn test_endpoint_for_point(point: [f64; 2]) -> PlanarBooleanSegmentCarrierEndpointFacts {
    PlanarBooleanSegmentCarrierEndpointFacts::from_projected_loop_boundary(
        point,
        format!("source endpoint {point:?}"),
        "test projected loop",
        "test projection stage",
        "test projection local basis",
    )
}
