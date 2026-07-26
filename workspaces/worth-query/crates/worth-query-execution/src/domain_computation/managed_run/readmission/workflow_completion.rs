use std::sync::Arc;

use worth_runtime_bridge::facade::RuntimeBridge;

use super::super::provider_restore::WorthQueryManagedGraphRestoreCommitOutcome;
use super::super::run_identity::WorthQueryManagedRunIdentity;
use super::super::{WorthQueryActiveWorkflowGraphExecution, WorthQueryRunningWorkflowRun};
use super::counters::WorthQueryReadmissionCounters;
use super::recovery::WorthQueryWorkflowReadmissionRecoveryRequired;
use super::workflow_abort::{abort_provider_pending, map_recovery_kind};
use super::workflow_outcome::{
    WorthQueryWorkflowReadmissionDenialKind, WorthQueryWorkflowReadmissionOutcome,
};
use super::workflow_state::{
    WorthQueryWorkflowArtifactGenerationPending, WorthQueryWorkflowCommitReady,
    WorthQueryWorkflowProviderAbortPending, WorthQueryWorkflowProviderGenerationRecoveryState,
    WorthQueryWorkflowProviderPendingRecoveryState, WorthQueryWorkflowProviderRecoveryState,
    WorthQueryWorkflowProviderRestorePending,
};
use crate::domain_computation::provider_session::graph_provider::bounded_step::WorthQueryGraphProviderStepArtifactContext;

pub(super) fn advance_artifact_generation(
    pending: WorthQueryWorkflowProviderRestorePending,
    bridge_runtime: &RuntimeBridge,
    mut counters: WorthQueryReadmissionCounters,
) -> WorthQueryWorkflowReadmissionOutcome {
    let WorthQueryWorkflowProviderRestorePending {
        state,
        stage_identity,
        provider,
        resource,
        bridge,
    } = pending;
    counters.attempted_artifact_generation();
    let registry = state.artifacts.registry();
    let generation = match registry.prepare_next_generation() {
        Ok(generation) => generation,
        Err(denial) => {
            return abort_provider_pending(
                WorthQueryWorkflowReadmissionDenialKind::ArtifactGenerationDenied,
                denial.detail(),
                WorthQueryWorkflowProviderAbortPending {
                    state,
                    provider,
                    resource,
                    bridge,
                },
                counters,
            );
        }
    };
    let production = match state
        .artifacts
        .production_authority_for_readmission(&stage_identity, &generation)
    {
        Ok(production) => production,
        Err(denial) => {
            let detail = Arc::from(denial.detail());
            if let Err(generation_rollback) = generation.abort() {
                return WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(
                    WorthQueryWorkflowReadmissionRecoveryRequired::provider_pending(
                        format!(
                            "{detail}; generation rollback failed: {}",
                            generation_rollback.detail()
                        ),
                        counters,
                        WorthQueryWorkflowProviderPendingRecoveryState {
                            state,
                            resource,
                            bridge,
                            provider,
                            generation_rollback,
                        },
                    ),
                );
            }
            return abort_provider_pending(
                WorthQueryWorkflowReadmissionDenialKind::ArtifactAuthorityDenied,
                detail,
                WorthQueryWorkflowProviderAbortPending {
                    state,
                    provider,
                    resource,
                    bridge,
                },
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
    commit_artifact_generation(
        WorthQueryWorkflowArtifactGenerationPending {
            state,
            provider,
            resource,
            bridge,
            generation,
            artifact_context,
        },
        bridge_runtime,
        counters,
    )
}

fn commit_artifact_generation(
    pending: WorthQueryWorkflowArtifactGenerationPending,
    bridge_runtime: &RuntimeBridge,
    mut counters: WorthQueryReadmissionCounters,
) -> WorthQueryWorkflowReadmissionOutcome {
    let WorthQueryWorkflowArtifactGenerationPending {
        state,
        provider,
        resource,
        bridge,
        generation,
        artifact_context,
    } = pending;
    let execution = match provider.commit(artifact_context) {
        WorthQueryManagedGraphRestoreCommitOutcome::Restored(execution) => execution,
        WorthQueryManagedGraphRestoreCommitOutcome::RecoveryRequired(recovery) => {
            let kind = map_recovery_kind(recovery.kind());
            let mut detail = recovery.detail().to_owned();
            if let Err(generation_rollback) = generation.abort() {
                detail.push_str("; generation rollback failed: ");
                detail.push_str(generation_rollback.detail());
                return WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(
                    WorthQueryWorkflowReadmissionRecoveryRequired::provider_generation(
                        detail,
                        counters,
                        WorthQueryWorkflowProviderGenerationRecoveryState {
                            state,
                            resource,
                            bridge,
                            provider: recovery,
                            generation_rollback,
                        },
                    ),
                );
            }
            return WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(
                WorthQueryWorkflowReadmissionRecoveryRequired::provider(
                    kind,
                    detail,
                    counters,
                    WorthQueryWorkflowProviderRecoveryState {
                        state,
                        resource,
                        bridge,
                        provider: recovery,
                    },
                ),
            );
        }
    };
    let committed_generation = generation.commit();
    counters.committed_artifact_generation();
    commit_workflow(
        WorthQueryWorkflowCommitReady {
            state: state.commit_artifact_generation(committed_generation),
            execution,
            resource,
            bridge,
        },
        bridge_runtime,
        counters,
    )
}

fn commit_workflow(
    pending: WorthQueryWorkflowCommitReady,
    bridge_runtime: &RuntimeBridge,
    mut counters: WorthQueryReadmissionCounters,
) -> WorthQueryWorkflowReadmissionOutcome {
    let WorthQueryWorkflowCommitReady {
        state,
        execution,
        resource,
        bridge,
    } = pending;
    let bridge_basis = bridge_runtime.commit_yielded_execution_basis_readmission(bridge);
    let resource_attempt = resource.commit();
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
