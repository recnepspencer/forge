use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::receipt::PlanarBooleanCommonPlaneOperandBProjectedRequest;

pub(crate) fn operand_b_projected_request_identity(
    request: &PlanarBooleanCommonPlaneOperandBProjectedRequest,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-common-plane-operand-b-projected-request".to_string(),
            format!(
                "source-operand:{}",
                request.source_operand_workload_identity()
            ),
            format!(
                "local-frame-selection:{}",
                request
                    .local_frame_selected_request()
                    .local_frame_selection_identity()
            ),
            format!(
                "operand-projection:{}",
                request
                    .projection_receipt()
                    .operand_projection_consumption_identity()
            ),
            format!(
                "projection-stage:{}",
                request.projection_receipt().projection_stage_identity()
            ),
        ],
    )
}
