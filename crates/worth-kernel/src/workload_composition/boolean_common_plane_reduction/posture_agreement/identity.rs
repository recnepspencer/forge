use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_composition::PlanarBooleanCommonPlanePlaneAgreedRequest;
use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlanePostureAgreementReceipt;

pub(super) fn agreed_request_identity(
    plane_agreed_request: &PlanarBooleanCommonPlanePlaneAgreedRequest,
    agreement_receipt: &PlanarBooleanCommonPlanePostureAgreementReceipt,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-common-plane-posture-agreed-request".to_string(),
            format!("request:{}", plane_agreed_request.request_identity()),
            format!(
                "scope-admission:{}",
                plane_agreed_request.scope_admission_identity()
            ),
            format!(
                "plane-agreement:{}",
                plane_agreed_request.plane_agreement_identity()
            ),
            format!("agreement:{}", agreement_receipt.agreement_identity()),
            format!(
                "shared-posture:{}",
                agreement_receipt.shared_posture_identity()
            ),
        ],
    )
}
