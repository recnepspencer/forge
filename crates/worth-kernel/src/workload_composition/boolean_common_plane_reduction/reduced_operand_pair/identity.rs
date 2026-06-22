use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::receipt::PlanarBooleanCommonPlaneReducedOperandPairRequest;

pub(crate) fn reduced_operand_pair_request_identity(
    request: &PlanarBooleanCommonPlaneReducedOperandPairRequest,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-common-plane-reduced-operand-pair-request".to_string(),
            format!(
                "source-left:{}",
                request.source_left_operand_workload_identity()
            ),
            format!(
                "source-right:{}",
                request.source_right_operand_workload_identity()
            ),
            format!(
                "local-frame-selection:{}",
                request.local_frame_selection_identity()
            ),
            format!("reduced-pair:{}", request.reduced_operand_pair_identity()),
            format!(
                "ordering:{:?}-{:?}",
                request.ordering_contract().first_slot_side(),
                request.ordering_contract().second_slot_side()
            ),
        ],
    )
}
