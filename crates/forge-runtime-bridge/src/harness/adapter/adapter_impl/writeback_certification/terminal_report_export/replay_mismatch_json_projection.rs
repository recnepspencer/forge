use serde_json::json;

use super::super::replay_mismatch::{
    WritebackReplayMismatchMatrix, WritebackRestartReplayMismatchMatrix,
};

pub(in crate::harness::adapter::adapter_impl) fn replay_mismatch_matrix_json(
    matrix: &WritebackReplayMismatchMatrix,
) -> serde_json::Value {
    json!({
        "expected_replay_digest": matrix.expected_replay_digest(),
        "expected_semantic_digest": matrix.expected_semantic_digest(),
        "expected_causality_digest": matrix.expected_causality_digest(),
        "expected_effect_intent_digest": matrix.expected_effect_intent_digest(),
        "expected_effect_intent_patch_canonical_basis": matrix
            .expected_effect_intent_patch_canonical_basis(),
        "replayed_replay_digest": matrix.replayed_replay_digest(),
        "replayed_semantic_digest": matrix.replayed_semantic_digest(),
        "replayed_effect_intent_digest": matrix.replayed_effect_intent_digest(),
        "replayed_effect_intent_patch_canonical_basis": matrix
            .replayed_effect_intent_patch_canonical_basis(),
        "replayed_causality_digest": matrix.replayed_causality_digest(),
        "failure_kind": format!("{:?}", matrix.failure_kind()),
        "failure_message": matrix.failure_message(),
        "semantic_mismatch_detected": matrix.semantic_mismatch_detected(),
        "diagnostic_detail_changed": matrix.diagnostic_detail_changed(),
        "restart_replay_matrix": replay_mismatch_restart_replay_json(matrix),
    })
}

pub(in crate::harness::adapter::adapter_impl) fn replay_mismatch_restart_replay_json(
    matrix: &WritebackReplayMismatchMatrix,
) -> serde_json::Value {
    restart_replay_json(matrix.restart_replay())
}

fn restart_replay_json(restart_replay: &WritebackRestartReplayMismatchMatrix) -> serde_json::Value {
    json!({
        "rebuilt_replay_digest": restart_replay.rebuilt_replay_digest(),
        "rebuilt_semantic_digest": restart_replay.rebuilt_semantic_digest(),
        "rebuilt_effect_intent_digest": restart_replay.rebuilt_effect_intent_digest(),
        "rebuilt_effect_intent_patch_canonical_basis": restart_replay
            .rebuilt_effect_intent_patch_canonical_basis(),
        "rebuilt_failure_kind": format!("{:?}", restart_replay.rebuilt_failure_kind()),
        "rebuilt_failure_message": restart_replay.rebuilt_failure_message(),
        "restart_mismatch_detected": restart_replay.restart_mismatch_detected(),
    })
}
