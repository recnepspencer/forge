use super::yield_fixture::YieldProvider;
use super::*;
use crate::domain_computation::provider_session::readmission::WorthQueryDirectResourceReadmissionPending;
use crate::domain_computation::WorthQueryYieldedDirectRun;

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
    assert!(recovery.checkpoint_retained());
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
    let retryable = match recovery.into_retryable() {
        Ok(retryable) => retryable,
        Err(_) => panic!("restored-execution release failure must retain the provider checkpoint"),
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
    assert!(!recovery.checkpoint_authority_retained());
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
    assert!(
        recovery.retry_to_yielded().is_err(),
        "released checkpoint cannot become retry-safe even when replacement cleanup also panicked"
    );
}
