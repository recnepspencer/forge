use forge_harness::facade::{
    certification_matrix, parity_suite, ExecutionProfile, ExecutionRequest, HarnessAdapter,
    RunRecord, ScenarioPlan,
};
use serde_json::json;

use crate::facade::BridgeRuntimePolicy;
use crate::harness::adapter::BridgeHarnessAdapter;
use crate::harness::fixtures::BridgeHarnessFixture;

use super::support::{committed_patch, registration, snapshot};

fn writeback_fixture(
    name: &str,
    policy: BridgeRuntimePolicy,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(policy)
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("writeback")
    .declare_observation("writeback")
    .compile()
}

fn direct_host_profile(name: &str) -> ExecutionProfile {
    ExecutionProfile::development(name)
}

fn wrapped_host_profile(name: &str) -> ExecutionProfile {
    ExecutionProfile::development(format!("{name}-wrapped-host"))
        .with_metadata("source_adapter_shape", "wrapped")
}

fn forensic_profile(name: &str) -> ExecutionProfile {
    ExecutionProfile::forensic(name)
}

fn execute_writeback_run(
    fixture_name: &str,
    policy: BridgeRuntimePolicy,
    profile: ExecutionProfile,
    request_name: &str,
    target: &str,
) -> RunRecord<String> {
    let adapter = BridgeHarnessAdapter;
    let fixture = writeback_fixture(fixture_name, policy);
    let mut runtime = adapter.create_runtime().expect("writeback harness runtime");
    adapter
        .prepare_runtime(&mut runtime, &profile)
        .expect("writeback harness prepare");
    adapter
        .load_fixture(&mut runtime, &fixture)
        .expect("writeback harness load fixture");
    adapter
        .execute(
            &mut runtime,
            &fixture,
            &ExecutionRequest::target(request_name, target.to_string()),
            &profile,
        )
        .expect("writeback harness execution")
}

fn certify_writeback_target(target: &str) -> serde_json::Value {
    let fixture = writeback_fixture(
        &format!("bridge-{target}"),
        BridgeRuntimePolicy::development(),
    );
    let request = ExecutionRequest::target(format!("{target}-control"), target.to_string());

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        direct_host_profile("baseline-direct-host"),
    )
    .candidates([wrapped_host_profile("candidate")])
    .compare()
    .expect("writeback certification parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);

    let control_run = execute_writeback_run(
        &format!("bridge-{target}-control"),
        BridgeRuntimePolicy::development(),
        direct_host_profile("baseline-direct-host"),
        &format!("{target}-control"),
        target,
    );
    let replay_run = execute_writeback_run(
        &format!("bridge-{target}-replay"),
        BridgeRuntimePolicy::development(),
        wrapped_host_profile("candidate"),
        &format!("{target}-replay"),
        target,
    );

    assert_eq!(control_run.summary, replay_run.summary);
    assert_eq!(control_run.extensions, replay_run.extensions);
    control_run.extensions["bridge_writeback_certification_bundle"].clone()
}

