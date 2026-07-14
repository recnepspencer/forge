use serde_json::json;

use crate::routing::canonicalization::digest_string;

use super::super::super::terminal_report_export::writeback_counter_snapshot_json;
use super::super::replay_loop_isolation::{
    ReplayLoopChangedCausalityIsolation, ReplayLoopCrossFamilyIsolation, ReplayLoopFamilyEvidence,
    ReplayLoopFeedbackIsolation, ReplayLoopSameFamilyEquivalence,
    WritebackReplayLoopIsolationMatrix,
};

pub(in crate::harness::adapter::adapter_impl) fn replay_loop_isolation_matrix_json(
    matrix: &WritebackReplayLoopIsolationMatrix,
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
    })
}

pub(in crate::harness::adapter::adapter_impl) fn replay_loop_certification_evidence_json(
    matrix: &WritebackReplayLoopIsolationMatrix,
    counter_snapshot: &crate::facade::BridgeWritebackCounters,
    counter_digest: &str,
) -> serde_json::Value {
    json!({
        "certification_shape": "cross-family-replay-loop-isolation",
        "writeback_digest": {
            "projected": matrix.projected_family().replay_bundle_digest(),
            "aspect": matrix.aspect_family().replay_bundle_digest(),
        },
        "effect_intent_digest": {
            "projected": matrix.projected_family().effect_intent_digest(),
            "aspect": matrix.aspect_family().effect_intent_digest(),
        },
        "causality_digest": {
            "projected": matrix.projected_family().causality_digest(),
            "aspect": matrix.aspect_family().causality_digest(),
        },
        "mutation_plan_digest": {
            "projected": matrix.projected_family().mapped_input_digest(),
            "aspect": matrix.aspect_family().mapped_input_digest(),
        },
        "idempotence_report": {
            "projected": matrix.projected_family().idempotence_digest(),
            "aspect": matrix.aspect_family().idempotence_digest(),
        },
        "loop_prevention_report": loop_isolation_json(matrix.cross_family_loop_isolation()),
        "truth_integrity_report": same_family_equivalence_json(matrix.same_family_equivalence()),
        "authority_boundary_matrix": changed_causality_json(
            matrix.same_family_changed_causality()
        ),
        "failure_digest": cross_family_failure_digest(matrix.cross_family_replay_isolation()),
        "replay_digest": {
            "projected": matrix.projected_family().replay_bundle_digest(),
            "aspect": matrix.aspect_family().replay_bundle_digest(),
        },
        "counter_snapshot": writeback_counter_snapshot_json(counter_snapshot),
        "counter_digest": counter_digest,
    })
}

fn family_json(family: &ReplayLoopFamilyEvidence) -> serde_json::Value {
    json!({
        "writeback_effect_artifact_digest": family.writeback_effect_artifact_digest(),
        "effect_intent_digest": family.effect_intent_digest(),
        "effect_intent_patch_canonical_basis": family.effect_intent_patch_canonical_basis(),
        "mapped_input_digest": family.mapped_input_digest(),
        "causality_digest": family.causality_digest(),
        "idempotence_digest": family.idempotence_digest(),
        "replay_bundle_digest": family.replay_bundle_digest(),
        "replay_semantic_digest": family.replay_semantic_digest(),
    })
}

fn cross_family_replay_json(isolation: &ReplayLoopCrossFamilyIsolation) -> serde_json::Value {
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
    equivalence: &ReplayLoopSameFamilyEquivalence,
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

fn changed_causality_json(isolation: &ReplayLoopChangedCausalityIsolation) -> serde_json::Value {
    json!({
        "causality_digest_separated": isolation.causality_digest_separated(),
        "semantic_digest_separated": isolation.semantic_digest_separated(),
        "bundle_digest_separated": isolation.bundle_digest_separated(),
        "failure_kind": format!("{:?}", isolation.failure_kind()),
        "family_replay_record_digest": isolation.family_replay_record_digest(),
        "decision_trace_digest": changed_causality_trace_digest(isolation),
    })
}

fn loop_isolation_json(isolation: &ReplayLoopFeedbackIsolation) -> serde_json::Value {
    json!({
        "incoming_feedback_provenance_digest": isolation.incoming_feedback_provenance_digest(),
        "incoming_feedback_causality_digest": isolation.incoming_feedback_causality_digest(),
        "disposition": format!("{:?}", isolation.disposition()),
        "digest": isolation.digest(),
    })
}

fn cross_family_failure_digest(isolation: &ReplayLoopCrossFamilyIsolation) -> String {
    digest_string(
        "bridge-writeback-family-replay-loop-cross-family",
        &isolation.error().to_string(),
    )
    .to_string()
}

fn cross_family_decision_trace_digest(isolation: &ReplayLoopCrossFamilyIsolation) -> String {
    digest_string(
        "bridge-writeback-family-replay-loop-cross-family-trace",
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

fn same_family_equivalence_trace_digest(equivalence: &ReplayLoopSameFamilyEquivalence) -> String {
    digest_string(
        "bridge-writeback-family-replay-loop-same-family-trace",
        &format!(
            "projected-bundle={}|rebuilt-bundle={}|execution-record={}",
            equivalence.projected_bundle().digest(),
            equivalence.rebuilt_projected_bundle().digest(),
            equivalence.rebuilt_execution_record().digest(),
        ),
    )
    .to_string()
}

fn changed_causality_trace_digest(isolation: &ReplayLoopChangedCausalityIsolation) -> String {
    digest_string(
        "bridge-writeback-family-replay-loop-same-family-drift-trace",
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
