use std::sync::Arc;

use worth_runtime_bridge::facade::{
    BridgeExecutionBasisReadmissionCleanupOutcome, BridgeExecutionBasisReadmissionOutcome,
    BridgeManagedExecutionIntent, RuntimeBridge,
};

use super::super::provider_restore::{
    self, WorthQueryManagedGraphRestoreCommitOutcome, WorthQueryManagedGraphRestoreOutcome,
    WorthQueryManagedGraphRestoreRecoveryKind,
};
use super::super::run_identity::WorthQueryManagedRunIdentity;
use super::super::{
    WorthQueryActiveDirectGraphExecution, WorthQueryRunningDirectRun, WorthQueryYieldedDirectRun,
};
use super::counters::WorthQueryReadmissionCounters;
use super::direct_outcome::{
    WorthQueryDirectReadmissionDenialKind, WorthQueryDirectReadmissionDenied,
    WorthQueryDirectReadmissionOutcome, WorthQueryDirectReadmissionRecoveryKind,
    WorthQueryDirectReadmissionRecoveryRequired,
};
use super::direct_state::{WorthQueryDirectYieldedParts, WorthQueryDirectYieldedState};
use crate::domain_computation::provider_session::readmission::WorthQueryDirectResourceReadmissionPending;
use crate::domain_computation::WorthQueryExecutionRuntime;

pub(in crate::domain_computation::managed_run) fn readmit_direct(
    yielded: WorthQueryYieldedDirectRun,
    query_runtime: &WorthQueryExecutionRuntime,
    bridge_runtime: &RuntimeBridge,
) -> WorthQueryDirectReadmissionOutcome {
    let mut counters = WorthQueryReadmissionCounters::default();
    counters.checked_preflight();
    if let Some((kind, detail)) = query_preflight_denial(&yielded, query_runtime) {
        return denied(kind, detail, yielded, counters);
    }
    let parts = WorthQueryDirectYieldedParts::from_yielded(yielded);
    let binding_identity = parts
        .resource_attempt
        .binding_authority()
        .binding_identity()
        .to_owned();
    let bridge_preflight =
        match bridge_runtime.preflight_yielded_execution_basis(parts.bridge, &binding_identity) {
            Ok(preflight) => preflight,
            Err(denial) => {
                let detail = Arc::from(denial.detail());
                return denied(
                    WorthQueryDirectReadmissionDenialKind::BridgeReadmissionDenied,
                    detail,
                    WorthQueryDirectYieldedParts {
                        state: parts.state,
                        resource_attempt: parts.resource_attempt,
                        bridge: denial.into_yielded(),
                        execution: parts.execution,
                    }
                    .into_yielded(),
                    counters,
                );
            }
        };
    let resource_pending =
        WorthQueryDirectResourceReadmissionPending::begin(parts.resource_attempt);
    counters.minted_fresh_resource_attempt();
    let intent = BridgeManagedExecutionIntent::new(
        binding_identity,
        resource_pending.attempt_identity().as_str(),
    );
    counters.attempted_bridge_readmission();
    let bridge_pending =
        match bridge_runtime.readmit_yielded_execution_basis(bridge_preflight, intent) {
            BridgeExecutionBasisReadmissionOutcome::Pending(pending) => pending,
            BridgeExecutionBasisReadmissionOutcome::Denied(denial) => {
                let detail = Arc::from(denial.detail());
                return denied(
                    WorthQueryDirectReadmissionDenialKind::BridgeReadmissionDenied,
                    detail,
                    WorthQueryDirectYieldedParts {
                        state: parts.state,
                        resource_attempt: resource_pending.abort(),
                        bridge: denial.into_yielded(),
                        execution: parts.execution,
                    }
                    .into_yielded(),
                    counters,
                );
            }
            BridgeExecutionBasisReadmissionOutcome::RecoveryRequired(recovery) => {
                let detail = Arc::from(recovery.detail());
                return WorthQueryDirectReadmissionOutcome::RecoveryRequired(
                    WorthQueryDirectReadmissionRecoveryRequired::bridge_cleanup(
                        detail,
                        counters,
                        parts.state,
                        resource_pending.abort(),
                        parts.execution,
                        recovery,
                    ),
                );
            }
        };
    restore_direct(
        parts.state,
        parts.execution,
        resource_pending,
        bridge_pending,
        counters,
    )
}

