use super::super::*;

use super::super::super::terminal_report_export::writeback_counter_snapshot_json;
use super::super::super::writeback_certification::{
    admission_boundary_certification_evidence_json, admission_boundary_matrix_json,
    authority_denial_certification_evidence_json, authority_denial_matrix_json,
    authority_denial_zero_residue_proof_json, duplicate_authority_boundary_matrix_json,
    duplicate_authority_matrix_json, duplicate_boundedness_proof_json,
    duplicate_idempotence_report_json, duplicate_loop_prevention_report_json,
    family_extension_certification_evidence_json, family_extension_matrix_json,
    feedback_certification_evidence_json, feedback_loop_matrix_json,
    mapper_parity_certification_evidence_json, mapper_parity_matrix_json,
    replay_loop_certification_evidence_json, replay_loop_isolation_matrix_json,
    replay_mismatch_matrix_json, replay_mismatch_restart_replay_json,
};
use crate::routing::canonicalization::digest_string;
use serde_json::json;
use std::collections::BTreeMap;

pub(in crate::harness::adapter::adapter_impl) fn certification_evidence_json(
    execution: &WritebackHarnessExecution,
) -> serde_json::Value {
    match execution {
        WritebackHarnessExecution::DuplicateCertification {
            duplicate_authority_matrix,
            counter_snapshot,
            ..
        } => json!({
            "certification_shape": "duplicate-certification",
            "writeback_digest": duplicate_authority_matrix.writeback_digest(),
            "effect_intent_digest": duplicate_authority_matrix.effect_intent_digest(),
            "effect_intent_patch_canonical_basis": duplicate_authority_matrix.effect_intent_patch_canonical_basis(),
            "causality_digest": duplicate_authority_matrix.causality_digest(),
            "mutation_plan_digest": duplicate_authority_matrix.mutation_plan_digest(),
            "idempotence_report": duplicate_idempotence_report_json(duplicate_authority_matrix),
            "loop_prevention_report": duplicate_loop_prevention_report_json(duplicate_authority_matrix),
            "truth_integrity_report": duplicate_boundedness_proof_json(duplicate_authority_matrix),
            "authority_boundary_matrix": duplicate_authority_boundary_matrix_json(duplicate_authority_matrix),
            "failure_digest": serde_json::Value::Null,
            "replay_digest": duplicate_authority_matrix.replay_digest(),
            "counter_snapshot": counter_snapshot_json(*counter_snapshot),
            "counter_digest": counter_snapshot.counters().digest(),
        }),
        WritebackHarnessExecution::AuthorityDenialCertification {
            failure_digest,
            authority_denial,
            zero_residue_report,
            counter_snapshot,
        } => {
            let counters = counter_snapshot.counters();
            authority_denial_certification_evidence_json(
                authority_denial,
                *zero_residue_report,
                failure_digest,
                &counters,
                counters.digest(),
            )
        }
        WritebackHarnessExecution::FeedbackLoopCertification {
            feedback_origin_matrix,
            counter_snapshot,
            ..
        } => {
            let counters = counter_snapshot.counters();
            feedback_certification_evidence_json(
                feedback_origin_matrix,
                &counters,
                counters.digest(),
            )
        }
        WritebackHarnessExecution::ReplayMismatchCertification {
            replay_validation_digest,
            replay_mismatch_matrix,
            counter_snapshot,
        } => json!({
            "certification_shape": "replay-mismatch-certification",
            "writeback_digest": {
                "expected": replay_mismatch_matrix.expected_replay_digest(),
                "replayed": replay_mismatch_matrix.replayed_replay_digest(),
            },
            "effect_intent_digest": {
                "expected": replay_mismatch_matrix.expected_effect_intent_digest(),
                "replayed": replay_mismatch_matrix.replayed_effect_intent_digest(),
            },
            "causality_digest": {
                "expected": replay_mismatch_matrix.expected_causality_digest(),
                "replayed": replay_mismatch_matrix.replayed_causality_digest(),
            },
            "mutation_plan_digest": {
                "expected": replay_mismatch_matrix.expected_semantic_digest(),
                "replayed": replay_mismatch_matrix.replayed_semantic_digest(),
            },
            "idempotence_report": serde_json::Value::Null,
            "loop_prevention_report": serde_json::Value::Null,
            "truth_integrity_report": replay_mismatch_restart_replay_json(replay_mismatch_matrix),
            "authority_boundary_matrix": {
                "failure_kind": format!("{:?}", replay_mismatch_matrix.failure_kind()),
                "restart_failure_kind": format!("{:?}", replay_mismatch_matrix.restart_failure_kind()),
            },
            "failure_digest": replay_validation_digest,
            "replay_digest": replay_mismatch_matrix.replayed_replay_digest(),
            "counter_snapshot": counter_snapshot_json(*counter_snapshot),
            "counter_digest": counter_snapshot.counters().digest(),
        }),
        WritebackHarnessExecution::ExtensibleFamilyCertification {
            family_extension_matrix,
            counter_snapshot,
            ..
        } => {
            let counters = counter_snapshot.counters();
            family_extension_certification_evidence_json(
                family_extension_matrix,
                &counters,
                counters.digest(),
            )
        }
        WritebackHarnessExecution::MultiFamilyAdmissionBoundaryCertification {
            admission_boundary_matrix,
            counter_snapshot,
            ..
        } => {
            let counters = counter_snapshot.counters();
            admission_boundary_certification_evidence_json(
                admission_boundary_matrix,
                &counters,
                counters.digest(),
            )
        }
        WritebackHarnessExecution::CrossFamilyReplayLoopIsolationCertification {
            replay_loop_matrix,
            counter_snapshot,
            ..
        } => {
            let counters = counter_snapshot.counters();
            replay_loop_certification_evidence_json(
                replay_loop_matrix,
                &counters,
                counters.digest(),
            )
        }
        WritebackHarnessExecution::HostMapperParityCertification {
            mapper_parity_matrix,
            counter_snapshot,
            ..
        } => {
            let counters = counter_snapshot.counters();
            mapper_parity_certification_evidence_json(
                mapper_parity_matrix,
                &counters,
                counters.digest(),
            )
        }
    }
}

