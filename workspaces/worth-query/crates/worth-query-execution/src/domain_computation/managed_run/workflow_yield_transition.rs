use std::sync::Arc;

use worth_relational::facade::runtime::RelationalExecutionBasisLease;
use worth_runtime_bridge::facade::{
    BridgeBoundExecutionBasis, BridgeExecutionBasisSignalTerminal, BridgeYieldedExecutionBasis,
};

use super::managed_graph_execution::WorthQueryManagedGraphExecution;
use super::provider_work::WorthQueryManagedProviderWorkLedger;
use super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution;
use super::workflow_yield_eligibility::{
    classify_workflow_yield_denial, freeze_and_classify_workflow_artifact_retention,
};
use super::workflow_yield_recovery::{
    running_recovery, terminalized_recovery, WorthQueryTerminalizedWorkflowYieldRecovery,
};
use super::yield_eligibility::WorthQueryManagedYieldSafePoint;
use super::{
    WorthQueryManagedRunCounters, WorthQueryPausedWorkflowGraphExecution,
    WorthQueryRunningWorkflowRun, WorthQueryWorkflowYieldDenied, WorthQueryWorkflowYieldOutcome,
    WorthQueryYieldRecoveryKind, WorthQueryYieldRecoveryResourceEvidence,
    WorthQueryYieldTransitionCounters, WorthQueryYieldedWorkflowRun,
};
use crate::domain_computation::artifact_owner::{
    WorthQueryArtifactOccurrenceLedger, WorthQueryWorkflowArtifactAuthority,
};
use crate::domain_computation::WorthQueryWorkflowExecutionResourceAttempt;

pub(super) fn yield_workflow_run(
    paused: WorthQueryPausedWorkflowGraphExecution,
) -> WorthQueryWorkflowYieldOutcome {
    let mut counters = WorthQueryYieldTransitionCounters::default();
    counters.classified_eligibility();
    if let Some((kind, detail)) = classify_workflow_yield_denial(&paused) {
        return WorthQueryWorkflowYieldOutcome::Denied(WorthQueryWorkflowYieldDenied {
            kind,
            detail: Arc::from(detail),
            paused,
            counters,
        });
    }
    if let Some((kind, detail)) =
        freeze_and_classify_workflow_artifact_retention(&paused, &mut counters)
    {
        return WorthQueryWorkflowYieldOutcome::Denied(WorthQueryWorkflowYieldDenied {
            kind,
            detail: Arc::from(detail),
            paused,
            counters,
        });
    }
    let bridge_pending = WorthQueryWorkflowYieldBridgePending::from_paused(paused, counters);
    let checkpoint_pending = match bridge_pending.finalize_bridge() {
        Ok(pending) => pending,
        Err(outcome) => return outcome,
    };
    let retained = match checkpoint_pending.suspend_checkpoint() {
        Ok(retained) => retained,
        Err(outcome) => return outcome,
    };
    retained.validate_and_mint()
}

struct WorthQueryWorkflowYieldBridgePending {
    logical_run_identity: Arc<str>,
    attempt_identity: Arc<str>,
    resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    bridge_basis: BridgeBoundExecutionBasis,
    relational_basis: RelationalExecutionBasisLease,
    run_counters: WorthQueryManagedRunCounters,
    artifacts: WorthQueryWorkflowArtifactAuthority,
    provider_work: WorthQueryManagedProviderWorkLedger,
    provider_artifact_occurrences: Arc<WorthQueryArtifactOccurrenceLedger>,
    execution: WorthQueryManagedGraphExecution,
    safe_point: WorthQueryManagedYieldSafePoint,
    yield_counters: WorthQueryYieldTransitionCounters,
}

impl WorthQueryWorkflowYieldBridgePending {
    fn from_paused(
        paused: WorthQueryPausedWorkflowGraphExecution,
        yield_counters: WorthQueryYieldTransitionCounters,
    ) -> Self {
        let WorthQueryPausedWorkflowGraphExecution { active, safe_point } = paused;
        let super::WorthQueryActiveWorkflowGraphExecution { running, execution } = active;
        let WorthQueryRunningWorkflowRun {
            logical_run_identity,
            identity,
            resource_attempt,
            bridge_basis,
            relational_basis,
            counters,
            artifacts,
            provider_work,
            provider_artifact_occurrences,
        } = running;
        Self {
            logical_run_identity,
            attempt_identity: identity,
            resource_attempt,
            bridge_basis,
            relational_basis,
            run_counters: counters,
            artifacts,
            provider_work,
            provider_artifact_occurrences,
            execution,
            safe_point,
            yield_counters,
        }
    }

