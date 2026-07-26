use super::fixture::{
    direct_admission_fixture, workflow_admission_fixture, DirectAdmissionFixture,
    FixtureDisposition, WorkflowAdmissionFixture, WORKFLOW_STAGE,
};
use crate::domain_computation::{
    WorthQueryDirectConvergenceYieldCleanupOutcome, WorthQueryDirectConvergenceYieldOutcome,
    WorthQueryDirectGraphStepOutcome, WorthQueryDirectYieldOutcome,
    WorthQueryGraphProviderCallKind, WorthQueryManagedGraphCallRequest,
    WorthQueryPendingDirectConvergenceIteration, WorthQueryPendingWorkflowConvergenceIteration,
    WorthQueryWorkflowConvergenceYieldCleanupOutcome, WorthQueryWorkflowConvergenceYieldOutcome,
    WorthQueryWorkflowGraphStepOutcome, WorthQueryWorkflowYieldOutcome,
};

#[test]
fn direct_yield_cleanup_preserves_epoch_and_managed_release_evidence() {
    let (pending, outcome) = direct_yield("direct-yield-cleanup");
    let yielded = match pending.admit_yield_outcome(outcome) {
        WorthQueryDirectConvergenceYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("matching direct yield must enter its convergence epoch"),
    };
    let epoch_identity = yielded.epoch_identity().to_owned();
    let cleanup = match yielded.cleanup() {
        WorthQueryDirectConvergenceYieldCleanupOutcome::Complete(cleanup) => cleanup,
        WorthQueryDirectConvergenceYieldCleanupOutcome::RecoveryRequired(_) => {
            panic!("cooperative direct checkpoint cleanup must complete")
        }
    };

    assert_eq!(cleanup.identity(), epoch_identity);
    assert_eq!(cleanup.counters().yield_count(), 1);
    assert_eq!(cleanup.counters().readmission_count(), 0);
    assert_eq!(cleanup.counters().cleanup_count(), 1);
    assert!(cleanup.incumbents().is_empty());
    assert!(cleanup.latest_report().is_none());
    assert!(cleanup.managed_receipt().checkpoint_release().is_some());
}

#[test]
fn workflow_yield_cleanup_preserves_epoch_and_managed_release_evidence() {
    let (pending, outcome) = workflow_yield("workflow-yield-cleanup");
    let yielded = match pending.admit_yield_outcome(outcome) {
        WorthQueryWorkflowConvergenceYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("matching workflow yield must enter its convergence epoch"),
    };
    let epoch_identity = yielded.epoch_identity().to_owned();
    let cleanup = match yielded.cleanup() {
        WorthQueryWorkflowConvergenceYieldCleanupOutcome::Complete(cleanup) => cleanup,
        WorthQueryWorkflowConvergenceYieldCleanupOutcome::Pending(_) => {
            panic!("artifact-free workflow checkpoint cleanup must not remain pending")
        }
        WorthQueryWorkflowConvergenceYieldCleanupOutcome::RecoveryRequired(_) => {
            panic!("cooperative workflow checkpoint cleanup must complete")
        }
    };

    assert_eq!(cleanup.identity(), epoch_identity);
    assert_eq!(cleanup.counters().yield_count(), 1);
    assert_eq!(cleanup.counters().readmission_count(), 0);
    assert_eq!(cleanup.counters().cleanup_count(), 1);
    assert!(cleanup.incumbents().is_empty());
    assert!(cleanup.latest_report().is_none());
    assert_eq!(
        cleanup
            .managed_receipt()
            .artifact_evidence()
            .retained_artifact_count(),
        0
    );
}

#[test]
fn foreign_direct_yield_preserves_both_epoch_authorities_without_recounting() {
    let (pending_a, outcome_a) = direct_yield("direct-yield-a");
    let (pending_b, outcome_b) = direct_yield("direct-yield-b");
    let (pending_a, yielded_b) = match pending_a.admit_yield_outcome(outcome_b) {
        WorthQueryDirectConvergenceYieldOutcome::RunMismatch { pending, yielded } => {
            (pending, yielded)
        }
        _ => panic!("foreign direct yield must remain outside the pending epoch"),
    };
    assert_eq!(pending_a.core.counters().yield_count(), 0);

    cleanup_direct(pending_a, outcome_a);
    cleanup_direct(pending_b, WorthQueryDirectYieldOutcome::Yielded(yielded_b));
}

