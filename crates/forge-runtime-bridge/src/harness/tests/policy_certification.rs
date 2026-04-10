use forge_harness::facade::{
    parity_suite, ExecutionProfile, ExecutionRequest, HarnessAdapter, RunRecord, ScenarioPlan,
};
use serde_json::json;

use crate::facade::BridgeRuntimePolicy;
use crate::harness::adapter::BridgeHarnessAdapter;
use crate::harness::fixtures::BridgeHarnessFixture;

use super::support::{committed_patch, registration, snapshot};

fn policy_fixture(
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
    .declare_input("policy")
    .declare_observation("policy")
    .compile()
}

fn direct_profile(name: &str) -> ExecutionProfile {
    ExecutionProfile::development(name)
}

fn sections_canonical_profile(name: &str) -> ExecutionProfile {
    ExecutionProfile::development(format!("{name}-sections-canonical"))
        .with_metadata("policy_builder_load_order", "sections_canonical")
}

fn sections_reverse_profile(name: &str) -> ExecutionProfile {
    ExecutionProfile::development(format!("{name}-sections-reverse"))
        .with_metadata("policy_builder_load_order", "sections_reverse")
}

fn forensic_policy_profile(name: &str) -> ExecutionProfile {
    ExecutionProfile::forensic(format!("{name}-forensic"))
        .with_metadata("policy_builder_load_order", "sections_reverse")
}

fn execute_policy_run(
    fixture_name: &str,
    policy: BridgeRuntimePolicy,
    profile: ExecutionProfile,
    request_name: &str,
    target: &str,
) -> RunRecord<String> {
    let adapter = BridgeHarnessAdapter;
    let fixture = policy_fixture(fixture_name, policy);
    let mut runtime = adapter.create_runtime().expect("policy harness runtime");
    adapter
        .prepare_runtime(&mut runtime, &profile)
        .expect("policy harness prepare");
    adapter
        .load_fixture(&mut runtime, &fixture)
        .expect("policy harness load fixture");
    adapter
        .execute(
            &mut runtime,
            &fixture,
            &ExecutionRequest::target(request_name, target.to_string()),
            &profile,
        )
        .expect("policy harness execution")
}

#[test]
fn policy_provenance_equivalence_bundle_is_builder_order_and_replay_stable() {
    let target = "policy-provenance-certify";
    let fixture = policy_fixture("bridge-policy-provenance-certification", BridgeRuntimePolicy::development());
    let request = ExecutionRequest::target("policy-provenance-control", target.to_string());

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        direct_profile("baseline-direct-host"),
    )
    .candidates([sections_canonical_profile("candidate")])
    .compare()
    .expect("policy provenance parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);

    let control_run = execute_policy_run(
        "bridge-policy-provenance-control",
        BridgeRuntimePolicy::development(),
        direct_profile("baseline-direct-host"),
        "policy-provenance-control",
        target,
    );
    let replay_run = execute_policy_run(
        "bridge-policy-provenance-replay",
        BridgeRuntimePolicy::development(),
        sections_canonical_profile("candidate"),
        "policy-provenance-replay",
        target,
    );
    let hostile_run = execute_policy_run(
        "bridge-policy-provenance-hostile",
        BridgeRuntimePolicy::development(),
        forensic_policy_profile("hostile"),
        "policy-provenance-hostile",
        target,
    );

    assert_eq!(control_run.summary, replay_run.summary);
    assert_eq!(control_run.extensions, replay_run.extensions);
    assert_eq!(control_run.summary, hostile_run.summary);

    let bundle = &control_run.extensions["bridge_policy_certification_bundle"];
    assert_eq!(bundle["policy_digest"], control_run.summary["policy_digest"]);
    assert_eq!(bundle["policy_matrix"], control_run.summary["policy_matrix"]);
    assert_eq!(
        bundle["policy_provenance_report"],
        control_run.summary["policy_provenance_report"]
    );
    assert_eq!(
        bundle["route_policy_matrix"],
        control_run.summary["route_policy_matrix"]
    );
    assert_eq!(bundle["routing_digest"], control_run.summary["routing_digest"]);
    assert_eq!(bundle["replay_digest"], control_run.summary["replay_digest"]);
    assert_eq!(
        bundle["counter_snapshot"]["declaration_count"],
        json!(2)
    );
    assert_eq!(
        bundle["counter_snapshot"]["declaration_width_count"],
        json!(8)
    );
    assert_eq!(
        bundle["counter_snapshot"]["admission_width_count"],
        json!(8)
    );
    assert_eq!(
        bundle["counter_snapshot"]["replay_bundle_count"],
        json!(2)
    );
    assert_eq!(
        bundle["counter_snapshot"]["ambient_policy_leak_count"],
        json!(0)
    );

    let rows = bundle["policy_provenance_report"]["rows"]
        .as_array()
        .expect("policy provenance rows should be an array");
    let route_rows = bundle["route_policy_matrix"]["rows"]
        .as_array()
        .expect("route policy rows should be an array");
    assert_eq!(rows.len(), 2);
    assert_eq!(route_rows.len(), 2);
    assert_ne!(rows[0]["policy_digest"], rows[1]["policy_digest"]);
    assert_ne!(
        route_rows[0]["route_planning_policy_digest"],
        route_rows[1]["route_planning_policy_digest"]
    );
    assert_ne!(
        route_rows[0]["semantic_route_planning_policy_digest"],
        route_rows[1]["semantic_route_planning_policy_digest"]
    );
    assert_eq!(rows[0]["replay_digest"], rows[0]["replay_digest"]);
    assert!(bundle["routing_digest"].is_string());
}

