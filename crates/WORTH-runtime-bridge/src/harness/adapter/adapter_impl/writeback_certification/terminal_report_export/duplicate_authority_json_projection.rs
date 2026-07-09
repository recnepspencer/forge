use serde_json::json;

use super::super::duplicate_authority::{
    DuplicateAttemptReport, DuplicateAuthorityBoundaryMatrix, DuplicateBoundednessProof,
    DuplicateIdempotenceReport, DuplicateLoopPreventionReport, DuplicateReplayBundleReport,
    WritebackDuplicateAuthorityMatrix,
};

pub(in crate::harness::adapter::adapter_impl) fn duplicate_authority_matrix_json(
    matrix: &WritebackDuplicateAuthorityMatrix,
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
        "truth_trigger_digest": matrix.truth_trigger_digest(),
        "route_digest": matrix.route_digest(),
        "first_attempt": attempt_report_json(matrix.first_attempt()),
        "repeated_attempt": attempt_report_json(matrix.repeated_attempt()),
        "boundedness_proof": boundedness_proof_json(matrix.boundedness_proof()),
    })
}

pub(in crate::harness::adapter::adapter_impl) fn duplicate_boundedness_proof_json(
    matrix: &WritebackDuplicateAuthorityMatrix,
) -> serde_json::Value {
    boundedness_proof_json(matrix.boundedness_proof())
}

pub(in crate::harness::adapter::adapter_impl) fn duplicate_idempotence_report_json(
    matrix: &WritebackDuplicateAuthorityMatrix,
) -> serde_json::Value {
    idempotence_report_json(matrix.idempotence_report())
}

pub(in crate::harness::adapter::adapter_impl) fn duplicate_loop_prevention_report_json(
    matrix: &WritebackDuplicateAuthorityMatrix,
) -> serde_json::Value {
    loop_prevention_report_json(matrix.loop_prevention_report())
}

pub(in crate::harness::adapter::adapter_impl) fn duplicate_authority_boundary_matrix_json(
    matrix: &WritebackDuplicateAuthorityMatrix,
) -> serde_json::Value {
    authority_boundary_matrix_json(matrix.authority_boundary_matrix())
}

fn replay_bundle_report_json(report: &DuplicateReplayBundleReport) -> serde_json::Value {
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

fn idempotence_report_json(report: &DuplicateIdempotenceReport) -> serde_json::Value {
    json!({
        "first_digest": report.first_digest(),
        "repeated_digest": report.repeated_digest(),
        "idempotence_class": format!("{:?}", report.idempotence_class()),
        "authoritative_state_before": report.authoritative_state_before(),
        "authoritative_state_after_first_commit": report.authoritative_state_after_first_commit(),
        "lowered_policy_digest": report.lowered_policy_digest(),
        "strategy_descriptor_digest": report.strategy_descriptor_digest(),
    })
}

fn loop_prevention_report_json(report: &DuplicateLoopPreventionReport) -> serde_json::Value {
    json!({
        "first_digest": report.first_digest(),
        "first_disposition": format!("{:?}", report.first_disposition()),
        "repeated_digest": report.repeated_digest(),
        "repeated_disposition": format!("{:?}", report.repeated_disposition()),
        "current_feedback_provenance_digest": report.current_feedback_provenance_digest(),
        "current_causality_digest": report.current_causality_digest(),
    })
}

fn authority_boundary_matrix_json(matrix: &DuplicateAuthorityBoundaryMatrix) -> serde_json::Value {
    json!({
        "contract_digest": matrix.contract_digest(),
        "strategy_basis_digest": matrix.strategy_basis_digest(),
        "first_strategy_coherence_digest": matrix.first_strategy_coherence_digest(),
        "first_strategy_coherence_disposition": format!(
            "{:?}",
            matrix.first_strategy_coherence_disposition()
        ),
        "first_candidate_digest": matrix.first_candidate_digest(),
        "repeated_strategy_coherence_digest": matrix.repeated_strategy_coherence_digest(),
        "repeated_strategy_coherence_disposition": format!(
            "{:?}",
            matrix.repeated_strategy_coherence_disposition()
        ),
        "repeated_candidate_digest": matrix.repeated_candidate_digest(),
        "first_authority_request_digest": matrix.first_authority_request_digest(),
        "repeated_authority_request_digest": matrix.repeated_authority_request_digest(),
        "first_authority_receipt_digest": matrix.first_authority_receipt_digest(),
        "repeated_authority_receipt_digest": matrix.repeated_authority_receipt_digest(),
    })
}

fn attempt_report_json(report: &DuplicateAttemptReport) -> serde_json::Value {
    json!({
        "idempotence_digest": report.idempotence_digest(),
        "outcome_digest": report.outcome_digest(),
        "replay_bundle_digest": report.replay_bundle_digest(),
        "outcome_class": format!("{:?}", report.outcome_class()),
    })
}

fn boundedness_proof_json(proof: &DuplicateBoundednessProof) -> serde_json::Value {
    json!({
        "authoritative_commit_count": proof.authoritative_commit_count(),
        "canonical_noop_count": proof.canonical_noop_count(),
        "duplicate_causality_detected": proof.duplicate_causality_detected(),
        "loop_converged": proof.loop_converged(),
    })
}
