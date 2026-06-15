use worth_spatial::facade::planar_boolean_common_plane::{
    PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
    PlanarBooleanCommonPlaneOperandSide,
};

fn main() {
    let _receipt = PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt {
        operand_side: PlanarBooleanCommonPlaneOperandSide::Left,
        local_frame_selection_identity: String::new(),
        shared_plane_receipt_identity: String::new(),
        shared_plane_identity: String::new(),
        plane_agreement_identity: String::new(),
        projection_stage_identity: String::new(),
        upstream_surface_support_identity: String::new(),
        certified_plane_support_identity: String::new(),
        projection_local_basis_identity: String::new(),
        projected_entity_count: 0,
        operand_projection_consumption_identity: String::new(),
    };
}