#[test]
fn duplicate_writeback_attempt_bundle_is_replay_safe_and_bounded() {
    let target = "writeback-duplicate-certify";
    let fixture = writeback_fixture(
        "bridge-writeback-duplicate-certification",
        BridgeRuntimePolicy::development(),
    );
    let request = ExecutionRequest::target("writeback-duplicate-control", target.to_string());

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        direct_host_profile("baseline-direct-host"),
    )
    .candidates([wrapped_host_profile("candidate")])
    .compare()
    .expect("writeback duplicate parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);

    let control_run = execute_writeback_run(
        "bridge-writeback-duplicate-control",
        BridgeRuntimePolicy::development(),
        direct_host_profile("baseline-direct-host"),
        "writeback-duplicate-control",
        target,
    );
    let replay_run = execute_writeback_run(
        "bridge-writeback-duplicate-replay",
        BridgeRuntimePolicy::development(),
        wrapped_host_profile("candidate"),
        "writeback-duplicate-replay",
        target,
    );
    let hostile_run = execute_writeback_run(
        "bridge-writeback-duplicate-hostile",
        BridgeRuntimePolicy::forensic(),
        forensic_profile("hostile-forensic"),
        "writeback-duplicate-hostile",
        target,
    );

    assert_eq!(control_run.summary, replay_run.summary);
    assert_eq!(control_run.extensions, replay_run.extensions);

    let bundle = &control_run.extensions["bridge_writeback_certification_bundle"];
    let hostile_bundle = &hostile_run.extensions["bridge_writeback_certification_bundle"];
    assert_eq!(
        bundle["certification_evidence"]["certification_shape"],
        json!("duplicate-certification")
    );
    assert_eq!(
        bundle["certification_evidence"]["writeback_digest"],
        bundle["duplicate_authority_matrix"]["writeback_digest"]
    );
    assert_eq!(
        bundle["certification_evidence"]["truth_integrity_report"],
        bundle["duplicate_authority_matrix"]["boundedness_proof"]
    );
    assert_eq!(
        bundle["counter_artifact"]["snapshot"],
        bundle["counter_snapshot"]
    );
    assert_eq!(
        bundle["counter_artifact"]["digest"],
        bundle["counter_digest"]
    );
    assert!(bundle["counter_artifact"]["canonical_basis"]
        .as_str()
        .is_some());
    assert_eq!(
        bundle["repeated_bundle_digest"],
        control_run.summary["repeated_bundle_digest"]
    );
    assert_eq!(
        bundle["replay_bundle_digest"],
        control_run.summary["replay_bundle_digest"]
    );
    assert_eq!(
        bundle["duplicate_authority_matrix"]["writeback_digest"],
        bundle["replay_bundle_digest"]
    );
    assert_eq!(
        bundle["duplicate_authority_matrix"]["replay_bundle_report"]["digest"],
        bundle["replay_bundle_digest"]
    );
    assert!(
        bundle["duplicate_authority_matrix"]["replay_bundle_report"]["semantic_digest"]
            .as_str()
            .is_some()
    );
    assert_eq!(
        bundle["duplicate_authority_matrix"]["replay_bundle_report"]["strategy_class"],
        json!("ProjectedStateDiffReconciliation")
    );
    assert_eq!(
        bundle["duplicate_authority_matrix"]["replay_bundle_report"]["retry_disposition"],
        json!("SemanticNoopSuppressionRequired")
    );
    assert_eq!(
        bundle["duplicate_authority_matrix"]["replay_bundle_report"]["outcome_class"],
        json!("CanonicalNoop")
    );
    assert_eq!(
        bundle["counter_snapshot"],
        control_run.summary["counter_snapshot"]
    );
    assert!(bundle["duplicate_authority_matrix"]["causality_digest"]
        .as_str()
        .is_some());
    assert!(
        bundle["duplicate_authority_matrix"]["authority_boundary_matrix"]
            ["first_authority_request_digest"]
            .as_str()
            .is_some()
    );
    assert!(
        bundle["duplicate_authority_matrix"]["authority_boundary_matrix"]
            ["first_authority_receipt_digest"]
            .as_str()
            .is_some()
    );
    assert_eq!(
        bundle["duplicate_authority_matrix"]["loop_prevention_report"]["first_disposition"],
        json!("AllowAuthoritativeAttempt")
    );
    assert_eq!(
        bundle["duplicate_authority_matrix"]["authority_boundary_matrix"]
            ["first_strategy_compatibility_disposition"],
        json!("Compatible")
    );
    assert_eq!(
        bundle["duplicate_authority_matrix"]["authority_boundary_matrix"]
            ["repeated_strategy_compatibility_disposition"],
        json!("Compatible")
    );
    assert!(
        bundle["duplicate_authority_matrix"]["authority_boundary_matrix"]["first_candidate_digest"]
            .as_str()
            .is_some()
    );
    assert!(
        bundle["duplicate_authority_matrix"]["authority_boundary_matrix"]
            ["repeated_candidate_digest"]
            .as_str()
            .is_some()
    );
    assert_eq!(
        bundle["duplicate_authority_matrix"]["boundedness_proof"]["authoritative_commit_count"],
        json!(1)
    );
    assert_eq!(
        bundle["duplicate_authority_matrix"]["boundedness_proof"]["canonical_noop_count"],
        json!(1)
    );
    assert_eq!(
        bundle["duplicate_authority_matrix"]["boundedness_proof"]["duplicate_causality_detected"],
        json!(true)
    );
    assert_eq!(
        bundle["duplicate_authority_matrix"]["boundedness_proof"]["loop_converged"],
        json!(true)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_loop_prevention_rejection_count"],
        json!(0)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_commit_count"],
        json!(1)
    );
    assert_eq!(bundle["counter_snapshot"]["writeback_noop_count"], json!(1));
    assert_eq!(
        bundle["counter_snapshot"]["writeback_authority_bypass_rejection_count"],
        json!(0)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_validation_rejection_count"],
        json!(0)
    );
    assert_eq!(
        bundle["duplicate_authority_matrix"]["boundedness_proof"],
        hostile_bundle["duplicate_authority_matrix"]["boundedness_proof"]
    );
    assert_eq!(
        bundle["counter_snapshot"],
        hostile_bundle["counter_snapshot"]
    );
    assert_eq!(bundle["counter_digest"], hostile_bundle["counter_digest"]);
    assert_eq!(
        bundle["duplicate_authority_matrix"]["route_digest"],
        hostile_bundle["duplicate_authority_matrix"]["route_digest"]
    );
}

