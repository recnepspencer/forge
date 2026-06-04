use serde_json::json;

use super::super::super::terminal_report_export::writeback_counter_snapshot_json;
use super::super::mapper_parity::{
    MapperParityFamilyEvidence, MapperParityProof, MapperParityShadowProtocolRejection,
    WritebackMapperParityMatrix,
};

pub(in crate::harness::adapter::adapter_impl) fn mapper_parity_matrix_json(
    matrix: &WritebackMapperParityMatrix,
) -> serde_json::Value {
    json!({
        "projected_family": mapper_parity_family_json(matrix.projected_family()),
        "aspect_family": mapper_parity_family_json(matrix.aspect_family()),
        "mapper_parity_matrix": mapper_parity_proof_json(matrix.mapper_parity_proof()),
        "shadow_protocol_rejection": shadow_protocol_rejection_json(
            matrix.shadow_protocol_rejection()
        ),
    })
}

pub(in crate::harness::adapter::adapter_impl) fn mapper_parity_certification_evidence_json(
    matrix: &WritebackMapperParityMatrix,
    counter_snapshot: &crate::facade::BridgeWritebackCounters,
    counter_digest: &str,
) -> serde_json::Value {
    json!({
        "certification_shape": "host-mapper-parity-and-shadow-protocol-rejection",
        "writeback_digest": {
            "projected": matrix.projected_family().replay_bundle_digest(),
            "aspect": matrix.aspect_family().replay_bundle_digest(),
        },
        "effect_intent_digest": {
            "projected": matrix.projected_family().effect_intent_digest(),
            "aspect": matrix.aspect_family().effect_intent_digest(),
        },
        "causality_digest": matrix.projected_family().causality_digest(),
        "mutation_plan_digest": {
            "projected": matrix.projected_family().mapped_input_digest(),
            "aspect": matrix.aspect_family().mapped_input_digest(),
        },
        "idempotence_report": serde_json::Value::Null,
        "loop_prevention_report": serde_json::Value::Null,
        "truth_integrity_report": mapper_parity_proof_json(matrix.mapper_parity_proof()),
        "authority_boundary_matrix": shadow_protocol_rejection_json(
            matrix.shadow_protocol_rejection()
        ),
        "failure_digest": matrix.shadow_protocol_rejection().failure_digest(),
        "replay_digest": {
            "projected": matrix.projected_family().replay_bundle_digest(),
            "aspect": matrix.aspect_family().replay_bundle_digest(),
        },
        "counter_snapshot": writeback_counter_snapshot_json(counter_snapshot),
        "counter_digest": counter_digest,
    })
}

fn mapper_parity_family_json(family: &MapperParityFamilyEvidence) -> serde_json::Value {
    json!({
        "writeback_effect_artifact_digest": family.writeback_effect_artifact_digest(),
        "effect_intent_digest": family.effect_intent_digest(),
        "effect_intent_patch_canonical_basis": family.effect_intent_patch_canonical_basis(),
        "causality_digest": family.causality_digest(),
        "mapped_input_digest": family.mapped_input_digest(),
        "mapper_envelope_digest": family.mapper_envelope_digest(),
        "replay_bundle_digest": family.replay_bundle_digest(),
    })
}

fn mapper_parity_proof_json(proof: &MapperParityProof) -> serde_json::Value {
    json!({
        "projected_mapper_envelope_retained": proof.projected_mapper_envelope_retained(),
        "aspect_mapper_envelope_retained": proof.aspect_mapper_envelope_retained(),
        "projected_mapped_input_retained": proof.projected_mapped_input_retained(),
        "aspect_mapped_input_retained": proof.aspect_mapped_input_retained(),
        "projected_family_mapper_record_digest": proof.projected_family_mapper_record_digest(),
        "aspect_family_mapper_record_digest": proof.aspect_family_mapper_record_digest(),
        "projected_family_execution_record_digest": proof
            .projected_family_execution_record_digest(),
        "aspect_family_execution_record_digest": proof.aspect_family_execution_record_digest(),
        "projected_admission_record_digest": proof.projected_admission_record_digest(),
        "aspect_admission_record_digest": proof.aspect_admission_record_digest(),
        "decision_trace_digest": proof.decision_trace_digest(),
    })
}

fn shadow_protocol_rejection_json(
    rejection: &MapperParityShadowProtocolRejection,
) -> serde_json::Value {
    json!({
        "failure_kind": format!("{:?}", rejection.failure_kind()),
        "failure_digest": rejection.failure_digest(),
        "decision_trace_digest": rejection.decision_trace_digest(),
        "effect_family_mismatch_rejected": rejection.effect_family_mismatch_rejected(),
        "no_shadow_protocol_mapper_envelope_retained": rejection
            .no_shadow_protocol_mapper_envelope_retained(),
    })
}
