use forge_harness::facade::{
    certification_matrix, parity_suite, ExecutionProfile, ExecutionRequest,
};
use forge_harness::runtime::HarnessAdapter;
use serde_json::json;

use crate::harness::adapter::BridgeHarnessAdapter;

use super::support::{
    direct_host_profile, drifting_adapter_profile, execute_source_run, hostile_target,
    materialize_batch_target, materialize_target, reject_open_snapshot_target,
    reject_snapshot_drift_target, rejecting_adapter_profile, replay_target, source_fixture,
    sources_first_profile, wrapped_host_profile,
};

#[test]
fn bridge_harness_source_control_hostile_and_replay_lanes_preserve_certification_truth() {
    let adapter = BridgeHarnessAdapter;
    let profile = ExecutionProfile::development("baseline");
    let fixture = source_fixture("bridge-source-certification");

    let mut control_runtime = adapter.create_runtime().expect("control harness runtime");
    adapter
        .prepare_runtime(&mut control_runtime, &profile)
        .expect("control harness prepare");
    adapter
        .load_fixture(&mut control_runtime, &fixture)
        .expect("control harness load fixture");
    let control_run = adapter
        .execute(
            &mut control_runtime,
            &fixture,
            &ExecutionRequest::target("source-control", materialize_target()),
            &profile,
        )
        .expect("control source execution");

    let mut replay_runtime = adapter.create_runtime().expect("replay harness runtime");
    adapter
        .prepare_runtime(&mut replay_runtime, &profile)
        .expect("replay harness prepare");
    adapter
        .load_fixture(&mut replay_runtime, &fixture)
        .expect("replay harness load fixture");
    let replay_run = adapter
        .execute(
            &mut replay_runtime,
            &fixture,
            &ExecutionRequest::target("source-replay", replay_target()),
            &profile,
        )
        .expect("replay source execution");
    let mut hostile_runtime = adapter.create_runtime().expect("hostile harness runtime");
    adapter
        .prepare_runtime(&mut hostile_runtime, &profile)
        .expect("hostile harness prepare");
    adapter
        .load_fixture(&mut hostile_runtime, &fixture)
        .expect("hostile harness load fixture");
    let hostile_run = adapter
        .execute(
            &mut hostile_runtime,
            &fixture,
            &ExecutionRequest::target("source-hostile", hostile_target()),
            &profile,
        )
        .expect("hostile source execution should report typed rejection");

    assert_eq!(
        control_run.summary["truth_view_digest"],
        replay_run.summary["truth_view_digest"]
    );
    assert_eq!(
        control_run.summary["source_contract_digest"],
        replay_run.summary["source_contract_digest"]
    );
    assert_eq!(
        control_run.summary["diagnostics_digest"],
        replay_run.summary["diagnostics_digest"]
    );
    assert_eq!(
        control_run.summary["failure_digest"],
        serde_json::Value::Null
    );
    assert_eq!(
        replay_run.summary["failure_digest"],
        serde_json::Value::Null
    );
    assert_eq!(
        control_run.summary["counter_snapshot"]["source_materialization_count"],
        serde_json::json!(1)
    );
    assert_eq!(
        control_run.summary["counter_snapshot"]["source_adapter_fallback_count"],
        serde_json::json!(0)
    );
    assert_eq!(
        replay_run.summary["counter_snapshot"]["source_replay_request_count"],
        serde_json::json!(1)
    );

    let control_bundle = &control_run.extensions["bridge_source_certification_bundle"];
    let replay_bundle = &replay_run.extensions["bridge_source_certification_bundle"];
    assert_eq!(
        control_bundle["truth_view_digest"],
        replay_bundle["truth_view_digest"]
    );
    assert_eq!(
        control_bundle["source_contract_digest"],
        replay_bundle["source_contract_digest"]
    );
    assert_eq!(control_bundle["routing_digest"], serde_json::Value::Null);
    assert_eq!(replay_bundle["routing_digest"], serde_json::Value::Null);
    assert_eq!(
        control_run.extensions["bridge_source_materialization_record"]
            ["source_declaration_identity"],
        serde_json::json!("source:analysis-history")
    );
    assert_eq!(
        replay_run.extensions["bridge_source_replay_record"]
            ["source_materialization_record_identity"],
        replay_run.extensions["bridge_source_materialization_record"]
            ["source_materialization_record_identity"]
    );
    assert_eq!(hostile_run.summary["failure_digest"].is_null(), false);
    assert_eq!(
        hostile_run.summary["truth_view_digest"],
        serde_json::Value::Null
    );
    assert_eq!(
        hostile_run.summary["counter_snapshot"]["source_contract_mismatch_count"],
        serde_json::json!(1)
    );
    assert_eq!(
        hostile_run.summary["counter_snapshot"]["source_materialization_count"],
        serde_json::json!(0)
    );
    assert_eq!(
        hostile_run.summary["counter_snapshot"]["source_adapter_fallback_count"],
        serde_json::json!(0)
    );
    assert_eq!(
        hostile_run.summary["counter_snapshot"]["retained_source_record_count"],
        serde_json::json!(0)
    );
    assert_eq!(
        hostile_run.summary["counter_snapshot"]["retained_failure_record_count"],
        serde_json::json!(0)
    );

    let rejection = &hostile_run.extensions["bridge_source_rejection"];
    assert_eq!(
        rejection["failure_kind"],
        serde_json::json!("SourceContractMismatch")
    );
    assert_eq!(
        rejection["source_declaration_identity"],
        serde_json::json!("source:hostile-missing")
    );
    let bundle = &hostile_run.extensions["bridge_source_certification_bundle"];
    assert_eq!(bundle["routing_digest"], serde_json::Value::Null);
    assert_eq!(
        bundle["counter_snapshot"]["source_contract_mismatch_count"],
        serde_json::json!(1)
    );
    assert_ne!(
        hostile_run.summary["failure_digest"],
        control_run.summary["failure_digest"]
    );
    assert_eq!(
        hostile_run.extensions["bridge_source_certification_bundle"]["failure_digest"],
        hostile_run.summary["failure_digest"]
    );
    assert_eq!(
        hostile_run.extensions["bridge_source_rejection"]["source_failure_identity"],
        hostile_run.extensions["bridge_source_rejection"]["explanation"]["failure_identity"]
    );
}