#[test]
fn writeback_bypass_rejection_is_typed_and_leaves_zero_authority_residue() {
    let fixture = writeback_fixture(
        "bridge-writeback-bypass-certification",
        BridgeRuntimePolicy::development(),
    );
    let request = ExecutionRequest::target(
        "writeback-bypass-control",
        "writeback-bypass-certify".to_string(),
    );

    let report = certification_matrix(
        BridgeHarnessAdapter,
        fixture,
        request,
        direct_host_profile("baseline-direct-host"),
    )
    .candidates([wrapped_host_profile("candidate")])
    .certify()
    .expect("writeback bypass certification matrix should certify");

    assert!(report.matched);
    assert_eq!(report.cases.len(), 1);

    let control_run = execute_writeback_run(
        "bridge-writeback-bypass-control",
        BridgeRuntimePolicy::development(),
        direct_host_profile("baseline-direct-host"),
        "writeback-bypass-control",
        "writeback-bypass-certify",
    );
    let replay_run = execute_writeback_run(
        "bridge-writeback-bypass-replay",
        BridgeRuntimePolicy::development(),
        wrapped_host_profile("candidate"),
        "writeback-bypass-replay",
        "writeback-bypass-certify",
    );

    assert_eq!(control_run.summary, replay_run.summary);
    assert_eq!(control_run.extensions, replay_run.extensions);

    let bundle = &control_run.extensions["bridge_writeback_certification_bundle"];
    assert_eq!(
        bundle["certification_evidence"]["certification_shape"],
        json!("bypass-certification")
    );
    assert_eq!(
        bundle["certification_evidence"]["failure_digest"],
        bundle["failure_digest"]
    );
    assert_eq!(
        bundle["certification_evidence"]["truth_integrity_report"],
        bundle["zero_residue_report"]
    );
    assert_eq!(
        bundle["counter_artifact"]["snapshot"],
        bundle["counter_snapshot"]
    );
    assert_eq!(
        bundle["counter_artifact"]["digest"],
        bundle["counter_digest"]
    );
    assert!(bundle["counter_artifact"]["canonical_basis"]
        .as_str()
        .is_some());
    assert_eq!(
        bundle["failure_digest"],
        control_run.summary["failure_digest"]
    );
    assert_eq!(
        bundle["bypass_rejection"]["failure_kind"],
        json!("PreviewWritebackRejected")
    );
    assert_eq!(
        bundle["bypass_rejection"]["authority_boundary_matrix"]["preview_validation_failure"]
            ["bypass_class"],
        json!("validation-short-circuit")
    );
    assert_eq!(
        bundle["bypass_rejection"]["authority_boundary_matrix"]["unbound_authority_failure"]
            ["bypass_class"],
        json!("unbound-authority-execution")
    );
    assert_eq!(
        bundle["bypass_rejection"]["authority_boundary_matrix"]["unbound_authority_failure"]
            ["failure_kind"],
        json!("AuthorityBypassRejected")
    );
    assert_eq!(
        bundle["bypass_rejection"]["authority_boundary_matrix"]["merge_authority_failure"]
            ["bypass_class"],
        json!("merge-authority-rejection")
    );
    assert_eq!(
        bundle["bypass_rejection"]["authority_boundary_matrix"]["merge_authority_failure"]
            ["failure_kind"],
        json!("MergeAuthorityRejected")
    );
    assert_eq!(
        bundle["bypass_rejection"]["authority_boundary_matrix"]["unsafe_feedback_failure"]
            ["bypass_class"],
        json!("unsafe-feedback-preauthority")
    );
    assert_eq!(
        bundle["bypass_rejection"]["authority_boundary_matrix"]["unsafe_feedback_failure"]
            ["failure_kind"],
        json!("InvariantRejected")
    );
    assert_eq!(
        bundle["bypass_rejection"]["authority_boundary_matrix"]["unsafe_feedback_failure"]
            ["authority_request_digest"],
        serde_json::Value::Null
    );
    assert_eq!(
        bundle["bypass_rejection"]["authority_boundary_matrix"]["unsafe_feedback_failure"]
            ["authority_receipt_digest"],
        serde_json::Value::Null
    );
    assert_eq!(
        bundle["bypass_rejection"]["authority_boundary_matrix"]["contradictory_feedback_failure"]
            ["bypass_class"],
        json!("contradictory-feedback-preauthority")
    );
    assert_eq!(
        bundle["bypass_rejection"]["authority_boundary_matrix"]["contradictory_feedback_failure"]
            ["failure_kind"],
        json!("InvariantRejected")
    );
    assert_eq!(
        bundle["bypass_rejection"]["authority_boundary_matrix"]["contradictory_feedback_failure"]
            ["authority_request_digest"],
        serde_json::Value::Null
    );
    assert_eq!(
        bundle["bypass_rejection"]["authority_boundary_matrix"]["contradictory_feedback_failure"]
            ["authority_receipt_digest"],
        serde_json::Value::Null
    );
    assert_eq!(
        bundle["bypass_rejection"]["loop_prevention_report"]["unsafe_feedback_partial"]
            ["disposition"],
        json!("RejectAsUnsafeFeedback")
    );
    assert_eq!(
        bundle["bypass_rejection"]["loop_prevention_report"]["unsafe_feedback_contradictory"]
            ["disposition"],
        json!("RejectAsUnsafeFeedback")
    );
    assert!(
        bundle["bypass_rejection"]["authority_boundary_matrix"]["merge_authority_failure"]
            ["authority_request_digest"]
            .as_str()
            .is_some()
    );
    assert!(
        bundle["bypass_rejection"]["authority_boundary_matrix"]["merge_authority_failure"]
            ["authority_receipt_digest"]
            .as_str()
            .is_some()
    );
    assert_eq!(
        bundle["bypass_rejection"]["replay_digest"],
        serde_json::Value::Null
    );
    assert_eq!(
        bundle["zero_residue_report"]["authoritative_commit_count"],
        json!(0)
    );
    assert_eq!(
        bundle["zero_residue_report"]["authoritative_artifact_count"],
        json!(0)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_failure_count"],
        json!(4)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_authority_bypass_rejection_count"],
        json!(1)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_validation_rejection_count"],
        json!(2)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_commit_count"],
        json!(0)
    );
    assert_eq!(bundle["counter_snapshot"]["writeback_noop_count"], json!(0));
    assert_eq!(
        bundle["counter_snapshot"]["writeback_request_count"],
        json!(1)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_effect_width"],
        json!(4)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_idempotence_check_count"],
        json!(4)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_loop_prevention_check_count"],
        json!(4)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_loop_prevention_rejection_count"],
        json!(2)
    );
}

