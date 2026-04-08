use forge_harness::facade::{certification_matrix, ExecutionProfile, ExecutionRequest, ScenarioPlan};
use forge_harness::runtime::HarnessAdapter;
use std::sync::Arc;

use crate::harness::adapter::BridgeHarnessAdapter;
use crate::harness::fixtures::BridgeHarnessFixture;
use crate::facade::{
    BridgeBulkWorkloadRequest, BridgeBulkWorkloadSegment, BridgeContinuityAuthorityBasis,
    BridgeHistoricalLineageAuthority, BridgeLineageContext, BridgeParallelAdmissionClass,
    BridgeParallelAdmissionReason, BridgeRouteRequest, TruthBranchIdentity, TruthSnapshotIdentity,
};
use super::support::{
    build_runtime, committed_patch, field_aspect_registration, field_slice_snapshot, registration,
    snapshot,
};

fn continuity_authority(
    branch: &str,
    snapshot: &str,
) -> BridgeHistoricalLineageAuthority {
    continuity_authority_with_successor(branch, snapshot, "entity:0:4:2")
}

fn continuity_authority_with_successor(
    branch: &str,
    snapshot: &str,
    successor: &str,
) -> BridgeHistoricalLineageAuthority {
    BridgeHistoricalLineageAuthority::try_new(
        BridgeContinuityAuthorityBasis::new(
            TruthBranchIdentity::new(branch),
            TruthSnapshotIdentity::new(snapshot),
        ),
        vec![Arc::from("lineage:test-successor")],
        vec![Arc::from(successor)],
        vec![7],
    )
    .expect("continuity authority should be canonical")
}

fn ambiguous_continuity_authority(
    branch: &str,
    snapshot: &str,
) -> BridgeHistoricalLineageAuthority {
    BridgeHistoricalLineageAuthority::try_new(
        BridgeContinuityAuthorityBasis::new(
            TruthBranchIdentity::new(branch),
            TruthSnapshotIdentity::new(snapshot),
        ),
        vec![
            Arc::from("lineage:test-a"),
            Arc::from("lineage:test-b"),
            Arc::from("lineage:test-c"),
        ],
        vec![Arc::from("entity:0:4:2"), Arc::from("entity:0:5:2")],
        vec![7, 8, 9],
    )
    .expect("ambiguous continuity authority should be canonical")
}

