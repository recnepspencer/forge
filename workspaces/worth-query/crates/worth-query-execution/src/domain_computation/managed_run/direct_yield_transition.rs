use std::sync::Arc;

use worth_relational::facade::runtime::RelationalExecutionBasisLease;
use worth_runtime_bridge::facade::{
    BridgeBoundExecutionBasis, BridgeExecutionBasisSignalTerminal, BridgeYieldedExecutionBasis,
};

use super::direct_yield_eligibility::classify_direct_yield_denial;
use super::direct_yield_recovery::{
    running_recovery, terminalized_recovery, WorthQueryTerminalizedDirectYieldRecovery,
};
use super::managed_graph_execution::WorthQueryManagedGraphExecution;
use super::provider_work::WorthQueryManagedProviderWorkLedger;
use super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution;
use super::yield_eligibility::WorthQueryManagedYieldSafePoint;
use super::{
    WorthQueryDirectYieldDenied, WorthQueryDirectYieldOutcome, WorthQueryManagedRunCounters,
    WorthQueryPausedDirectGraphExecution, WorthQueryRunningDirectRun, WorthQueryYieldRecoveryKind,
    WorthQueryYieldRecoveryResourceEvidence, WorthQueryYieldTransitionCounters,
    WorthQueryYieldedDirectRun,
};
use crate::domain_computation::WorthQueryDirectExecutionResourceAttempt;

