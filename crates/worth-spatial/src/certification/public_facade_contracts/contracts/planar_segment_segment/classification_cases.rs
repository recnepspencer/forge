use worth_spatial::facade::planar_segment_segment::{
    CertifiedProjectedSegment2D, CertifiedSegmentSegment2D,
    CertifiedSegmentSegment2DClassification, SegmentContactPolicy,
};

use super::proof_fixture::{certified_frame, projected_point, segment_contracts};

#[test]
fn certified_segment_segment_2d_classifies_contact_classes_without_predicate_wiring() {
    let cases = [
        (
            "proper",
            ([0.0, 0.0, 0.0], [4.0e-9, 4.0e-9, 0.0]),
            ([0.0, 4.0e-9, 0.0], [4.0e-9, 0.0, 0.0]),
            CertifiedSegmentSegment2DClassification::ProperCrossing,
        ),
        (
            "endpoint",
            ([0.0, 0.0, 0.0], [4.0e-9, 0.0, 0.0]),
            ([4.0e-9, 0.0, 0.0], [4.0e-9, 3.0e-9, 0.0]),
            CertifiedSegmentSegment2DClassification::EndpointTouch,
        ),
        (
            "collinear-overlap",
            ([0.0, 0.0, 0.0], [4.0e-9, 0.0, 0.0]),
            ([2.0e-9, 0.0, 0.0], [6.0e-9, 0.0, 0.0]),
            CertifiedSegmentSegment2DClassification::CollinearOverlap,
        ),
        (
            "collinear-disjoint",
            ([0.0, 0.0, 0.0], [1.0e-9, 0.0, 0.0]),
            ([2.0e-9, 0.0, 0.0], [3.0e-9, 0.0, 0.0]),
            CertifiedSegmentSegment2DClassification::CollinearDisjoint,
        ),
        (
            "identical",
            ([0.0, 0.0, 0.0], [4.0e-9, 0.0, 0.0]),
            ([0.0, 0.0, 0.0], [4.0e-9, 0.0, 0.0]),
            CertifiedSegmentSegment2DClassification::Identical,
        ),
        (
            "reverse-identical",
            ([0.0, 0.0, 0.0], [4.0e-9, 0.0, 0.0]),
            ([4.0e-9, 0.0, 0.0], [0.0, 0.0, 0.0]),
            CertifiedSegmentSegment2DClassification::ReverseIdentical,
        ),
        (
            "disjoint",
            ([0.0, 0.0, 0.0], [1.0e-9, 0.0, 0.0]),
            ([0.0, 2.0e-9, 0.0], [1.0e-9, 2.0e-9, 0.0]),
            CertifiedSegmentSegment2DClassification::Disjoint,
        ),
    ];

    for (case, first, second, expected) in cases {
        let frame = certified_frame(case, "movement:rotation-cancelled", "transform:stable");
        let contracts = segment_contracts(case);
        let first_segment = CertifiedProjectedSegment2D::from_projected_endpoints(
            format!("segment:{case}:tool"),
            projected_point(case, &frame, "point:tool:start", first.0),
            projected_point(case, &frame, "point:tool:end", first.1),
        )
        .expect("first segment");
        let second_segment = CertifiedProjectedSegment2D::from_projected_endpoints(
            format!("segment:{case}:host"),
            projected_point(case, &frame, "point:host:start", second.0),
            projected_point(case, &frame, "point:host:end", second.1),
        )
        .expect("second segment");

        let plan = CertifiedSegmentSegment2D::classify(first_segment, second_segment)
            .within_topology_basis(format!("topology:{case}:neighborhood"))
            .with_policy(SegmentContactPolicy::CertifyContactsDenyImprintRequired)
            .compile(&contracts)
            .expect("compiled plan");

        assert_eq!(plan.required_predicate_count(), 4);
        assert_eq!(plan.projection_receipt_count(), 4);
        let receipt = plan.certify().expect("segment classification receipt");
        assert_eq!(receipt.classification(), expected, "{case}");
        assert_eq!(receipt.counters().segment_pairs_evaluated(), 1);
        assert_eq!(receipt.counters().projection_receipts_consumed(), 4);
        assert_eq!(receipt.counters().orientation_receipts_consumed(), 4);
    }
}

#[test]
fn certified_segment_segment_2d_requires_explicit_topology_basis() {
    let frame = certified_frame(
        "missing-topology-basis",
        "movement:rotation-cancelled",
        "transform:stable",
    );
    let contracts = segment_contracts("missing-topology-basis");
    let first_segment = CertifiedProjectedSegment2D::from_projected_endpoints(
        "segment:tool",
        projected_point(
            "missing-topology-basis",
            &frame,
            "point:tool:start",
            [0.0, 0.0, 0.0],
        ),
        projected_point(
            "missing-topology-basis",
            &frame,
            "point:tool:end",
            [4.0e-9, 0.0, 0.0],
        ),
    )
    .expect("first segment");
    let second_segment = CertifiedProjectedSegment2D::from_projected_endpoints(
        "segment:host",
        projected_point(
            "missing-topology-basis",
            &frame,
            "point:host:start",
            [2.0e-9, 0.0, 0.0],
        ),
        projected_point(
            "missing-topology-basis",
            &frame,
            "point:host:end",
            [6.0e-9, 0.0, 0.0],
        ),
    )
    .expect("second segment");

    let denial = match CertifiedSegmentSegment2D::classify(first_segment, second_segment)
        .compile(&contracts)
    {
        Ok(_) => panic!("topology basis must be explicit"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DDenialKind::MissingTopologyBasisIdentity
    );
}

#[test]
fn certified_segment_segment_2d_reports_policy_required_collinear_overlap_without_imprint() {
    let world = "policy-required-overlap";
    let frame = certified_frame(world, "movement:rotation-cancelled", "transform:stable");
    let contracts = segment_contracts(world);
    let first_segment = CertifiedProjectedSegment2D::from_projected_endpoints(
        "segment:tool",
        projected_point(world, &frame, "point:tool:start", [0.0, 0.0, 0.0]),
        projected_point(world, &frame, "point:tool:end", [4.0e-9, 0.0, 0.0]),
    )
    .expect("first segment");
    let second_segment = CertifiedProjectedSegment2D::from_projected_endpoints(
        "segment:host",
        projected_point(world, &frame, "point:host:start", [2.0e-9, 0.0, 0.0]),
        projected_point(world, &frame, "point:host:end", [6.0e-9, 0.0, 0.0]),
    )
    .expect("second segment");

    let receipt = CertifiedSegmentSegment2D::classify(first_segment, second_segment)
        .within_topology_basis("topology:policy-required-overlap")
        .with_policy(SegmentContactPolicy::RequireImprintForCollinearOverlap)
        .compile(&contracts)
        .expect("compiled plan")
        .certify()
        .expect("receipt");

    assert_eq!(
        receipt.classification(),
        CertifiedSegmentSegment2DClassification::PolicyRequiredOrUncertain
    );
    assert_eq!(receipt.counters().orientation_receipts_consumed(), 4);
}
