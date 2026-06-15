use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::receipt::PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt;

pub(crate) fn operand_projection_consumption_identity(
    receipt: &PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-common-plane-operand-projection-consumption".to_string(),
            format!("operand-side:{:?}", receipt.operand_side()),
            format!(
                "local-frame-selection:{}",
                receipt.local_frame_selection_identity()
            ),
            format!(
                "shared-plane-receipt:{}",
                receipt.shared_plane_receipt_identity()
            ),
            format!("shared-plane:{}", receipt.shared_plane_identity()),
            format!("plane-agreement:{}", receipt.plane_agreement_identity()),
            format!("projection-stage:{}", receipt.projection_stage_identity()),
            format!(
                "surface-support:{}",
                receipt.upstream_surface_support_identity()
            ),
            format!(
                "plane-support:{}",
                receipt.certified_plane_support_identity()
            ),
            format!(
                "projection-basis:{}",
                receipt.projection_local_basis_identity()
            ),
            format!("projected-entities:{}", receipt.projected_entity_count()),
        ],
    )
}
