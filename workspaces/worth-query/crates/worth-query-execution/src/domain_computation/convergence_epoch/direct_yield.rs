use super::{
    WorthQueryPendingDirectConvergenceIteration, WorthQueryStartedDirectConvergenceIteration,
};
use crate::domain_computation::{
    WorthQueryDirectReadmissionCleanupRequired, WorthQueryDirectReadmissionDenied,
    WorthQueryDirectReadmissionOutcome, WorthQueryDirectReadmissionRecoveryRequired,
    WorthQueryDirectReadmissionRecoveryRetryOutcome as ManagedRecoveryRetryOutcome,
    WorthQueryDirectYieldDenied, WorthQueryDirectYieldOutcome,
    WorthQueryDirectYieldRecoveryRequired, WorthQueryYieldedDirectRun,
};

pub enum WorthQueryDirectConvergenceYieldOutcome {
    Yielded(WorthQueryYieldedDirectConvergenceIteration),
    Denied {
        pending: WorthQueryPendingDirectConvergenceIteration,
        denial: WorthQueryDirectYieldDenied,
    },
    RecoveryRequired {
        pending: WorthQueryPendingDirectConvergenceIteration,
        recovery: WorthQueryDirectYieldRecoveryRequired,
    },
    RunMismatch {
        pending: WorthQueryPendingDirectConvergenceIteration,
        yielded: WorthQueryYieldedDirectRun,
    },
}

pub struct WorthQueryYieldedDirectConvergenceIteration {
    pending: WorthQueryPendingDirectConvergenceIteration,
    yielded: WorthQueryYieldedDirectRun,
}

impl WorthQueryYieldedDirectConvergenceIteration {
    pub fn epoch_identity(&self) -> &str {
        self.pending.core.identity()
    }

    pub fn logical_run_identity(&self) -> &str {
        self.pending.core.logical_run_identity()
    }

    pub fn yielded_run(&self) -> &WorthQueryYieldedDirectRun {
        &self.yielded
    }

    pub fn readmit_same_runtime(
        mut self,
        query_runtime: &crate::domain_computation::WorthQueryExecutionRuntime,
        bridge_runtime: &worth_runtime_bridge::facade::RuntimeBridge,
    ) -> WorthQueryDirectConvergenceReadmissionOutcome {
        match self
            .yielded
            .readmit_same_runtime(query_runtime, bridge_runtime)
        {
            WorthQueryDirectReadmissionOutcome::Readmitted(readmitted) => {
                self.pending.core.counters_mut().resumed();
                self.pending.expected_run_identity = readmitted.active().run_identity().into();
                WorthQueryDirectConvergenceReadmissionOutcome::Readmitted(
                    WorthQueryStartedDirectConvergenceIteration {
                        pending: self.pending,
                        execution: readmitted.into_active(),
                    },
                )
            }
            WorthQueryDirectReadmissionOutcome::Denied(denial) => {
                WorthQueryDirectConvergenceReadmissionOutcome::Denied(
                    WorthQueryDirectConvergenceReadmissionDenied {
                        pending: self.pending,
                        denial,
                    },
                )
            }
            WorthQueryDirectReadmissionOutcome::RecoveryRequired(recovery) => {
                WorthQueryDirectConvergenceReadmissionOutcome::RecoveryRequired(
                    WorthQueryDirectConvergenceReadmissionRecoveryRequired {
                        pending: self.pending,
                        recovery,
                    },
                )
            }
        }
    }
}

pub struct WorthQueryDirectConvergenceReadmissionDenied {
    pending: WorthQueryPendingDirectConvergenceIteration,
    denial: WorthQueryDirectReadmissionDenied,
}

impl WorthQueryDirectConvergenceReadmissionDenied {
    pub fn managed_denial(&self) -> &WorthQueryDirectReadmissionDenied {
        &self.denial
    }

    pub fn into_yielded(self) -> WorthQueryYieldedDirectConvergenceIteration {
        WorthQueryYieldedDirectConvergenceIteration {
            pending: self.pending,
            yielded: self.denial.into_yielded(),
        }
    }
}

pub struct WorthQueryDirectConvergenceReadmissionRecoveryRequired {
    pending: WorthQueryPendingDirectConvergenceIteration,
    recovery: WorthQueryDirectReadmissionRecoveryRequired,
}