fn restore_direct(
    state: WorthQueryDirectYieldedState,
    execution: super::super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution,
    resource_pending: WorthQueryDirectResourceReadmissionPending,
    bridge_pending: worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionPending,
    mut counters: WorthQueryReadmissionCounters,
) -> WorthQueryDirectReadmissionOutcome {
    let contract = match super::super::step_contract_admission::admit_managed_step_contract(
        execution.contract().clone(),
        bridge_pending.step_contract(),
    ) {
        Ok(contract) => contract,
        Err(denial) => {
            return abort_to_denial(
                WorthQueryDirectReadmissionDenialKind::ProviderStepContractDenied(denial.kind()),
                denial.detail(),
                state,
                execution,
                resource_pending,
                bridge_pending,
                counters,
            )
        }
    };
    let fresh_call = match execution.call.remint_for_readmission(
        resource_pending.provider_session(),
        resource_pending.evidence(),
    ) {
        Ok(call) => call,
        Err(denial) => {
            return abort_to_denial(
                WorthQueryDirectReadmissionDenialKind::ProviderCallBindingDenied,
                format!("provider call readmission denied: {denial:?}"),
                state,
                execution,
                resource_pending,
                bridge_pending,
                counters,
            );
        }
    };
    counters.attempted_provider_restore();
    let provider_pending = match provider_restore::restore(execution, fresh_call, contract) {
        WorthQueryManagedGraphRestoreOutcome::Pending(pending) => pending,
        WorthQueryManagedGraphRestoreOutcome::Denied(denial) => {
            let detail = Arc::from(denial.detail());
            return abort_to_denial(
                WorthQueryDirectReadmissionDenialKind::ProviderRestoreDenied,
                detail,
                state,
                denial.into_retained(),
                resource_pending,
                bridge_pending,
                counters,
            );
        }
        WorthQueryManagedGraphRestoreOutcome::RecoveryRequired(recovery) => {
            let kind = recovery_kind(recovery.kind());
            let detail = Arc::from(recovery.detail());
            return WorthQueryDirectReadmissionOutcome::RecoveryRequired(
                WorthQueryDirectReadmissionRecoveryRequired::provider(
                    kind,
                    detail,
                    counters,
                    state,
                    resource_pending,
                    bridge_pending,
                    recovery,
                ),
            );
        }
    };
    let execution = match provider_pending.commit(None) {
        WorthQueryManagedGraphRestoreCommitOutcome::Restored(execution) => execution,
        WorthQueryManagedGraphRestoreCommitOutcome::RecoveryRequired(recovery) => {
            let kind = recovery_kind(recovery.kind());
            let detail = Arc::from(recovery.detail());
            return WorthQueryDirectReadmissionOutcome::RecoveryRequired(
                WorthQueryDirectReadmissionRecoveryRequired::provider(
                    kind,
                    detail,
                    counters,
                    state,
                    resource_pending,
                    bridge_pending,
                    recovery,
                ),
            );
        }
    };
    let bridge_basis = bridge_pending.commit();
    let resource_attempt = resource_pending.commit();
    let identity = WorthQueryManagedRunIdentity::resumed(
        "direct",
        Arc::clone(&state.logical_run_identity),
        resource_attempt.attempt_identity().as_str(),
        &bridge_basis,
        &state.relational_basis,
    );
    let (logical_run_identity, identity) = identity.into_parts();
    let provider_work = state
        .provider_work
        .rebind_provider_session(resource_attempt.provider_session().identity());
    counters.committed_attempt();
    WorthQueryDirectReadmissionOutcome::Readmitted(WorthQueryActiveDirectGraphExecution::new(
        WorthQueryRunningDirectRun {
            logical_run_identity,
            identity,
            resource_attempt,
            bridge_basis,
            relational_basis: state.relational_basis,
            counters: state.run_counters,
            provider_work,
        },
        execution,
    ))
}