#[test]
fn bridge_origin_feedback_lane_converges_without_second_authoritative_commit() {
    let target = "writeback-feedback-certify";
    let fixture = writeback_fixture(
        "bridge-writeback-feedback-certification",
        BridgeRuntimePolicy::development(),
    );
    let request = ExecutionRequest::target("writeback-feedback-control", target.to_string());

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        direct_host_profile("baseline-direct-host"),
    )
    .candidates([wrapped_host_profile("candidate")])
    .compare()
    .expect("writeback feedback parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);

    let control_run = execute_writeback_run(
        "bridge-writeback-feedback-control",
        BridgeRuntimePolicy::development(),
        direct_host_profile("baseline-direct-host"),
        "writeback-feedback-control",
        target,
    );
    let replay_run = execute_writeback_run(
        "bridge-writeback-feedback-replay",
        BridgeRuntimePolicy::development(),
        wrapped_host_profile("candidate"),
        "writeback-feedback-replay",
        target,
    );
    let hostile_run = execute_writeback_run(
        "bridge-writeback-feedback-hostile",
        BridgeRuntimePolicy::forensic(),
        forensic_profile("hostile-forensic"),
        "writeback-feedback-hostile",
        target,
    );

    assert_eq!(control_run.summary, replay_run.summary);
    assert_eq!(control_run.extensions, replay_run.extensions);

    let bundle = &control_run.extensions["bridge_writeback_certification_bundle"];
    let hostile_bundle = &hostile_run.extensions["bridge_writeback_certification_bundle"];
    assert_eq!(
        bundle["certification_evidence"]["certification_shape"],
        json!("feedback-loop-certification")
    );
    assert_eq!(
        bundle["certification_evidence"]["writeback_digest"],
        bundle["feedback_origin_matrix"]["writeback_digest"]
    );
    assert_eq!(
        bundle["certification_evidence"]["truth_integrity_report"],
        bundle["feedback_origin_matrix"]["boundedness_proof"]
    );
    assert_eq!(
        bundle["counter_artifact"]["snapshot"],
        bundle["counter_snapshot"]
    );
    assert_eq!(
        bundle["counter_artifact"]["digest"],
        bundle["counter_digest"]
    );
    assert!(bundle["counter_artifact"]["canonical_basis"]
        .as_str()
        .is_some());
    assert_eq!(
        bundle["feedback_loop_digest"],
        control_run.summary["feedback_loop_digest"]
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["writeback_digest"],
        bundle["feedback_origin_matrix"]["replay_digest"]
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["replay_bundle_report"]["digest"],
        bundle["feedback_origin_matrix"]["replay_digest"]
    );
    assert!(
        bundle["feedback_origin_matrix"]["replay_bundle_report"]["semantic_digest"]
            .as_str()
            .is_some()
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["replay_bundle_report"]["strategy_class"],
        json!("ProjectedStateDiffReconciliation")
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["replay_bundle_report"]["retry_disposition"],
        json!("SemanticNoopSuppressionRequired")
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["replay_bundle_report"]["outcome_class"],
        json!("CanonicalNoop")
    );
    assert_eq!(
        bundle["feedback_route_digest"],
        control_run.summary["feedback_route_digest"]
    );
    assert_eq!(
        bundle["counter_snapshot"],
        control_run.summary["counter_snapshot"]
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["boundedness_proof"]["authoritative_commit_count"],
        json!(1)
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["boundedness_proof"]["replayed_feedback_outcome_class"],
        json!("CanonicalNoop")
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["loop_prevention_disposition"],
        json!("CanonicalNoop")
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["loop_prevention_report"]["disposition"],
        json!("CanonicalNoop")
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["changed_effect_feedback_matrix"]["failure_kind"],
        json!("InvariantRejected")
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["changed_effect_feedback_matrix"]
            ["same_causality_as_initial"],
        json!(true)
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["changed_effect_feedback_matrix"]
            ["same_feedback_provenance_as_initial"],
        json!(false)
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["authority_boundary_matrix"]
            ["strategy_compatibility_disposition"],
        json!("Compatible")
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["authority_boundary_matrix"]["candidate_digest"],
        serde_json::Value::Null
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["authority_boundary_matrix"]["authority_request_digest"],
        serde_json::Value::Null
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["interleaved_truth_matrix"]
            ["ordinary_truth_commit_identity"],
        json!("commit-ordinary")
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["interleaved_truth_matrix"]
            ["interleaving_preserved_single_authoritative_commit"],
        json!(true)
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["restart_replay_matrix"]
            ["rebuilt_loop_prevention_disposition"],
        json!("CanonicalNoop")
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["restart_replay_matrix"]
            ["rebuilt_authority_receipt_present"],
        json!(false)
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["restart_replay_matrix"]
            ["replay_equivalent_to_live_feedback"],
        json!(true)
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["boundedness_proof"]["feedback_publication_routed"],
        json!(true)
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["boundedness_proof"]
            ["changed_effect_retrigger_failure_kind"],
        json!("InvariantRejected")
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["boundedness_proof"]["ordinary_truth_interleaved"],
        json!(true)
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["boundedness_proof"]["restart_replay_converged"],
        json!(true)
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["boundedness_proof"]["feedback_converged"],
        json!(true)
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["boundedness_proof"]["replayed_authority_receipt_present"],
        json!(false)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_request_count"],
        json!(1)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_commit_count"],
        json!(1)
    );
    assert_eq!(bundle["counter_snapshot"]["writeback_noop_count"], json!(2));
    assert_eq!(
        bundle["counter_snapshot"]["writeback_causality_match_count"],
        json!(3)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_effect_width"],
        json!(4)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_idempotence_check_count"],
        json!(4)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_loop_prevention_check_count"],
        json!(4)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_loop_prevention_rejection_count"],
        json!(1)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_failure_count"],
        json!(1)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_replay_request_count"],
        json!(0)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_authority_bypass_rejection_count"],
        json!(0)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_validation_rejection_count"],
        json!(1)
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["feedback_provenance_digest"],
        bundle["feedback_origin_matrix"]["carried_feedback_provenance_digest"]
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["initial_causality_digest"],
        bundle["feedback_origin_matrix"]["carried_causality_digest"]
    );
    assert_eq!(
        bundle["feedback_origin_matrix"]["boundedness_proof"],
        hostile_bundle["feedback_origin_matrix"]["boundedness_proof"]
    );
    assert_eq!(
        bundle["counter_snapshot"],
        hostile_bundle["counter_snapshot"]
    );
    assert_eq!(bundle["counter_digest"], hostile_bundle["counter_digest"]);
}

