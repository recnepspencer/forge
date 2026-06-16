use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitSourceEdgeCarrier;
use worth_spatial::facade::planar_boolean_events::PlanarBooleanLoopRole;

fn main() {
    let _ = PlanarBooleanSplitSourceEdgeCarrier {
        recovered_carrier_identity: String::from("forged recovered carrier"),
        operand_side: PlanarBooleanCommonPlaneOperandSide::Left,
        source_face_identity: String::from("face"),
        source_loop_identity: String::from("loop"),
        source_edge_identity: String::from("edge"),
        loop_role: PlanarBooleanLoopRole::OuterBoundary,
        carrier_identity: String::from("carrier"),
        start_source_endpoint_identity: String::from("start"),
        start_projected_endpoint_fact_identity: String::from("start projected"),
        end_source_endpoint_identity: String::from("end"),
        end_projected_endpoint_fact_identity: String::from("end projected"),
        local_frame_identity: String::from("local frame"),
        projection_stage_identity: String::from("projection"),
        precision_basis_identity: String::from("precision"),
    };
}
