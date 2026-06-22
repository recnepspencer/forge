use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneLocalFrameSelectionReceipt;

use super::receipt::PlanarBooleanCommonPlaneLocalFrameSelectedRequest;

pub(crate) fn selected_request_identity(
    request: &PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
    receipt: &PlanarBooleanCommonPlaneLocalFrameSelectionReceipt,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-common-plane-local-frame-selected-request".to_string(),
            format!(
                "shared-plane-request:{}",
                request
                    .shared_plane_identified_request()
                    .shared_plane_identified_request_identity()
            ),
            format!(
                "shared-plane-receipt:{}",
                receipt.shared_plane_receipt_identity()
            ),
            format!("shared-plane:{}", receipt.shared_plane_identity()),
            format!("plane-agreement:{}", receipt.plane_agreement_identity()),
            format!("local-frame-fact:{}", receipt.local_frame_fact_digest()),
            format!("frame:{}", receipt.frame_identity()),
            format!("precision:{}", receipt.precision_fact_digest()),
            format!("topology:{}", receipt.topology_basis_identity()),
            format!(
                "movement-rotation:{}",
                receipt.movement_rotation_posture_identity()
            ),
        ],
    )
}
