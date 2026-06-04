use forge_harness::facade::{
    certification_matrix, parity_suite, ExecutionProfile, ExecutionRequest,
};
use forge_harness::runtime::HarnessAdapter;

use crate::harness::adapter::{BridgeHarnessAdapter, BridgeHarnessTargetId};

use super::support::{
    direct_host_profile, drifting_adapter_profile, execute_source_run, hostile_target,
    materialize_batch_target, materialize_target, reject_open_snapshot_target,
    reject_snapshot_drift_target, rejecting_adapter_profile, replay_target, source_fixture,
    sources_first_profile, wrapped_host_profile,
};

#[test]
fn source_control_hostile_and_replay_lanes_export_terminal_records() {
    let adapter = BridgeHarnessAdapter;
    let profile = ExecutionProfile::development("baseline");
    let fixture = source_fixture("bridge-source-certification");

    let control_run = execute_with_adapter(
        adapter,
        fixture.clone(),
        &profile,
        "source-control",
        materialize_target(),
    );
    let replay_run = execute_with_adapter(
        adapter,
        fixture.clone(),
        &profile,
        "source-replay",
        replay_target(),
    );
    let hostile_run = execute_with_adapter(
        adapter,
        fixture,
        &profile,
        "source-hostile",
        hostile_target(),
    );

    assert!(control_run
        .extensions
        .contains_key("bridge_source_materialization_record"));
    assert!(replay_run
        .extensions
        .contains_key("bridge_source_replay_record"));
    assert!(hostile_run
        .extensions
        .contains_key("bridge_source_rejection"));
    assert!(control_run
        .extensions
        .contains_key("bridge_source_certification_bundle"));
    assert!(hostile_run
        .extensions
        .contains_key("bridge_source_certification_bundle"));
}

#[test]
fn multi_host_source_materialization_exports_match() {
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

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);
    assert!(report.results[0].comparison.matched);
    assert_eq!(direct_run.summary, wrapped_run.summary);
    assert_eq!(direct_run.extensions, wrapped_run.extensions);
}

#[test]
fn multi_host_batch_source_materialization_exports_match() {
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
}

#[test]
fn source_capability_rejection_matrix_exports_match() {
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

    assert!(report.matched);
    assert_eq!(report.cases.len(), 1);
    assert!(report.cases[0].comparison.matched);
    assert_eq!(direct_run.summary, wrapped_run.summary);
    assert_eq!(direct_run.extensions, wrapped_run.extensions);
}

#[test]
fn source_builder_swap_exports_match() {
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

    assert!(report.matched);
    assert_eq!(report.cases.len(), 1);
    assert!(report.cases[0].comparison.matched);
    assert_eq!(baseline_run.summary, swapped_run.summary);
    assert_eq!(baseline_run.extensions, swapped_run.extensions);
}

#[test]
fn source_adapter_open_rejection_exports_terminal_rejection_record() {
    let run = execute_source_run(
        rejecting_adapter_profile("hostile"),
        "source-materialize-reject-open",
        reject_open_snapshot_target(),
    );

    assert!(run.extensions.contains_key("bridge_source_rejection"));
    assert!(run
        .extensions
        .contains_key("bridge_source_certification_bundle"));
}

#[test]
fn source_adapter_identity_drift_exports_terminal_rejection_record() {
    let run = execute_source_run(
        drifting_adapter_profile("hostile"),
        "source-materialize-drift-identity",
        reject_snapshot_drift_target(),
    );

    assert!(run.extensions.contains_key("bridge_source_rejection"));
    assert!(run
        .extensions
        .contains_key("bridge_source_certification_bundle"));
}

fn execute_with_adapter(
    adapter: BridgeHarnessAdapter,
    fixture: forge_harness::facade::ScenarioFixture<crate::harness::fixtures::BridgeHarnessFixture>,
    profile: &ExecutionProfile,
    request_name: &str,
    target: BridgeHarnessTargetId,
) -> forge_harness::facade::RunRecord<BridgeHarnessTargetId> {
    let mut runtime = adapter.create_runtime().expect("source harness runtime");
    adapter
        .prepare_runtime(&mut runtime, profile)
        .expect("source harness prepare");
    adapter
        .load_fixture(&mut runtime, &fixture)
        .expect("source harness load fixture");
    adapter
        .execute(
            &mut runtime,
            &fixture,
            &ExecutionRequest::target(request_name, target),
            profile,
        )
        .expect("source harness execution")
}