impl WorthQueryDirectConvergenceReadmissionRecoveryRequired {
    pub fn managed_recovery(&self) -> &WorthQueryDirectReadmissionRecoveryRequired {
        &self.recovery
    }

    pub fn retry_to_yielded(self) -> WorthQueryDirectConvergenceReadmissionRecoveryRetryOutcome {
        let Self { pending, recovery } = self;
        match recovery.retry_to_yielded() {
            ManagedRecoveryRetryOutcome::Yielded(yielded) => {
                WorthQueryDirectConvergenceReadmissionRecoveryRetryOutcome::Yielded(
                    WorthQueryYieldedDirectConvergenceIteration { pending, yielded },
                )
            }
            ManagedRecoveryRetryOutcome::RecoveryRequired(recovery) => {
                WorthQueryDirectConvergenceReadmissionRecoveryRetryOutcome::RecoveryRequired(Self {
                    pending,
                    recovery,
                })
            }
            ManagedRecoveryRetryOutcome::CleanupRequired(cleanup) => {
                WorthQueryDirectConvergenceReadmissionRecoveryRetryOutcome::CleanupRequired(
                    WorthQueryDirectConvergenceReadmissionCleanupRequired { pending, cleanup },
                )
            }
        }
    }

    pub fn into_cleanup(self) -> WorthQueryDirectConvergenceReadmissionCleanupRequired {
        WorthQueryDirectConvergenceReadmissionCleanupRequired {
            pending: self.pending,
            cleanup: self.recovery.into_cleanup(),
        }
    }
}

pub enum WorthQueryDirectConvergenceReadmissionRecoveryRetryOutcome {
    Yielded(WorthQueryYieldedDirectConvergenceIteration),
    RecoveryRequired(WorthQueryDirectConvergenceReadmissionRecoveryRequired),
    CleanupRequired(WorthQueryDirectConvergenceReadmissionCleanupRequired),
}

pub struct WorthQueryDirectConvergenceReadmissionCleanupRequired {
    pending: WorthQueryPendingDirectConvergenceIteration,
    cleanup: WorthQueryDirectReadmissionCleanupRequired,
}

impl WorthQueryDirectConvergenceReadmissionCleanupRequired {
    pub fn managed_cleanup(&self) -> &WorthQueryDirectReadmissionCleanupRequired {
        &self.cleanup
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthQueryPendingDirectConvergenceIteration,
        WorthQueryDirectReadmissionCleanupRequired,
    ) {
        (self.pending, self.cleanup)
    }
}

pub enum WorthQueryDirectConvergenceReadmissionOutcome {
    Readmitted(WorthQueryStartedDirectConvergenceIteration),
    Denied(WorthQueryDirectConvergenceReadmissionDenied),
    RecoveryRequired(WorthQueryDirectConvergenceReadmissionRecoveryRequired),
}

impl WorthQueryPendingDirectConvergenceIteration {
    pub fn admit_yield_outcome(
        mut self,
        outcome: WorthQueryDirectYieldOutcome,
    ) -> WorthQueryDirectConvergenceYieldOutcome {
        match outcome {
            WorthQueryDirectYieldOutcome::Yielded(yielded)
                if yielded.yielded_attempt_identity() == self.expected_run_identity.as_ref()
                    && yielded.logical_run_identity() == self.core.logical_run_identity() =>
            {
                self.core.counters_mut().yielded();
                WorthQueryDirectConvergenceYieldOutcome::Yielded(
                    WorthQueryYieldedDirectConvergenceIteration {
                        pending: self,
                        yielded,
                    },
                )
            }
            WorthQueryDirectYieldOutcome::Yielded(yielded) => {
                WorthQueryDirectConvergenceYieldOutcome::RunMismatch {
                    pending: self,
                    yielded,
                }
            }
            WorthQueryDirectYieldOutcome::Denied(denial) => {
                WorthQueryDirectConvergenceYieldOutcome::Denied {
                    pending: self,
                    denial,
                }
            }
            WorthQueryDirectYieldOutcome::RecoveryRequired(recovery) => {
                WorthQueryDirectConvergenceYieldOutcome::RecoveryRequired {
                    pending: self,
                    recovery,
                }
            }
        }
    }
}
