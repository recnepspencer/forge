use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::receipt::PlanarBooleanCommonPlaneReducedOperandPairReceipt;

pub(crate) fn reduced_operand_pair_identity(
    receipt: &PlanarBooleanCommonPlaneReducedOperandPairReceipt,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-common-plane-reduced-operand-pair".to_string(),
            format!(
                "shared-plane-receipt:{}",
                receipt.shared_plane_receipt_identity()
            ),
            format!("shared-plane:{}", receipt.shared_plane_identity()),
            format!("plane-agreement:{}", receipt.plane_agreement_identity()),
            format!(
                "local-frame-selection:{}",
                receipt.local_frame_selection_identity()
            ),
            format!(
                "projection-basis:{}",
                receipt.projection_local_basis_identity()
            ),
            format!(
                "left-projection-stage:{}",
                receipt.left_projection_stage_identity()
            ),
            format!(
                "right-projection-stage:{}",
                receipt.right_projection_stage_identity()
            ),
            format!("left-projection:{}", receipt.left_projection_identity()),
            format!("right-projection:{}", receipt.right_projection_identity()),
            format!(
                "ordering:{:?}-{:?}",
                receipt.ordering_contract().first_slot_side(),
                receipt.ordering_contract().second_slot_side()
            ),
        ],
    )
}
