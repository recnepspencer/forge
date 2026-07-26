use super::yield_fixture::YieldProvider;
use super::*;

pub(super) fn yielded_direct() -> (
    crate::domain_computation::WorthQueryYieldedDirectRun,
    RuntimeBridge,
    WorthQueryExecutionRuntime,
) {
    let (running, graph, bridge, runtime) = managed_graph_run_with_provider_and_runtime(
        WorthQueryOperationGraphAccess::Observe,
        YieldProvider::installed(5),
    );
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Observe,
                "direct-readmission",
            ),
        )
        .expect("yield provider should begin");
    let paused = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("yield provider did not pause"),
    };
    let yielded = match paused.yield_run() {
        crate::domain_computation::WorthQueryDirectYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("eligible run did not yield"),
    };
    (yielded, bridge, runtime)
}

pub(super) fn yielded_direct_with_provider(
    provider: YieldProvider,
) -> (
    crate::domain_computation::WorthQueryYieldedDirectRun,
    RuntimeBridge,
    WorthQueryExecutionRuntime,
) {
    let (running, graph, bridge, runtime) = managed_graph_run_with_provider_and_runtime(
        WorthQueryOperationGraphAccess::Observe,
        provider,
    );
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Observe,
                "direct-readmission-provider-edge",
            ),
        )
        .expect("yield provider should begin");
    let paused = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("yield provider did not pause"),
    };
    let yielded = match paused.yield_run() {
        crate::domain_computation::WorthQueryDirectYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("eligible run did not yield"),
    };
    (yielded, bridge, runtime)
}

#[test]
fn direct_readmission_mints_fresh_attempts_and_transfers_capacity() {
    let (yielded, bridge, runtime) = yielded_direct();
    let logical = yielded.logical_run_identity().to_owned();
    let managed_attempt = yielded.yielded_attempt_identity().to_owned();
    let resource_attempt = yielded.resource_attempt_identity().to_owned();
    let provider_session = yielded.provider_session_identity().to_owned();
    let bridge_basis = yielded.bridge().basis_identity().as_str().to_owned();
    let bridge_request = yielded.bridge_request_identity().to_owned();
    let reservation_count = yielded.retained_capacity_reservation_count();
    let active = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryDirectReadmissionOutcome::Readmitted(readmitted) => {
            readmitted.into_active()
        }
        _ => panic!("same-runtime readmission should succeed"),
    };
    assert_eq!(active.logical_run_identity(), logical);
    assert_ne!(active.run_identity(), managed_attempt);
    assert_ne!(active.resource_attempt_identity(), resource_attempt);
    assert_ne!(active.provider_session_identity(), provider_session);
    assert_ne!(active.bridge_basis_identity(), bridge_basis);
    assert_ne!(active.bridge_request_identity(), bridge_request);
    assert_eq!(
        active.retained_capacity_reservation_count(),
        reservation_count
    );
    assert_eq!(reservation_count, 2);
    assert!(!active.provider_call_identity().is_empty());

    let completion = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("restored provider did not complete"),
    };
    let terminal = completion.into_running().completed().unwrap();
    assert_eq!(terminal.logical_run_identity(), logical);
    assert_eq!(terminal.provider_work().completed_work_units(), 4);
    assert!(terminal.cleanup().is_ok());
}

#[test]
fn query_preflight_denial_returns_the_exact_yielded_capability_without_fresh_work() {
    let (yielded, bridge, runtime) = yielded_direct();
    let checkpoint = yielded.checkpoint().identity().to_owned();
    let resource_attempt = yielded.resource_attempt_identity().to_owned();
    let foreign_runtime = query_runtime();
    let denied = match yielded.readmit_same_runtime(&foreign_runtime, &bridge) {
        crate::domain_computation::WorthQueryDirectReadmissionOutcome::Denied(denied) => denied,
        _ => panic!("foreign Query runtime should deny"),
    };
    assert_eq!(
        denied.kind(),
        crate::domain_computation::WorthQueryDirectReadmissionDenialKind::ForeignQueryRuntime
    );
    assert_eq!(denied.counters().preflight_check_count(), 1);
    assert_eq!(denied.counters().fresh_resource_attempt_count(), 0);
    assert_eq!(denied.counters().bridge_readmission_attempt_count(), 0);
    let yielded = denied.into_yielded();
    assert_eq!(yielded.checkpoint().identity(), checkpoint);
    assert_eq!(yielded.resource_attempt_identity(), resource_attempt);

    let active = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryDirectReadmissionOutcome::Readmitted(readmitted) => {
            readmitted.into_active()
        }
        _ => panic!("returned yielded capability should remain readmittable"),
    };
    let completion = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("restored provider did not complete"),
    };
    assert!(completion
        .into_running()
        .completed()
        .unwrap()
        .cleanup()
        .is_ok());
}

