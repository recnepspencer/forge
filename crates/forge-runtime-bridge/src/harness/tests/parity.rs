use forge_harness::facade::{parity_suite, ExecutionProfile, ExecutionRequest, ScenarioPlan};

use crate::harness::adapter::BridgeHarnessTargetId;

use super::support::{
    build_runtime, committed_patch, field_aspect_registration, field_slice_snapshot, registration,
    snapshot,
};
use crate::facade::{
    BridgeBulkWorkloadRequest, BridgeBulkWorkloadSegment, BridgeContinuityAuthorityBasis,
    BridgeHistoricalLineageAuthority, BridgeHistoricalResolvedLineageIdentity,
    BridgeHistoricalResolvedRecordIdentity, BridgeLineageContext, BridgeRouteRequest,
    TruthBranchIdentity, TruthSnapshotIdentity,
};
use crate::harness::adapter::BridgeHarnessAdapter;
use crate::harness::fixtures::BridgeHarnessFixture;
use crate::truth_identity_fixtures::{truth_branch, truth_commit, truth_patch, truth_snapshot};

fn continuity_authority(
    branch_identity: TruthBranchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
) -> BridgeHistoricalLineageAuthority {
    BridgeHistoricalLineageAuthority::try_new(
        BridgeContinuityAuthorityBasis::new(branch_identity, snapshot_identity),
        vec![BridgeHistoricalResolvedLineageIdentity::new(
            "lineage:test-successor",
        )],
        vec![BridgeHistoricalResolvedRecordIdentity::new("entity:0:4:2")],
        vec![7],
    )
    .expect("continuity authority should be canonical")
}

fn name_field_key() -> forge_foundational::facade::FieldKey {
    forge_foundational::facade::FieldKey::new("name".to_owned()).expect("valid harness field key")
}