#[test]
fn bridge_certification_matrix_reports_diagnostics_for_candidate_profiles() {
    let fixture = ScenarioPlan::new(
        "bridge-certification",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());

    let report = certification_matrix(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([ExecutionProfile::forensic("forensic")])
    .certify()
    .expect("bridge certification matrix should succeed");

    assert!(report.matched);
    assert!(report.baseline_diagnostics_summary.is_some());
    assert_eq!(report.cases.len(), 1);
}

#[test]
fn bridge_continuity_certification_matrix_reports_candidate_profile_parity() {
    let fixture = ScenarioPlan::new(
        "bridge-continuity-certification",
        BridgeHarnessFixture::new(vec![registration()])
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

    let report = certification_matrix(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([ExecutionProfile::forensic("forensic")])
    .certify()
    .expect("bridge continuity certification matrix should succeed");

    assert!(report.matched);
    assert!(report.baseline_diagnostics_summary.is_some());
    assert_eq!(report.cases.len(), 1);
}

#[test]
fn bridge_harness_branch_divergence_changes_continuity_outcome_explicitly() {
    let adapter = BridgeHarnessAdapter;
    let profile = ExecutionProfile::development("baseline");
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());

    let main_fixture = ScenarioPlan::new(
        "bridge-continuity-main",
        BridgeHarnessFixture::new(vec![registration()])
            .with_aspect_mapping(field_aspect_registration())
            .with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            ))
            .with_continuity_authority(
                "user",
                continuity_authority_with_successor("main", "snapshot-a", "entity:0:4:2"),
            )
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(field_slice_snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();

    let feature_fixture = ScenarioPlan::new(
        "bridge-continuity-feature",
        BridgeHarnessFixture::new(vec![registration()])
            .with_aspect_mapping(field_aspect_registration())
            .with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    TruthBranchIdentity::new("feature"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            ))
            .with_continuity_authority(
                "user",
                continuity_authority_with_successor("feature", "snapshot-a", "entity:0:5:2"),
            )
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(field_slice_snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();

    let mut main_runtime = adapter.create_runtime().expect("main harness runtime");
    adapter
        .prepare_runtime(&mut main_runtime, &profile)
        .expect("main harness prepare");
    adapter
        .load_fixture(&mut main_runtime, &main_fixture)
        .expect("main harness load fixture");
    let main_run = adapter
        .execute(&mut main_runtime, &main_fixture, &request, &profile)
        .expect("main harness execute");

    let mut feature_runtime = adapter.create_runtime().expect("feature harness runtime");
    adapter
        .prepare_runtime(&mut feature_runtime, &profile)
        .expect("feature harness prepare");
    adapter
        .load_fixture(&mut feature_runtime, &feature_fixture)
        .expect("feature harness load fixture");
    let feature_run = adapter
        .execute(&mut feature_runtime, &feature_fixture, &request, &profile)
        .expect("feature harness execute");

    assert_ne!(
        main_run.summary["continuity_identity"],
        feature_run.summary["continuity_identity"]
    );
    assert_ne!(
        main_run.extensions["bridge_continuity_record"]["source_branch"],
        feature_run.extensions["bridge_continuity_record"]["source_branch"]
    );
}

#[test]
fn bridge_harness_continuity_certifies_ambiguous_rejection_explicitly() {
    let adapter = BridgeHarnessAdapter;
    let profile = ExecutionProfile::development("baseline");
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());
    let fixture = ScenarioPlan::new(
        "bridge-continuity-ambiguous",
        BridgeHarnessFixture::new(vec![registration()])
            .with_aspect_mapping(field_aspect_registration())
            .with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            ))
            .with_continuity_authority("user", ambiguous_continuity_authority("main", "snapshot-a"))
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(field_slice_snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();

    let mut runtime = adapter.create_runtime().expect("harness runtime");
    adapter
        .prepare_runtime(&mut runtime, &profile)
        .expect("harness prepare");
    adapter
        .load_fixture(&mut runtime, &fixture)
        .expect("harness load fixture");
    let run = adapter
        .execute(&mut runtime, &fixture, &request, &profile)
        .expect("harness execute");

    assert_eq!(
        run.extensions["bridge_continuity_record"]["outcome_classes"][0],
        "RejectedAmbiguousSuccessor"
    );
}

#[test]
fn bridge_historical_certification_matrix_reports_candidate_profile_parity() {
    let fixture = ScenarioPlan::new(
        "bridge-historical-certification",
        BridgeHarnessFixture::new(vec![registration()])
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

    let report = certification_matrix(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([ExecutionProfile::forensic("forensic")])
    .certify()
    .expect("bridge historical certification matrix should succeed");

    assert!(report.matched);
    assert!(report.baseline_diagnostics_summary.is_some());
    assert_eq!(report.cases.len(), 1);
}

#[test]
fn bridge_bulk_certifies_exact_counters_for_parallel_admitted_workload() {
    let source = crate::harness::fixtures::InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let runtime = build_runtime(
        source,
        crate::harness::fixtures::RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let plan = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("parallel-admitted workload should plan");

    assert_eq!(
        plan.execution_plan().parallel_admission().class(),
        BridgeParallelAdmissionClass::ParallelPreparationAdmitted
    );
    assert_eq!(
        plan.execution_plan().parallel_admission().reason(),
        BridgeParallelAdmissionReason::AdmittedOperational
    );
    assert_eq!(plan.packet_set().counters().bulk_packet_count(), 6);
    assert_eq!(plan.execution_plan().counters().bulk_routed_item_count(), 2);
    assert_eq!(
        plan.execution_plan().counters().bulk_normalized_workload_width(),
        13
    );
    assert_eq!(plan.execution_plan().counters().bulk_packet_entry_count(), 6);
    assert_eq!(plan.execution_plan().counters().bulk_reduction_input_count(), 4);
    assert_eq!(plan.execution_plan().counters().bulk_reduction_output_count(), 4);
    assert_eq!(
        plan.execution_plan().counters().bulk_packet_queue_depth_peak(),
        6
    );
    assert_eq!(
        plan.execution_plan().counters().bulk_reducer_input_buffer_peak(),
        4
    );
    assert_eq!(
        plan.execution_plan().counters().bulk_replay_mismatch_count(),
        0
    );
    assert_eq!(
        plan.execution_plan().counters().bulk_parallel_legal_count(),
        1
    );
    assert_eq!(
        plan.execution_plan().counters().bulk_parallel_profitable_count(),
        1
    );
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_parallel_preparation_admitted_count(),
        1
    );
    assert_eq!(
        plan.execution_plan().counters().bulk_parallel_fallback_to_serial_count(),
        0
    );
}

#[test]
fn bridge_bulk_certifies_exact_counters_for_serial_fallback_workload() {
    let source = crate::harness::fixtures::InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        crate::harness::fixtures::RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let plan = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("serial-fallback workload should plan");

    assert_eq!(
        plan.execution_plan().parallel_admission().class(),
        BridgeParallelAdmissionClass::SerialRequired
    );
    assert_eq!(
        plan.execution_plan().parallel_admission().reason(),
        BridgeParallelAdmissionReason::SharedPublicationReductionTarget
    );
    assert_eq!(plan.packet_set().counters().bulk_packet_count(), 5);
    assert_eq!(plan.execution_plan().counters().bulk_routed_item_count(), 2);
    assert_eq!(
        plan.execution_plan().counters().bulk_normalized_workload_width(),
        11
    );
    assert_eq!(plan.execution_plan().counters().bulk_packet_entry_count(), 6);
    assert_eq!(plan.execution_plan().counters().bulk_reduction_input_count(), 3);
    assert_eq!(plan.execution_plan().counters().bulk_reduction_output_count(), 3);
    assert_eq!(
        plan.execution_plan().counters().bulk_packet_queue_depth_peak(),
        5
    );
    assert_eq!(
        plan.execution_plan().counters().bulk_reducer_input_buffer_peak(),
        3
    );
    assert_eq!(
        plan.execution_plan().counters().bulk_replay_mismatch_count(),
        0
    );
    assert_eq!(
        plan.execution_plan().counters().bulk_parallel_legal_count(),
        1
    );
    assert_eq!(
        plan.execution_plan().counters().bulk_parallel_profitable_count(),
        0
    );
    assert_eq!(
        plan.execution_plan().counters().bulk_parallel_fallback_to_serial_count(),
        1
    );
    assert_eq!(plan.execution_plan().planning_failures().len(), 1);
}
