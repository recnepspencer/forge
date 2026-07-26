use super::yield_checkpoint_fixture::{
    PanicDropCheckpoint, PanicProbeAndDropCheckpoint, PanicProbeCheckpoint,
    RestoreExecutionDropPanicCheckpoint, RestoreFailureCheckpoint, RestorePanicCheckpoint,
    YieldCheckpoint,
};
use super::*;

#[derive(Clone, Copy)]
pub(super) enum YieldSuspension {
    Checkpoint {
        retained_bytes: u64,
    },
    CheckpointProbePanic,
    CheckpointDropPanic {
        retained_bytes: u64,
    },
    CheckpointProbeAndDropPanic,
    CheckpointRestoreFailure {
        retained_bytes: u64,
    },
    CheckpointRestorePanic {
        retained_bytes: u64,
    },
    CheckpointRestoreExecutionDropPanic {
        retained_bytes: u64,
        checkpoint_drop_panics: bool,
    },
    Failure,
    Panic,
}

#[derive(Clone, Copy)]
pub(super) struct YieldProvider {
    yield_installed: bool,
    checkpoint_available: bool,
    record_effect: bool,
    suspension: YieldSuspension,
    execution_drop_panics: bool,
}

impl YieldProvider {
    pub(super) const fn installed(retained_bytes: u64) -> Self {
        Self {
            yield_installed: true,
            checkpoint_available: true,
            record_effect: false,
            suspension: YieldSuspension::Checkpoint { retained_bytes },
            execution_drop_panics: false,
        }
    }

    pub(super) const fn installed_with_partial_effect(retained_bytes: u64) -> Self {
        Self {
            yield_installed: true,
            checkpoint_available: true,
            record_effect: true,
            suspension: YieldSuspension::Checkpoint { retained_bytes },
            execution_drop_panics: false,
        }
    }

    pub(super) const fn without_installed_yield() -> Self {
        Self {
            yield_installed: false,
            checkpoint_available: true,
            record_effect: false,
            suspension: YieldSuspension::Checkpoint { retained_bytes: 3 },
            execution_drop_panics: false,
        }
    }

    pub(super) const fn without_checkpoint_evidence() -> Self {
        Self {
            yield_installed: true,
            checkpoint_available: false,
            record_effect: false,
            suspension: YieldSuspension::Checkpoint { retained_bytes: 3 },
            execution_drop_panics: false,
        }
    }

    pub(super) const fn suspension_failure() -> Self {
        Self {
            yield_installed: true,
            checkpoint_available: true,
            record_effect: false,
            suspension: YieldSuspension::Failure,
            execution_drop_panics: false,
        }
    }

    pub(super) const fn suspension_panic() -> Self {
        Self {
            yield_installed: true,
            checkpoint_available: true,
            record_effect: false,
            suspension: YieldSuspension::Panic,
            execution_drop_panics: false,
        }
    }

    pub(super) const fn checkpoint_probe_panic() -> Self {
        Self {
            yield_installed: true,
            checkpoint_available: true,
            record_effect: false,
            suspension: YieldSuspension::CheckpointProbePanic,
            execution_drop_panics: false,
        }
    }

    pub(super) const fn checkpoint_drop_panic() -> Self {
        Self {
            yield_installed: true,
            checkpoint_available: true,
            record_effect: false,
            suspension: YieldSuspension::CheckpointDropPanic { retained_bytes: 3 },
            execution_drop_panics: false,
        }
    }

    pub(super) const fn checkpoint_restore_failure(retained_bytes: u64) -> Self {
        Self {
            yield_installed: true,
            checkpoint_available: true,
            record_effect: false,
            suspension: YieldSuspension::CheckpointRestoreFailure { retained_bytes },
            execution_drop_panics: false,
        }
    }

    pub(super) const fn checkpoint_restore_panic(retained_bytes: u64) -> Self {
        Self {
            yield_installed: true,
            checkpoint_available: true,
            record_effect: false,
            suspension: YieldSuspension::CheckpointRestorePanic { retained_bytes },
            execution_drop_panics: false,
        }
    }

    pub(super) const fn restored_execution_drop_panic(retained_bytes: u64) -> Self {
        Self {
            yield_installed: true,
            checkpoint_available: true,
            record_effect: false,
            suspension: YieldSuspension::CheckpointRestoreExecutionDropPanic {
                retained_bytes,
                checkpoint_drop_panics: false,
            },
            execution_drop_panics: false,
        }
    }

    pub(super) const fn checkpoint_and_restored_execution_drop_panic(retained_bytes: u64) -> Self {
        Self {
            yield_installed: true,
            checkpoint_available: true,
            record_effect: false,
            suspension: YieldSuspension::CheckpointRestoreExecutionDropPanic {
                retained_bytes,
                checkpoint_drop_panics: true,
            },
            execution_drop_panics: false,
        }
    }

    pub(super) const fn checkpoint_probe_and_drop_panic() -> Self {
        Self {
            yield_installed: true,
            checkpoint_available: true,
            record_effect: false,
            suspension: YieldSuspension::CheckpointProbeAndDropPanic,
            execution_drop_panics: false,
        }
    }

    pub(super) const fn suspension_and_execution_drop_panic() -> Self {
        Self {
            yield_installed: true,
            checkpoint_available: true,
            record_effect: false,
            suspension: YieldSuspension::Panic,
            execution_drop_panics: true,
        }
    }

