use super::fixture::{
    direct_admission_fixture, workflow_admission_fixture, workflow_yield_pending_admission_fixture,
    DirectAdmissionFixture, FixtureDisposition, WorkflowAdmissionFixture, WORKFLOW_STAGE,
};
use crate::domain_computation::artifact_owner::WorthQueryMoveOnlyArtifactHandle;
use crate::domain_computation::{
    WorthQueryDirectConvergenceStepOutcome, WorthQueryDirectConvergenceYieldCleanupOutcome,
    WorthQueryDirectConvergenceYieldOutcome, WorthQueryGraphProviderCallKind,
    WorthQueryManagedGraphCallRequest, WorthQueryWorkflowConvergenceStepOutcome,
    WorthQueryWorkflowConvergenceYieldCleanupOutcome, WorthQueryWorkflowConvergenceYieldOutcome,
    WorthQueryYieldedDirectConvergenceIteration, WorthQueryYieldedWorkflowConvergenceIteration,
};

#[test]
fn direct_yield_cleanup_preserves_its_closed_epoch_receipt() {
    let yielded = direct_yield("direct-yield-cleanup");
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
    assert_eq!(cleanup.counters().cleanup_attempt_count(), 1);
    assert_eq!(cleanup.counters().cleanup_completion_count(), 1);
    assert!(cleanup.incumbents().is_empty());
    assert!(cleanup.latest_report().is_none());
}

#[test]
fn workflow_yield_cleanup_preserves_its_closed_epoch_receipt() {
    let yielded = workflow_yield("workflow-yield-cleanup");
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
    assert_eq!(cleanup.counters().cleanup_attempt_count(), 1);
    assert_eq!(cleanup.counters().cleanup_completion_count(), 1);
    assert!(cleanup.incumbents().is_empty());
    assert!(cleanup.latest_report().is_none());
}

#[test]
fn direct_yield_closed_recovery_counts_the_cleanup_as_complete() {
    let yielded = direct_yield_with_disposition(
        FixtureDisposition::YieldThenCheckpointDropPanic,
        "direct-yield-closed-recovery",
    );
    let receipt = match yielded.cleanup() {
        WorthQueryDirectConvergenceYieldCleanupOutcome::RecoveryRequired(receipt) => receipt,
        WorthQueryDirectConvergenceYieldCleanupOutcome::Complete(_) => {
            panic!("checkpoint drop panic claimed complete cleanup")
        }
    };

    assert_eq!(receipt.counters().cleanup_attempt_count(), 1);
    assert_eq!(receipt.counters().cleanup_completion_count(), 1);
    assert!(receipt.incumbents().is_empty());
    assert!(receipt.latest_report().is_none());
}

#[test]
fn workflow_yield_closed_recovery_counts_the_cleanup_as_complete() {
    let yielded = workflow_yield_with_disposition(
        FixtureDisposition::YieldThenCheckpointDropPanic,
        "workflow-yield-closed-recovery",
    );
    let receipt = match yielded.cleanup() {
        WorthQueryWorkflowConvergenceYieldCleanupOutcome::RecoveryRequired(receipt) => receipt,
        WorthQueryWorkflowConvergenceYieldCleanupOutcome::Complete(_) => {
            panic!("checkpoint drop panic claimed complete cleanup")
        }
        WorthQueryWorkflowConvergenceYieldCleanupOutcome::Pending(_) => {
            panic!("artifact-free checkpoint drop panic claimed pending cleanup")
        }
    };

    assert_eq!(receipt.counters().cleanup_attempt_count(), 1);
    assert_eq!(receipt.counters().cleanup_completion_count(), 1);
    assert!(receipt.incumbents().is_empty());
    assert!(receipt.latest_report().is_none());
}

