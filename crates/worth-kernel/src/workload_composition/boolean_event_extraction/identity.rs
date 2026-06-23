use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::request::PlanarBooleanEventExtractionRequest;

pub(crate) fn event_extraction_request_identity(
    request: &PlanarBooleanEventExtractionRequest,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-event-extraction-request".to_string(),
            format!(
                "reduced-request:{}",
                request.reduced_operand_pair_request_identity()
            ),
            format!("reduced-pair:{}", request.reduced_operand_pair_identity()),
            format!("shared-plane:{}", request.shared_plane_identity()),
            format!("precision:{}", request.precision_agreement_identity()),
            format!("local-frame:{}", request.local_frame_selection_identity()),
            format!("left-projection:{}", request.left_projection_identity()),
            format!("right-projection:{}", request.right_projection_identity()),
            format!(
                "left-projection-stage:{}",
                request.left_projection_stage_identity()
            ),
            format!(
                "right-projection-stage:{}",
                request.right_projection_stage_identity()
            ),
            format!(
                "source-left:{}",
                request.source_left_operand_workload_identity()
            ),
            format!(
                "source-right:{}",
                request.source_right_operand_workload_identity()
            ),
        ],
    )
}
