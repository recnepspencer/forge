use worth_runtime_bridge::facade::BridgeExecutionBasisFinalizationFailureKind;

use super::terminal_fixture::{converged_terminal, workflow_converged_terminal};
use crate::domain_computation::{
    WorthQueryConvergenceTerminalKind, WorthQueryManagedRunCleanupDisposition,
    WorthQueryWorkflowConvergenceCleanupOutcome,
};

#[test]
fn direct_cleanup_recovery_preserves_epoch_evidence_and_counts_the_retry() {
    let terminal = converged_terminal();
    let identity = terminal.identity().to_owned();
    let failure = std::thread::spawn(move || match terminal.cleanup() {
        Ok(_) => panic!("foreign thread finalized the convergence Signal basis"),
        Err(failure) => failure,
    })
    .join()
    .expect("foreign-thread cleanup must return its recovery authority");

    assert_eq!(failure.identity(), identity);
    assert_eq!(failure.kind(), WorthQueryConvergenceTerminalKind::Converged);
    assert_eq!(failure.counters().cleanup_count(), 1);
    assert_eq!(failure.incumbents().len(), 1);
    assert!(failure.latest_report().is_some());
    assert!(failure.domain_failure().is_none());
    assert_eq!(
        failure.managed_failure().failure_kind(),
        BridgeExecutionBasisFinalizationFailureKind::SignalRuntimeThreadAffinityViolation
    );

    let receipt = match failure.retry() {
        Ok(receipt) => receipt,
        Err(_) => panic!("owner thread did not finalize the same convergence terminal"),
    };
    assert_eq!(receipt.identity(), identity);
    assert_eq!(receipt.kind(), WorthQueryConvergenceTerminalKind::Converged);
    assert_eq!(receipt.counters().cleanup_count(), 2);
    assert_eq!(receipt.incumbents().len(), 1);
    assert!(receipt.latest_report().is_some());
    assert_eq!(
        receipt.managed_receipt().disposition(),
        WorthQueryManagedRunCleanupDisposition::CleanupComplete
    );
}

#[test]
fn workflow_cleanup_recovery_preserves_epoch_evidence_and_counts_the_retry() {
    let terminal = workflow_converged_terminal();
    let identity = terminal.identity().to_owned();
    let failure = std::thread::spawn(move || match terminal.cleanup() {
        WorthQueryWorkflowConvergenceCleanupOutcome::RecoveryRequired(failure) => failure,
        WorthQueryWorkflowConvergenceCleanupOutcome::Complete(_) => {
            panic!("foreign thread finalized the workflow convergence Signal basis")
        }
        WorthQueryWorkflowConvergenceCleanupOutcome::Pending(_) => {
            panic!("workflow convergence fixture retained an unexpected artifact owner")
        }
    })
    .join()
    .expect("foreign-thread workflow cleanup must return recovery authority");

    assert_eq!(failure.identity(), identity);
    assert_eq!(failure.kind(), WorthQueryConvergenceTerminalKind::Converged);
    assert_eq!(failure.counters().cleanup_count(), 1);
    assert_eq!(failure.incumbents().len(), 1);
    assert!(failure.latest_report().is_some());
    assert!(failure.domain_failure().is_none());
    assert_eq!(
        failure.managed_failure().failure_kind(),
        BridgeExecutionBasisFinalizationFailureKind::SignalRuntimeThreadAffinityViolation
    );

    let receipt = match failure.retry() {
        WorthQueryWorkflowConvergenceCleanupOutcome::Complete(receipt) => receipt,
        WorthQueryWorkflowConvergenceCleanupOutcome::Pending(_) => {
            panic!("owner-thread retry retained an unexpected artifact owner")
        }
        WorthQueryWorkflowConvergenceCleanupOutcome::RecoveryRequired(_) => {
            panic!("owner-thread retry did not finalize the workflow convergence terminal")
        }
    };
    assert_eq!(receipt.identity(), identity);
    assert_eq!(receipt.kind(), WorthQueryConvergenceTerminalKind::Converged);
    assert_eq!(receipt.counters().cleanup_count(), 2);
    assert_eq!(receipt.incumbents().len(), 1);
    assert!(receipt.latest_report().is_some());
    assert_eq!(
        receipt.managed_receipt().disposition(),
        WorthQueryManagedRunCleanupDisposition::CleanupComplete
    );
}
