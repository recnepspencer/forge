use worth_kernel::workload_composition::PlanarBooleanCommonPlaneReducedOperandPairRequest;
use worth_spatial::facade::planar_boolean_events::PlanarBooleanCanonicalSegmentSet;

#[path = "public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

#[test]
fn canonical_segments_preserve_operand_partition_and_carrier_count() {
    reduced_pair_support::run_with_large_stack(|| {
        let canonical_segments = canonical_segments_from_catalog(
            "phase7.2 canonical segment partition and carrier count",
        );
        let carriers = carrier_set_from_catalog(
            "phase7.2 canonical segment partition and carrier count comparison",
        );

        assert_eq!(canonical_segments.left().len(), carriers.left().len());
        assert_eq!(canonical_segments.right().len(), carriers.right().len());
        assert_eq!(
            canonical_segments.total_segment_count(),
            carriers.total_carrier_count()
        );
        assert!(canonical_segments.left().iter().all(|segment| {
            segment.normalized_endpoints().low().parameter() == 0.0
                && segment.normalized_endpoints().high().parameter() == 1.0
        }));
    });
}

#[test]
fn canonical_segment_identity_is_replay_stable_from_reduced_pair_carriers() {
    reduced_pair_support::run_with_large_stack(|| {
        let first = canonical_segments_from_catalog("phase7.2 canonical segment replay stable");
        let second = canonical_segments_from_catalog("phase7.2 canonical segment replay stable");

        assert_eq!(
            canonical_segment_identities(&first),
            canonical_segment_identities(&second)
        );
    });
}

#[test]
fn canonical_segments_preserve_projection_precision_and_source_sense() {
    reduced_pair_support::run_with_large_stack(|| {
        let (_, operand_a, operand_b) =
            reduced_pair_support::event_carrier_projected_operand_requests_from_catalog(
                "phase7.2 canonical segment provenance",
            );
        let reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("reduced pair should certify");
        let canonical_segments = reduced_pair
            .segment_carrier_set()
            .expect("carrier set should certify")
            .canonical_segment_set()
            .expect("canonical segments should certify");

        for segment in canonical_segments
            .left()
            .iter()
            .chain(canonical_segments.right().iter())
        {
            assert!(!segment.canonical_segment_identity().is_empty());
            assert!(!segment.carrier_identity().is_empty());
            assert!(!segment.source_face_identity().is_empty());
            assert!(!segment.source_loop_identity().is_empty());
            assert!(!segment.source_edge_identity().is_empty());
            assert_eq!(
                segment.local_frame_identity(),
                reduced_pair.local_frame_selection_identity()
            );
            assert_eq!(
                segment.precision_basis_identity(),
                reduced_pair.precision_agreement_identity()
            );
            assert_endpoint_order_is_canonical(
                segment.normalized_endpoints().low().point(),
                segment.normalized_endpoints().high().point(),
            );
        }
    });
}

#[test]
fn canonical_segment_identity_does_not_depend_on_debug_or_display_strings() {
    reduced_pair_support::run_with_large_stack(|| {
        let canonical_segments =
            canonical_segments_from_catalog("phase7.2 canonical segment debug independence");
        let segment = canonical_segments
            .left()
            .first()
            .expect("catalog pair should produce a left canonical segment");

        let debug_rendering = format!("{segment:?}");
        assert_ne!(segment.canonical_segment_identity(), debug_rendering);
        assert!(!segment
            .canonical_segment_identity()
            .contains("PlanarBooleanCanonicalSegment"));
        assert!(!segment.canonical_segment_identity().contains("[0"));
    });
}

fn canonical_segments_from_catalog(
    readiness_scope: &'static str,
) -> PlanarBooleanCanonicalSegmentSet {
    carrier_set_from_catalog(readiness_scope)
        .canonical_segment_set()
        .expect("canonical segments should derive from carrier proof")
}

fn carrier_set_from_catalog(
    readiness_scope: &'static str,
) -> worth_spatial::facade::planar_boolean_events::PlanarBooleanSegmentCarrierSet {
    let (_, operand_a, operand_b) =
        reduced_pair_support::event_carrier_projected_operand_requests_from_catalog(
            readiness_scope,
        );
    PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
        operand_a, operand_b,
    )
    .expect("reduced pair should certify")
    .segment_carrier_set()
    .expect("segment carrier set should certify")
}

fn canonical_segment_identities(
    canonical_segments: &PlanarBooleanCanonicalSegmentSet,
) -> Vec<String> {
    canonical_segments
        .left()
        .iter()
        .chain(canonical_segments.right().iter())
        .map(|segment| segment.canonical_segment_identity().to_string())
        .collect()
}

fn assert_endpoint_order_is_canonical(low: [f64; 2], high: [f64; 2]) {
    assert!(
        low[0] < high[0] || (low[0] == high[0] && low[1] <= high[1]),
        "normalized endpoint order must be lexicographic low/high"
    );
}
