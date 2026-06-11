use worth_spatial::facade::planar_segment_segment::{
    CertifiedProjectedSegment2D, CertifiedSegmentSegment2D,
};

use super::proof_fixture::{certified_frame, projected_point, segment_contracts};

#[test]
fn movement_rotation_posture_participates_in_segment_evidence() {
    let stable = classify_with_posture("segment-move-stable", "movement:stable");
    let rotated = classify_with_posture("segment-move-rotated", "movement:rotated-equivalent");

    assert_eq!(stable.classification(), rotated.classification());
    assert_ne!(
        stable.mutation_evidence().evidence_digest(),
        rotated.mutation_evidence().evidence_digest()
    );
}

fn classify_with_posture(
    world: &'static str,
    posture: &'static str,
) -> worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DReceipt {
    let frame = certified_frame(world, posture, "transform:posture-specific");
    let contracts = segment_contracts(world);
    let tool_edge = CertifiedProjectedSegment2D::from_projected_endpoints(
        "segment:tool-edge",
        projected_point(world, &frame, "point:tool:start", [0.0, 0.0, 0.0]),
        projected_point(world, &frame, "point:tool:end", [4.0e-9, 4.0e-9, 0.0]),
    )
    .expect("tool edge");
    let host_edge = CertifiedProjectedSegment2D::from_projected_endpoints(
        "segment:host-edge",
        projected_point(world, &frame, "point:host:start", [0.0, 4.0e-9, 0.0]),
        projected_point(world, &frame, "point:host:end", [4.0e-9, 0.0, 0.0]),
    )
    .expect("host edge");

    CertifiedSegmentSegment2D::classify(tool_edge, host_edge)
        .within_topology_basis("topology:movement-rotation")
        .compile(&contracts)
        .expect("compiled plan")
        .certify()
        .expect("receipt")
}
