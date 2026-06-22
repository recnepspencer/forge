use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::admitted_scope::PlanarBooleanCommonPlaneAdmittedOperandScope;
use crate::workload_composition::PlanarBooleanCommonPlaneReductionRequest;

pub(super) fn admitted_request_identity(
    request: &PlanarBooleanCommonPlaneReductionRequest,
    admitted_scope: PlanarBooleanCommonPlaneAdmittedOperandScope,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-common-plane-scope-admitted-request".to_string(),
            format!("request:{}", request.request_identity()),
            format!("scope:{}", admitted_scope.query_key()),
            format!(
                "recipe:{}",
                request.operand_pair_recipe().recipe().query_key()
            ),
        ],
    )
}