#[test]
fn multi_host_adapters_preserve_canonical_truth_view_results() {
    let fixture = source_fixture("bridge-source-parity");
    let request = ExecutionRequest::target("source-materialize", materialize_target());

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        direct_host_profile("baseline-direct-host"),
    )
    .candidates([wrapped_host_profile("candidate")])
    .compare()
    .expect("source multi-host parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);

    let direct_run = execute_source_run(
        direct_host_profile("baseline-direct-host"),
        "source-materialize-direct",
        materialize_target(),
    );
    let wrapped_run = execute_source_run(
        wrapped_host_profile("candidate"),
        "source-materialize-wrapped",
        materialize_target(),
    );
    let adapter_parity_matrix = json!({
        "baseline_profile": "baseline-direct-host",
        "candidate_profile": "candidate-wrapped-host",
        "matched": report.results[0].comparison.matched,
        "truth_view_digest": direct_run.summary["truth_view_digest"],
        "source_contract_digest": direct_run.summary["source_contract_digest"],
        "routing_digest": direct_run.extensions["bridge_source_certification_bundle"]["routing_digest"],
        "diagnostics_digest": direct_run.summary["diagnostics_digest"],
        "failure_digest": direct_run.summary["failure_digest"],
    });

    assert_eq!(direct_run.summary, wrapped_run.summary);
    assert_eq!(direct_run.extensions, wrapped_run.extensions);
    assert_eq!(adapter_parity_matrix["matched"], json!(true));
    assert_eq!(
        adapter_parity_matrix["truth_view_digest"],
        direct_run.summary["truth_view_digest"]
    );
}

#[test]
fn multi_host_adapters_preserve_canonical_truth_view_results_for_batch_source_materialization() {
    let direct_run = execute_source_run(
        direct_host_profile("baseline-direct-host"),
        "source-materialize-batch-direct",
        materialize_batch_target(),
    );
    let wrapped_run = execute_source_run(
        wrapped_host_profile("candidate"),
        "source-materialize-batch-wrapped",
        materialize_batch_target(),
    );

    assert_eq!(direct_run.summary, wrapped_run.summary);
    assert_eq!(direct_run.extensions, wrapped_run.extensions);
    assert_eq!(
        direct_run.summary["counter_snapshot"]["source_packet_count"],
        json!(2)
    );
    assert_eq!(
        direct_run.summary["truth_view_digest"],
        direct_run.summary["materialized_packet_set_digest"]
    );
}

