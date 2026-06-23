use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanCanonicalSegment, PlanarBooleanLoopRole, PlanarBooleanNormalizedEndpointPair,
};

fn main() {
    let _ = PlanarBooleanCanonicalSegment {
        operand_side: PlanarBooleanCommonPlaneOperandSide::Left,
        source_face_identity: String::from("face"),
        source_loop_identity: String::from("loop"),
        source_edge_identity: String::from("edge"),
        loop_role: PlanarBooleanLoopRole::OuterBoundary,
        carrier_identity: String::from("carrier"),
        normalized_endpoints: unavailable_normalized_endpoints(),
        local_frame_identity: String::from("frame"),
        projection_stage_identity: String::from("projection"),
        precision_basis_identity: String::from("precision"),
        canonical_segment_identity: String::from("forged"),
    };
}

fn unavailable_normalized_endpoints() -> PlanarBooleanNormalizedEndpointPair {
    panic!("compile-fail fixture must never construct normalized endpoint facts")
}