#[test]
fn workflow_yield_pending_then_retry_counts_two_attempts_and_one_completion() {
    let (yielded, artifact) = workflow_yield_with_pending_artifact("workflow-yield-pending");
    let borrowed = artifact
        .borrow("convergence cleanup pending proof")
        .expect("installed candidate contract must admit shared observation");
    let pending = match yielded.cleanup() {
        WorthQueryWorkflowConvergenceYieldCleanupOutcome::Pending(pending) => pending,
        _ => panic!("live artifact handle and borrow must keep workflow cleanup pending"),
    };
    assert_eq!(pending.counters().cleanup_attempt_count(), 1);
    assert_eq!(pending.counters().cleanup_completion_count(), 0);

    drop(borrowed);
    drop(artifact);
    let receipt = match pending.retry() {
        WorthQueryWorkflowConvergenceYieldCleanupOutcome::Complete(receipt) => receipt,
        _ => panic!("released artifact ownership must permit cleanup completion"),
    };
    assert_eq!(receipt.counters().cleanup_attempt_count(), 2);
    assert_eq!(receipt.counters().cleanup_completion_count(), 1);
}

#[test]
fn same_scope_direct_cleanup_peers_keep_their_own_epochs() {
    let first = direct_yield("same-direct-cleanup-scope");
    let second = direct_yield("same-direct-cleanup-scope");
    let first_identity = first.epoch_identity().to_owned();
    let second_identity = second.epoch_identity().to_owned();
    assert_ne!(first_identity, second_identity);

    let second = match second.cleanup() {
        WorthQueryDirectConvergenceYieldCleanupOutcome::Complete(receipt) => receipt,
        WorthQueryDirectConvergenceYieldCleanupOutcome::RecoveryRequired(_) => {
            panic!("cooperative second direct peer must complete cleanup")
        }
    };
    let first = match first.cleanup() {
        WorthQueryDirectConvergenceYieldCleanupOutcome::Complete(receipt) => receipt,
        WorthQueryDirectConvergenceYieldCleanupOutcome::RecoveryRequired(_) => {
            panic!("cooperative first direct peer must complete cleanup")
        }
    };
    assert_eq!(first.identity(), first_identity);
    assert_eq!(second.identity(), second_identity);
    assert_eq!(first.counters().cleanup_attempt_count(), 1);
    assert_eq!(second.counters().cleanup_attempt_count(), 1);
}

#[test]
fn same_stage_workflow_pending_and_complete_peers_do_not_cross_pair() {
    let (first, artifact) = workflow_yield_with_pending_artifact("same-workflow-cleanup-scope");
    let borrowed = artifact
        .borrow("interleaved cleanup association proof")
        .expect("installed candidate contract must admit shared observation");
    let second = workflow_yield("same-workflow-cleanup-scope");
    let first_identity = first.epoch_identity().to_owned();
    let second_identity = second.epoch_identity().to_owned();
    assert_ne!(first_identity, second_identity);

    let pending = match first.cleanup() {
        WorthQueryWorkflowConvergenceYieldCleanupOutcome::Pending(pending) => pending,
        _ => panic!("live first peer artifact must retain pending cleanup authority"),
    };
    let second = match second.cleanup() {
        WorthQueryWorkflowConvergenceYieldCleanupOutcome::Complete(receipt) => receipt,
        _ => panic!("artifact-free second workflow peer must complete cleanup"),
    };
    assert_eq!(pending.identity(), first_identity);
    assert_eq!(second.identity(), second_identity);
    assert_eq!(pending.counters().cleanup_attempt_count(), 1);
    assert_eq!(pending.counters().cleanup_completion_count(), 0);
    assert_eq!(second.counters().cleanup_attempt_count(), 1);
    assert_eq!(second.counters().cleanup_completion_count(), 1);

    drop(borrowed);
    drop(artifact);
    let first = match pending.retry() {
        WorthQueryWorkflowConvergenceYieldCleanupOutcome::Complete(receipt) => receipt,
        _ => panic!("rightful first peer must complete after its artifact owners close"),
    };
    assert_eq!(first.identity(), first_identity);
    assert_eq!(first.counters().cleanup_attempt_count(), 2);
    assert_eq!(first.counters().cleanup_completion_count(), 1);
}

