use super::{
    WorthQueryPendingWorkflowConvergenceIteration, WorthQueryStartedWorkflowConvergenceIteration,
};
use crate::domain_computation::{
    WorthQueryReadmissionEvidence, WorthQueryWorkflowReadmissionCleanupRequired,
    WorthQueryWorkflowReadmissionDenied, WorthQueryWorkflowReadmissionOutcome,
    WorthQueryWorkflowReadmissionRecoveryRequired, WorthQueryWorkflowReadmissionTerminalRecovery,
    WorthQueryWorkflowReadmissionYieldReassemblyOutcome as ManagedYieldReassemblyOutcome,
    WorthQueryWorkflowReadmissionYieldReassemblyRecovery, WorthQueryWorkflowYieldDenied,
    WorthQueryWorkflowYieldOutcome, WorthQueryWorkflowYieldRecoveryRequired,
    WorthQueryYieldedWorkflowRun,
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
            WorthQueryWorkflowReadmissionOutcome::Readmitted(readmitted) => {
                let evidence = readmitted.readmission_evidence();
                self.pending.core.counters_mut().resumed();
                self.pending.expected_run_identity = readmitted.active().run_identity().into();
                WorthQueryWorkflowConvergenceReadmissionOutcome::Readmitted(
                    WorthQueryReadmittedWorkflowConvergenceIteration {
                        started: WorthQueryStartedWorkflowConvergenceIteration {
                            pending: self.pending,
                            execution: readmitted.into_active(),
                        },
                        evidence,
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
                let recovery = match recovery {
                    WorthQueryWorkflowReadmissionRecoveryRequired::YieldReassembly(recovery) => {
                        WorthQueryWorkflowConvergenceReadmissionRecoveryRequired::YieldReassembly(
                            WorthQueryWorkflowConvergenceReadmissionYieldReassemblyRecovery {
                                pending: self.pending,
                                recovery,
                            },
                        )
                    }
                    WorthQueryWorkflowReadmissionRecoveryRequired::TerminalCleanup(recovery) => {
                        WorthQueryWorkflowConvergenceReadmissionRecoveryRequired::TerminalCleanup(
                            WorthQueryWorkflowConvergenceReadmissionTerminalRecovery {
                                pending: self.pending,
                                recovery,
                            },
                        )
                    }
                };
                WorthQueryWorkflowConvergenceReadmissionOutcome::RecoveryRequired(recovery)
            }
        }
    }
}

#[must_use = "readmitted convergence iteration must continue through its started authority"]
pub struct WorthQueryReadmittedWorkflowConvergenceIteration {
    started: WorthQueryStartedWorkflowConvergenceIteration,
    evidence: WorthQueryReadmissionEvidence,
}

impl WorthQueryReadmittedWorkflowConvergenceIteration {
    pub const fn readmission_evidence(&self) -> WorthQueryReadmissionEvidence {
        self.evidence
    }

    pub fn into_started(self) -> WorthQueryStartedWorkflowConvergenceIteration {
        self.started
    }
}

#[must_use = "convergence readmission denial retains the exact yielded iteration authority"]
pub struct WorthQueryWorkflowConvergenceReadmissionDenied {
    pending: WorthQueryPendingWorkflowConvergenceIteration,
    denial: WorthQueryWorkflowReadmissionDenied,
}

impl WorthQueryWorkflowConvergenceReadmissionDenied {
    pub const fn readmission_evidence(&self) -> WorthQueryReadmissionEvidence {
        self.denial.readmission_evidence()
    }

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

#[must_use = "convergence readmission recovery must be resolved by authority posture"]
pub enum WorthQueryWorkflowConvergenceReadmissionRecoveryRequired {
    YieldReassembly(WorthQueryWorkflowConvergenceReadmissionYieldReassemblyRecovery),
    TerminalCleanup(WorthQueryWorkflowConvergenceReadmissionTerminalRecovery),
}

#[must_use = "convergence yield reassembly must retry Bridge cleanup or enter cleanup"]
pub struct WorthQueryWorkflowConvergenceReadmissionYieldReassemblyRecovery {
    pending: WorthQueryPendingWorkflowConvergenceIteration,
    recovery: WorthQueryWorkflowReadmissionYieldReassemblyRecovery,
}

#[must_use = "terminal convergence readmission recovery can only enter cleanup"]
pub struct WorthQueryWorkflowConvergenceReadmissionTerminalRecovery {
    pending: WorthQueryPendingWorkflowConvergenceIteration,
    recovery: WorthQueryWorkflowReadmissionTerminalRecovery,
}

impl WorthQueryWorkflowConvergenceReadmissionYieldReassemblyRecovery {
    pub const fn readmission_evidence(&self) -> WorthQueryReadmissionEvidence {
        self.recovery.readmission_evidence()
    }

