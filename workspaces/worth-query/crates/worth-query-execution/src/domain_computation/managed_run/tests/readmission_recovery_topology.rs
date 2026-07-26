use super::yield_fixture::YieldProvider;
use super::*;
use crate::domain_computation::provider_session::readmission::WorthQueryDirectResourceReadmissionPending;
use crate::domain_computation::WorthQueryYieldedDirectRun;

#[test]
fn query_bridge_cleanup_failure_returns_exact_owner_retry_authority() {
    let (yielded, bridge, runtime) = super::readmission_direct::yielded_direct_with_provider(
        YieldProvider::checkpoint_restore_failure(7),
    );
    let checkpoint = yielded.checkpoint().identity().to_owned();
    let (pending, counters) = match super::super::readmission::prepare_direct_provider_restore(
        yielded, &runtime, &bridge,
    ) {
        Ok(prepared) => prepared,
        Err(_) => panic!("owner-thread Query phases should reach provider restore"),
    };
    let recovery = std::thread::spawn(move || {
        match super::super::readmission::restore_direct(pending, &bridge, counters) {
            crate::domain_computation::WorthQueryDirectReadmissionOutcome::RecoveryRequired(
                recovery,
            ) => recovery,
            _ => panic!("foreign-thread Query rollback must retain Bridge cleanup authority"),
        }
    })
    .join()
    .expect("Query bridge cleanup recovery must remain in-process");

    assert_eq!(
        recovery.kind(),
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryKind::BridgeCleanupFailed
    );
    assert!(recovery.detail().contains("belongs to thread"));
    assert_eq!(
        recovery.posture(),
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryPosture::
            YieldReassemblyPending
    );
    let yielded = match recovery.retry_to_yielded() {
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryRetryOutcome::Yielded(
            yielded,
        ) => yielded,
        _ => panic!("Signal owner thread must reconstruct the exact yielded Query authority"),
    };
    assert_eq!(yielded.checkpoint().identity(), checkpoint);
    let cleanup = complete_direct_yield_cleanup(yielded);
    assert!(cleanup.bridge().reservation_released());
    assert!(cleanup.relational().released());
    assert_eq!(cleanup.attempt().capacity().released_reservation_count(), 2);
}

#[test]
fn checkpoint_release_panic_reports_exact_non_retryable_physical_posture() {
    let (yielded, bridge, runtime) = super::readmission_direct::yielded_direct_with_provider(
        YieldProvider::checkpoint_drop_panic(),
    );
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
    assert_eq!(
        recovery.posture(),
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryPosture::
            TerminalCleanupRequired
    );
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
    let cleanup = match recovery.retry_to_yielded() {
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryRetryOutcome::
            CleanupRequired(cleanup) => cleanup,
        _ => panic!("released checkpoint recovery must become cleanup-only authority"),
    };
    let receipt = match cleanup.finish() {
        crate::domain_computation::WorthQueryDirectReadmissionCleanupOutcome::RecoveryRequired(
            receipt,
        ) => receipt,
        _ => panic!("checkpoint release panic must remain visible after terminal cleanup"),
    };
    assert_eq!(
        receipt.checkpoint_release().disposition(),
        crate::domain_computation::WorthQueryProviderCheckpointReleaseDisposition::Panicked
    );
    assert!(receipt.bridge().reservation_released());
    assert!(receipt.relational().released());
    assert_eq!(receipt.attempt().capacity().released_reservation_count(), 2);
}

