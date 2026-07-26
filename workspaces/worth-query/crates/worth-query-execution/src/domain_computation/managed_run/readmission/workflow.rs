use std::sync::Arc;

use worth_runtime_bridge::facade::{
    BridgeExecutionBasisReadmissionOutcome, BridgeManagedExecutionIntent, RuntimeBridge,
};

use super::super::provider_restore::{
    self, WorthQueryManagedGraphRestoreCommitOutcome, WorthQueryManagedGraphRestoreOutcome,
};
use super::super::run_identity::WorthQueryManagedRunIdentity;
use super::super::{
    WorthQueryActiveWorkflowGraphExecution, WorthQueryRunningWorkflowRun,
    WorthQueryYieldedWorkflowRun,
};
use super::counters::WorthQueryReadmissionCounters;
use super::workflow_abort::{abort_provider_pending, abort_without_provider, map_recovery_kind};
use super::workflow_outcome::{
    WorthQueryWorkflowReadmissionDenialKind, WorthQueryWorkflowReadmissionDenied,
    WorthQueryWorkflowReadmissionOutcome, WorthQueryWorkflowReadmissionRecoveryRequired,
};
use super::workflow_preflight::query_preflight_denial;
use super::workflow_state::{WorthQueryWorkflowYieldedParts, WorthQueryWorkflowYieldedState};
use crate::domain_computation::provider_session::graph_provider::bounded_step::WorthQueryGraphProviderStepArtifactContext;
use crate::domain_computation::provider_session::readmission::WorthQueryWorkflowResourceReadmissionPending;
use crate::domain_computation::WorthQueryExecutionRuntime;