pub(in crate::harness::adapter::adapter_impl) fn summary_json(
    execution: &WritebackHarnessExecution,
) -> serde_json::Value {
    match execution {
        WritebackHarnessExecution::DuplicateCertification {
            first_bundle_digest,
            repeated_bundle_digest,
            replay_bundle_digest,
            duplicate_authority_matrix,
            counter_snapshot,
        } => json!({
            "first_bundle_digest": first_bundle_digest,
            "repeated_bundle_digest": repeated_bundle_digest,
            "replay_bundle_digest": replay_bundle_digest,
            "duplicate_authority_matrix": duplicate_authority_matrix_json(duplicate_authority_matrix),
            "counter_artifact": counter_artifact_json(*counter_snapshot),
            "certification_evidence": certification_evidence_json(execution),
            "counter_snapshot": counter_snapshot_json(*counter_snapshot),
            "counter_digest": counter_snapshot.counters().digest(),
        }),
        WritebackHarnessExecution::AuthorityDenialCertification {
            failure_digest,
            authority_denial,
            zero_residue_report,
            counter_snapshot,
        } => json!({
            "failure_digest": failure_digest,
            "authority_denial": authority_denial_matrix_json(authority_denial),
            "zero_residue_report": authority_denial_zero_residue_proof_json(*zero_residue_report),
            "counter_artifact": counter_artifact_json(*counter_snapshot),
            "certification_evidence": certification_evidence_json(execution),
            "counter_snapshot": counter_snapshot_json(*counter_snapshot),
            "counter_digest": counter_snapshot.counters().digest(),
        }),
        WritebackHarnessExecution::FeedbackLoopCertification {
            feedback_loop_digest,
            feedback_route_identity,
            feedback_origin_matrix,
            counter_snapshot,
        } => json!({
            "feedback_loop_digest": feedback_loop_digest,
            "feedback_route_digest": digest_string(
                "bridge-writeback-feedback-route",
                feedback_route_identity.as_str()
            ).to_string(),
            "feedback_origin_matrix": feedback_loop_matrix_json(feedback_origin_matrix),
            "counter_artifact": counter_artifact_json(*counter_snapshot),
            "certification_evidence": certification_evidence_json(execution),
            "counter_snapshot": counter_snapshot_json(*counter_snapshot),
            "counter_digest": counter_snapshot.counters().digest(),
        }),
        WritebackHarnessExecution::ReplayMismatchCertification {
            replay_validation_digest,
            replay_mismatch_matrix,
            counter_snapshot,
        } => json!({
            "replay_validation_digest": replay_validation_digest,
            "replay_mismatch_matrix": replay_mismatch_matrix_json(replay_mismatch_matrix),
            "counter_artifact": counter_artifact_json(*counter_snapshot),
            "certification_evidence": certification_evidence_json(execution),
            "counter_snapshot": counter_snapshot_json(*counter_snapshot),
            "counter_digest": counter_snapshot.counters().digest(),
        }),
        WritebackHarnessExecution::ExtensibleFamilyCertification {
            family_extension_digest,
            family_extension_matrix,
            counter_snapshot,
        } => json!({
            "family_extension_digest": family_extension_digest,
            "family_extension_matrix": family_extension_matrix_json(family_extension_matrix),
            "counter_artifact": counter_artifact_json(*counter_snapshot),
            "certification_evidence": certification_evidence_json(execution),
            "counter_snapshot": counter_snapshot_json(*counter_snapshot),
            "counter_digest": counter_snapshot.counters().digest(),
        }),
        WritebackHarnessExecution::MultiFamilyAdmissionBoundaryCertification {
            family_extension_digest,
            admission_boundary_matrix,
            counter_snapshot,
        } => json!({
            "family_extension_digest": family_extension_digest,
            "multi_family_admission_boundary_matrix": admission_boundary_matrix_json(admission_boundary_matrix),
            "counter_artifact": counter_artifact_json(*counter_snapshot),
            "certification_evidence": certification_evidence_json(execution),
            "counter_snapshot": counter_snapshot_json(*counter_snapshot),
            "counter_digest": counter_snapshot.counters().digest(),
        }),
        WritebackHarnessExecution::CrossFamilyReplayLoopIsolationCertification {
            family_extension_digest,
            replay_loop_matrix,
            counter_snapshot,
        } => json!({
            "family_extension_digest": family_extension_digest,
            "cross_family_replay_loop_isolation_matrix": replay_loop_isolation_matrix_json(replay_loop_matrix),
            "counter_artifact": counter_artifact_json(*counter_snapshot),
            "certification_evidence": certification_evidence_json(execution),
            "counter_snapshot": counter_snapshot_json(*counter_snapshot),
            "counter_digest": counter_snapshot.counters().digest(),
        }),
        WritebackHarnessExecution::HostMapperParityCertification {
            family_extension_digest,
            mapper_parity_matrix,
            counter_snapshot,
        } => json!({
            "family_extension_digest": family_extension_digest,
            "host_mapper_parity_matrix": mapper_parity_matrix_json(mapper_parity_matrix),
            "counter_artifact": counter_artifact_json(*counter_snapshot),
            "certification_evidence": certification_evidence_json(execution),
            "counter_snapshot": counter_snapshot_json(*counter_snapshot),
            "counter_digest": counter_snapshot.counters().digest(),
        }),
    }
}

pub(in crate::harness::adapter::adapter_impl) fn extensions_json(
    execution: &WritebackHarnessExecution,
) -> BTreeMap<String, serde_json::Value> {
    BTreeMap::from([(
        "bridge_writeback_certification_bundle".to_string(),
        summary_json(execution),
    )])
}

fn counter_artifact_json(counter_snapshot: WritebackCounterSnapshot) -> serde_json::Value {
    let counters = counter_snapshot.counters();
    json!({
        "snapshot": counter_snapshot_json(counter_snapshot),
        "canonical_basis": counters.canonical_basis(),
        "digest": counters.digest(),
    })
}

fn counter_snapshot_json(counter_snapshot: WritebackCounterSnapshot) -> serde_json::Value {
    writeback_counter_snapshot_json(&counter_snapshot.counters())
}
