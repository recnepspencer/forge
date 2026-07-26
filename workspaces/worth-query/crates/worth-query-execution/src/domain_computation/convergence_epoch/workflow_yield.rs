use super::{
    WorthQueryPendingWorkflowConvergenceIteration, WorthQueryStartedWorkflowConvergenceIteration,
};
use crate::domain_computation::{
    WorthQueryWorkflowReadmissionDenied, WorthQueryWorkflowReadmissionOutcome,
    WorthQueryWorkflowReadmissionRecoveryRequired,
    WorthQueryWorkflowReadmissionRecoveryRetryOutcome as ManagedRecoveryRetryOutcome,
    WorthQueryWorkflowYieldDenied, WorthQueryWorkflowYieldOutcome,
    WorthQueryWorkflowYieldRecoveryRequired, WorthQueryYieldedWorkflowRun,
};

pub enum WorthQueryWorkflowConvergenceYieldOutcome {
    Yielded(WorthQueryYieldedWorkflowConvergenceIteration),
    Denied {
        pending: WorthQueryPendingWorkflowConvergenceIteration,
        denial: WorthQueryWorkflowYieldDenied,
    },
    RecoveryRequired {
        pending: WorthQueryPendingWorkflowConvergenceIteration,
        recovery: WorthQueryWorkflowYieldRecoveryRequired,
    },
    RunMismatch {
        pending: WorthQueryPendingWorkflowConvergenceIteration,
        yielded: WorthQueryYieldedWorkflowRun,
    },
}

pub struct WorthQueryYieldedWorkflowConvergenceIteration {
    pending: WorthQueryPendingWorkflowConvergenceIteration,
    yielded: WorthQueryYieldedWorkflowRun,
}

impl WorthQueryYieldedWorkflowConvergenceIteration {
    pub fn epoch_identity(&self) -> &str {
        self.pending.core.identity()
    }

    pub fn logical_run_identity(&self) -> &str {
        self.pending.core.logical_run_identity()
    }

    pub fn yielded_run(&self) -> &WorthQueryYieldedWorkflowRun {
        &self.yielded
    }

    pub fn readmit_same_runtime(
        mut self,
        query_runtime: &crate::domain_computation::WorthQueryExecutionRuntime,
        bridge_runtime: &worth_runtime_bridge::facade::RuntimeBridge,
    ) -> WorthQueryWorkflowConvergenceReadmissionOutcome {
        match self
            .yielded
            .readmit_same_runtime(query_runtime, bridge_runtime)
        {
            WorthQueryWorkflowReadmissionOutcome::Readmitted(execution) => {
                self.pending.core.counters_mut().resumed();
                self.pending.expected_run_identity = execution.run_identity().into();
                WorthQueryWorkflowConvergenceReadmissionOutcome::Readmitted(
                    WorthQueryStartedWorkflowConvergenceIteration {
                        pending: self.pending,
                        execution,
                    },
                )
            }
            WorthQueryWorkflowReadmissionOutcome::Denied(denial) => {
                WorthQueryWorkflowConvergenceReadmissionOutcome::Denied(
                    WorthQueryWorkflowConvergenceReadmissionDenied {
                        pending: self.pending,
                        denial,
                    },
                )
            }
            WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(recovery) => {
                WorthQueryWorkflowConvergenceReadmissionOutcome::RecoveryRequired(
                    WorthQueryWorkflowConvergenceReadmissionRecoveryRequired {
                        pending: self.pending,
                        recovery,
                    },
                )
            }
        }
    }
}

pub struct WorthQueryWorkflowConvergenceReadmissionDenied {
    pending: WorthQueryPendingWorkflowConvergenceIteration,
    denial: WorthQueryWorkflowReadmissionDenied,
}

impl WorthQueryWorkflowConvergenceReadmissionDenied {
    pub fn managed_denial(&self) -> &WorthQueryWorkflowReadmissionDenied {
        &self.denial
    }

    pub fn into_yielded(self) -> WorthQueryYieldedWorkflowConvergenceIteration {
        WorthQueryYieldedWorkflowConvergenceIteration {
            pending: self.pending,
            yielded: self.denial.into_yielded(),
        }
    }
}

pub struct WorthQueryWorkflowConvergenceReadmissionRecoveryRequired {
    pending: WorthQueryPendingWorkflowConvergenceIteration,
    recovery: WorthQueryWorkflowReadmissionRecoveryRequired,
}

impl WorthQueryWorkflowConvergenceReadmissionRecoveryRequired {
    pub fn managed_recovery(&self) -> &WorthQueryWorkflowReadmissionRecoveryRequired {
        &self.recovery
    }

    pub fn retry_to_yielded(
        self,
    ) -> Result<WorthQueryWorkflowConvergenceReadmissionRecoveryRetryOutcome, Self> {
        let Self { pending, recovery } = self;
        match recovery.retry_to_yielded() {
            Ok(ManagedRecoveryRetryOutcome::Yielded(yielded)) => Ok(
                WorthQueryWorkflowConvergenceReadmissionRecoveryRetryOutcome::Yielded(
                    WorthQueryYieldedWorkflowConvergenceIteration { pending, yielded },
                ),
            ),
            Ok(ManagedRecoveryRetryOutcome::RecoveryRequired(recovery)) => Ok(
                WorthQueryWorkflowConvergenceReadmissionRecoveryRetryOutcome::RecoveryRequired(
                    Self { pending, recovery },
                ),
            ),
            Err(recovery) => Err(Self { pending, recovery }),
        }
    }
}

pub enum WorthQueryWorkflowConvergenceReadmissionRecoveryRetryOutcome {
    Yielded(WorthQueryYieldedWorkflowConvergenceIteration),
    RecoveryRequired(WorthQueryWorkflowConvergenceReadmissionRecoveryRequired),
}

pub enum WorthQueryWorkflowConvergenceReadmissionOutcome {
    Readmitted(WorthQueryStartedWorkflowConvergenceIteration),
    Denied(WorthQueryWorkflowConvergenceReadmissionDenied),
    RecoveryRequired(WorthQueryWorkflowConvergenceReadmissionRecoveryRequired),
}

impl WorthQueryPendingWorkflowConvergenceIteration {
    pub fn admit_yield_outcome(
        mut self,
        outcome: WorthQueryWorkflowYieldOutcome,
    ) -> WorthQueryWorkflowConvergenceYieldOutcome {
        match outcome {
            WorthQueryWorkflowYieldOutcome::Yielded(yielded)
                if yielded.yielded_attempt_identity() == self.expected_run_identity.as_ref()
                    && yielded.logical_run_identity() == self.core.logical_run_identity() =>
            {
                self.core.counters_mut().yielded();
                WorthQueryWorkflowConvergenceYieldOutcome::Yielded(
                    WorthQueryYieldedWorkflowConvergenceIteration {
                        pending: self,
                        yielded,
                    },
                )
            }
            WorthQueryWorkflowYieldOutcome::Yielded(yielded) => {
                WorthQueryWorkflowConvergenceYieldOutcome::RunMismatch {
                    pending: self,
                    yielded,
                }
            }
            WorthQueryWorkflowYieldOutcome::Denied(denial) => {
                WorthQueryWorkflowConvergenceYieldOutcome::Denied {
                    pending: self,
                    denial,
                }
            }
            WorthQueryWorkflowYieldOutcome::RecoveryRequired(recovery) => {
                WorthQueryWorkflowConvergenceYieldOutcome::RecoveryRequired {
                    pending: self,
                    recovery,
                }
            }
        }
    }
}
