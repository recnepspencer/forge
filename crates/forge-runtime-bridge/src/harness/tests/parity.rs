use forge_harness::facade::{
    parity_suite, ExecutionProfile, ExecutionRequest, ScenarioPlan,
};
use std::sync::Arc;

use crate::harness::adapter::BridgeHarnessAdapter;
use crate::harness::fixtures::BridgeHarnessFixture;
use crate::facade::{
    BridgeBulkWorkloadRequest, BridgeBulkWorkloadSegment, BridgeContinuityAuthorityBasis,
    BridgeHistoricalLineageAuthority, BridgeLineageContext, BridgeRouteRequest,
    TruthBranchIdentity, TruthSnapshotIdentity,
};
use super::support::{
    build_runtime, committed_patch, field_aspect_registration, field_slice_snapshot, registration,
    snapshot,
};

fn continuity_authority(
    branch: &str,
    snapshot: &str,
) -> BridgeHistoricalLineageAuthority {
    BridgeHistoricalLineageAuthority::try_new(
        BridgeContinuityAuthorityBasis::new(
            TruthBranchIdentity::new(branch),
            TruthSnapshotIdentity::new(snapshot),
        ),
        vec![Arc::from("lineage:test-successor")],
        vec![Arc::from("entity:0:4:2")],
        vec![7],
    )
    .expect("continuity authority should be canonical")
}

#[test]
fn bridge_harness_parity_proves_routing_truth_is_invariant_across_diagnostics_tiers() {
    let fixture = ScenarioPlan::new(
        "bridge-parity",
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([
        ExecutionProfile::operational("operational"),
        ExecutionProfile::forensic("forensic"),
    ])
    .compare()
    .expect("bridge parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 2);
}

#[test]
fn bridge_harness_parity_proves_fine_grained_slice_truth_is_invariant_across_diagnostics_tiers() {
    let fixture = ScenarioPlan::new(
        "bridge-fine-grained-parity",
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_aspect_mapping(field_aspect_registration())
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(field_slice_snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([
        ExecutionProfile::operational("operational"),
        ExecutionProfile::forensic("forensic"),
    ])
    .compare()
    .expect("fine-grained bridge parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 2);
}

#[test]
fn bridge_harness_parity_proves_continuity_truth_is_invariant_across_diagnostics_tiers() {
    let fixture = ScenarioPlan::new(
        "bridge-continuity-parity",
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_aspect_mapping(field_aspect_registration())
            .with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            ))
            .with_continuity_authority("user", continuity_authority("main", "snapshot-a"))
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(field_slice_snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([
        ExecutionProfile::operational("operational"),
        ExecutionProfile::forensic("forensic"),
    ])
    .compare()
    .expect("continuity parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 2);
}

#[test]
fn bridge_harness_parity_proves_historical_truth_is_invariant_across_diagnostics_tiers() {
    let fixture = ScenarioPlan::new(
        "bridge-historical-parity",
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("history-commit:main:commit-a")
    .declare_observation("historical")
    .compile();
    let request = ExecutionRequest::target(
        "historical-commit-a",
        "history-commit:main:commit-a".to_string(),
    );

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([
        ExecutionProfile::operational("operational"),
        ExecutionProfile::forensic("forensic"),
    ])
    .compare()
    .expect("historical parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 2);
}

#[test]
fn bridge_bulk_planning_truth_is_invariant_across_diagnostics_tiers() {
    let request = BridgeBulkWorkloadRequest::new(vec![
        BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
        BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
    ]);

    let development_source = crate::harness::fixtures::InMemoryRelationalBridgeSource::default();
    development_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    development_source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    development_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    development_source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let development = crate::facade::RuntimeBridge::builder()
        .with_relational_source(development_source)
        .with_signal_sink(crate::harness::fixtures::RecordingSignalBridgeSink::default())
        .with_policy(crate::facade::BridgeRuntimePolicy::development())
        .register_mapping(registration())
        .build()
        .expect("development runtime");

    let operational_source = crate::harness::fixtures::InMemoryRelationalBridgeSource::default();
    operational_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    operational_source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    operational_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    operational_source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let operational = crate::facade::RuntimeBridge::builder()
        .with_relational_source(operational_source)
        .with_signal_sink(crate::harness::fixtures::RecordingSignalBridgeSink::default())
        .with_policy(crate::facade::BridgeRuntimePolicy::operational())
        .register_mapping(registration())
        .build()
        .expect("operational runtime");

    let development_plan = development
        .plan_bulk_workload(request.clone())
        .expect("development bulk workload should plan");
    let operational_plan = operational
        .plan_bulk_workload(request)
        .expect("operational bulk workload should plan");

    assert_eq!(development_plan.workload_identity(), operational_plan.workload_identity());
    assert_eq!(
        development_plan.canonical_request().digest(),
        operational_plan.canonical_request().digest()
    );
    assert_eq!(
        development_plan.normalized_summary().digest(),
        operational_plan.normalized_summary().digest()
    );
    assert_eq!(
        development_plan.canonical_planning_identity(),
        operational_plan.canonical_planning_identity()
    );
    assert_eq!(development_plan.packet_set().digest(), operational_plan.packet_set().digest());
    assert_eq!(
        development_plan.execution_plan().reduced_artifact().digest(),
        operational_plan.execution_plan().reduced_artifact().digest()
    );
    assert_eq!(
        development_plan.execution_plan().legality_decision(),
        operational_plan.execution_plan().legality_decision()
    );
    assert_eq!(
        development_plan.execution_plan().profitability_decision(),
        operational_plan.execution_plan().profitability_decision()
    );
}

#[test]
fn parallel_preparation_admission_remains_parity_safe_with_serial_required_path() {
    let admitted_source = crate::harness::fixtures::InMemoryRelationalBridgeSource::default();
    admitted_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    admitted_source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    admitted_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    admitted_source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let admitted_sink = crate::harness::fixtures::RecordingSignalBridgeSink::default();
    let admitted_runtime = build_runtime(admitted_source, admitted_sink.clone(), vec![registration()]);

    let admitted_result = admitted_runtime
        .deliver_bulk_workload_plan(
            admitted_runtime
                .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
                    BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
                    BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
                ]))
                .expect("parallel-admitted bulk workload should plan"),
        )
        .expect("parallel-admitted bulk workload should deliver");

    let serial_source = crate::harness::fixtures::InMemoryRelationalBridgeSource::default();
    serial_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    serial_source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-a", "name"));
    serial_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let serial_sink = crate::harness::fixtures::RecordingSignalBridgeSink::default();
    let serial_runtime = build_runtime(serial_source, serial_sink.clone(), vec![registration()]);

    let serial_result = serial_runtime
        .deliver_bulk_workload_plan(
            serial_runtime
                .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
                    BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
                    BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
                ]))
                .expect("serial-required bulk workload should plan"),
        )
        .expect("serial-required bulk workload should deliver");

    assert_eq!(admitted_result.summary().delivered_route_count(), 2);
    assert_eq!(serial_result.summary().delivered_route_count(), 2);
    assert_eq!(admitted_result.summary().delivered_target_count(), 2);
    assert_eq!(serial_result.summary().delivered_target_count(), 2);
    assert_eq!(admitted_sink.deliveries().len(), 2);
    assert_eq!(serial_sink.deliveries().len(), 2);
}