    pub(super) const fn suspension_failure_and_execution_drop_panic() -> Self {
        Self {
            yield_installed: true,
            checkpoint_available: true,
            record_effect: false,
            suspension: YieldSuspension::Failure,
            execution_drop_panics: true,
        }
    }

    pub(super) const fn checkpoint_and_execution_drop_panic(checkpoint_drop_panics: bool) -> Self {
        Self {
            yield_installed: true,
            checkpoint_available: true,
            record_effect: false,
            suspension: if checkpoint_drop_panics {
                YieldSuspension::CheckpointDropPanic { retained_bytes: 3 }
            } else {
                YieldSuspension::Checkpoint { retained_bytes: 3 }
            },
            execution_drop_panics: true,
        }
    }

    pub(super) const fn over_ceiling_checkpoint_with_drop_panic(retained_bytes: u64) -> Self {
        Self {
            yield_installed: true,
            checkpoint_available: true,
            record_effect: false,
            suspension: YieldSuspension::CheckpointDropPanic { retained_bytes },
            execution_drop_panics: false,
        }
    }
}

pub(super) struct YieldExecution {
    step_ordinal: u8,
    checkpoint_available: bool,
    record_effect: bool,
    suspension: YieldSuspension,
    execution_drop_panics: bool,
}

impl YieldExecution {
    pub(super) const fn restored() -> Self {
        Self {
            step_ordinal: 1,
            checkpoint_available: false,
            record_effect: false,
            suspension: YieldSuspension::Failure,
            execution_drop_panics: false,
        }
    }

    pub(super) const fn restored_with_drop_panic() -> Self {
        Self {
            step_ordinal: 1,
            checkpoint_available: false,
            record_effect: false,
            suspension: YieldSuspension::Failure,
            execution_drop_panics: true,
        }
    }
}

impl Drop for YieldExecution {
    fn drop(&mut self) {
        if self.execution_drop_panics {
            panic!("yield fixture provider execution destructor panicked")
        }
    }
}

impl WorthQueryGraphProviderExecution for YieldExecution {
    fn advance(
        &mut self,
        step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        for _ in 0..2 {
            step.perform_work_unit(|| Ok(()))?;
        }
        if self.step_ordinal == 0 {
            if self.record_effect {
                step.apply_effect(|| Ok(()))?;
            }
            step.observe_retained_bytes(3).map_err(step_failure)?;
            if self.checkpoint_available {
                step.record_checkpoint_available().map_err(step_failure)?;
            }
            self.step_ordinal = 1;
            return Ok(WorthQueryGraphProviderStepDisposition::continue_work());
        }
        WorthQueryGraphProviderStepDisposition::complete("yield-fixture-complete")
            .map_err(WorthQueryGraphProviderFailure::new)
    }

    fn suspend(
        &mut self,
    ) -> Result<
        Box<dyn crate::domain_computation::WorthQueryGraphProviderCheckpoint>,
        WorthQueryGraphProviderFailure,
    > {
        match self.suspension {
            YieldSuspension::Checkpoint { retained_bytes } => {
                Ok(Box::new(YieldCheckpoint { retained_bytes }))
            }
            YieldSuspension::CheckpointProbePanic => Ok(Box::new(PanicProbeCheckpoint)),
            YieldSuspension::CheckpointDropPanic { retained_bytes } => {
                Ok(Box::new(PanicDropCheckpoint { retained_bytes }))
            }
            YieldSuspension::CheckpointProbeAndDropPanic => {
                Ok(Box::new(PanicProbeAndDropCheckpoint))
            }
            YieldSuspension::CheckpointRestoreFailure { retained_bytes } => {
                Ok(Box::new(RestoreFailureCheckpoint { retained_bytes }))
            }
            YieldSuspension::CheckpointRestorePanic { retained_bytes } => {
                Ok(Box::new(RestorePanicCheckpoint { retained_bytes }))
            }
            YieldSuspension::CheckpointRestoreExecutionDropPanic {
                retained_bytes,
                checkpoint_drop_panics,
            } => Ok(Box::new(RestoreExecutionDropPanicCheckpoint {
                retained_bytes,
                checkpoint_drop_panics,
            })),
            YieldSuspension::Failure => Err(WorthQueryGraphProviderFailure::new(
                "yield fixture suspension failed",
            )),
            YieldSuspension::Panic => panic!("yield fixture suspension panicked"),
        }
    }

    fn dispose(&mut self) -> Result<(), WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

impl WorthQueryGraphParticipationProvider<ManagedGraph> for YieldProvider {
    type Execution = YieldExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport {
        if self.record_effect {
            crate::domain_computation::provider_session::execution_resource_support_with_yield_and_partial_effects(
                "yield-fixture",
                8,
            )
        } else if self.yield_installed {
            crate::domain_computation::provider_session::execution_resource_support_with_yield(
                "yield-fixture",
                8,
            )
        } else {
            crate::domain_computation::provider_session::execution_resource_support(
                "yield-fixture",
                8,
            )
        }
    }

    fn begin(
        &self,
        _call: &WorthQueryGraphProviderCall,
    ) -> Result<Self::Execution, WorthQueryGraphProviderFailure> {
        Ok(YieldExecution {
            step_ordinal: 0,
            checkpoint_available: self.checkpoint_available,
            record_effect: self.record_effect,
            suspension: self.suspension,
            execution_drop_panics: self.execution_drop_panics,
        })
    }
}

fn step_failure(
    denial: crate::domain_computation::WorthQueryGraphProviderStepDenial,
) -> WorthQueryGraphProviderFailure {
    WorthQueryGraphProviderFailure::new(denial.detail())
}
