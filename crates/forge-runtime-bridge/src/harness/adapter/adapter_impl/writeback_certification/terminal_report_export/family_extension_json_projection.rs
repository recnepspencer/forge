use serde_json::json;

use crate::routing::canonicalization::digest_string;

use super::super::super::terminal_report_export::writeback_counter_snapshot_json;
use super::super::family_extension::{
    FamilyExtensionChangedCausalityIsolation, FamilyExtensionCrossFamilyReplayIsolation,
    FamilyExtensionFamilyEvidence, FamilyExtensionLoopIsolation, FamilyExtensionMapperParityProof,
    FamilyExtensionSameFamilyEquivalence, FamilyExtensionShadowProtocolRejection,
    WritebackFamilyExtensionMatrix,
};

pub(in crate::harness::adapter::adapter_impl) fn family_extension_matrix_json(
    matrix: &WritebackFamilyExtensionMatrix,
) -> serde_json::Value {
    json!({
        "projected_family": family_json(matrix.projected_family()),
        "aspect_family": family_json(matrix.aspect_family()),
        "cross_family_replay_isolation": cross_family_replay_json(
            matrix.cross_family_replay_isolation()
        ),
        "same_family_equivalence": same_family_equivalence_json(
            matrix.same_family_equivalence()
        ),
        "same_family_changed_causality": changed_causality_json(
            matrix.same_family_changed_causality()
        ),
        "cross_family_loop_isolation": loop_isolation_json(matrix.cross_family_loop_isolation()),
        "mapper_parity_matrix": mapper_parity_json(matrix.mapper_parity_proof()),
        "shadow_protocol_rejection": shadow_protocol_json(matrix.shadow_protocol_rejection()),
    })
}

pub(in crate::harness::adapter::adapter_impl) fn family_extension_certification_evidence_json(
    matrix: &WritebackFamilyExtensionMatrix,
    counter_snapshot: &crate::facade::BridgeWritebackCounters,
    counter_digest: &str,
) -> serde_json::Value {
    json!({
        "certification_shape": "extensible-family-certification",
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
        "loop_prevention_report": loop_isolation_json(matrix.cross_family_loop_isolation()),
        "truth_integrity_report": mapper_parity_json(matrix.mapper_parity_proof()),
        "authority_boundary_matrix": shadow_protocol_json(matrix.shadow_protocol_rejection()),
        "failure_digest": shadow_protocol_failure_digest(matrix.shadow_protocol_rejection()),
        "replay_digest": {
            "projected": matrix.projected_family().replay_bundle_digest(),
            "aspect": matrix.aspect_family().replay_bundle_digest(),
        },
        "counter_snapshot": writeback_counter_snapshot_json(counter_snapshot),
        "counter_digest": counter_digest,
    })
}

fn family_json(family: &FamilyExtensionFamilyEvidence) -> serde_json::Value {
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
        "authority_receipt_digest": family.authority_receipt_digest(),
    })
}

fn cross_family_replay_json(
    isolation: &FamilyExtensionCrossFamilyReplayIsolation,
) -> serde_json::Value {
    json!({
        "semantic_digest_separated": isolation.semantic_digest_separated(),
        "bundle_digest_separated": isolation.bundle_digest_separated(),
        "failure_kind": format!("{:?}", isolation.failure_kind()),
        "failure_digest": cross_family_failure_digest(isolation),
        "family_replay_record_digest": isolation.family_replay_record_digest(),
        "decision_trace_digest": cross_family_decision_trace_digest(isolation),
    })
}

fn same_family_equivalence_json(
    equivalence: &FamilyExtensionSameFamilyEquivalence,
) -> serde_json::Value {
    json!({
        "semantic_digest_equal": equivalence.semantic_digest_equal(),
        "bundle_digest_equal": equivalence.bundle_digest_equal(),
        "effect_intent_digest_equal": equivalence.effect_intent_digest_equal(),
        "mapped_input_digest_equal": equivalence.mapped_input_digest_equal(),
        "family_execution_record_digest": equivalence.family_execution_record_digest(),
        "decision_trace_digest": same_family_equivalence_trace_digest(equivalence),
    })
}

fn changed_causality_json(
    isolation: &FamilyExtensionChangedCausalityIsolation,
) -> serde_json::Value {
    json!({
        "causality_digest_separated": isolation.causality_digest_separated(),
        "semantic_digest_separated": isolation.semantic_digest_separated(),
        "bundle_digest_separated": isolation.bundle_digest_separated(),
        "failure_kind": format!("{:?}", isolation.failure_kind()),
        "family_replay_record_digest": isolation.family_replay_record_digest(),
        "decision_trace_digest": changed_causality_trace_digest(isolation),
    })
}

