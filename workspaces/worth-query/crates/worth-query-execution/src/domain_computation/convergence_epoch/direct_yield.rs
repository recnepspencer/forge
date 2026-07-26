use super::{
    WorthQueryPendingDirectConvergenceIteration, WorthQueryStartedDirectConvergenceIteration,
};
use crate::domain_computation::{
    WorthQueryDirectReadmissionCleanupRequired, WorthQueryDirectReadmissionDenied,
    WorthQueryDirectReadmissionOutcome, WorthQueryDirectReadmissionRecoveryRequired,
    WorthQueryDirectReadmissionTerminalRecovery,
    WorthQueryDirectReadmissionYieldReassemblyOutcome as ManagedYieldReassemblyOutcome,
    WorthQueryDirectReadmissionYieldReassemblyRecovery, WorthQueryDirectYieldDenied,
    WorthQueryDirectYieldOutcome, WorthQueryDirectYieldRecoveryRequired,
    WorthQueryReadmissionEvidence, WorthQueryYieldedDirectRun,
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
                let evidence = readmitted.readmission_evidence();
                self.pending.core.counters_mut().resumed();
                self.pending.expected_run_identity = readmitted.active().run_identity().into();
                WorthQueryDirectConvergenceReadmissionOutcome::Readmitted(
                    WorthQueryReadmittedDirectConvergenceIteration {
                        started: WorthQueryStartedDirectConvergenceIteration {
                            pending: self.pending,
                            execution: readmitted.into_active(),
                        },
                        evidence,
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
                let recovery = match recovery {
                    WorthQueryDirectReadmissionRecoveryRequired::YieldReassembly(recovery) => {
                        WorthQueryDirectConvergenceReadmissionRecoveryRequired::YieldReassembly(
                            WorthQueryDirectConvergenceReadmissionYieldReassemblyRecovery {
                                pending: self.pending,
                                recovery,
                            },
                        )
                    }
                    WorthQueryDirectReadmissionRecoveryRequired::TerminalCleanup(recovery) => {
                        WorthQueryDirectConvergenceReadmissionRecoveryRequired::TerminalCleanup(
                            WorthQueryDirectConvergenceReadmissionTerminalRecovery {
                                pending: self.pending,
                                recovery,
                            },
                        )
                    }
                };
                WorthQueryDirectConvergenceReadmissionOutcome::RecoveryRequired(recovery)
            }
        }
    }
}

#[must_use = "readmitted convergence iteration must continue through its started authority"]
pub struct WorthQueryReadmittedDirectConvergenceIteration {
    started: WorthQueryStartedDirectConvergenceIteration,
    evidence: WorthQueryReadmissionEvidence,
}

impl WorthQueryReadmittedDirectConvergenceIteration {
    pub const fn readmission_evidence(&self) -> WorthQueryReadmissionEvidence {
        self.evidence
    }

    pub fn into_started(self) -> WorthQueryStartedDirectConvergenceIteration {
        self.started
    }
}

#[must_use = "convergence readmission denial retains the exact yielded iteration authority"]
pub struct WorthQueryDirectConvergenceReadmissionDenied {
    pending: WorthQueryPendingDirectConvergenceIteration,
    denial: WorthQueryDirectReadmissionDenied,
}

impl WorthQueryDirectConvergenceReadmissionDenied {
    pub const fn readmission_evidence(&self) -> WorthQueryReadmissionEvidence {
        self.denial.readmission_evidence()
    }

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

#[must_use = "convergence readmission recovery must be resolved by authority posture"]
pub enum WorthQueryDirectConvergenceReadmissionRecoveryRequired {
    YieldReassembly(WorthQueryDirectConvergenceReadmissionYieldReassemblyRecovery),
    TerminalCleanup(WorthQueryDirectConvergenceReadmissionTerminalRecovery),
}

#[must_use = "convergence yield reassembly must retry Bridge cleanup or enter cleanup"]
pub struct WorthQueryDirectConvergenceReadmissionYieldReassemblyRecovery {
    pending: WorthQueryPendingDirectConvergenceIteration,
    recovery: WorthQueryDirectReadmissionYieldReassemblyRecovery,
}

#[must_use = "terminal convergence readmission recovery can only enter cleanup"]
pub struct WorthQueryDirectConvergenceReadmissionTerminalRecovery {
    pending: WorthQueryPendingDirectConvergenceIteration,
    recovery: WorthQueryDirectReadmissionTerminalRecovery,
}

impl WorthQueryDirectConvergenceReadmissionYieldReassemblyRecovery {
    pub const fn readmission_evidence(&self) -> WorthQueryReadmissionEvidence {
        self.recovery.readmission_evidence()
    }

    pub fn managed_recovery(&self) -> &WorthQueryDirectReadmissionYieldReassemblyRecovery {
        &self.recovery
    }

    pub fn retry_to_yielded(self) -> WorthQueryDirectConvergenceYieldReassemblyOutcome {
        let Self { pending, recovery } = self;
        match recovery.retry_to_yielded() {
            ManagedYieldReassemblyOutcome::Yielded(reassembled) => {
                let evidence = reassembled.readmission_evidence();
                WorthQueryDirectConvergenceYieldReassemblyOutcome::Yielded(
                    WorthQueryDirectConvergenceYieldReassembled {
                        yielded: WorthQueryYieldedDirectConvergenceIteration {
                            pending,
                            yielded: reassembled.into_yielded(),
                        },
                        evidence,
                    },
                )
            }
            ManagedYieldReassemblyOutcome::RecoveryRequired(recovery) => {
                WorthQueryDirectConvergenceYieldReassemblyOutcome::RecoveryRequired(
                    WorthQueryDirectConvergenceReadmissionYieldReassemblyRecovery {
                        pending,
                        recovery,
                    },
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

impl WorthQueryDirectConvergenceReadmissionTerminalRecovery {
    pub const fn readmission_evidence(&self) -> WorthQueryReadmissionEvidence {
        self.recovery.readmission_evidence()
    }

    pub fn managed_recovery(&self) -> &WorthQueryDirectReadmissionTerminalRecovery {
        &self.recovery
    }

    pub fn into_cleanup(self) -> WorthQueryDirectConvergenceReadmissionCleanupRequired {
        WorthQueryDirectConvergenceReadmissionCleanupRequired {
            pending: self.pending,
            cleanup: self.recovery.into_cleanup(),
        }
    }
}

#[must_use = "convergence yield reassembly retains yielded or exact recovery authority"]
pub enum WorthQueryDirectConvergenceYieldReassemblyOutcome {
    Yielded(WorthQueryDirectConvergenceYieldReassembled),
    RecoveryRequired(WorthQueryDirectConvergenceReadmissionYieldReassemblyRecovery),
}

#[must_use = "reassembled convergence yield retains exact yielded authority and owner evidence"]
pub struct WorthQueryDirectConvergenceYieldReassembled {
    yielded: WorthQueryYieldedDirectConvergenceIteration,
    evidence: WorthQueryReadmissionEvidence,
}

impl WorthQueryDirectConvergenceYieldReassembled {
    pub const fn readmission_evidence(&self) -> WorthQueryReadmissionEvidence {
        self.evidence
    }

    pub fn into_yielded(self) -> WorthQueryYieldedDirectConvergenceIteration {
        self.yielded
    }
}

#[must_use = "convergence readmission cleanup retains managed cleanup authority"]
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

#[must_use = "convergence readmission outcomes retain started, yielded, or recovery authority"]
pub enum WorthQueryDirectConvergenceReadmissionOutcome {
    Readmitted(WorthQueryReadmittedDirectConvergenceIteration),
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