#[test]
fn bridge_harness_parity_proves_routing_truth_is_invariant_across_diagnostics_tiers() {
    let fixture = ScenarioPlan::new(
        "bridge-parity",
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_committed_patch(committed_patch(
                commit_a(),
                patch_a(),
                snapshot_a(),
                name_field_key(),
            ))
            .with_snapshot(snapshot(snapshot_a(), "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target(
        "deliver-commit-a",
        BridgeHarnessTargetId::committed_route(commit_a()),
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
            .with_committed_patch(committed_patch(
                commit_a(),
                patch_a(),
                snapshot_a(),
                name_field_key(),
            ))
            .with_snapshot(field_slice_snapshot(snapshot_a(), "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target(
        "deliver-commit-a",
        BridgeHarnessTargetId::committed_route(commit_a()),
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
                BridgeContinuityAuthorityBasis::new(main_branch(), snapshot_a()),
            ))
            .with_continuity_authority("user", continuity_authority(main_branch(), snapshot_a()))
            .with_committed_patch(committed_patch(
                commit_a(),
                patch_a(),
                snapshot_a(),
                name_field_key(),
            ))
            .with_snapshot(field_slice_snapshot(snapshot_a(), "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target(
        "deliver-commit-a",
        BridgeHarnessTargetId::committed_route(commit_a()),
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
            .with_committed_patch(committed_patch(
                commit_a(),
                patch_a(),
                snapshot_a(),
                name_field_key(),
            ))
            .with_snapshot(snapshot(snapshot_a(), "alice")),
    )
    .declare_input("history-commit:main:commit-a")
    .declare_observation("historical")
    .compile();
    let request = ExecutionRequest::target(
        "historical-commit-a",
        BridgeHarnessTargetId::historical_commit(main_branch(), commit_a()),
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
        BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(commit_a())),
        BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(commit_b())),
    ]);

    let development_source = crate::harness::fixtures::InMemoryRelationalBridgeSource::default();
    development_source.insert_committed_patch(committed_patch(
        commit_a(),
        patch_a(),
        snapshot_a(),
        name_field_key(),
    ));
    development_source.insert_committed_patch(committed_patch(
        commit_b(),
        patch_b(),
        snapshot_b(),
        name_field_key(),
    ));
    development_source.insert_snapshot(snapshot(snapshot_a(), "alice"));
    development_source.insert_snapshot(snapshot(snapshot_b(), "bob"));
    let development = crate::facade::RuntimeBridge::builder()
        .with_relational_source(development_source)
        .with_signal_sink(crate::harness::fixtures::RecordingSignalBridgeSink::default())
        .with_policy(crate::facade::BridgeRuntimePolicy::development())
        .register_mapping(registration())
        .build()
        .expect("development runtime");

    let operational_source = crate::harness::fixtures::InMemoryRelationalBridgeSource::default();
    operational_source.insert_committed_patch(committed_patch(
        commit_a(),
        patch_a(),
        snapshot_a(),
        name_field_key(),
    ));
    operational_source.insert_committed_patch(committed_patch(
        commit_b(),
        patch_b(),
        snapshot_b(),
        name_field_key(),
    ));
    operational_source.insert_snapshot(snapshot(snapshot_a(), "alice"));
    operational_source.insert_snapshot(snapshot(snapshot_b(), "bob"));
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

    assert_eq!(
        development_plan.workload_identity(),
        operational_plan.workload_identity()
    );
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
    assert_eq!(
        development_plan.packet_set().digest(),
        operational_plan.packet_set().digest()
    );
    assert_eq!(
        development_plan
            .execution_plan()
            .reduced_artifact()
            .digest(),
        operational_plan
            .execution_plan()
            .reduced_artifact()
            .digest()
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
    admitted_source.insert_committed_patch(committed_patch(
        commit_a(),
        patch_a(),
        snapshot_a(),
        name_field_key(),
    ));
    admitted_source.insert_committed_patch(committed_patch(
        commit_b(),
        patch_b(),
        snapshot_b(),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    admitted_source.insert_snapshot(snapshot(snapshot_a(), "alice"));
    admitted_source.insert_snapshot(snapshot(snapshot_b(), "bob"));
    let admitted_sink = crate::harness::fixtures::RecordingSignalBridgeSink::default();
    let admitted_runtime =
        build_runtime(admitted_source, admitted_sink.clone(), vec![registration()]);

    let admitted_result = admitted_runtime
        .deliver_bulk_workload_plan(
            admitted_runtime
                .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
                    BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(commit_a())),
                    BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(commit_b())),
                ]))
                .expect("parallel-admitted bulk workload should plan"),
        )
        .expect("parallel-admitted bulk workload should deliver");

    let serial_source = crate::harness::fixtures::InMemoryRelationalBridgeSource::default();
    serial_source.insert_committed_patch(committed_patch(
        commit_a(),
        patch_a(),
        snapshot_a(),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    serial_source.insert_committed_patch(committed_patch(
        commit_b(),
        patch_b(),
        snapshot_a(),
        name_field_key(),
    ));
    serial_source.insert_snapshot(snapshot(snapshot_a(), "alice"));
    let serial_sink = crate::harness::fixtures::RecordingSignalBridgeSink::default();
    let serial_runtime = build_runtime(serial_source, serial_sink.clone(), vec![registration()]);

    let serial_result = serial_runtime
        .deliver_bulk_workload_plan(
            serial_runtime
                .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
                    BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(commit_a())),
                    BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(commit_b())),
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

fn main_branch() -> TruthBranchIdentity {
    truth_branch("main")
}

fn commit_a() -> crate::facade::TruthCommitIdentity {
    truth_commit(1)
}

fn commit_b() -> crate::facade::TruthCommitIdentity {
    truth_commit(2)
}

fn patch_a() -> crate::facade::TruthPatchIdentity {
    truth_patch(1)
}

fn patch_b() -> crate::facade::TruthPatchIdentity {
    truth_patch(2)
}

fn snapshot_a() -> TruthSnapshotIdentity {
    truth_snapshot(1, 1)
}

fn snapshot_b() -> TruthSnapshotIdentity {
    truth_snapshot(2, 1)
}
