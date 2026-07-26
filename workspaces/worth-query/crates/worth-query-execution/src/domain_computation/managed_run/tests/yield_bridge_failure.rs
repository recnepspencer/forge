use worth_runtime_bridge::facade::{
    BridgeExecutionBasisFinalizationFailureKind, BridgeManagedExecutionCancellationReason,
};

use super::yield_fixture::YieldProvider;
use super::*;

#[test]
fn bridge_terminalization_failure_preserves_the_paused_run_for_retry_or_cleanup() {
    let (running, graph) = managed_graph_run_with_provider(
        WorthQueryOperationGraphAccess::Observe,
        YieldProvider::installed(5),
    );
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Observe,
                "yield-bridge-thread-affinity",
            ),
        )
        .unwrap();
    let paused = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("provider did not pause"),
    };
    let recovery = std::thread::spawn(move || match paused.yield_run() {
        crate::domain_computation::WorthQueryDirectYieldOutcome::RecoveryRequired(recovery) => {
            recovery
        }
        _ => panic!("foreign-thread bridge finalization did not require recovery"),
    })
    .join()
    .expect("foreign-thread yield probe should return recovery authority");
    assert_eq!(
        recovery.kind(),
        crate::domain_computation::WorthQueryYieldRecoveryKind::BridgeTerminalization(
            BridgeExecutionBasisFinalizationFailureKind::SignalRuntimeThreadAffinityViolation,
        )
    );
    assert!(recovery.running_attempt_recoverable());

    let paused = match recovery.into_paused() {
        Ok(paused) => paused,
        Err(_) => panic!("bridge failure consumed the still-running attempt"),
    };
    paused
        .active
        .request_cancellation(BridgeManagedExecutionCancellationReason::HostRequested)
        .unwrap();
    let terminal = match paused.advance() {
        WorthQueryDirectGraphStepOutcome::Cancelled(terminal) => terminal,
        _ => panic!("recovered paused run did not retain cancellation authority"),
    };
    assert!(terminal.cleanup().is_ok());
}