#[test]
fn workflow_checkpoint_failure_and_generation_disruption_become_cleanup_only_authority() {
    let (provider, registry_slot) = YieldProvider::artifact_generation_rollback_failure(7);
    let (yielded, bridge, runtime, _producer) =
        super::readmission_workflow::yielded_workflow(provider);
    let generation = yielded.artifact_evidence().production_generation();
    let registry = yielded.artifacts.registry();
    *registry_slot
        .lock()
        .expect("workflow rollback fixture registry slot must remain available") = Some(registry);

    let recovery = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(
            recovery,
        ) => recovery,
        _ => panic!("coupled checkpoint and generation failure must require recovery"),
    };
    assert_eq!(
        recovery.kind(),
        crate::domain_computation::WorthQueryWorkflowReadmissionRecoveryKind::
            ArtifactGenerationRollbackFailed
    );
    assert_eq!(
        recovery.posture(),
        crate::domain_computation::WorthQueryWorkflowReadmissionRecoveryPosture::
            ArtifactGenerationCleanupRequired
    );
    assert_eq!(
        recovery
            .checkpoint_release()
            .expect("provider checkpoint release evidence must survive")
            .disposition(),
        crate::domain_computation::WorthQueryProviderCheckpointReleaseDisposition::Panicked
    );

    let cleanup = match recovery.retry_to_yielded() {
        crate::domain_computation::WorthQueryWorkflowReadmissionRecoveryRetryOutcome::
            CleanupRequired(cleanup) => cleanup,
        _ => panic!("failed generation rollback must make readmission retry unavailable"),
    };
    let receipt = match cleanup.finish() {
        crate::domain_computation::WorthQueryWorkflowReadmissionCleanupOutcome::RecoveryRequired(
            receipt,
        ) => receipt,
        _ => panic!("typed generation and provider recovery evidence must survive cleanup"),
    };
    let rollback = receipt
        .generation_rollback()
        .expect("cleanup receipt must carry typed generation rollback evidence");
    assert_eq!(rollback.prior_generation(), generation);
    assert_eq!(rollback.pending_generation(), generation + 1);
    assert_eq!(
        rollback.denial_kind(),
        crate::domain_computation::WorthQueryArtifactDenialKind::StaleLifecycleGeneration
    );
    assert_eq!(
        receipt.checkpoint_release().disposition(),
        crate::domain_computation::WorthQueryProviderCheckpointReleaseDisposition::Panicked
    );
    assert!(receipt.bridge().reservation_released());
    assert!(receipt.relational().released());
    assert_eq!(receipt.attempt().capacity().released_reservation_count(), 3);
    assert_eq!(receipt.artifact_evidence().retained_artifact_count(), 0);
}

#[test]
fn restored_execution_abort_panic_preserves_the_checkpoint_and_exact_release_posture() {
    let (yielded, _bridge, _runtime) = super::readmission_direct::yielded_direct_with_provider(
        YieldProvider::restored_execution_drop_panic(7),
    );
    let WorthQueryYieldedDirectRun {
        logical_run_identity,
        attempt_identity,
        resource_attempt,
        relational_basis,
        bridge,
        execution,
        run_counters,
        mut provider_work,
        yield_counters,
    } = yielded;
    let contract = super::super::step_contract_admission::admit_managed_step_contract(
        execution.contract().clone(),
        bridge.step_contract(),
    )
    .unwrap_or_else(|denial| {
        panic!(
            "causal yielded provider contract was not admitted: {:?}",
            denial.kind()
        )
    });
    let resource_pending = WorthQueryDirectResourceReadmissionPending::begin(resource_attempt);
    let fresh_call = execution
        .call
        .remint_for_readmission(
            resource_pending.provider_session(),
            resource_pending.evidence(),
        )
        .expect("retained provider call should remint for its fresh Query attempt");
    let pending = match super::super::provider_restore::restore(execution, fresh_call, contract) {
        super::super::provider_restore::WorthQueryManagedGraphRestoreOutcome::Pending(pending) => {
            pending
        }
        _ => panic!("provider should restore before the injected release failure"),
    };
    let recovery = match pending.abort() {
        super::super::provider_restore::WorthQueryManagedGraphRestoreAbortOutcome::RecoveryRequired(
            recovery,
        ) => recovery,
        _ => panic!("restored execution destructor panic must require recovery"),
    };
    assert_eq!(
        recovery.kind(),
        super::super::provider_restore::WorthQueryManagedGraphRestoreRecoveryKind::RestoredExecutionReleaseRecoveryRequired
    );
    assert!(recovery.checkpoint_release().is_none());
    let restored_release = recovery
        .restored_execution_release_evidence()
        .expect("replacement execution release evidence must remain available");
    assert_eq!(
        restored_release.disposal(),
        crate::domain_computation::WorthQueryProviderExecutionDisposalDisposition::Completed
    );
    assert_eq!(
        restored_release.destructor(),
        crate::domain_computation::WorthQueryProviderExecutionDestructorDisposition::Panicked
    );
    let retryable = match recovery.retry_or_cleanup() {
        super::super::provider_restore::WorthQueryManagedGraphRestoreRecoveryRetryOutcome::
            Retryable(retryable) => retryable,
        _ => panic!("restored-execution release failure must retain the provider checkpoint"),
    };
    provider_work.record_provider_execution_release(
        retryable
            .restored_execution_release
            .as_ref()
            .expect("retryable recovery carries replacement release evidence"),
    );
    let yielded = WorthQueryYieldedDirectRun {
        logical_run_identity,
        attempt_identity,
        resource_attempt: resource_pending.abort(),
        relational_basis,
        bridge,
        execution: retryable.retained,
        run_counters,
        provider_work,
        yield_counters,
    };
    let cleanup = complete_direct_yield_cleanup(yielded);
    assert!(cleanup
        .provider_work()
        .provider_execution_release()
        .recovery_evidence()
        .is_some());
    assert_eq!(
        cleanup
            .checkpoint()
            .expect("retained checkpoint should release during yielded cleanup")
            .retained_bytes(),
        7
    );
}