#[test]
fn writeback_replay_mismatch_is_typed_and_counted() {
    let fixture = writeback_fixture(
        "bridge-writeback-replay-mismatch-certification",
        BridgeRuntimePolicy::development(),
    );
    let request = ExecutionRequest::target(
        "writeback-replay-mismatch-control",
        "writeback-replay-mismatch-certify".to_string(),
    );

    let report = certification_matrix(
        BridgeHarnessAdapter,
        fixture,
        request,
        direct_host_profile("baseline-direct-host"),
    )
    .candidates([wrapped_host_profile("candidate")])
    .certify()
    .expect("writeback replay mismatch certification matrix should certify");

    assert!(report.matched);
    assert_eq!(report.cases.len(), 1);

    let control_run = execute_writeback_run(
        "bridge-writeback-replay-mismatch-control",
        BridgeRuntimePolicy::development(),
        direct_host_profile("baseline-direct-host"),
        "writeback-replay-mismatch-control",
        "writeback-replay-mismatch-certify",
    );
    let replay_run = execute_writeback_run(
        "bridge-writeback-replay-mismatch-replay",
        BridgeRuntimePolicy::development(),
        wrapped_host_profile("candidate"),
        "writeback-replay-mismatch-replay",
        "writeback-replay-mismatch-certify",
    );

    assert_eq!(control_run.summary, replay_run.summary);
    assert_eq!(control_run.extensions, replay_run.extensions);

    let bundle = &control_run.extensions["bridge_writeback_certification_bundle"];
    assert_eq!(
        bundle["certification_evidence"]["certification_shape"],
        json!("replay-mismatch-certification")
    );
    assert_eq!(
        bundle["certification_evidence"]["failure_digest"],
        bundle["replay_validation_digest"]
    );
    assert_eq!(
        bundle["certification_evidence"]["truth_integrity_report"],
        bundle["replay_mismatch_matrix"]["restart_replay_matrix"]
    );
    assert_eq!(
        bundle["counter_artifact"]["snapshot"],
        bundle["counter_snapshot"]
    );
    assert_eq!(
        bundle["counter_artifact"]["digest"],
        bundle["counter_digest"]
    );
    assert!(bundle["counter_artifact"]["canonical_basis"]
        .as_str()
        .is_some());
    assert_eq!(
        bundle["replay_validation_digest"],
        control_run.summary["replay_validation_digest"]
    );
    assert_eq!(
        bundle["replay_mismatch_matrix"]["failure_kind"],
        json!("ReplayMismatch")
    );
    assert_eq!(
        bundle["replay_mismatch_matrix"]["semantic_mismatch_detected"],
        json!(true)
    );
    assert_eq!(
        bundle["replay_mismatch_matrix"]["diagnostic_detail_changed"],
        json!(true)
    );
    assert_eq!(
        bundle["replay_mismatch_matrix"]["restart_replay_matrix"]["rebuilt_failure_kind"],
        json!("ReplayMismatch")
    );
    assert_eq!(
        bundle["replay_mismatch_matrix"]["restart_replay_matrix"]["restart_mismatch_detected"],
        json!(true)
    );
    assert_ne!(
        bundle["replay_mismatch_matrix"]["expected_effect_digest"],
        bundle["replay_mismatch_matrix"]["replayed_effect_digest"]
    );
    assert_ne!(
        bundle["replay_mismatch_matrix"]["expected_semantic_digest"],
        bundle["replay_mismatch_matrix"]["replayed_semantic_digest"]
    );
    assert_ne!(
        bundle["replay_mismatch_matrix"]["expected_semantic_digest"],
        bundle["replay_mismatch_matrix"]["restart_replay_matrix"]["rebuilt_semantic_digest"]
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_replay_request_count"],
        json!(2)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_replay_mismatch_count"],
        json!(2)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_failure_count"],
        json!(2)
    );
}