#[test]
fn step_contract_mismatch_denies_before_resource_or_signal_authority() {
    let (mut yielded, bridge, runtime) = yielded_direct();
    let checkpoint = yielded.checkpoint().identity().to_owned();
    yielded.execution.contract = foreign_safe_point_contract();
    let denied = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryDirectReadmissionOutcome::Denied(denied) => denied,
        _ => panic!("step-contract mismatch should deny during effect-free preflight"),
    };
    assert_eq!(
        denied.kind(),
        crate::domain_computation::WorthQueryDirectReadmissionDenialKind::
            ProviderStepContractDenied(
                crate::domain_computation::WorthQueryManagedStepContractDenialKind::
                    SafePointFamilyMismatch,
            )
    );
    assert_eq!(denied.counters().fresh_resource_attempt_count(), 0);
    assert_eq!(denied.counters().bridge_readmission_attempt_count(), 0);
    assert_eq!(denied.counters().provider_restore_attempt_count(), 0);
    let yielded = denied.into_yielded();
    assert_eq!(yielded.checkpoint().identity(), checkpoint);
    complete_direct_yield_cleanup(yielded);
}

#[test]
fn provider_restore_denial_preserves_the_exact_checkpoint_and_capacity_package() {
    let (yielded, bridge, runtime) =
        yielded_direct_with_provider(YieldProvider::checkpoint_restore_failure(7));
    let checkpoint = yielded.checkpoint().identity().to_owned();
    let resource_attempt = yielded.resource_attempt_identity().to_owned();
    let reservations = yielded.retained_capacity_reservation_count();
    let denied = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryDirectReadmissionOutcome::Denied(denied) => denied,
        _ => panic!("ordinary provider restore failure should deny"),
    };
    assert_eq!(
        denied.kind(),
        crate::domain_computation::WorthQueryDirectReadmissionDenialKind::ProviderRestoreDenied
    );
    assert_eq!(denied.counters().fresh_resource_attempt_count(), 1);
    assert_eq!(denied.counters().bridge_readmission_attempt_count(), 1);
    assert_eq!(denied.counters().provider_restore_attempt_count(), 1);
    assert_eq!(denied.counters().committed_attempt_count(), 0);
    let yielded = denied.into_yielded();
    assert_eq!(yielded.checkpoint().identity(), checkpoint);
    assert_eq!(yielded.resource_attempt_identity(), resource_attempt);
    assert_eq!(yielded.retained_capacity_reservation_count(), reservations);
    let cleanup = complete_direct_yield_cleanup(yielded);
    assert_eq!(cleanup.checkpoint().unwrap().identity(), checkpoint);
}

#[test]
fn provider_restore_panic_remains_typed_recovery_with_retained_authority() {
    let (yielded, bridge, runtime) =
        yielded_direct_with_provider(YieldProvider::checkpoint_restore_panic(7));
    let recovery = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryDirectReadmissionOutcome::RecoveryRequired(
            recovery,
        ) => recovery,
        _ => panic!("provider restore panic must not become ordinary denial"),
    };
    assert_eq!(
        recovery.kind(),
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryKind::ProviderRestorePanicked
    );
    assert_eq!(recovery.counters().provider_restore_attempt_count(), 1);
    assert_eq!(recovery.counters().committed_attempt_count(), 0);
    assert_eq!(
        recovery.posture(),
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryPosture::
            TerminalCleanupRequired
    );
    assert!(recovery.checkpoint_release().is_none());
    assert!(recovery.restored_execution_release_evidence().is_none());
    let recovery = match recovery {
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryRequired::TerminalCleanup(
            recovery,
        ) => recovery,
        _ => panic!("provider panic must not expose yield-reassembly authority"),
    };
    let receipt = match recovery.into_cleanup().finish() {
        crate::domain_computation::WorthQueryDirectReadmissionCleanupOutcome::Complete(receipt) => {
            receipt
        }
        _ => panic!("retained checkpoint should release through terminal cleanup"),
    };
    assert_eq!(
        receipt.checkpoint_release().checkpoint().retained_bytes(),
        7
    );
    assert!(receipt.bridge().reservation_released());
    assert!(receipt.relational().released());
}

