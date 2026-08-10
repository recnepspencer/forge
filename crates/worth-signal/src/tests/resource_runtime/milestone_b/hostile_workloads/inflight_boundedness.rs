use super::super::super::support::{
    resource_async_inflight_pressure_workload, resource_declaration,
};
use super::super::super::{
    resource_certification_builder, CompletionDenialClass, DeniedResourceCompletion,
    ResourceBoundaryKind, ResourceDensityStrategy, ResourceNodeId, ResourceRequestIntent,
    SignalGraph, TestRuntime,
};

#[test]
fn resource_async_inflight_pressure_workload_keeps_matching_local_and_bounded() {
    let outcome = resource_async_inflight_pressure_workload();

    assert_eq!(
        outcome.pressure_performance.boundary(),
        ResourceBoundaryKind::CompletionBatchAdmission
    );
    assert_eq!(outcome.pressure_performance.input_width(), 4);
    assert_eq!(outcome.pressure_performance.admitted_count(), 1);
    assert_eq!(outcome.pressure_performance.denied_count(), 3);
    assert_eq!(outcome.pressure_performance.lifecycle_transition_count(), 1);
    assert_eq!(
        outcome.pressure_performance.operational_allocation_count(),
        3
    );
    assert_eq!(
        outcome
            .pressure_performance
            .retained_history_allocation_count(),
        0
    );
    assert_eq!(
        outcome.pressure_performance.diagnostics_allocation_count(),
        4
    );
    assert_eq!(
        outcome
            .pressure_performance
            .facade_report_allocation_count(),
        1
    );
    assert_eq!(
        outcome.pressure_performance.density_strategy(),
        ResourceDensityStrategy::BurstySortedDeduplicated
    );
    assert_eq!(outcome.pressure_batch.denied_completions().len(), 3);
    assert!(outcome.pressure_batch.denied_completions().iter().any(
        |denied: &DeniedResourceCompletion| denied.class() == CompletionDenialClass::Duplicate
    ));
    assert!(outcome
        .pressure_batch
        .denied_completions()
        .iter()
        .any(|denied: &DeniedResourceCompletion| denied.class()
            == CompletionDenialClass::Contradictory));
    assert!(outcome
        .pressure_batch
        .denied_completions()
        .iter()
        .any(|denied: &DeniedResourceCompletion| denied.class()
            == CompletionDenialClass::UnknownRequest));
    assert_eq!(outcome.telemetry.resource_retry_admission_count, 1);
    assert_eq!(outcome.telemetry.resource_retry_schedule_count, 1);
    assert_eq!(
        outcome
            .telemetry
            .resource_retry_already_scheduled_denial_count,
        1
    );
    assert_eq!(
        outcome
            .telemetry
            .resource_superseded_completion_denial_count,
        1
    );
    assert_eq!(
        outcome.telemetry.resource_duplicate_completion_denial_count,
        1
    );
    assert_eq!(
        outcome
            .telemetry
            .resource_contradictory_completion_denial_count,
        1
    );
    assert_eq!(
        outcome
            .telemetry
            .resource_unknown_request_completion_denial_count,
        2
    );
    assert_eq!(outcome.telemetry.resource_stale_completion_denial_count, 1);
    assert_eq!(outcome.telemetry.resource_branch_restore_count, 1);
    assert!(
        outcome.branch_restore_report.broad_rebuild_denial_count() > 0,
        "branch restore under async pressure must report bounded broad-rebuild denial evidence"
    );
    assert!(
        outcome.branch_restore_report.restored_in_flight_width() > 0,
        "branch restore should carry live inflight width under pressure"
    );
    assert_eq!(
        outcome.runtime_summary.in_flight_request_count(),
        outcome.replay_after_restore.in_flight_width() as u64,
        "runtime summary and replay reconstruction must agree on retained inflight width after pressure churn"
    );
    assert!(
        !outcome.drifted_branch_handle_live_after_restore,
        "restore must not leave post-snapshot drift as ghost inflight state"
    );
    assert_eq!(
        outcome
            .zombie_completion_after_restore
            .denied_completion()
            .expect("restored-away zombie completion should deny explicitly")
            .class(),
        CompletionDenialClass::UnknownRequest
    );
    assert_eq!(
        outcome
            .pre_restore_completion_after_restore
            .denied_completion()
            .expect("pre-restore completion should deny under the restored branch epoch")
            .class(),
        CompletionDenialClass::Stale
    );
    assert!(
        outcome
            .pre_restore_completion_after_restore
            .admitted_completion()
            .is_none(),
        "restore must preserve inflight truth without letting pre-restore completion authority survive branch-epoch rotation"
    );
    assert!(
        outcome.telemetry.resource_hot_in_flight_lookup_count >= 4,
        "completion matching and churn should remain attributable through hot inflight lookups"
    );
}

#[test]
fn resource_async_liveness_failures_preserve_inflight_truth_and_reject_zombie_completion() {
    let outcome = resource_async_inflight_pressure_workload();

    assert!(
        !outcome.drifted_branch_handle_live_after_restore,
        "restored-away drift must not survive as ghost inflight state"
    );
    assert_eq!(
        outcome
            .zombie_completion_after_restore
            .denied_completion()
            .expect("zombie completion after restore should deny explicitly")
            .class(),
        CompletionDenialClass::UnknownRequest
    );
    assert_eq!(
        outcome
            .pre_restore_completion_after_restore
            .denied_completion()
            .expect("pre-restore completion should be stale after restore rekeys the branch epoch")
            .class(),
        CompletionDenialClass::Stale
    );
    assert!(
        outcome
            .pre_restore_completion_after_restore
            .admitted_completion()
            .is_none(),
        "restore must not let pre-restore completion authority survive even while it preserves live inflight truth"
    );
    assert_eq!(
        outcome.runtime_summary.in_flight_request_count(),
        outcome.replay_after_restore.in_flight_width() as u64,
        "runtime summary and replay reconstruction must stay aligned after zombie denial"
    );
}

#[test]
fn resource_inflight_certification_rejects_non_hostile_pressure_evidence() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit");

    let err = resource_certification_builder()
        .with_async_inflight_boundedness(
            runtime.resource_runtime_summary(),
            &runtime.reconstruct_resource_replay_summary(),
            runtime.telemetry().resource,
            admitted.performance(),
        )
        .expect_err("trivial one-request evidence must not certify hostile inflight boundedness");

    assert!(err
        .to_string()
        .contains("requires hostile async pressure evidence"));
}