    pub fn managed_recovery(&self) -> &WorthQueryWorkflowReadmissionYieldReassemblyRecovery {
        &self.recovery
    }

    pub fn retry_to_yielded(self) -> WorthQueryWorkflowConvergenceYieldReassemblyOutcome {
        let Self { pending, recovery } = self;
        match recovery.retry_to_yielded() {
            ManagedYieldReassemblyOutcome::Yielded(reassembled) => {
                let evidence = reassembled.readmission_evidence();
                WorthQueryWorkflowConvergenceYieldReassemblyOutcome::Yielded(
                    WorthQueryWorkflowConvergenceYieldReassembled {
                        yielded: WorthQueryYieldedWorkflowConvergenceIteration {
                            pending,
                            yielded: reassembled.into_yielded(),
                        },
                        evidence,
                    },
                )
            }
            ManagedYieldReassemblyOutcome::RecoveryRequired(recovery) => {
                WorthQueryWorkflowConvergenceYieldReassemblyOutcome::RecoveryRequired(
                    WorthQueryWorkflowConvergenceReadmissionYieldReassemblyRecovery {
                        pending,
                        recovery,
                    },
                )
            }
        }
    }

    pub fn into_cleanup(self) -> WorthQueryWorkflowConvergenceReadmissionCleanupRequired {
        WorthQueryWorkflowConvergenceReadmissionCleanupRequired {
            pending: self.pending,
            cleanup: self.recovery.into_cleanup(),
        }
    }
}

impl WorthQueryWorkflowConvergenceReadmissionTerminalRecovery {
    pub const fn readmission_evidence(&self) -> WorthQueryReadmissionEvidence {
        self.recovery.readmission_evidence()
    }

    pub fn managed_recovery(&self) -> &WorthQueryWorkflowReadmissionTerminalRecovery {
        &self.recovery
    }

    pub fn into_cleanup(self) -> WorthQueryWorkflowConvergenceReadmissionCleanupRequired {
        WorthQueryWorkflowConvergenceReadmissionCleanupRequired {
            pending: self.pending,
            cleanup: self.recovery.into_cleanup(),
        }
    }
}

#[must_use = "convergence yield reassembly retains yielded or exact recovery authority"]
pub enum WorthQueryWorkflowConvergenceYieldReassemblyOutcome {
    Yielded(WorthQueryWorkflowConvergenceYieldReassembled),
    RecoveryRequired(WorthQueryWorkflowConvergenceReadmissionYieldReassemblyRecovery),
}

#[must_use = "reassembled convergence yield retains exact yielded authority and owner evidence"]
pub struct WorthQueryWorkflowConvergenceYieldReassembled {
    yielded: WorthQueryYieldedWorkflowConvergenceIteration,
    evidence: WorthQueryReadmissionEvidence,
}

impl WorthQueryWorkflowConvergenceYieldReassembled {
    pub const fn readmission_evidence(&self) -> WorthQueryReadmissionEvidence {
        self.evidence
    }

    pub fn into_yielded(self) -> WorthQueryYieldedWorkflowConvergenceIteration {
        self.yielded
    }
}

#[must_use = "convergence readmission cleanup retains managed cleanup authority"]
pub struct WorthQueryWorkflowConvergenceReadmissionCleanupRequired {
    pending: WorthQueryPendingWorkflowConvergenceIteration,
    cleanup: WorthQueryWorkflowReadmissionCleanupRequired,
}

impl WorthQueryWorkflowConvergenceReadmissionCleanupRequired {
    pub fn managed_cleanup(&self) -> &WorthQueryWorkflowReadmissionCleanupRequired {
        &self.cleanup
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthQueryPendingWorkflowConvergenceIteration,
        WorthQueryWorkflowReadmissionCleanupRequired,
    ) {
        (self.pending, self.cleanup)
    }
}

#[must_use = "convergence readmission outcomes retain started, yielded, or recovery authority"]
pub enum WorthQueryWorkflowConvergenceReadmissionOutcome {
    Readmitted(WorthQueryReadmittedWorkflowConvergenceIteration),
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
