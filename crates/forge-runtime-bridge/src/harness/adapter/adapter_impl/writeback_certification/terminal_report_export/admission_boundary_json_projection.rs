use serde_json::json;

use super::super::super::terminal_report_export::writeback_counter_snapshot_json;
use super::super::admission_boundary::{
    AdmissionBoundaryAuthorityProof, AdmissionBoundaryFamilyAdmissionProof,
    AdmissionBoundaryFamilyEvidence, AdmissionBoundaryShadowProtocolRejection,
    WritebackAdmissionBoundaryMatrix,
};

pub(in crate::harness::adapter::adapter_impl) fn admission_boundary_matrix_json(
    matrix: &WritebackAdmissionBoundaryMatrix,
) -> serde_json::Value {
    json!({
        "projected_family": family_json(matrix.projected_family()),
        "aspect_family": family_json(matrix.aspect_family()),
        "family_admission_matrix": family_admission_json(matrix.family_admission_proof()),
        "authority_boundary_matrix": authority_boundary_json(matrix.authority_boundary_proof()),
        "shadow_protocol_rejection": shadow_protocol_json(matrix.shadow_protocol_rejection()),
        "failure_digest": matrix.shadow_protocol_rejection().failure_digest(),
    })
}

pub(in crate::harness::adapter::adapter_impl) fn admission_boundary_certification_evidence_json(
    matrix: &WritebackAdmissionBoundaryMatrix,
    counter_snapshot: &crate::facade::BridgeWritebackCounters,
    counter_digest: &str,
) -> serde_json::Value {
    json!({
        "certification_shape": "multi-family-admission-boundary",
        "writeback_digest": matrix.projected_family().replay_bundle_digest(),
        "effect_intent_digest": {
            "projected": matrix.projected_family().effect_intent_digest(),
            "aspect": matrix.aspect_family().effect_intent_digest(),
        },
        "causality_digest": matrix.projected_family().causality_digest(),
        "mutation_plan_digest": {
            "projected": matrix.projected_family().mapped_input_digest(),
            "aspect": matrix.aspect_family().mapped_input_digest(),
        },
        "idempotence_report": {
            "projected": matrix.projected_family().idempotence_digest(),
            "aspect": matrix.aspect_family().idempotence_digest(),
        },
        "loop_prevention_report": serde_json::Value::Null,
        "truth_integrity_report": family_admission_json(matrix.family_admission_proof()),
        "authority_boundary_matrix": authority_boundary_json(matrix.authority_boundary_proof()),
        "failure_digest": matrix.shadow_protocol_rejection().failure_digest(),
        "replay_digest": {
            "projected": matrix.projected_family().replay_bundle_digest(),
            "aspect": matrix.aspect_family().replay_bundle_digest(),
        },
        "counter_snapshot": writeback_counter_snapshot_json(counter_snapshot),
        "counter_digest": counter_digest,
    })
}

fn family_json(family: &AdmissionBoundaryFamilyEvidence) -> serde_json::Value {
    json!({
        "admission_record_digest": family.admission_record_digest(),
        "contract_digest": family.contract_digest(),
        "writeback_effect_artifact_digest": family.writeback_effect_artifact_digest(),
        "effect_intent_digest": family.effect_intent_digest(),
        "effect_intent_patch_canonical_basis": family.effect_intent_patch_canonical_basis(),
        "mapped_input_digest": family.mapped_input_digest(),
        "mapper_envelope_digest": family.mapper_envelope_digest(),
        "causality_digest": family.causality_digest(),
        "idempotence_digest": family.idempotence_digest(),
        "replay_bundle_digest": family.replay_bundle_digest(),
        "replay_semantic_digest": family.replay_semantic_digest(),
    })
}

fn family_admission_json(proof: &AdmissionBoundaryFamilyAdmissionProof) -> serde_json::Value {
    json!({
        "projected_family_admitted": proof.projected_family_admitted(),
        "aspect_family_admitted": proof.aspect_family_admitted(),
        "projected_admission_record_digest": proof.projected_admission_record_digest(),
        "aspect_admission_record_digest": proof.aspect_admission_record_digest(),
        "projected_contract_digest": proof.projected_contract_digest(),
        "aspect_contract_digest": proof.aspect_contract_digest(),
        "family_digest_separated": proof.family_digest_separated(),
        "projected_strategy_matches_family": proof.projected_strategy_matches_family(),
        "aspect_strategy_matches_family": proof.aspect_strategy_matches_family(),
        "decision_trace_digest": proof.decision_trace_digest(),
    })
}

fn authority_boundary_json(proof: &AdmissionBoundaryAuthorityProof) -> serde_json::Value {
    json!({
        "projected_authority_commit_digest": proof.projected_authority_commit_digest(),
        "aspect_authority_commit_digest": proof.aspect_authority_commit_digest(),
        "distinct_authority_artifacts": proof.distinct_authority_artifacts(),
        "failure_kind": format!("{:?}", proof.failure_kind()),
        "failure_digest": proof.failure_digest(),
        "decision_trace_digest": proof.decision_trace_digest(),
    })
}

fn shadow_protocol_json(rejection: &AdmissionBoundaryShadowProtocolRejection) -> serde_json::Value {
    json!({
        "failure_kind": format!("{:?}", rejection.failure_kind()),
        "failure_digest": rejection.failure_digest(),
        "projected_admission_record_digest": rejection.projected_admission_record_digest(),
        "aspect_admission_record_digest": rejection.aspect_admission_record_digest(),
        "effect_family_mismatch_rejected": rejection.effect_family_mismatch_rejected(),
        "no_shadow_protocol_admission_record": rejection.no_shadow_protocol_admission_record(),
        "decision_trace_digest": rejection.decision_trace_digest(),
    })
}
