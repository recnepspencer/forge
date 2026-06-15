use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlanePrecisionAgreementReceipt;

use crate::workload_composition::PlanarBooleanCommonPlanePostureAgreedRequest;

pub(super) fn agreed_request_identity(
    posture_agreed_request: &PlanarBooleanCommonPlanePostureAgreedRequest,
    precision_receipt: &PlanarBooleanCommonPlanePrecisionAgreementReceipt,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-common-plane-precision-agreed-request".to_string(),
            format!("request:{}", posture_agreed_request.request_identity()),
            format!(
                "scope-admission:{}",
                posture_agreed_request.scope_admission_identity()
            ),
            format!(
                "plane-agreement:{}",
                posture_agreed_request.plane_agreement_identity()
            ),
            format!(
                "posture-agreement:{}",
                posture_agreed_request.posture_agreement_identity()
            ),
            format!(
                "precision-receipt:{}",
                precision_receipt.precision_agreement_receipt_identity()
            ),
            format!("precision:{}", precision_receipt.precision_fact_digest()),
            format!(
                "local-frame:{}",
                precision_receipt.local_frame_fact_digest()
            ),
            format!("topology:{}", precision_receipt.topology_basis_identity()),
            format!(
                "movement-rotation:{}",
                precision_receipt.movement_rotation_posture_identity()
            ),
        ],
    )
}