#[test]
fn policy_rejection_bundle_stays_typed_and_leaves_zero_fallback_residue() {
    let target = "policy-rejection-certify";

    let control_run = execute_policy_run(
        "bridge-policy-rejection-control",
        BridgeRuntimePolicy::development(),
        direct_profile("baseline-direct-host"),
        "policy-rejection-control",
        target,
    );
    let replay_run = execute_policy_run(
        "bridge-policy-rejection-replay",
        BridgeRuntimePolicy::development(),
        sections_canonical_profile("candidate"),
        "policy-rejection-replay",
        target,
    );
    let hostile_run = execute_policy_run(
        "bridge-policy-rejection-hostile",
        BridgeRuntimePolicy::development(),
        sections_reverse_profile("hostile"),
        "policy-rejection-hostile",
        target,
    );

    assert_eq!(control_run.summary, replay_run.summary);
    assert_eq!(control_run.extensions, replay_run.extensions);
    assert_eq!(control_run.summary, hostile_run.summary);

    let bundle = &control_run.extensions["bridge_policy_certification_bundle"];
    assert!(bundle["policy_digest"].is_null());
    assert_eq!(bundle["failure_digest"], control_run.summary["failure_digest"]);
    assert_eq!(
        bundle["policy_provenance_report"]["rows"]
            .as_array()
            .expect("rejection provenance report should exist")
            .len(),
        0
    );
    assert_eq!(
        bundle["counter_snapshot"]["admitted_contract_count"],
        json!(0)
    );
    assert_eq!(
        bundle["counter_snapshot"]["rejected_contract_count"],
        json!(2)
    );
    assert_eq!(
        bundle["counter_snapshot"]["substantive_illegality_count"],
        json!(2)
    );
    assert_eq!(
        bundle["counter_snapshot"]["fallback_success_count"],
        json!(0)
    );

    let rows = bundle["policy_matrix"]["rows"]
        .as_array()
        .expect("policy rejection rows should be an array");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["failure_kind"], json!("UnsupportedExecutionMode"));
    assert_eq!(rows[0]["stage"], json!("Validation"));
    assert_eq!(rows[1]["failure_kind"], json!("ReplayPolicyConflict"));
    assert_eq!(rows[1]["stage"], json!("Admission"));
}

#[test]
fn ambient_policy_leak_resistance_bundle_preserves_preview_equivalence_under_interleave() {
    let target = "policy-ambient-leak-certify";
    let fixture = policy_fixture("bridge-policy-ambient-leak-certification", BridgeRuntimePolicy::development());
    let request = ExecutionRequest::target("policy-ambient-leak-control", target.to_string());

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        direct_profile("baseline-direct-host"),
    )
    .candidates([sections_reverse_profile("candidate")])
    .compare()
    .expect("policy ambient leak parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);

    let control_run = execute_policy_run(
        "bridge-policy-ambient-leak-control",
        BridgeRuntimePolicy::development(),
        direct_profile("baseline-direct-host"),
        "policy-ambient-leak-control",
        target,
    );
    let replay_run = execute_policy_run(
        "bridge-policy-ambient-leak-replay",
        BridgeRuntimePolicy::development(),
        sections_reverse_profile("candidate"),
        "policy-ambient-leak-replay",
        target,
    );

    assert_eq!(control_run.summary, replay_run.summary);
    assert_eq!(control_run.extensions, replay_run.extensions);

    let bundle = &control_run.extensions["bridge_policy_certification_bundle"];
    assert_eq!(bundle["policy_digest"], control_run.summary["policy_digest"]);
    assert_eq!(bundle["policy_matrix"], control_run.summary["policy_matrix"]);
    assert_eq!(
        bundle["policy_provenance_report"],
        control_run.summary["policy_provenance_report"]
    );
    assert_eq!(
        bundle["request_policy_matrix"],
        control_run.summary["request_policy_matrix"]
    );
    assert_eq!(bundle["replay_digest"], control_run.summary["replay_digest"]);
    assert_eq!(
        bundle["counter_snapshot"]["policy_request_count"],
        json!(3)
    );
    assert_eq!(
        bundle["counter_snapshot"]["declaration_count"],
        json!(3)
    );
    assert_eq!(
        bundle["counter_snapshot"]["override_count"],
        json!(0)
    );
    assert_eq!(
        bundle["counter_snapshot"]["truth_view_interleave_count"],
        json!(2)
    );
    assert_eq!(
        bundle["counter_snapshot"]["ambient_policy_leak_count"],
        json!(0)
    );

    let rows = bundle["request_policy_matrix"]["rows"]
        .as_array()
        .expect("request policy matrix rows should be an array");
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0]["semantic_policy_digest"], rows[2]["semantic_policy_digest"]);
    assert_eq!(
        rows[0]["semantic_route_planning_policy_digest"],
        rows[2]["semantic_route_planning_policy_digest"]
    );
    assert_ne!(rows[0]["semantic_policy_digest"], rows[1]["semantic_policy_digest"]);
    assert_ne!(
        rows[0]["semantic_route_planning_policy_digest"],
        rows[1]["semantic_route_planning_policy_digest"]
    );
    assert_eq!(
        bundle["request_policy_matrix"]["branch_local_resolution"],
        json!("Admitted")
    );
    assert_eq!(
        bundle["request_policy_matrix"]["historical_resolution"],
        json!("Admitted")
    );
}