#[test]
fn source_capability_rejection_matrix_is_harness_certified() {
    let fixture = source_fixture("bridge-source-capability-rejection");
    let request = ExecutionRequest::target("source-hostile", hostile_target());

    let report = certification_matrix(
        BridgeHarnessAdapter,
        fixture,
        request,
        direct_host_profile("baseline-direct-host"),
    )
    .candidates([wrapped_host_profile("candidate")])
    .certify()
    .expect("source capability rejection matrix should certify");

    assert!(report.matched);
    assert_eq!(report.cases.len(), 1);

    let direct_run = execute_source_run(
        direct_host_profile("baseline-direct-host"),
        "source-hostile-direct",
        hostile_target(),
    );
    let wrapped_run = execute_source_run(
        wrapped_host_profile("candidate"),
        "source-hostile-wrapped",
        hostile_target(),
    );
    let capability_matrix = json!({
        "baseline_profile": "baseline-direct-host",
        "candidate_profile": "candidate-wrapped-host",
        "matched": report.cases[0].comparison.matched,
        "failure_digest": direct_run.summary["failure_digest"],
        "diagnostics_digest": direct_run.summary["diagnostics_digest"],
        "source_contract_mismatch_count": direct_run.summary["counter_snapshot"]["source_contract_mismatch_count"],
        "source_materialization_count": direct_run.summary["counter_snapshot"]["source_materialization_count"],
        "source_adapter_fallback_count": direct_run.summary["counter_snapshot"]["source_adapter_fallback_count"],
    });

    assert_eq!(direct_run.summary, wrapped_run.summary);
    assert_eq!(direct_run.extensions, wrapped_run.extensions);
    assert_eq!(capability_matrix["matched"], json!(true));
    assert_eq!(
        capability_matrix["source_contract_mismatch_count"],
        json!(1)
    );
    assert_eq!(capability_matrix["source_materialization_count"], json!(0));
    assert_eq!(capability_matrix["source_adapter_fallback_count"], json!(0));
}

#[test]
fn source_builder_swap_parity_is_harness_certified() {
    let fixture = source_fixture("bridge-source-builder-parity");
    let request = ExecutionRequest::target("source-materialize", materialize_target());

    let report = certification_matrix(
        BridgeHarnessAdapter,
        fixture,
        request,
        direct_host_profile("baseline-direct-host"),
    )
    .candidates([sources_first_profile("candidate")])
    .certify()
    .expect("source builder swap matrix should certify");

    assert!(report.matched);
    assert_eq!(report.cases.len(), 1);

    let baseline_run = execute_source_run(
        direct_host_profile("baseline-direct-host"),
        "source-materialize-baseline",
        materialize_target(),
    );
    let swapped_run = execute_source_run(
        sources_first_profile("candidate"),
        "source-materialize-sources-first",
        materialize_target(),
    );
    let setup_parity_matrix = json!({
        "baseline_profile": "baseline-direct-host",
        "candidate_profile": "candidate-sources-first",
        "matched": report.cases[0].comparison.matched,
        "truth_view_digest": baseline_run.summary["truth_view_digest"],
        "source_contract_digest": baseline_run.summary["source_contract_digest"],
        "diagnostics_digest": baseline_run.summary["diagnostics_digest"],
        "failure_digest": baseline_run.summary["failure_digest"],
    });

    assert_eq!(baseline_run.summary, swapped_run.summary);
    assert_eq!(baseline_run.extensions, swapped_run.extensions);
    assert_eq!(setup_parity_matrix["matched"], json!(true));
    assert_eq!(
        setup_parity_matrix["failure_digest"],
        serde_json::Value::Null
    );
}

#[test]
fn source_adapter_open_rejection_is_typed_and_leaves_zero_false_success_residue() {
    let run = execute_source_run(
        rejecting_adapter_profile("hostile"),
        "source-materialize-reject-open",
        reject_open_snapshot_target(),
    );

    let rejection = &run.extensions["bridge_source_rejection"];
    let bundle = &run.extensions["bridge_source_certification_bundle"];
    assert_eq!(
        rejection["failure_class"],
        serde_json::json!("SourceMaterializationRejected")
    );
    assert_eq!(
        rejection["failure_kind"],
        serde_json::json!("SnapshotAcquisitionFailure")
    );
    assert_eq!(
        bundle["counter_snapshot"]["source_materialization_count"],
        serde_json::json!(0)
    );
    assert_eq!(
        bundle["counter_snapshot"]["retained_source_failure_record_count"],
        serde_json::json!(1)
    );
}

#[test]
fn source_adapter_identity_drift_is_typed_and_leaves_zero_false_success_residue() {
    let run = execute_source_run(
        drifting_adapter_profile("hostile"),
        "source-materialize-drift-identity",
        reject_snapshot_drift_target(),
    );

    let rejection = &run.extensions["bridge_source_rejection"];
    let bundle = &run.extensions["bridge_source_certification_bundle"];
    assert_eq!(
        rejection["failure_class"],
        serde_json::json!("AdapterCapabilityDrift")
    );
    assert_eq!(
        rejection["failure_kind"],
        serde_json::json!("SnapshotIdentityMismatch")
    );
    assert_eq!(
        bundle["counter_snapshot"]["source_materialization_count"],
        serde_json::json!(0)
    );
    assert_eq!(
        bundle["counter_snapshot"]["retained_source_failure_record_count"],
        serde_json::json!(1)
    );
}