    fn finalize_bridge(
        mut self,
    ) -> Result<WorthQueryWorkflowYieldCheckpointPending, WorthQueryWorkflowYieldOutcome> {
        self.yield_counters.attempted_bridge_finalization();
        let bridge = match self.bridge_basis.yield_execution_basis() {
            Ok(receipt) => receipt,
            Err(failure) => {
                let kind = failure.kind();
                let detail = Arc::from(failure.detail());
                let running = WorthQueryRunningWorkflowRun {
                    logical_run_identity: self.logical_run_identity,
                    identity: self.attempt_identity,
                    resource_attempt: self.resource_attempt,
                    bridge_basis: failure.into_basis(),
                    relational_basis: self.relational_basis,
                    counters: self.run_counters,
                    artifacts: self.artifacts,
                    provider_work: self.provider_work,
                    provider_artifact_occurrences: self.provider_artifact_occurrences,
                };
                return Err(running_recovery(
                    WorthQueryYieldRecoveryKind::BridgeTerminalization(kind),
                    detail,
                    self.yield_counters,
                    WorthQueryPausedWorkflowGraphExecution {
                        active: super::WorthQueryActiveWorkflowGraphExecution {
                            running,
                            execution: self.execution,
                        },
                        safe_point: self.safe_point,
                    },
                ));
            }
        };
        let mut pending = WorthQueryWorkflowYieldCheckpointPending {
            logical_run_identity: self.logical_run_identity,
            attempt_identity: self.attempt_identity,
            resource_attempt: self.resource_attempt,
            relational_basis: self.relational_basis,
            run_counters: self.run_counters,
            artifacts: self.artifacts,
            provider_work: self.provider_work,
            provider_artifact_occurrences: self.provider_artifact_occurrences,
            execution: self.execution,
            bridge,
            yield_counters: self.yield_counters,
        };
        if !pending.bridge.receipt().signal_transition_performed()
            || pending.bridge.receipt().signal_terminal()
                != BridgeExecutionBasisSignalTerminal::Cancelled
        {
            pending.provider_work.interrupt_step_call();
            let terminal = pending.bridge.receipt().signal_terminal();
            return Err(pending.recovery(
                WorthQueryYieldRecoveryKind::SignalAttemptAlreadyTerminal(terminal),
                Arc::from(
                    "Signal attempt terminalized before Query could perform the yield transition",
                ),
            ));
        }
        Ok(pending)
    }
}

struct WorthQueryWorkflowYieldCheckpointPending {
    logical_run_identity: Arc<str>,
    attempt_identity: Arc<str>,
    resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    relational_basis: RelationalExecutionBasisLease,
    run_counters: WorthQueryManagedRunCounters,
    artifacts: WorthQueryWorkflowArtifactAuthority,
    provider_work: WorthQueryManagedProviderWorkLedger,
    provider_artifact_occurrences: Arc<WorthQueryArtifactOccurrenceLedger>,
    execution: WorthQueryManagedGraphExecution,
    bridge: BridgeYieldedExecutionBasis,
    yield_counters: WorthQueryYieldTransitionCounters,
}

impl WorthQueryWorkflowYieldCheckpointPending {
    fn suspend_checkpoint(
        self,
    ) -> Result<WorthQueryWorkflowYieldRetained, WorthQueryWorkflowYieldOutcome> {
        let Self {
            logical_run_identity,
            attempt_identity,
            resource_attempt,
            relational_basis,
            run_counters,
            artifacts,
            mut provider_work,
            provider_artifact_occurrences,
            execution,
            bridge,
            mut yield_counters,
        } = self;
        yield_counters.attempted_checkpoint_suspension();
        let retained = match execution.suspend() {
            Ok(suspension) => {
                yield_counters.observed_checkpoint_retained_bytes(1);
                provider_work
                    .record_provider_execution_release(&suspension.provider_execution_release);
                suspension.retained
            }
            Err(failure) => {
                yield_counters.observed_checkpoint_retained_bytes(
                    failure.checkpoint_retained_byte_probe_count(),
                );
                provider_work
                    .record_provider_execution_release(failure.provider_execution_release());
                provider_work.abandon();
                let kind = failure.kind();
                let detail = Arc::from(failure.detail());
                return Err(terminalized_recovery(
                    WorthQueryYieldRecoveryKind::ProviderCheckpointSuspension(kind),
                    detail,
                    yield_counters,
                    WorthQueryTerminalizedWorkflowYieldRecovery {
                        logical_run_identity,
                        attempt_identity,
                        resource_attempt,
                        relational_basis,
                        artifacts,
                        bridge,
                        run_counters,
                        provider_work: provider_work.into_evidence(),
                    },
                    WorthQueryYieldRecoveryResourceEvidence::provider_checkpoint_suspension(
                        failure,
                    ),
                ));
            }
        };
        Ok(WorthQueryWorkflowYieldRetained {
            logical_run_identity,
            attempt_identity,
            resource_attempt,
            relational_basis,
            run_counters,
            artifacts,
            provider_work,
            provider_artifact_occurrences,
            bridge,
            yield_counters,
            retained,
        })
    }

