use serde_json::json;

use super::super::super::terminal_report_export::writeback_counter_snapshot_json;
use super::super::feedback_loop::{
    FeedbackAuthorityBoundaryMatrix, FeedbackBoundednessProof, FeedbackChangedEffectMatrix,
    FeedbackIdempotenceReport, FeedbackInterleavedTruthMatrix, FeedbackLoopPreventionReport,
    FeedbackReplayBundleReport, FeedbackRestartReplayMatrix, WritebackFeedbackLoopMatrix,
};

pub(in crate::harness::adapter::adapter_impl) fn feedback_loop_matrix_json(
    matrix: &WritebackFeedbackLoopMatrix,
) -> serde_json::Value {
    json!({
        "writeback_digest": matrix.writeback_digest(),
        "writeback_effect_artifact_digest": matrix.writeback_effect_artifact_digest(),
        "effect_intent_digest": matrix.effect_intent_digest(),
        "effect_intent_patch_canonical_basis": matrix.effect_intent_patch_canonical_basis(),
        "causality_digest": matrix.causality_digest(),
        "replay_digest": matrix.replay_digest(),
        "mutation_plan_digest": matrix.mutation_plan_digest(),
        "replay_bundle_report": replay_bundle_report_json(matrix.replay_bundle_report()),
        "idempotence_report": idempotence_report_json(matrix.idempotence_report()),
        "loop_prevention_report": loop_prevention_report_json(matrix.loop_prevention_report()),
        "authority_boundary_matrix": authority_boundary_matrix_json(matrix.authority_boundary_matrix()),
        "changed_effect_feedback_matrix": changed_effect_feedback_matrix_json(
            matrix.changed_effect_feedback_matrix()
        ),
        "interleaved_truth_matrix": interleaved_truth_matrix_json(matrix.interleaved_truth_matrix()),
        "restart_replay_matrix": restart_replay_matrix_json(matrix.restart_replay_matrix()),
        "feedback_provenance_digest": matrix.feedback_provenance_digest(),
        "carried_causality_digest": matrix.carried_causality_digest(),
        "carried_feedback_provenance_digest": matrix.carried_feedback_provenance_digest(),
        "initial_causality_digest": matrix.initial_causality_digest(),
        "feedback_route_digest": matrix.feedback_route_digest(),
        "loop_prevention_digest": matrix.loop_prevention_digest(),
        "loop_prevention_disposition": matrix.loop_prevention_disposition(),
        "boundedness_proof": feedback_boundedness_proof_json(matrix),
    })
}

pub(in crate::harness::adapter::adapter_impl) fn feedback_certification_evidence_json(
    matrix: &WritebackFeedbackLoopMatrix,
    counter_snapshot: &crate::facade::BridgeWritebackCounters,
    counter_digest: &str,
) -> serde_json::Value {
    json!({
        "certification_shape": "feedback-loop-certification",
        "writeback_digest": matrix.writeback_digest(),
        "effect_intent_digest": matrix.effect_intent_digest(),
        "causality_digest": matrix.causality_digest(),
        "mutation_plan_digest": matrix.mutation_plan_digest(),
        "idempotence_report": feedback_idempotence_report_json(matrix),
        "loop_prevention_report": feedback_loop_prevention_report_json(matrix),
        "truth_integrity_report": feedback_boundedness_proof_json(matrix),
        "authority_boundary_matrix": feedback_authority_boundary_matrix_json(matrix),
        "failure_digest": matrix.changed_effect_feedback_matrix().failure_digest(),
        "replay_digest": matrix.replay_digest(),
        "counter_snapshot": writeback_counter_snapshot_json(counter_snapshot),
        "counter_digest": counter_digest,
    })
}

pub(in crate::harness::adapter::adapter_impl) fn feedback_boundedness_proof_json(
    matrix: &WritebackFeedbackLoopMatrix,
) -> serde_json::Value {
    boundedness_proof_json(matrix.boundedness_proof())
}

pub(in crate::harness::adapter::adapter_impl) fn feedback_idempotence_report_json(
    matrix: &WritebackFeedbackLoopMatrix,
) -> serde_json::Value {
    idempotence_report_json(matrix.idempotence_report())
}

pub(in crate::harness::adapter::adapter_impl) fn feedback_loop_prevention_report_json(
    matrix: &WritebackFeedbackLoopMatrix,
) -> serde_json::Value {
    loop_prevention_report_json(matrix.loop_prevention_report())
}

pub(in crate::harness::adapter::adapter_impl) fn feedback_authority_boundary_matrix_json(
    matrix: &WritebackFeedbackLoopMatrix,
) -> serde_json::Value {
    authority_boundary_matrix_json(matrix.authority_boundary_matrix())
}

fn replay_bundle_report_json(report: &FeedbackReplayBundleReport) -> serde_json::Value {
    json!({
        "digest": report.digest(),
        "semantic_digest": report.semantic_digest(),
        "effect_intent_digest": report.effect_intent_digest(),
        "effect_intent_patch_canonical_basis": report.effect_intent_patch_canonical_basis(),
        "strategy_class": format!("{:?}", report.strategy_class()),
        "strategy_descriptor_digest": report.strategy_descriptor_digest(),
        "causality_digest": report.causality_digest(),
        "lowered_policy_digest": report.lowered_policy_digest(),
        "retry_disposition": format!("{:?}", report.retry_disposition()),
        "outcome_class": format!("{:?}", report.outcome_class()),
        "authoritative_artifact_digest": report.authoritative_artifact_digest(),
    })
}