fn workflow_yield_with_pending_artifact(
    call_identity: &str,
) -> (
    WorthQueryYieldedWorkflowConvergenceIteration,
    WorthQueryMoveOnlyArtifactHandle,
) {
    let (fixture, artifact_receiver) = workflow_yield_pending_admission_fixture();
    let WorkflowAdmissionFixture {
        runtime,
        operation,
        contract,
        managed,
        graph,
        bridge: _,
    } = fixture;
    let admitted = runtime
        .admit_workflow_convergence_epoch(&operation, contract, managed, graph)
        .unwrap_or_else(|_| panic!("artifact workflow authorities must admit"));
    let epoch = admitted
        .start()
        .unwrap_or_else(|_| panic!("artifact workflow convergence must start"));
    let started = epoch
        .begin_stage_iteration(WORKFLOW_STAGE, call(call_identity))
        .unwrap_or_else(|_| panic!("artifact workflow iteration must start"));
    let paused = match started.advance() {
        WorthQueryWorkflowConvergenceStepOutcome::Continue(paused) => paused,
        _ => panic!("artifact workflow must reach a yield safe point"),
    };
    let artifact = artifact_receiver
        .recv()
        .expect("production provider step must issue the move-only artifact handle");
    let yielded = match paused.yield_iteration() {
        WorthQueryWorkflowConvergenceYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("artifact workflow must yield"),
    };
    (yielded, artifact)
}

fn direct_yield(call_identity: &str) -> WorthQueryYieldedDirectConvergenceIteration {
    direct_yield_with_disposition(FixtureDisposition::YieldThenConverged, call_identity)
}

fn direct_yield_with_disposition(
    disposition: FixtureDisposition,
    call_identity: &str,
) -> WorthQueryYieldedDirectConvergenceIteration {
    let DirectAdmissionFixture {
        runtime,
        operation,
        alternate_basis_operation: _,
        contract,
        managed,
        graph,
        bridge: _,
    } = direct_admission_fixture(disposition);
    let epoch = runtime
        .admit_direct_convergence_epoch(&operation, contract, managed, graph)
        .unwrap_or_else(|_| panic!("direct convergence authorities must admit"))
        .start();
    let started = epoch
        .begin_iteration(call(call_identity))
        .unwrap_or_else(|_| panic!("direct convergence iteration must start"));
    let paused = match started.advance() {
        WorthQueryDirectConvergenceStepOutcome::Continue(paused) => paused,
        _ => panic!("direct yield fixture must reach a safe point"),
    };
    match paused.yield_iteration() {
        WorthQueryDirectConvergenceYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("direct yield fixture must retain its exact epoch association"),
    }
}

fn workflow_yield(call_identity: &str) -> WorthQueryYieldedWorkflowConvergenceIteration {
    workflow_yield_with_disposition(FixtureDisposition::YieldThenConverged, call_identity)
}

fn workflow_yield_with_disposition(
    disposition: FixtureDisposition,
    call_identity: &str,
) -> WorthQueryYieldedWorkflowConvergenceIteration {
    let WorkflowAdmissionFixture {
        runtime,
        operation,
        contract,
        managed,
        graph,
        bridge: _,
    } = workflow_admission_fixture(disposition);
    let admitted = runtime
        .admit_workflow_convergence_epoch(&operation, contract, managed, graph)
        .unwrap_or_else(|_| panic!("workflow convergence authorities must admit"));
    let epoch = admitted
        .start()
        .unwrap_or_else(|_| panic!("workflow convergence epoch must start"));
    let started = epoch
        .begin_stage_iteration(WORKFLOW_STAGE, call(call_identity))
        .unwrap_or_else(|_| panic!("workflow convergence iteration must start"));
    let paused = match started.advance() {
        WorthQueryWorkflowConvergenceStepOutcome::Continue(paused) => paused,
        _ => panic!("workflow yield fixture must reach a safe point"),
    };
    match paused.yield_iteration() {
        WorthQueryWorkflowConvergenceYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("workflow yield fixture must retain its exact epoch association"),
    }
}

fn call(identity: &str) -> WorthQueryManagedGraphCallRequest {
    WorthQueryManagedGraphCallRequest::new(WorthQueryGraphProviderCallKind::Observe, identity)
}