#[test]
fn foreign_workflow_yield_preserves_both_epoch_authorities_without_recounting() {
    let (pending_a, outcome_a) = workflow_yield("workflow-yield-a");
    let (pending_b, outcome_b) = workflow_yield("workflow-yield-b");
    let (pending_a, yielded_b) = match pending_a.admit_yield_outcome(outcome_b) {
        WorthQueryWorkflowConvergenceYieldOutcome::RunMismatch { pending, yielded } => {
            (pending, yielded)
        }
        _ => panic!("foreign workflow yield must remain outside the pending epoch"),
    };
    assert_eq!(pending_a.core.counters().yield_count(), 0);

    cleanup_workflow(pending_a, outcome_a);
    cleanup_workflow(
        pending_b,
        WorthQueryWorkflowYieldOutcome::Yielded(yielded_b),
    );
}

fn direct_yield(
    call_identity: &str,
) -> (
    WorthQueryPendingDirectConvergenceIteration,
    WorthQueryDirectYieldOutcome,
) {
    let DirectAdmissionFixture {
        runtime,
        operation,
        alternate_basis_operation: _,
        contract,
        managed,
        graph,
        bridge: _,
    } = direct_admission_fixture(FixtureDisposition::YieldThenConverged);
    let epoch = runtime
        .admit_direct_convergence_epoch(&operation, contract, managed, graph)
        .unwrap_or_else(|_| panic!("direct convergence authorities must admit"))
        .start();
    let started = epoch
        .begin_iteration(call(call_identity))
        .unwrap_or_else(|_| panic!("direct convergence iteration must start"));
    let (pending, active) = started.into_parts();
    let paused = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("direct yield fixture must reach a safe point"),
    };
    (pending, paused.yield_run())
}

fn workflow_yield(
    call_identity: &str,
) -> (
    WorthQueryPendingWorkflowConvergenceIteration,
    WorthQueryWorkflowYieldOutcome,
) {
    let WorkflowAdmissionFixture {
        runtime,
        operation,
        contract,
        managed,
        graph,
        bridge: _,
    } = workflow_admission_fixture(FixtureDisposition::YieldThenConverged);
    let admitted = runtime
        .admit_workflow_convergence_epoch(&operation, contract, managed, graph)
        .unwrap_or_else(|_| panic!("workflow convergence authorities must admit"));
    let epoch = admitted
        .start()
        .unwrap_or_else(|_| panic!("workflow convergence epoch must start"));
    let started = epoch
        .begin_stage_iteration(WORKFLOW_STAGE, call(call_identity))
        .unwrap_or_else(|_| panic!("workflow convergence iteration must start"));
    let (pending, active) = started.into_parts();
    let paused = match active.advance() {
        WorthQueryWorkflowGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("workflow yield fixture must reach a safe point"),
    };
    (pending, paused.yield_run())
}

fn cleanup_direct(
    pending: WorthQueryPendingDirectConvergenceIteration,
    outcome: WorthQueryDirectYieldOutcome,
) {
    let yielded = match pending.admit_yield_outcome(outcome) {
        WorthQueryDirectConvergenceYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("returned direct authorities must rejoin their owning epoch"),
    };
    assert!(matches!(
        yielded.cleanup(),
        WorthQueryDirectConvergenceYieldCleanupOutcome::Complete(_)
    ));
}

fn cleanup_workflow(
    pending: WorthQueryPendingWorkflowConvergenceIteration,
    outcome: WorthQueryWorkflowYieldOutcome,
) {
    let yielded = match pending.admit_yield_outcome(outcome) {
        WorthQueryWorkflowConvergenceYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("returned workflow authorities must rejoin their owning epoch"),
    };
    assert!(matches!(
        yielded.cleanup(),
        WorthQueryWorkflowConvergenceYieldCleanupOutcome::Complete(_)
    ));
}

fn call(identity: &str) -> WorthQueryManagedGraphCallRequest {
    WorthQueryManagedGraphCallRequest::new(WorthQueryGraphProviderCallKind::Observe, identity)
}