fn idempotence_report_json(report: &FeedbackIdempotenceReport) -> serde_json::Value {
    json!({
        "initial_digest": report.initial_digest(),
        "replayed_digest": report.replayed_digest(),
        "idempotence_class": report.idempotence_class(),
        "initial_authoritative_state_digest": report.initial_authoritative_state_digest(),
        "replayed_authoritative_state_digest": report.replayed_authoritative_state_digest(),
        "lowered_policy_digest": report.lowered_policy_digest(),
        "strategy_descriptor_digest": report.strategy_descriptor_digest(),
    })
}

fn loop_prevention_report_json(report: &FeedbackLoopPreventionReport) -> serde_json::Value {
    json!({
        "digest": report.digest(),
        "disposition": report.disposition(),
        "current_feedback_provenance_digest": report.current_feedback_provenance_digest(),
        "current_causality_digest": report.current_causality_digest(),
        "incoming_feedback_provenance_digest": report.incoming_feedback_provenance_digest(),
        "incoming_feedback_causality_digest": report.incoming_feedback_causality_digest(),
    })
}

fn authority_boundary_matrix_json(matrix: &FeedbackAuthorityBoundaryMatrix) -> serde_json::Value {
    json!({
        "contract_digest": matrix.contract_digest(),
        "strategy_basis_digest": matrix.strategy_basis_digest(),
        "strategy_coherence_digest": matrix.strategy_coherence_digest(),
        "strategy_coherence_disposition": matrix.strategy_coherence_disposition(),
        "candidate_digest": matrix.candidate_digest(),
        "authority_request_digest": matrix.authority_request_digest(),
        "authority_receipt_digest": matrix.authority_receipt_digest(),
    })
}

fn changed_effect_feedback_matrix_json(matrix: &FeedbackChangedEffectMatrix) -> serde_json::Value {
    json!({
        "writeback_effect_artifact_digest": matrix.writeback_effect_artifact_digest(),
        "effect_intent_digest": matrix.effect_intent_digest(),
        "effect_intent_patch_canonical_basis": matrix.effect_intent_patch_canonical_basis(),
        "causality_digest": matrix.causality_digest(),
        "idempotence_digest": matrix.idempotence_digest(),
        "failure_kind": format!("{:?}", matrix.failure_kind()),
        "failure_digest": matrix.failure_digest(),
        "same_causality_as_initial": matrix.same_causality_as_initial(),
        "same_feedback_provenance_as_initial": matrix.same_feedback_provenance_as_initial(),
    })
}

fn interleaved_truth_matrix_json(matrix: &FeedbackInterleavedTruthMatrix) -> serde_json::Value {
    json!({
        "ordinary_truth_commit_identity": matrix.ordinary_truth_commit_identity(),
        "ordinary_truth_route_digest": matrix.ordinary_truth_route_digest(),
        "bridge_feedback_commit_identity": matrix.bridge_feedback_commit_identity(),
        "interleaving_preserved_single_authoritative_commit":
            matrix.interleaving_preserved_single_authoritative_commit(),
    })
}

fn restart_replay_matrix_json(matrix: &FeedbackRestartReplayMatrix) -> serde_json::Value {
    json!({
        "rebuilt_contract_digest": matrix.rebuilt_contract_digest(),
        "rebuilt_writeback_effect_artifact_digest": matrix.rebuilt_writeback_effect_artifact_digest(),
        "rebuilt_effect_intent_digest": matrix.rebuilt_effect_intent_digest(),
        "rebuilt_idempotence_digest": matrix.rebuilt_idempotence_digest(),
        "rebuilt_loop_prevention_digest": matrix.rebuilt_loop_prevention_digest(),
        "rebuilt_loop_prevention_disposition": matrix.rebuilt_loop_prevention_disposition(),
        "rebuilt_outcome_digest": matrix.rebuilt_outcome_digest(),
        "rebuilt_replay_bundle_digest": matrix.rebuilt_replay_bundle_digest(),
        "rebuilt_authority_receipt_present": matrix.rebuilt_authority_receipt_present(),
        "replay_equivalent_to_live_feedback": matrix.replay_equivalent_to_live_feedback(),
    })
}

fn boundedness_proof_json(proof: &FeedbackBoundednessProof) -> serde_json::Value {
    json!({
        "authoritative_commit_count": proof.authoritative_commit_count(),
        "replayed_feedback_outcome_class": format!(
            "{:?}",
            proof.replayed_feedback_outcome_class()
        ),
        "changed_effect_retrigger_failure_kind": format!(
            "{:?}",
            proof.changed_effect_retrigger_failure_kind()
        ),
        "feedback_publication_routed": proof.feedback_publication_routed(),
        "ordinary_truth_interleaved": proof.ordinary_truth_interleaved(),
        "feedback_converged": proof.feedback_converged(),
        "restart_replay_converged": proof.restart_replay_converged(),
        "replayed_authority_receipt_present": proof.replayed_authority_receipt_present(),
    })
}
