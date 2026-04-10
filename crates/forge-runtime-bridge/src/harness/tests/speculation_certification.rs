use forge_harness::facade::{
    parity_suite, ExecutionProfile, ExecutionRequest, HarnessAdapter, RunRecord, ScenarioPlan,
};
use serde_json::json;

use crate::facade::BridgeRuntimePolicy;
use crate::harness::adapter::BridgeHarnessAdapter;
use crate::harness::fixtures::BridgeHarnessFixture;

use super::support::{committed_patch, registration, snapshot};

fn speculation_fixture(
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
    .declare_input("speculation")
    .declare_observation("speculation")
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

fn execute_speculation_run(
    fixture_name: &str,
    policy: BridgeRuntimePolicy,
    profile: ExecutionProfile,
    request_name: &str,
    target: &str,
) -> RunRecord<String> {
    let adapter = BridgeHarnessAdapter;
    let fixture = speculation_fixture(fixture_name, policy);
    let mut runtime = adapter
        .create_runtime()
        .expect("speculation harness runtime");
    adapter
        .prepare_runtime(&mut runtime, &profile)
        .expect("speculation harness prepare");
    adapter
        .load_fixture(&mut runtime, &fixture)
        .expect("speculation harness load fixture");
    adapter
        .execute(
            &mut runtime,
            &fixture,
            &ExecutionRequest::target(request_name, target.to_string()),
            &profile,
        )
        .expect("speculation harness execution")
}

#[test]
fn speculative_discard_zero_residue_bundle_is_canonical_and_host_parity_safe() {
    let target = "speculation-discard-certify";
    let fixture = speculation_fixture(
        "bridge-speculation-discard-certification",
        BridgeRuntimePolicy::development(),
    );
    let request = ExecutionRequest::target("speculation-discard-control", target.to_string());

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        direct_host_profile("baseline-direct-host"),
    )
    .candidates([wrapped_host_profile("candidate")])
    .compare()
    .expect("speculation discard parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);

    let control_run = execute_speculation_run(
        "bridge-speculation-discard-control",
        BridgeRuntimePolicy::development(),
        direct_host_profile("baseline-direct-host"),
        "speculation-discard-control",
        target,
    );
    let replay_run = execute_speculation_run(
        "bridge-speculation-discard-replay",
        BridgeRuntimePolicy::development(),
        wrapped_host_profile("candidate"),
        "speculation-discard-replay",
        target,
    );
    let hostile_run = execute_speculation_run(
        "bridge-speculation-discard-hostile",
        BridgeRuntimePolicy::forensic(),
        forensic_profile("hostile-forensic"),
        "speculation-discard-hostile",
        target,
    );

    assert_eq!(control_run.summary, replay_run.summary);
    assert_eq!(control_run.extensions, replay_run.extensions);
    assert_eq!(control_run.summary, hostile_run.summary);
    assert_eq!(
        control_run.extensions["bridge_speculation_certification_bundle"],
        hostile_run.extensions["bridge_speculation_certification_bundle"]
    );

    let bundle = &control_run.extensions["bridge_speculation_certification_bundle"];
    assert_eq!(
        bundle["speculative_resource_digest"],
        control_run.summary["speculative_resource_digest"]
    );
    assert_eq!(
        bundle["discard_residue_report"],
        control_run.summary["discard_residue_report"]
    );
    assert_eq!(
        bundle["routing_digest"],
        control_run.summary["routing_digest"]
    );
    assert_eq!(
        bundle["counter_snapshot"],
        control_run.summary["counter_snapshot"]
    );
    assert_eq!(
        bundle["discard_residue_report"]["authoritative_residue_count"],
        json!(0)
    );
    assert_eq!(
        bundle["counter_snapshot"]["preview_session_count_touched"],
        json!(1)
    );
}

#[test]
fn speculative_commit_boundary_bundle_stays_replay_safe_and_tier_explicit() {
    let target = "speculation-promotion-certify";
    let fixture = speculation_fixture(
        "bridge-speculation-promotion-certification",
        BridgeRuntimePolicy::development(),
    );
    let request = ExecutionRequest::target("speculation-promotion-control", target.to_string());

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        direct_host_profile("baseline-direct-host"),
    )
    .candidates([wrapped_host_profile("candidate")])
    .compare()
    .expect("speculation promotion parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);

    let control_run = execute_speculation_run(
        "bridge-speculation-promotion-control",
        BridgeRuntimePolicy::development(),
        direct_host_profile("baseline-direct-host"),
        "speculation-promotion-control",
        target,
    );
    let replay_run = execute_speculation_run(
        "bridge-speculation-promotion-replay",
        BridgeRuntimePolicy::development(),
        wrapped_host_profile("candidate"),
        "speculation-promotion-replay",
        target,
    );
    let hostile_run = execute_speculation_run(
        "bridge-speculation-promotion-hostile",
        BridgeRuntimePolicy::forensic(),
        forensic_profile("hostile-forensic"),
        "speculation-promotion-hostile",
        target,
    );

    assert_eq!(control_run.summary, replay_run.summary);
    assert_eq!(control_run.extensions, replay_run.extensions);

    let control_bundle = &control_run.extensions["bridge_speculation_certification_bundle"];
    let hostile_bundle = &hostile_run.extensions["bridge_speculation_certification_bundle"];
    assert_eq!(
        control_bundle["speculative_commit_digest"],
        control_run.summary["speculative_commit_digest"]
    );
    assert_eq!(
        control_bundle["preview_vs_authoritative_matrix"],
        control_run.summary["preview_vs_authoritative_matrix"]
    );
    assert_eq!(
        control_bundle["replay_digest"],
        control_run.summary["replay_digest"]
    );
    assert_eq!(
        control_bundle["diagnostics_digest"],
        control_run.summary["diagnostics_digest"]
    );
    assert_eq!(
        control_bundle["speculative_commit_digest"],
        hostile_bundle["speculative_commit_digest"]
    );
    assert_eq!(
        control_bundle["preview_vs_authoritative_matrix"],
        hostile_bundle["preview_vs_authoritative_matrix"]
    );
    assert_eq!(
        control_bundle["replay_digest"],
        hostile_bundle["replay_digest"]
    );
    assert_eq!(
        control_bundle["counter_snapshot"],
        hostile_bundle["counter_snapshot"]
    );
    assert_ne!(
        control_bundle["diagnostics_digest"],
        hostile_bundle["diagnostics_digest"]
    );
    assert_eq!(
        control_bundle["preview_vs_authoritative_matrix"]["promoted_preview"]
            ["preview_session_identity"],
        json!("harness:speculation-promotion")
    );
    assert_eq!(
        control_bundle["preview_vs_authoritative_matrix"]["discarded_preview"]
            ["preview_session_identity"],
        json!("harness:speculation-discard-sibling")
    );
    assert_eq!(
        control_run.extensions["bridge_speculation_record"]["discard_replay_explanation"]
            ["lifecycle_outcome"],
        json!("Discarded")
    );
    assert_eq!(
        control_run.extensions["bridge_speculation_record"]["replay_explanation"]
            ["lifecycle_outcome"],
        json!("Promoted")
    );
}

#[test]
fn preview_lifecycle_churn_bundle_stays_bounded_and_branch_isolated() {
    let target = "speculation-churn-certify";
    let fixture = speculation_fixture(
        "bridge-speculation-churn-certification",
        BridgeRuntimePolicy::development(),
    );
    let request = ExecutionRequest::target("speculation-churn-control", target.to_string());

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        direct_host_profile("baseline-direct-host"),
    )
    .candidates([wrapped_host_profile("candidate")])
    .compare()
    .expect("speculation churn parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);

    let control_run = execute_speculation_run(
        "bridge-speculation-churn-control",
        BridgeRuntimePolicy::development(),
        direct_host_profile("baseline-direct-host"),
        "speculation-churn-control",
        target,
    );
    let replay_run = execute_speculation_run(
        "bridge-speculation-churn-replay",
        BridgeRuntimePolicy::development(),
        wrapped_host_profile("candidate"),
        "speculation-churn-replay",
        target,
    );
    let hostile_run = execute_speculation_run(
        "bridge-speculation-churn-hostile",
        BridgeRuntimePolicy::forensic(),
        forensic_profile("hostile-forensic"),
        "speculation-churn-hostile",
        target,
    );

    assert_eq!(control_run.summary, replay_run.summary);
    assert_eq!(control_run.extensions, replay_run.extensions);
    assert_eq!(control_run.summary, hostile_run.summary);

    let bundle = &control_run.extensions["bridge_speculation_certification_bundle"];
    assert_eq!(
        bundle["preview_lifecycle_digest"],
        control_run.summary["preview_lifecycle_digest"]
    );
    assert_eq!(
        bundle["resource_bound_report"],
        control_run.summary["resource_bound_report"]
    );
    assert_eq!(
        bundle["branch_isolation_matrix"],
        control_run.summary["branch_isolation_matrix"]
    );
    assert_eq!(
        bundle["counter_snapshot"],
        control_run.summary["counter_snapshot"]
    );
    assert_eq!(
        bundle["branch_isolation_matrix"]["rows"]
            .as_array()
            .map(Vec::len),
        Some(3)
    );
    let baseline_route_digest =
        bundle["branch_isolation_matrix"]["baseline_authoritative_route_digest"].clone();
    let final_route_digest =
        bundle["branch_isolation_matrix"]["final_authoritative_route_digest"].clone();
    assert_eq!(baseline_route_digest, final_route_digest);
    for row in bundle["branch_isolation_matrix"]["rows"]
        .as_array()
        .expect("branch isolation rows should be an array")
    {
        assert_eq!(
            row["authoritative_route_digest_after_discard"],
            baseline_route_digest
        );
    }
    assert_eq!(
        bundle["resource_bound_report"]["retained_preview_execution_record_count"],
        json!(3)
    );
    assert_eq!(
        bundle["resource_bound_report"]["retained_preview_discard_record_count"],
        json!(3)
    );
    assert_eq!(
        bundle["resource_bound_report"]["retained_preview_promotion_record_count"],
        json!(0)
    );
    assert_eq!(
        bundle["counter_snapshot"]["preview_session_count_touched"],
        json!(3)
    );
    assert_eq!(
        bundle["resource_bound_report"]["authoritative_route_observation_count"],
        json!(5)
    );
    assert_eq!(
        bundle["counter_snapshot"]["authoritative_route_observation_count"],
        json!(5)
    );
}