#[test]
fn checkpoint_and_restored_execution_drop_panics_preserve_both_physical_dispositions() {
    let (yielded, bridge, runtime) = super::readmission_direct::yielded_direct_with_provider(
        YieldProvider::checkpoint_and_restored_execution_drop_panic(7),
    );
    let recovery = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryDirectReadmissionOutcome::RecoveryRequired(
            recovery,
        ) => recovery,
        _ => panic!("causal checkpoint and replacement cleanup panics must require recovery"),
    };
    assert_eq!(
        recovery.kind(),
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryKind::CheckpointReleasePanicked
    );
    assert_eq!(
        recovery.posture(),
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryPosture::
            TerminalCleanupRequired
    );
    assert_eq!(
        recovery
            .checkpoint_release()
            .expect("released checkpoint must expose its disposition")
            .disposition(),
        crate::domain_computation::WorthQueryProviderCheckpointReleaseDisposition::Panicked
    );
    let restored_release = recovery
        .restored_execution_release_evidence()
        .expect("replacement execution release evidence must remain available");
    assert_eq!(
        restored_release.disposal(),
        crate::domain_computation::WorthQueryProviderExecutionDisposalDisposition::Completed
    );
    assert_eq!(
        restored_release.destructor(),
        crate::domain_computation::WorthQueryProviderExecutionDestructorDisposition::Panicked
    );
    let cleanup = match recovery.retry_to_yielded() {
        crate::domain_computation::WorthQueryDirectReadmissionRecoveryRetryOutcome::
            CleanupRequired(cleanup) => cleanup,
        _ => panic!(
            "released checkpoint cannot become retry-safe even when replacement cleanup also panicked"
        ),
    };
    let receipt = match cleanup.finish() {
        crate::domain_computation::WorthQueryDirectReadmissionCleanupOutcome::RecoveryRequired(
            receipt,
        ) => receipt,
        _ => panic!("both physical failures must survive complete terminal cleanup"),
    };
    assert_eq!(
        receipt.checkpoint_release().disposition(),
        crate::domain_computation::WorthQueryProviderCheckpointReleaseDisposition::Panicked
    );
    assert!(receipt
        .restored_execution_release()
        .expect("restored execution release evidence must remain attached")
        .recovery_required());
    assert!(receipt.relational().released());
    assert_eq!(receipt.attempt().capacity().released_reservation_count(), 2);
}