#[test]
fn extensible_writeback_families_remain_parity_safe_and_family_isolated() {
    let bundle = certify_writeback_target("writeback-family-extension-certify");
    assert_eq!(
        bundle["certification_evidence"]["certification_shape"],
        json!("extensible-family-certification")
    );
    assert!(bundle["family_extension_digest"].as_str().is_some());
    assert_eq!(
        bundle["family_extension_matrix"]["projected_family"]["causality_digest"],
        bundle["family_extension_matrix"]["aspect_family"]["causality_digest"]
    );
    assert!(
        bundle["family_extension_matrix"]["projected_family"]["admission_record_digest"]
            .as_str()
            .is_some()
    );
    assert!(
        bundle["family_extension_matrix"]["aspect_family"]["admission_record_digest"]
            .as_str()
            .is_some()
    );
    assert_eq!(
        bundle["family_extension_matrix"]["cross_family_replay_isolation"]
            ["semantic_digest_separated"],
        json!(true)
    );
    assert_eq!(
        bundle["family_extension_matrix"]["cross_family_replay_isolation"]
            ["bundle_digest_separated"],
        json!(true)
    );
    assert_eq!(
        bundle["family_extension_matrix"]["cross_family_replay_isolation"]["failure_kind"],
        json!("ReplayMismatch")
    );
    assert!(
        bundle["family_extension_matrix"]["cross_family_replay_isolation"]
            ["family_replay_record_digest"]
            .as_str()
            .is_some()
    );
    assert!(
        bundle["family_extension_matrix"]["cross_family_replay_isolation"]["decision_trace_digest"]
            .as_str()
            .is_some()
    );
    assert_eq!(
        bundle["family_extension_matrix"]["same_family_equivalence"]["semantic_digest_equal"],
        json!(true)
    );
    assert_eq!(
        bundle["family_extension_matrix"]["same_family_equivalence"]["bundle_digest_equal"],
        json!(true)
    );
    assert_eq!(
        bundle["family_extension_matrix"]["same_family_equivalence"]["effect_digest_equal"],
        json!(true)
    );
    assert_eq!(
        bundle["family_extension_matrix"]["same_family_equivalence"]["mapped_input_digest_equal"],
        json!(true)
    );
    assert!(bundle["family_extension_matrix"]["same_family_equivalence"]
        ["family_execution_record_digest"]
        .as_str()
        .is_some());
    assert!(
        bundle["family_extension_matrix"]["same_family_equivalence"]["decision_trace_digest"]
            .as_str()
            .is_some()
    );
    assert_eq!(
        bundle["family_extension_matrix"]["same_family_changed_causality"]
            ["causality_digest_separated"],
        json!(true)
    );
    assert_eq!(
        bundle["family_extension_matrix"]["same_family_changed_causality"]
            ["semantic_digest_separated"],
        json!(true)
    );
    assert_eq!(
        bundle["family_extension_matrix"]["same_family_changed_causality"]
            ["bundle_digest_separated"],
        json!(true)
    );
    assert_eq!(
        bundle["family_extension_matrix"]["same_family_changed_causality"]["failure_kind"],
        json!("ReplayMismatch")
    );
    assert!(
        bundle["family_extension_matrix"]["same_family_changed_causality"]
            ["family_replay_record_digest"]
            .as_str()
            .is_some()
    );
    assert!(
        bundle["family_extension_matrix"]["same_family_changed_causality"]["decision_trace_digest"]
            .as_str()
            .is_some()
    );
    assert_eq!(
        bundle["family_extension_matrix"]["cross_family_loop_isolation"]["disposition"],
        json!("RejectAsUnsafeFeedback")
    );
    assert_eq!(
        bundle["family_extension_matrix"]["mapper_parity_matrix"]
            ["projected_mapper_envelope_retained"],
        json!(true)
    );
    assert_eq!(
        bundle["family_extension_matrix"]["mapper_parity_matrix"]
            ["aspect_mapper_envelope_retained"],
        json!(true)
    );
    assert_eq!(
        bundle["family_extension_matrix"]["mapper_parity_matrix"]
            ["projected_mapped_input_retained"],
        json!(true)
    );
    assert_eq!(
        bundle["family_extension_matrix"]["mapper_parity_matrix"]["aspect_mapped_input_retained"],
        json!(true)
    );
    assert!(bundle["family_extension_matrix"]["mapper_parity_matrix"]
        ["projected_family_mapper_record_digest"]
        .as_str()
        .is_some());
    assert!(bundle["family_extension_matrix"]["mapper_parity_matrix"]
        ["aspect_family_mapper_record_digest"]
        .as_str()
        .is_some());
    assert!(bundle["family_extension_matrix"]["mapper_parity_matrix"]
        ["projected_family_execution_record_digest"]
        .as_str()
        .is_some());
    assert!(bundle["family_extension_matrix"]["mapper_parity_matrix"]
        ["aspect_family_execution_record_digest"]
        .as_str()
        .is_some());
    assert!(
        bundle["family_extension_matrix"]["mapper_parity_matrix"]["decision_trace_digest"]
            .as_str()
            .is_some()
    );
    assert_eq!(
        bundle["family_extension_matrix"]["shadow_protocol_rejection"]["failure_kind"],
        json!("FamilyBindingMismatch")
    );
    assert!(
        bundle["family_extension_matrix"]["shadow_protocol_rejection"]["decision_trace_digest"]
            .as_str()
            .is_some()
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_commit_count"],
        json!(4)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_family_lookup_count"],
        json!(7)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_family_dispatch_count"],
        json!(7)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_mapper_lowering_count"],
        json!(4)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_decision_record_append_count"],
        json!(11)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_request_count"],
        json!(4)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_failure_count"],
        json!(2)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_replay_request_count"],
        json!(3)
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_replay_mismatch_count"],
        json!(2)
    );
}