fn loop_isolation_json(isolation: &FamilyExtensionLoopIsolation) -> serde_json::Value {
    json!({
        "incoming_feedback_provenance_digest": isolation.incoming_feedback_provenance_digest(),
        "incoming_feedback_causality_digest": isolation.incoming_feedback_causality_digest(),
        "disposition": format!("{:?}", isolation.disposition()),
        "digest": isolation.digest(),
    })
}

fn cross_family_failure_digest(isolation: &FamilyExtensionCrossFamilyReplayIsolation) -> String {
    digest_string(
        "bridge-writeback-family-cross-replay",
        &isolation.error().to_string(),
    )
    .to_string()
}

fn cross_family_decision_trace_digest(
    isolation: &FamilyExtensionCrossFamilyReplayIsolation,
) -> String {
    digest_string(
        "bridge-writeback-family-cross-replay-trace",
        &format!(
            "projected-bundle={}|aspect-bundle={}|replay-record={}|failure={:?}",
            isolation.projected_bundle().digest(),
            isolation.aspect_bundle().digest(),
            isolation.replay_record().digest(),
            isolation.error().kind(),
        ),
    )
    .to_string()
}

fn same_family_equivalence_trace_digest(
    equivalence: &FamilyExtensionSameFamilyEquivalence,
) -> String {
    digest_string(
        "bridge-writeback-family-same-family-trace",
        &format!(
            "projected-bundle={}|rebuilt-bundle={}|execution-record={}",
            equivalence.projected_bundle().digest(),
            equivalence.rebuilt_projected_bundle().digest(),
            equivalence.rebuilt_execution_record().digest(),
        ),
    )
    .to_string()
}

fn changed_causality_trace_digest(isolation: &FamilyExtensionChangedCausalityIsolation) -> String {
    digest_string(
        "bridge-writeback-family-same-family-drift-trace",
        &format!(
            "projected-bundle={}|changed-bundle={}|replay-record={}|failure={:?}",
            isolation.projected_bundle().digest(),
            isolation.changed_projected_bundle().digest(),
            isolation.replay_record().digest(),
            isolation.error().kind(),
        ),
    )
    .to_string()
}

fn mapper_parity_json(proof: &FamilyExtensionMapperParityProof) -> serde_json::Value {
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
        "decision_trace_digest": mapper_parity_decision_trace_digest(proof),
    })
}

fn shadow_protocol_json(rejection: &FamilyExtensionShadowProtocolRejection) -> serde_json::Value {
    json!({
        "failure_kind": format!("{:?}", rejection.failure_kind()),
        "failure_digest": shadow_protocol_failure_digest(rejection),
        "decision_trace_digest": shadow_protocol_decision_trace_digest(rejection),
        "effect_family_mismatch_rejected": rejection.effect_family_mismatch_rejected(),
        "no_shadow_protocol_mapper_envelope_retained": rejection
            .no_shadow_protocol_mapper_envelope_retained(),
    })
}

fn mapper_parity_decision_trace_digest(proof: &FamilyExtensionMapperParityProof) -> String {
    digest_string(
        "bridge-writeback-family-mapper-trace",
        &format!(
            "projected-admission={}|aspect-admission={}|projected-mapper={}|aspect-mapper={}|projected-execution={}|aspect-execution={}",
            proof.projected_admission_record().digest(),
            proof.aspect_admission_record().digest(),
            proof.projected_execution_record()
                .mapper_record_digest()
                .unwrap_or("none"),
            proof.aspect_execution_record()
                .mapper_record_digest()
                .unwrap_or("none"),
            proof.projected_execution_record().digest(),
            proof.aspect_execution_record().digest(),
        ),
    )
    .to_string()
}

fn shadow_protocol_failure_digest(rejection: &FamilyExtensionShadowProtocolRejection) -> String {
    digest_string(
        "bridge-writeback-family-shadow-protocol",
        &rejection.error().to_string(),
    )
    .to_string()
}

fn shadow_protocol_decision_trace_digest(
    rejection: &FamilyExtensionShadowProtocolRejection,
) -> String {
    digest_string(
        "bridge-writeback-family-shadow-protocol-trace",
        &format!(
            "shadow={:?}|projected-admission={}|aspect-admission={}",
            rejection.error().kind(),
            rejection.projected_admission_record().digest(),
            rejection.aspect_admission_record().digest(),
        ),
    )
    .to_string()
}
