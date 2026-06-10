use worth_spatial::facade::planar_segment_segment::{
    CertifiedProjectedSegment2D, CertifiedSegmentSegment2D, CertifiedSegmentSegment2DDenialKind,
};

use super::proof_fixture::{certified_frame, projected_point, segment_contracts};

#[test]
fn certified_segment_segment_2d_denies_mixed_movement_rotation_projection_basis() {
    let stable_frame = certified_frame(
        "mixed-movement-stable",
        "movement:rotation-cancelled",
        "transform:stable",
    );
    let rotated_frame = certified_frame(
        "mixed-movement-rotated",
        "movement:semantic-rotation",
        "transform:rotated",
    );
    let contracts = segment_contracts("mixed-movement");
    let tool_edge = CertifiedProjectedSegment2D::from_projected_endpoints(
        "segment:tool",
        projected_point(
            "mixed-movement-stable",
            &stable_frame,
            "point:tool:start",
            [0.0, 0.0, 0.0],
        ),
        projected_point(
            "mixed-movement-stable",
            &stable_frame,
            "point:tool:end",
            [4.0e-9, 0.0, 0.0],
        ),
    )
    .expect("tool edge");
    let host_edge = CertifiedProjectedSegment2D::from_projected_endpoints(
        "segment:host",
        projected_point(
            "mixed-movement-rotated",
            &rotated_frame,
            "point:host:start",
            [2.0e-9, 0.0, 0.0],
        ),
        projected_point(
            "mixed-movement-rotated",
            &rotated_frame,
            "point:host:end",
            [6.0e-9, 0.0, 0.0],
        ),
    )
    .expect("host edge");

    let denial = compile_denial(tool_edge, host_edge, &contracts);

    assert_eq!(
        denial.kind(),
        CertifiedSegmentSegment2DDenialKind::FrameBasisMismatch
    );
}

#[test]
fn certified_segment_segment_2d_denies_mixed_frame_projection_basis_before_predicates() {
    let first_frame = certified_frame(
        "mixed-frame-first",
        "movement:rotation-cancelled",
        "transform:first-frame",
    );
    let second_frame = certified_frame(
        "mixed-frame-second",
        "movement:rotation-cancelled",
        "transform:second-frame",
    );
    let contracts = segment_contracts("mixed-frame");
    let tool_edge = CertifiedProjectedSegment2D::from_projected_endpoints(
        "segment:tool",
        projected_point(
            "mixed-frame-first",
            &first_frame,
            "point:tool:start",
            [0.0, 0.0, 0.0],
        ),
        projected_point(
            "mixed-frame-first",
            &first_frame,
            "point:tool:end",
            [4.0e-9, 0.0, 0.0],
        ),
    )
    .expect("tool edge");
    let host_edge = CertifiedProjectedSegment2D::from_projected_endpoints(
        "segment:host",
        projected_point(
            "mixed-frame-second",
            &second_frame,
            "point:host:start",
            [2.0e-9, 0.0, 0.0],
        ),
        projected_point(
            "mixed-frame-second",
            &second_frame,
            "point:host:end",
            [6.0e-9, 0.0, 0.0],
        ),
    )
    .expect("host edge");

    let denial = compile_denial(tool_edge, host_edge, &contracts);

    assert_eq!(
        denial.kind(),
        CertifiedSegmentSegment2DDenialKind::FrameBasisMismatch
    );
}

fn compile_denial<SC, PC>(
    tool_edge: CertifiedProjectedSegment2D,
    host_edge: CertifiedProjectedSegment2D,
    contracts: &worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DContracts<
        SC,
        PC,
    >,
) -> worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DDenial
where
    SC: forge_query::facade::ForgeQueryDomainOperatingContext<
        worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DQueryDomain,
    >,
    PC: forge_query::facade::ForgeQueryDomainOperatingContext<
        worth_spatial::facade::planar_predicates::PlanarPredicateAuthorityQueryDomain,
    >,
{
    match CertifiedSegmentSegment2D::classify(tool_edge, host_edge)
        .within_topology_basis("topology:mixed-basis")
        .compile(contracts)
    {
        Ok(_) => panic!("mixed projection basis must deny before predicate certification"),
        Err(denial) => denial,
    }
}