#[test]
fn multi_family_writeback_admission_boundary_stays_bridge_native() {
    let bundle = certify_writeback_target("writeback-family-admission-boundary-certify");
    assert_eq!(
        bundle["certification_evidence"]["certification_shape"],
        json!("multi-family-admission-boundary")
    );
    let matrix = &bundle["multi_family_admission_boundary_matrix"];
    assert!(matrix["projected_family"]["admission_record_digest"]
        .as_str()
        .is_some());
    assert!(matrix["aspect_family"]["admission_record_digest"]
        .as_str()
        .is_some());
    assert_eq!(
        matrix["family_admission_matrix"]["projected_family_admitted"],
        json!(true)
    );
    assert_eq!(
        matrix["family_admission_matrix"]["aspect_family_admitted"],
        json!(true)
    );
    assert_eq!(
        matrix["family_admission_matrix"]["family_digest_separated"],
        json!(true)
    );
    assert!(matrix["family_admission_matrix"]["decision_trace_digest"]
        .as_str()
        .is_some());
    assert_eq!(
        matrix["authority_boundary_matrix"]["failure_kind"],
        json!("FamilyBindingMismatch")
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_family_lookup_count"],
        json!(2)
    );
}

#[test]
fn cross_family_replay_and_loop_isolation_remains_family_correct() {
    let bundle = certify_writeback_target("writeback-family-replay-loop-isolation-certify");
    assert_eq!(
        bundle["certification_evidence"]["certification_shape"],
        json!("cross-family-replay-loop-isolation")
    );
    let matrix = &bundle["cross_family_replay_loop_isolation_matrix"];
    assert_eq!(
        matrix["cross_family_replay_isolation"]["semantic_digest_separated"],
        json!(true)
    );
    assert_eq!(
        matrix["cross_family_replay_isolation"]["bundle_digest_separated"],
        json!(true)
    );
    assert_eq!(
        matrix["same_family_equivalence"]["semantic_digest_equal"],
        json!(true)
    );
    assert_eq!(
        matrix["same_family_changed_causality"]["semantic_digest_separated"],
        json!(true)
    );
    assert_eq!(
        matrix["cross_family_loop_isolation"]["disposition"],
        json!("RejectAsUnsafeFeedback")
    );
    assert_eq!(
        bundle["counter_snapshot"]["writeback_decision_record_append_count"],
        json!(11)
    );
}