    fn recovery(
        self,
        kind: WorthQueryYieldRecoveryKind,
        detail: Arc<str>,
    ) -> WorthQueryWorkflowYieldOutcome {
        terminalized_recovery(
            kind,
            detail,
            self.yield_counters,
            WorthQueryTerminalizedWorkflowYieldRecovery {
                logical_run_identity: self.logical_run_identity,
                attempt_identity: self.attempt_identity,
                resource_attempt: self.resource_attempt,
                relational_basis: self.relational_basis,
                artifacts: self.artifacts,
                bridge: self.bridge,
                run_counters: self.run_counters,
                provider_work: self.provider_work.into_evidence(),
            },
            WorthQueryYieldRecoveryResourceEvidence::default(),
        )
    }
}

struct WorthQueryWorkflowYieldRetained {
    logical_run_identity: Arc<str>,
    attempt_identity: Arc<str>,
    resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    relational_basis: RelationalExecutionBasisLease,
    run_counters: WorthQueryManagedRunCounters,
    artifacts: WorthQueryWorkflowArtifactAuthority,
    provider_work: WorthQueryManagedProviderWorkLedger,
    provider_artifact_occurrences: Arc<WorthQueryArtifactOccurrenceLedger>,
    bridge: BridgeYieldedExecutionBasis,
    yield_counters: WorthQueryYieldTransitionCounters,
    retained: WorthQueryRetainedManagedGraphExecution,
}

impl WorthQueryWorkflowYieldRetained {
    fn validate_and_mint(mut self) -> WorthQueryWorkflowYieldOutcome {
        self.yield_counters.observed_artifact_registry();
        let artifact_evidence = self.artifacts.registry().freeze_production();
        let ceiling = self
            .resource_attempt
            .operation_resources()
            .envelope()
            .yield_contract()
            .expect("eligibility established the installed yield contract")
            .retained_bytes_ceiling();
        let retained_total = self
            .retained
            .checkpoint_evidence()
            .retained_bytes()
            .saturating_add(u64::try_from(artifact_evidence.retained_bytes()).unwrap_or(u64::MAX));
        self.yield_counters.validated_retained_resources();
        if retained_total > ceiling {
            return self.recover_retained_bytes_exceeded();
        }
        self.provider_work.interrupt_step_call();
        self.provider_work
            .settle_artifacts(self.provider_artifact_occurrences.snapshot());
        self.yield_counters.minted_yielded_capability();
        WorthQueryWorkflowYieldOutcome::Yielded(WorthQueryYieldedWorkflowRun {
            logical_run_identity: self.logical_run_identity,
            attempt_identity: self.attempt_identity,
            resource_attempt: self.resource_attempt,
            relational_basis: self.relational_basis,
            bridge: self.bridge,
            execution: self.retained,
            artifacts: self.artifacts,
            artifact_evidence,
            run_counters: self.run_counters,
            provider_work: self.provider_work,
            provider_artifact_occurrences: self.provider_artifact_occurrences,
            yield_counters: self.yield_counters,
        })
    }

    fn recover_retained_bytes_exceeded(self) -> WorthQueryWorkflowYieldOutcome {
        let Self {
            logical_run_identity,
            attempt_identity,
            resource_attempt,
            relational_basis,
            run_counters,
            artifacts,
            mut provider_work,
            provider_artifact_occurrences: _,
            bridge,
            yield_counters,
            retained,
        } = self;
        let checkpoint_release = retained.release();
        let release_recovery_required = checkpoint_release.disposition().recovery_required();
        provider_work.abandon();
        terminalized_recovery(
            WorthQueryYieldRecoveryKind::RetainedBytesExceeded,
            Arc::from(if release_recovery_required {
                "over-ceiling checkpoint panicked during release"
            } else {
                "checkpoint and artifacts exceed the installed retained-byte ceiling"
            }),
            yield_counters,
            WorthQueryTerminalizedWorkflowYieldRecovery {
                logical_run_identity,
                attempt_identity,
                resource_attempt,
                relational_basis,
                artifacts,
                bridge,
                run_counters,
                provider_work: provider_work.into_evidence(),
            },
            WorthQueryYieldRecoveryResourceEvidence::retained_bytes_exceeded(checkpoint_release),
        )
    }
}