fn abort_to_denial(
    kind: WorthQueryDirectReadmissionDenialKind,
    detail: impl Into<Arc<str>>,
    state: WorthQueryDirectYieldedState,
    execution: super::super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution,
    resource_pending: WorthQueryDirectResourceReadmissionPending,
    bridge_pending: worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionPending,
    counters: WorthQueryReadmissionCounters,
) -> WorthQueryDirectReadmissionOutcome {
    let detail = detail.into();
    match bridge_pending.abort() {
        BridgeExecutionBasisReadmissionCleanupOutcome::Complete(bridge) => denied(
            kind,
            detail,
            WorthQueryDirectYieldedParts {
                state,
                resource_attempt: resource_pending.abort(),
                bridge,
                execution,
            }
            .into_yielded(),
            counters,
        ),
        BridgeExecutionBasisReadmissionCleanupOutcome::RecoveryRequired(recovery) => {
            WorthQueryDirectReadmissionOutcome::RecoveryRequired(
                WorthQueryDirectReadmissionRecoveryRequired::bridge_cleanup(
                    format!("{detail}; Bridge cleanup failed: {}", recovery.detail()),
                    counters,
                    state,
                    resource_pending.abort(),
                    execution,
                    recovery,
                ),
            )
        }
    }
}

fn query_preflight_denial(
    yielded: &WorthQueryYieldedDirectRun,
    runtime: &WorthQueryExecutionRuntime,
) -> Option<(WorthQueryDirectReadmissionDenialKind, &'static str)> {
    let operation = yielded.resource_attempt.binding_authority();
    if !operation.belongs_to(runtime) {
        return Some((
            WorthQueryDirectReadmissionDenialKind::ForeignQueryRuntime,
            "yielded run belongs to a different Query execution runtime",
        ));
    }
    if !operation.belongs_to_current_installation(runtime) {
        return Some((
            WorthQueryDirectReadmissionDenialKind::StaleInstallationGeneration,
            "yielded run belongs to a stale installed-operation generation",
        ));
    }
    if yielded
        .resource_attempt
        .retained_capacity_reservation_count()
        == 0
    {
        return Some((
            WorthQueryDirectReadmissionDenialKind::RetainedCapacityMismatch,
            "yielded run no longer owns its nonempty capacity-reservation package",
        ));
    }
    if !yielded.relational_basis.is_live() {
        return Some((
            WorthQueryDirectReadmissionDenialKind::RelationalLeaseNotLive,
            "yielded Relational execution-basis lease is no longer live",
        ));
    }
    if !yielded.execution.provider_generation_matches_anchor() {
        return Some((
            WorthQueryDirectReadmissionDenialKind::ProviderCheckpointMismatch,
            "provider checkpoint generation no longer matches its retained provider anchor",
        ));
    }
    None
}

fn denied(
    kind: WorthQueryDirectReadmissionDenialKind,
    detail: impl Into<Arc<str>>,
    yielded: WorthQueryYieldedDirectRun,
    counters: WorthQueryReadmissionCounters,
) -> WorthQueryDirectReadmissionOutcome {
    WorthQueryDirectReadmissionOutcome::Denied(WorthQueryDirectReadmissionDenied::new(
        kind, detail, yielded, counters,
    ))
}

fn recovery_kind(
    kind: WorthQueryManagedGraphRestoreRecoveryKind,
) -> WorthQueryDirectReadmissionRecoveryKind {
    match kind {
        WorthQueryManagedGraphRestoreRecoveryKind::ProviderRestorePanicked => {
            WorthQueryDirectReadmissionRecoveryKind::ProviderRestorePanicked
        }
        WorthQueryManagedGraphRestoreRecoveryKind::RestoredExecutionReleaseRecoveryRequired => {
            WorthQueryDirectReadmissionRecoveryKind::RestoredExecutionReleaseRecoveryRequired
        }
        WorthQueryManagedGraphRestoreRecoveryKind::CheckpointReleasePanicked => {
            WorthQueryDirectReadmissionRecoveryKind::CheckpointReleasePanicked
        }
    }
}