#[test]
fn host_mapper_parity_rejects_shadow_protocol_behavior() {
    let bundle = certify_writeback_target("writeback-family-mapper-parity-certify");
    assert_eq!(
        bundle["certification_evidence"]["certification_shape"],
        json!("host-mapper-parity-and-shadow-protocol-rejection")
    );
    let matrix = &bundle["host_mapper_parity_matrix"];
    assert_eq!(
        matrix["mapper_parity_matrix"]["projected_mapper_envelope_retained"],
        json!(true)
    );
    assert_eq!(
        matrix["mapper_parity_matrix"]["aspect_mapper_envelope_retained"],
        json!(true)
    );
    assert_eq!(
        matrix["mapper_parity_matrix"]["projected_mapped_input_retained"],
        json!(true)
    );
    assert_eq!(
        matrix["mapper_parity_matrix"]["aspect_mapped_input_retained"],
        json!(true)
    );
    assert_eq!(
        matrix["shadow_protocol_rejection"]["failure_kind"],
        json!("FamilyBindingMismatch")
    );
    assert!(matrix["mapper_parity_matrix"]["decision_trace_digest"]
        .as_str()
        .is_some());
    assert_eq!(
        bundle["counter_snapshot"]["writeback_mapper_lowering_count"],
        json!(2)
    );
}