#[test]
fn provider_restore_rejection_after_admission_carries_exact_release_evidence() {
    let (yielded, bridge, runtime) =
        yielded_direct_with_provider(YieldProvider::checkpoint_restore_reject_after_admission(7));
    let recovery = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryDirectReadmissionOutcome::RecoveryRequired(
            recovery,
        ) => recovery,
        _ => panic!("post-admission restore rejection became ordinary retryable denial"),
    };
    assert_eq!(
        recovery.kind(),
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryKind::
            ProviderRestoreRejectedAfterExecutionAdmission
    );
    let release = recovery
        .restored_execution_release_evidence()
        .expect("rejected admitted restore must carry physical-release evidence");
    assert_eq!(
        release.disposal(),
        crate::domain_computation::WorthQueryProviderExecutionDisposalDisposition::Completed
    );
    assert_eq!(
        release.destructor(),
        crate::domain_computation::WorthQueryProviderExecutionDestructorDisposition::Completed
    );
    assert_eq!(
        recovery.posture(),
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryPosture::
            TerminalCleanupRequired
    );
    let recovery = match recovery {
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryRequired::TerminalCleanup(
            recovery,
        ) => recovery,
        _ => panic!("post-admission provider rejection must not expose retry authority"),
    };
    match recovery.into_cleanup().finish() {
        crate::domain_computation::WorthQueryDirectReadmissionCleanupOutcome::Complete(receipt) => {
            assert_eq!(
                receipt.checkpoint_release().disposition(),
                crate::domain_computation::WorthQueryProviderCheckpointReleaseDisposition::Released
            );
            assert!(!receipt
                .restored_execution_release()
                .expect("replacement release evidence must survive cleanup")
                .recovery_required());
        }
        _ => panic!("released replacement and checkpoint should complete terminal cleanup"),
    }
}

#[test]
fn restore_panic_and_replacement_destructor_panic_flow_into_terminal_cleanup() {
    let (yielded, bridge, runtime) =
        yielded_direct_with_provider(YieldProvider::checkpoint_restore_panic_after_admission(7));
    let recovery = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryDirectReadmissionOutcome::RecoveryRequired(
            recovery,
        ) => recovery,
        _ => panic!("double restore failure escaped typed recovery"),
    };
    assert_eq!(
        recovery.kind(),
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryKind::ProviderRestorePanicked
    );
    let release = recovery
        .restored_execution_release_evidence()
        .expect("restore panic must retain replacement release evidence");
    assert_eq!(
        release.disposal(),
        crate::domain_computation::WorthQueryProviderExecutionDisposalDisposition::Completed
    );
    assert_eq!(
        release.destructor(),
        crate::domain_computation::WorthQueryProviderExecutionDestructorDisposition::Panicked
    );
    assert_eq!(
        recovery.posture(),
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryPosture::
            TerminalCleanupRequired
    );
    let recovery = match recovery {
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryRequired::TerminalCleanup(
            recovery,
        ) => recovery,
        _ => panic!("provider panic must not expose yield-reassembly authority"),
    };
    let receipt = match recovery.into_cleanup().finish() {
        crate::domain_computation::WorthQueryDirectReadmissionCleanupOutcome::RecoveryRequired(
            receipt,
        ) => receipt,
        _ => panic!("destructor uncertainty must remain visible after terminal cleanup"),
    };
    let carried_release = receipt
        .provider_work()
        .provider_execution_release()
        .recovery_evidence()
        .expect("cleanup evidence must retain replacement release uncertainty");
    assert_eq!(
        carried_release.destructor(),
        crate::domain_computation::WorthQueryProviderExecutionDestructorDisposition::Panicked
    );
}

fn foreign_safe_point_contract(
) -> worth_query_installation::facade::WorthQueryInstalledBoundedStepContract {
    use worth_query_declaration::facade::domain_computation::{
        WorthQueryCancellationSafePointFamily, WorthQueryExecutionMode,
        WorthQueryResourceDimension, WorthQueryResourceLimitRequest,
        WorthQuerySemanticScaleRequest,
    };

    let resources = WorthQueryResourceLimitRequest::bounded(1)
        .with(WorthQueryResourceDimension::CancellationPollingInterval, 1)
        .with(WorthQueryResourceDimension::QueueDepth, 1)
        .with(WorthQueryResourceDimension::ChunkWidth, 1)
        .with(WorthQueryResourceDimension::ScratchBytes, 1)
        .with(WorthQueryResourceDimension::RetainedBytes, 1)
        .with(WorthQueryResourceDimension::DeadlineNanos, 1);
    worth_query_installation::facade::WorthQueryExecutionResourceEnvelope::new(
        WorthQuerySemanticScaleRequest::bounded(1),
        resources,
        WorthQueryExecutionMode::Asynchronous,
        None,
        WorthQueryCancellationSafePointFamily::new("foreign-readmission-safe-point")
            .expect("test safe-point family should be valid"),
    )
    .bounded_step_contract()
    .expect("fully bounded test envelope should expose a step contract")
}
