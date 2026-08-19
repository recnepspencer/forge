use crate::data::dependency::DependencySnapshot;
use crate::facade::{
    DiagnosticsAvailability, ReplayEventKind, SignalBranchHandle, SignalBranchId, SignalGraph,
    SignalObservationAdmissionDenial, SignalObservationCompletion, SignalObservationRequest,
    SignalRuntime, SignalRuntimePolicy,
};
use crate::tests::support::{GraphDependencyBatchExt, ASPECT_A};

#[test]
fn snapshot_boundary_interrupts_active_observation_and_denies_stale_finish() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::operational());
    let session = graph
        .begin_observation_session(SignalObservationRequest::counters())
        .unwrap();

    let snapshot = graph.capture_snapshot();

    assert_eq!(
        graph.last_observation_completion(),
        Some(SignalObservationCompletion::InterruptedByBoundary)
    );
    assert!(graph.finish_observation_session(&session).is_err());
    assert!(graph
        .invalidation_performed_counters()
        .values()
        .into_iter()
        .all(|value| value == 0));
    assert_eq!(
        snapshot.meta.schema_version,
        crate::state::SignalSnapshotMeta::SCHEMA_VERSION
    );
}

#[test]
fn restore_boundary_preserves_interruption_after_graph_replacement() {
    let mut graph = SignalGraph::new();
    let snapshot = graph.capture_snapshot();
    let session = graph
        .begin_observation_session(SignalObservationRequest::operation())
        .unwrap();
    let old_instance = graph.runtime_instance_id();

    graph.restore_snapshot(&snapshot).unwrap();

    assert_ne!(graph.runtime_instance_id(), old_instance);
    assert_eq!(
        graph.last_observation_completion(),
        Some(SignalObservationCompletion::InterruptedByBoundary)
    );
    assert!(graph.finish_observation_session(&session).is_err());
}

#[test]
fn restore_boundary_rebuilds_dependency_snapshots_when_current_state_already_matches() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();
    let target = graph.node().build();
    graph.append_dependency(target, source, ASPECT_A).unwrap();
    let mut dependency_snapshot = DependencySnapshot::empty();
    dependency_snapshot.record(source, ASPECT_A, 7, None);
    graph.set_dep_snapshot(target, dependency_snapshot).unwrap();

    let snapshot = graph.capture_snapshot();
    graph.restore_snapshot(&snapshot).unwrap();

    assert_eq!(
        graph.get_dep_snapshot(target).unwrap().entries()[0].cached_version,
        7,
        "restore must retain explicit dependency snapshots even when no current delta exists"
    );
}

#[test]
fn stored_runtime_restore_interrupts_the_active_observation_session() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let snapshot = runtime.capture_snapshot().unwrap();
    let session = runtime
        .begin_observation_session(SignalObservationRequest::counters())
        .unwrap();

    runtime.restore_snapshot(&snapshot).unwrap();

    assert_eq!(
        runtime.graph().last_observation_completion(),
        Some(SignalObservationCompletion::InterruptedByBoundary)
    );
    assert!(runtime.finish_observation_session(&session).is_err());
}

#[test]
fn failed_restore_does_not_interrupt_an_active_observation() {
    let mut graph = SignalGraph::new();
    let mut snapshot = graph.capture_snapshot();
    snapshot.meta.schema_version = snapshot.meta.schema_version.saturating_add(1);
    let session = graph
        .begin_observation_session(SignalObservationRequest::counters())
        .unwrap();

    assert!(graph.restore_snapshot(&snapshot).is_err());
    assert!(matches!(
        graph.begin_observation_session(SignalObservationRequest::work()),
        Err(SignalObservationAdmissionDenial::SessionAlreadyActive)
    ));
    assert_eq!(
        graph.cancel_observation_session(&session).unwrap(),
        SignalObservationCompletion::Cancelled
    );
}

#[test]
fn non_current_branch_snapshot_interrupts_the_current_observation_session() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let feature = runtime.create_branch("phase7-feature").unwrap();
    let session = runtime
        .begin_observation_session(SignalObservationRequest::counters())
        .unwrap();

    runtime.capture_branch_snapshot(feature).unwrap();

    assert_eq!(
        runtime.graph().last_observation_completion(),
        Some(SignalObservationCompletion::InterruptedByBoundary)
    );
    assert!(runtime.finish_observation_session(&session).is_err());
}

#[test]
fn non_current_branch_snapshot_is_recorded_in_target_replay() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let feature = runtime.create_branch("phase7-replay-snapshot").unwrap();

    let snapshot = runtime.capture_branch_snapshot(feature.clone()).unwrap();
    assert!(runtime
        .observe()
        .replay_for_branch(feature.id)
        .frames
        .iter()
        .any(|event| {
            event.kind == ReplayEventKind::SnapshotCaptured
                && event.branch_id == feature.id
                && event.snapshot_id == Some(snapshot.meta.snapshot_id)
        }));
}

#[test]
fn switching_branches_interrupts_the_active_observation_boundary() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let main = runtime.current_branch();
    let feature = runtime.create_branch("phase7-switch-boundary").unwrap();
    let session = runtime
        .begin_observation_session(SignalObservationRequest::counters())
        .unwrap();

    runtime.switch_branch(feature).unwrap();
    runtime.switch_branch(main).unwrap();

    assert_eq!(
        runtime.graph().last_observation_completion(),
        Some(SignalObservationCompletion::InterruptedByBoundary)
    );
    assert!(runtime.finish_observation_session(&session).is_err());
}

#[test]
fn denied_branch_snapshot_preserves_the_active_observation_session() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let session = runtime
        .begin_observation_session(SignalObservationRequest::counters())
        .unwrap();
    let unknown = SignalBranchHandle {
        id: SignalBranchId(u64::MAX),
        name: "missing-phase7-branch".to_string(),
        parent_branch_id: None,
        head_snapshot_id: None,
    };

    assert!(runtime.capture_branch_snapshot(unknown).is_err());
    assert!(matches!(
        runtime.begin_observation_session(SignalObservationRequest::work()),
        Err(SignalObservationAdmissionDenial::SessionAlreadyActive)
    ));
    runtime.cancel_observation_session(&session).unwrap();
}

#[test]
fn on_demand_history_distinguishes_unactivated_from_policy_omission() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    graph.set_runtime_policy(SignalRuntimePolicy::operational());

    let (_, inactive_explanation) = graph.materialize_explanation_artifact(node).unwrap();
    let (_, inactive_provenance) = graph.materialize_provenance_artifact(node).unwrap();
    assert_eq!(
        inactive_explanation,
        DiagnosticsAvailability::ObservationNotActivated
    );
    assert_eq!(
        inactive_provenance,
        DiagnosticsAvailability::ObservationNotActivated
    );

    let session = graph
        .begin_observation_session(SignalObservationRequest::facts())
        .unwrap();
    graph.cancel_observation_session(&session).unwrap();
    let snapshot = graph.capture_snapshot();
    graph.restore_snapshot(&snapshot).unwrap();

    let (_, activated_explanation) = graph.materialize_explanation_artifact(node).unwrap();
    let (_, activated_provenance) = graph.materialize_provenance_artifact(node).unwrap();
    assert_eq!(
        activated_explanation,
        DiagnosticsAvailability::ReconstructedAvailable
    );
    assert_eq!(
        activated_provenance,
        DiagnosticsAvailability::ReconstructedAvailable
    );
}