pub(super) fn yield_direct_run(
    paused: WorthQueryPausedDirectGraphExecution,
) -> WorthQueryDirectYieldOutcome {
    let mut counters = WorthQueryYieldTransitionCounters::default();
    counters.classified_eligibility();
    if let Some((kind, detail)) = classify_direct_yield_denial(&paused) {
        return WorthQueryDirectYieldOutcome::Denied(WorthQueryDirectYieldDenied {
            kind,
            detail: Arc::from(detail),
            paused,
            counters,
        });
    }
    let bridge_pending = WorthQueryDirectYieldBridgePending::from_paused(paused, counters);
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

struct WorthQueryDirectYieldBridgePending {
    logical_run_identity: Arc<str>,
    attempt_identity: Arc<str>,
    resource_attempt: WorthQueryDirectExecutionResourceAttempt,
    bridge_basis: BridgeBoundExecutionBasis,
    relational_basis: RelationalExecutionBasisLease,
    run_counters: WorthQueryManagedRunCounters,
    provider_work: WorthQueryManagedProviderWorkLedger,
    execution: WorthQueryManagedGraphExecution,
    safe_point: WorthQueryManagedYieldSafePoint,
    yield_counters: WorthQueryYieldTransitionCounters,
}

impl WorthQueryDirectYieldBridgePending {
    fn from_paused(
        paused: WorthQueryPausedDirectGraphExecution,
        yield_counters: WorthQueryYieldTransitionCounters,
    ) -> Self {
        let WorthQueryPausedDirectGraphExecution { active, safe_point } = paused;
        let super::WorthQueryActiveDirectGraphExecution { running, execution } = active;
        let WorthQueryRunningDirectRun {
            logical_run_identity,
            identity,
            resource_attempt,
            bridge_basis,
            relational_basis,
            counters,
            provider_work,
        } = running;
        Self {
            logical_run_identity,
            attempt_identity: identity,
            resource_attempt,
            bridge_basis,
            relational_basis,
            run_counters: counters,
            provider_work,
            execution,
            safe_point,
            yield_counters,
        }
    }

    fn finalize_bridge(
        mut self,
    ) -> Result<WorthQueryDirectYieldCheckpointPending, WorthQueryDirectYieldOutcome> {
        self.yield_counters.attempted_bridge_finalization();
        let bridge = match self.bridge_basis.yield_execution_basis() {
            Ok(receipt) => receipt,
            Err(failure) => {
                let kind = failure.kind();
                let detail = Arc::from(failure.detail());
                let running = WorthQueryRunningDirectRun {
                    logical_run_identity: self.logical_run_identity,
                    identity: self.attempt_identity,
                    resource_attempt: self.resource_attempt,
                    bridge_basis: failure.into_basis(),
                    relational_basis: self.relational_basis,
                    counters: self.run_counters,
                    provider_work: self.provider_work,
                };
                return Err(running_recovery(
                    WorthQueryYieldRecoveryKind::BridgeTerminalization(kind),
                    detail,
                    self.yield_counters,
                    WorthQueryPausedDirectGraphExecution {
                        active: super::WorthQueryActiveDirectGraphExecution {
                            running,
                            execution: self.execution,
                        },
                        safe_point: self.safe_point,
                    },
                ));
            }
        };
        let mut pending = WorthQueryDirectYieldCheckpointPending {
            logical_run_identity: self.logical_run_identity,
            attempt_identity: self.attempt_identity,
            resource_attempt: self.resource_attempt,
            relational_basis: self.relational_basis,
            run_counters: self.run_counters,
            provider_work: self.provider_work,
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

struct WorthQueryDirectYieldCheckpointPending {
    logical_run_identity: Arc<str>,
    attempt_identity: Arc<str>,
    resource_attempt: WorthQueryDirectExecutionResourceAttempt,
    relational_basis: RelationalExecutionBasisLease,
    run_counters: WorthQueryManagedRunCounters,
    provider_work: WorthQueryManagedProviderWorkLedger,
    execution: WorthQueryManagedGraphExecution,
    bridge: BridgeYieldedExecutionBasis,
    yield_counters: WorthQueryYieldTransitionCounters,
}

impl WorthQueryDirectYieldCheckpointPending {
    fn suspend_checkpoint(
        self,
    ) -> Result<WorthQueryDirectYieldRetained, WorthQueryDirectYieldOutcome> {
        let Self {
            logical_run_identity,
            attempt_identity,
            resource_attempt,
            relational_basis,
            run_counters,
            mut provider_work,
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
                    WorthQueryTerminalizedDirectYieldRecovery {
                        logical_run_identity,
                        attempt_identity,
                        resource_attempt,
                        relational_basis,
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
        Ok(WorthQueryDirectYieldRetained {
            logical_run_identity,
            attempt_identity,
            resource_attempt,
            relational_basis,
            run_counters,
            provider_work,
            bridge,
            yield_counters,
            retained,
        })
    }

    fn recovery(
        self,
        kind: WorthQueryYieldRecoveryKind,
        detail: Arc<str>,
    ) -> WorthQueryDirectYieldOutcome {
        terminalized_recovery(
            kind,
            detail,
            self.yield_counters,
            WorthQueryTerminalizedDirectYieldRecovery {
                logical_run_identity: self.logical_run_identity,
                attempt_identity: self.attempt_identity,
                resource_attempt: self.resource_attempt,
                relational_basis: self.relational_basis,
                bridge: self.bridge,
                run_counters: self.run_counters,
                provider_work: self.provider_work.into_evidence(),
            },
            WorthQueryYieldRecoveryResourceEvidence::default(),
        )
    }
}

struct WorthQueryDirectYieldRetained {
    logical_run_identity: Arc<str>,
    attempt_identity: Arc<str>,
    resource_attempt: WorthQueryDirectExecutionResourceAttempt,
    relational_basis: RelationalExecutionBasisLease,
    run_counters: WorthQueryManagedRunCounters,
    provider_work: WorthQueryManagedProviderWorkLedger,
    bridge: BridgeYieldedExecutionBasis,
    yield_counters: WorthQueryYieldTransitionCounters,
    retained: WorthQueryRetainedManagedGraphExecution,
}

impl WorthQueryDirectYieldRetained {
    fn validate_and_mint(mut self) -> WorthQueryDirectYieldOutcome {
        let ceiling = self
            .resource_attempt
            .resources()
            .envelope()
            .yield_contract()
            .expect("eligibility established the installed yield contract")
            .retained_bytes_ceiling();
        self.yield_counters.validated_retained_resources();
        if self.retained.checkpoint_evidence().retained_bytes() > ceiling {
            let checkpoint_release = self.retained.release();
            let release_recovery_required = checkpoint_release.disposition().recovery_required();
            self.provider_work.abandon();
            return terminalized_recovery(
                WorthQueryYieldRecoveryKind::RetainedBytesExceeded,
                Arc::from(if release_recovery_required {
                    "provider checkpoint exceeds the retained-byte ceiling and panicked during release"
                } else {
                    "provider checkpoint exceeds the installed retained-byte ceiling"
                }),
                self.yield_counters,
                WorthQueryTerminalizedDirectYieldRecovery {
                    logical_run_identity: self.logical_run_identity,
                    attempt_identity: self.attempt_identity,
                    resource_attempt: self.resource_attempt,
                    relational_basis: self.relational_basis,
                    bridge: self.bridge,
                    run_counters: self.run_counters,
                    provider_work: self.provider_work.into_evidence(),
                },
                WorthQueryYieldRecoveryResourceEvidence::retained_bytes_exceeded(
                    checkpoint_release,
                ),
            );
        }
        self.provider_work.interrupt_step_call();
        self.yield_counters.minted_yielded_capability();
        WorthQueryDirectYieldOutcome::Yielded(WorthQueryYieldedDirectRun {
            logical_run_identity: self.logical_run_identity,
            attempt_identity: self.attempt_identity,
            resource_attempt: self.resource_attempt,
            relational_basis: self.relational_basis,
            bridge: self.bridge,
            execution: self.retained,
            run_counters: self.run_counters,
            provider_work: self.provider_work,
            yield_counters: self.yield_counters,
        })
    }
}
