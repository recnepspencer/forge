use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt;

use crate::workload_composition::PlanarBooleanCommonPlanePrecisionAgreedRequest;

pub(super) fn identified_request_identity(
    precision_agreed_request: &PlanarBooleanCommonPlanePrecisionAgreedRequest,
    identity_receipt: &PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-common-plane-shared-plane-identified-request".to_string(),
            format!("request:{}", precision_agreed_request.request_identity()),
            format!(
                "scope-admission:{}",
                precision_agreed_request.scope_admission_identity()
            ),
            format!(
                "plane-agreement:{}",
                precision_agreed_request.plane_agreement_identity()
            ),
            format!(
                "posture-agreement:{}",
                precision_agreed_request.posture_agreement_identity()
            ),
            format!(
                "precision-agreement:{}",
                precision_agreed_request.precision_agreement_identity()
            ),
            format!(
                "shared-plane-receipt:{}",
                identity_receipt.shared_plane_receipt_identity()
            ),
            format!("shared-plane:{}", identity_receipt.shared_plane_identity()),
        ],
    )
}
