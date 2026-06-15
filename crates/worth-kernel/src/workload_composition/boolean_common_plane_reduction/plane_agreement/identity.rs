use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_composition::PlanarBooleanCommonPlaneScopeAdmittedRequest;
use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneAgreementReceipt;

pub(super) fn agreed_request_identity(
    admitted_request: &PlanarBooleanCommonPlaneScopeAdmittedRequest,
    agreement_receipt: &PlanarBooleanCommonPlaneAgreementReceipt,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-common-plane-plane-agreed-request".to_string(),
            format!("request:{}", admitted_request.request_identity()),
            format!(
                "scope-admission:{}",
                admitted_request.scope_admission_identity()
            ),
            format!("agreement:{}", agreement_receipt.agreement_identity()),
            format!("shared-plane:{}", agreement_receipt.shared_plane_identity()),
        ],
    )
}