pub(in crate::domain_computation::managed_run) fn readmit_workflow(
    yielded: WorthQueryYieldedWorkflowRun,
    query_runtime: &WorthQueryExecutionRuntime,
    bridge_runtime: &RuntimeBridge,
) -> WorthQueryWorkflowReadmissionOutcome {
    let mut counters = WorthQueryReadmissionCounters::default();
    counters.checked_preflight();
    if let Some((kind, detail)) = query_preflight_denial(&yielded, query_runtime) {
        return denied(kind, detail, yielded, counters);
    }
    let parts = WorthQueryWorkflowYieldedParts::from_yielded(yielded);
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
                    WorthQueryWorkflowReadmissionDenialKind::BridgeReadmissionDenied,
                    detail,
                    WorthQueryWorkflowYieldedParts {
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
        WorthQueryWorkflowResourceReadmissionPending::begin(parts.resource_attempt);
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
                    WorthQueryWorkflowReadmissionDenialKind::BridgeReadmissionDenied,
                    detail,
                    WorthQueryWorkflowYieldedParts {
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
                return WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(
                    WorthQueryWorkflowReadmissionRecoveryRequired::bridge_cleanup(
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
    restore_workflow(
        parts.state,
        parts.execution,
        resource_pending,
        bridge_pending,
        counters,
    )
}

fn restore_workflow(
    state: WorthQueryWorkflowYieldedState,
    execution: super::super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution,
    resource_pending: WorthQueryWorkflowResourceReadmissionPending,
    bridge_pending: worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionPending,
    mut counters: WorthQueryReadmissionCounters,
) -> WorthQueryWorkflowReadmissionOutcome {
    let Some(stage_identity) = execution.call.stage_identity().map(str::to_owned) else {
        return abort_without_provider(
            WorthQueryWorkflowReadmissionDenialKind::WorkflowStageResourcesUnavailable,
            "retained workflow provider call has no stage identity",
            state,
            execution,
            resource_pending,
            bridge_pending,
            counters,
        );
    };
    let Some((_resources, stage_evidence)) =
        resource_pending.stage_resources_and_evidence(&stage_identity)
    else {
        return abort_without_provider(
            WorthQueryWorkflowReadmissionDenialKind::WorkflowStageResourcesUnavailable,
            "fresh workflow attempt has no resources for the retained stage",
            state,
            execution,
            resource_pending,
            bridge_pending,
            counters,
        );
    };
    let contract = match super::super::step_contract_admission::admit_managed_step_contract(
        execution.contract().clone(),
        bridge_pending.step_contract(),
    ) {
        Ok(contract) => contract,
        Err(denial) => {
            return abort_without_provider(
                WorthQueryWorkflowReadmissionDenialKind::ProviderStepContractDenied(denial.kind()),
                denial.detail(),
                state,
                execution,
                resource_pending,
                bridge_pending,
                counters,
            )
        }
    };
    let fresh_call = match execution
        .call
        .remint_for_readmission(resource_pending.provider_session(), &stage_evidence)
    {
        Ok(call) => call,
        Err(denial) => {
            return abort_without_provider(
                WorthQueryWorkflowReadmissionDenialKind::ProviderCallBindingDenied,
                format!("workflow provider call readmission denied: {denial:?}"),
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
            return abort_without_provider(
                WorthQueryWorkflowReadmissionDenialKind::ProviderRestoreDenied,
                detail,
                state,
                denial.into_retained(),
                resource_pending,
                bridge_pending,
                counters,
            );
        }
        WorthQueryManagedGraphRestoreOutcome::RecoveryRequired(recovery) => {
            let kind = map_recovery_kind(recovery.kind());
            let detail = Arc::from(recovery.detail());
            return WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(
                WorthQueryWorkflowReadmissionRecoveryRequired::provider(
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
    advance_artifact_generation(
        state,
        stage_identity,
        provider_pending,
        resource_pending,
        bridge_pending,
        counters,
    )
}

fn advance_artifact_generation(
    state: WorthQueryWorkflowYieldedState,
    stage_identity: String,
    provider_pending: super::super::provider_restore::WorthQueryManagedGraphRestorePending,
    resource_pending: WorthQueryWorkflowResourceReadmissionPending,
    bridge_pending: worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionPending,
    mut counters: WorthQueryReadmissionCounters,
) -> WorthQueryWorkflowReadmissionOutcome {
    counters.attempted_artifact_generation();
    let registry = state.artifacts.registry();
    let generation_pending = match registry.prepare_next_generation() {
        Ok(pending) => pending,
        Err(denial) => {
            return abort_provider_pending(
                WorthQueryWorkflowReadmissionDenialKind::ArtifactGenerationDenied,
                denial.detail(),
                state,
                provider_pending,
                resource_pending,
                bridge_pending,
                counters,
            );
        }
    };
    let production = match state
        .artifacts
        .production_authority_for_readmission(&stage_identity, &generation_pending)
    {
        Ok(production) => production,
        Err(denial) => {
            let detail = Arc::from(denial.detail());
            if let Err(abort) = generation_pending.abort() {
                return WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(
                    WorthQueryWorkflowReadmissionRecoveryRequired::provider_pending(
                        format!("{detail}; generation rollback failed: {}", abort.detail()),
                        counters,
                        state,
                        resource_pending,
                        bridge_pending,
                        provider_pending,
                    ),
                );
            }
            return abort_provider_pending(
                WorthQueryWorkflowReadmissionDenialKind::ArtifactAuthorityDenied,
                detail,
                state,
                provider_pending,
                resource_pending,
                bridge_pending,
                counters,
            );
        }
    };
    let artifact_context = production.map(|authority| {
        WorthQueryGraphProviderStepArtifactContext::new(
            authority,
            Arc::clone(&state.provider_artifact_occurrences),
        )
    });
    let execution = match provider_pending.commit(artifact_context) {
        WorthQueryManagedGraphRestoreCommitOutcome::Restored(execution) => execution,
        WorthQueryManagedGraphRestoreCommitOutcome::RecoveryRequired(recovery) => {
            let kind = map_recovery_kind(recovery.kind());
            let mut detail = recovery.detail().to_owned();
            if let Err(abort) = generation_pending.abort() {
                detail.push_str("; generation rollback failed: ");
                detail.push_str(abort.detail());
            }
            return WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(
                WorthQueryWorkflowReadmissionRecoveryRequired::provider(
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
    generation_pending.commit();
    counters.committed_artifact_generation();
    commit_workflow(state, execution, resource_pending, bridge_pending, counters)
}

fn commit_workflow(
    state: WorthQueryWorkflowYieldedState,
    execution: super::super::managed_graph_execution::WorthQueryManagedGraphExecution,
    resource_pending: WorthQueryWorkflowResourceReadmissionPending,
    bridge_pending: worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionPending,
    mut counters: WorthQueryReadmissionCounters,
) -> WorthQueryWorkflowReadmissionOutcome {
    let bridge_basis = bridge_pending.commit();
    let resource_attempt = resource_pending.commit();
    let identity = WorthQueryManagedRunIdentity::resumed(
        "workflow",
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
    WorthQueryWorkflowReadmissionOutcome::Readmitted(WorthQueryActiveWorkflowGraphExecution::new(
        WorthQueryRunningWorkflowRun {
            logical_run_identity,
            identity,
            resource_attempt,
            bridge_basis,
            relational_basis: state.relational_basis,
            counters: state.run_counters,
            artifacts: state.artifacts,
            provider_work,
            provider_artifact_occurrences: state.provider_artifact_occurrences,
        },
        execution,
    ))
}

fn denied(
    kind: WorthQueryWorkflowReadmissionDenialKind,
    detail: impl Into<Arc<str>>,
    yielded: WorthQueryYieldedWorkflowRun,
    counters: WorthQueryReadmissionCounters,
) -> WorthQueryWorkflowReadmissionOutcome {
    WorthQueryWorkflowReadmissionOutcome::Denied(WorthQueryWorkflowReadmissionDenied::new(
        kind, detail, yielded, counters,
    ))
}
