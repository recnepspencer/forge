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
        crate::domain_computation::WorthQueryDirectReadmissionOutcome::Readmitted(active) => active,
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
        crate::domain_computation::WorthQueryDirectReadmissionOutcome::Readmitted(active) => active,
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
    assert_eq!(recovery.retained_authority_count(), 4);
    assert!(recovery.checkpoint_authority_retained());
    assert!(recovery.checkpoint_release().is_none());
    assert!(recovery.restored_execution_release_evidence().is_none());
    assert!(recovery.bridge_cleanup_pending());
    assert!(recovery.fresh_resource_attempt_pending());
    let yielded = match recovery.retry_to_yielded() {
        Ok(
            crate::domain_computation::WorthQueryDirectReadmissionRecoveryRetryOutcome::Yielded(
                yielded,
            ),
        ) => yielded,
        _ => panic!("retained checkpoint recovery should return the yielded authority"),
    };
    assert_eq!(yielded.checkpoint().retained_bytes(), 7);
    assert_eq!(
        complete_direct_yield_cleanup(yielded)
            .checkpoint()
            .unwrap()
            .retained_bytes(),
        7
    );
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
    assert!(recovery.checkpoint_authority_retained());
    let yielded = match recovery.retry_to_yielded() {
        Ok(
            crate::domain_computation::WorthQueryDirectReadmissionRecoveryRetryOutcome::Yielded(
                yielded,
            ),
        ) => yielded,
        _ => panic!("successfully released replacement should restore yielded authority"),
    };
    complete_direct_yield_cleanup(yielded);
}

#[test]
fn restore_panic_and_replacement_destructor_panic_flow_into_yielded_cleanup() {
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
    assert!(recovery.checkpoint_authority_retained());
    let yielded = match recovery.retry_to_yielded() {
        Ok(
            crate::domain_computation::WorthQueryDirectReadmissionRecoveryRetryOutcome::Yielded(
                yielded,
            ),
        ) => yielded,
        _ => panic!("retained checkpoint recovery did not return yielded authority"),
    };
    let carried_release = yielded
        .provider_work()
        .provider_execution_release()
        .recovery_evidence()
        .cloned()
        .expect("yielded authority must retain replacement release uncertainty");
    assert_eq!(
        carried_release.destructor(),
        crate::domain_computation::WorthQueryProviderExecutionDestructorDisposition::Panicked
    );
    let cleanup = complete_direct_yield_cleanup(yielded);
    assert!(cleanup
        .provider_work()
        .provider_execution_release()
        .recovery_evidence()
        .is_some());
}

#[test]
fn checkpoint_release_panic_reports_exact_non_retryable_physical_posture() {
    let (yielded, bridge, runtime) =
        yielded_direct_with_provider(YieldProvider::checkpoint_drop_panic());
    let checkpoint = yielded.checkpoint().identity().to_owned();
    let recovery = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryDirectReadmissionOutcome::RecoveryRequired(
            recovery,
        ) => recovery,
        _ => panic!("checkpoint release panic must require recovery"),
    };
    assert_eq!(
        recovery.kind(),
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryKind::CheckpointReleasePanicked
    );
    assert_eq!(recovery.checkpoint().identity(), checkpoint);
    assert!(!recovery.checkpoint_authority_retained());
    assert_eq!(
        recovery.checkpoint_release().unwrap().disposition(),
        crate::domain_computation::WorthQueryProviderCheckpointReleaseDisposition::Panicked
    );
    let restored_release = recovery
        .restored_execution_release_evidence()
        .expect("replacement execution was physically released");
    assert_eq!(
        restored_release.disposal(),
        crate::domain_computation::WorthQueryProviderExecutionDisposalDisposition::Completed
    );
    assert_eq!(
        restored_release.destructor(),
        crate::domain_computation::WorthQueryProviderExecutionDestructorDisposition::Completed
    );
    assert!(recovery.bridge_cleanup_pending());
    assert!(recovery.fresh_resource_attempt_pending());
    let recovery = match recovery.retry_to_yielded() {
        Err(recovery) => recovery,
        _ => panic!("released checkpoint recovery must not claim retry safety"),
    };
    assert!(!recovery.checkpoint_authority_retained());
}
