use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanLoopRole, PlanarBooleanSegmentCarrier, PlanarBooleanSegmentCarrierEndpointFacts,
};

fn main() {
    let _ = PlanarBooleanSegmentCarrier {
        operand_side: PlanarBooleanCommonPlaneOperandSide::Left,
        source_face_identity: String::from("face"),
        source_loop_identity: String::from("loop"),
        source_edge_identity: String::from("edge"),
        loop_role: PlanarBooleanLoopRole::OuterBoundary,
        start: unavailable_endpoint_fact(),
        end: unavailable_endpoint_fact(),
        local_frame_identity: String::from("frame"),
        projection_stage_identity: String::from("projection"),
        precision_basis_identity: String::from("precision"),
        carrier_identity: String::from("forged"),
    };
}

fn unavailable_endpoint_fact() -> PlanarBooleanSegmentCarrierEndpointFacts {
    panic!("compile-fail fixture must never construct endpoint facts")
}
