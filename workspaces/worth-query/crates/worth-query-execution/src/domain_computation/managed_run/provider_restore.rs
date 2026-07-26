use std::sync::Arc;

use super::managed_graph_execution::{
    WorthQueryManagedGraphExecution, WorthQueryRestoredManagedGraphExecutionParts,
};
use super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution;
use crate::domain_computation::provider_session::graph_provider::bounded_step::{
    WorthQueryGraphProviderRestoreMemory, WorthQueryGraphProviderStepArtifactContext,
    WorthQueryOwnedGraphProviderExecution, WorthQueryProviderCheckpointRestoreInvocation,
    WorthQueryProviderExecutionReleaseEvidence,
};
use crate::domain_computation::{
    WorthQueryGraphProviderCall, WorthQueryProviderCheckpointEvidence,
    WorthQueryProviderCheckpointReleaseEvidence,
};

pub(super) enum WorthQueryManagedGraphRestoreOutcome {
    Pending(WorthQueryManagedGraphRestorePending),
    Denied(WorthQueryManagedGraphRestoreDenied),
    RecoveryRequired(WorthQueryManagedGraphRestoreRecoveryRequired),
}

pub(super) struct WorthQueryManagedGraphRestorePending {
    retained: WorthQueryRetainedManagedGraphExecution,
    fresh_call: WorthQueryGraphProviderCall,
    contract: super::step_contract_admission::WorthQueryAdmittedManagedStepContract,
    restored_execution: WorthQueryOwnedGraphProviderExecution,
}

pub(super) enum WorthQueryManagedGraphRestoreAbortOutcome {
    Aborted(WorthQueryRetainedManagedGraphExecution),
    RecoveryRequired(WorthQueryManagedGraphRestoreRecoveryRequired),
}

pub(super) enum WorthQueryManagedGraphRestoreCommitOutcome {
    Restored(WorthQueryManagedGraphExecution),
    RecoveryRequired(WorthQueryManagedGraphRestoreRecoveryRequired),
}

pub(super) struct WorthQueryManagedGraphRestoreDenied {
    detail: Arc<str>,
    retained: WorthQueryRetainedManagedGraphExecution,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryManagedGraphRestoreRecoveryKind {
    ProviderRestorePanicked,
    RestoredExecutionReleaseRecoveryRequired,
    CheckpointReleasePanicked,
}

pub(super) struct WorthQueryManagedGraphRestoreRecoveryRequired {
    kind: WorthQueryManagedGraphRestoreRecoveryKind,
    detail: Arc<str>,
    resource: WorthQueryManagedGraphRestoreRecoveryResource,
}

pub(super) enum WorthQueryManagedGraphRestoreRecoveryResource {
    Retained(WorthQueryRetainedManagedGraphExecution),
    RetainedAfterRestoredRelease {
        retained: WorthQueryRetainedManagedGraphExecution,
        restored_execution: WorthQueryProviderExecutionReleaseEvidence,
    },
    Released {
        checkpoint: WorthQueryProviderCheckpointReleaseEvidence,
        restored_execution: WorthQueryProviderExecutionReleaseEvidence,
    },
}

pub(super) fn restore(
    retained: WorthQueryRetainedManagedGraphExecution,
    fresh_call: WorthQueryGraphProviderCall,
    contract: super::step_contract_admission::WorthQueryAdmittedManagedStepContract,
) -> WorthQueryManagedGraphRestoreOutcome {
    let mut memory = WorthQueryGraphProviderRestoreMemory::new(retained.memory.clone());
    match retained.checkpoint.invoke_restore(&fresh_call, &mut memory) {
        WorthQueryProviderCheckpointRestoreInvocation::Returned(Err(failure)) => {
            WorthQueryManagedGraphRestoreOutcome::Denied(WorthQueryManagedGraphRestoreDenied {
                detail: Arc::from(failure.detail()),
                retained,
            })
        }
        WorthQueryProviderCheckpointRestoreInvocation::Panicked => {
            WorthQueryManagedGraphRestoreOutcome::RecoveryRequired(
                WorthQueryManagedGraphRestoreRecoveryRequired {
                    kind: WorthQueryManagedGraphRestoreRecoveryKind::ProviderRestorePanicked,
                    detail: Arc::from("provider checkpoint restore panicked"),
                    resource: WorthQueryManagedGraphRestoreRecoveryResource::Retained(retained),
                },
            )
        }
        WorthQueryProviderCheckpointRestoreInvocation::Returned(Ok(execution)) => {
            WorthQueryManagedGraphRestoreOutcome::Pending(WorthQueryManagedGraphRestorePending {
                retained,
                fresh_call,
                contract,
                restored_execution: WorthQueryOwnedGraphProviderExecution::new(execution),
            })
        }
    }
}

impl WorthQueryManagedGraphRestorePending {
    pub(super) fn checkpoint_evidence(&self) -> &WorthQueryProviderCheckpointEvidence {
        self.retained.checkpoint_evidence()
    }

    pub(super) fn abort(self) -> WorthQueryManagedGraphRestoreAbortOutcome {
        let Self {
            retained,
            fresh_call,
            contract: _,
            restored_execution,
        } = self;
        drop(fresh_call);
        let restored_execution = restored_execution.release();
        if restored_execution.recovery_required() {
            WorthQueryManagedGraphRestoreAbortOutcome::RecoveryRequired(
                WorthQueryManagedGraphRestoreRecoveryRequired {
                    kind:
                        WorthQueryManagedGraphRestoreRecoveryKind::RestoredExecutionReleaseRecoveryRequired,
                    detail: Arc::from(
                        "replacement provider execution required physical-release recovery while restore was aborted",
                    ),
                    resource:
                        WorthQueryManagedGraphRestoreRecoveryResource::RetainedAfterRestoredRelease {
                            retained,
                            restored_execution,
                        },
                },
            )
        } else {
            WorthQueryManagedGraphRestoreAbortOutcome::Aborted(retained)
        }
    }

    pub(super) fn commit(
        self,
        artifact_context: Option<WorthQueryGraphProviderStepArtifactContext>,
    ) -> WorthQueryManagedGraphRestoreCommitOutcome {
        let Self {
            retained,
            fresh_call,
            contract,
            restored_execution,
        } = self;
        finish_restore(
            retained,
            fresh_call,
            contract,
            artifact_context,
            restored_execution,
        )
    }
}

fn finish_restore(
    retained: WorthQueryRetainedManagedGraphExecution,
    fresh_call: WorthQueryGraphProviderCall,
    admitted_contract: super::step_contract_admission::WorthQueryAdmittedManagedStepContract,
    artifact_context: Option<WorthQueryGraphProviderStepArtifactContext>,
    restored_execution: WorthQueryOwnedGraphProviderExecution,
) -> WorthQueryManagedGraphRestoreCommitOutcome {
    let WorthQueryRetainedManagedGraphExecution {
        call,
        checkpoint,
        contract: _,
        memory,
        completed_work_units,
        applied_effect_count,
        peak_scratch_bytes,
        retained_bytes,
        projection,
        artifact_context: old_artifact_context,
        produced_artifact_count,
        retained_artifact_count,
        disposed_artifact_count,
    } = retained;
    let anchor = checkpoint.provider_anchor();
    let checkpoint_release = checkpoint.release();
    drop((call, old_artifact_context));
    if checkpoint_release.disposition().recovery_required() {
        let restored_execution = restored_execution.release();
        return WorthQueryManagedGraphRestoreCommitOutcome::RecoveryRequired(
            WorthQueryManagedGraphRestoreRecoveryRequired {
                kind: WorthQueryManagedGraphRestoreRecoveryKind::CheckpointReleasePanicked,
                detail: Arc::from(
                    "provider checkpoint panicked after replacement execution was restored",
                ),
                resource: WorthQueryManagedGraphRestoreRecoveryResource::Released {
                    checkpoint: checkpoint_release,
                    restored_execution,
                },
            },
        );
    }
    WorthQueryManagedGraphRestoreCommitOutcome::Restored(WorthQueryManagedGraphExecution::restored(
        WorthQueryRestoredManagedGraphExecutionParts {
            call: fresh_call,
            execution: restored_execution.into_execution(),
            anchor,
            contract: admitted_contract,
            memory,
            completed_work_units,
            applied_effect_count,
            peak_scratch_bytes,
            retained_bytes,
            projection,
            artifact_context,
            produced_artifact_count,
            retained_artifact_count,
            disposed_artifact_count,
        },
    ))
}

impl WorthQueryManagedGraphRestoreDenied {
    pub(super) fn detail(&self) -> &str {
        &self.detail
    }

    pub(super) fn into_retained(self) -> WorthQueryRetainedManagedGraphExecution {
        self.retained
    }
}

impl WorthQueryManagedGraphRestoreRecoveryRequired {
    pub(super) const fn kind(&self) -> WorthQueryManagedGraphRestoreRecoveryKind {
        self.kind
    }

    pub(super) fn detail(&self) -> &str {
        &self.detail
    }

    pub(super) fn checkpoint_evidence(&self) -> &WorthQueryProviderCheckpointEvidence {
        match &self.resource {
            WorthQueryManagedGraphRestoreRecoveryResource::Retained(retained)
            | WorthQueryManagedGraphRestoreRecoveryResource::RetainedAfterRestoredRelease {
                retained,
                ..
            } => retained.checkpoint_evidence(),
            WorthQueryManagedGraphRestoreRecoveryResource::Released { checkpoint, .. } => {
                checkpoint.checkpoint()
            }
        }
    }

    pub(super) fn checkpoint_release(
        &self,
    ) -> Option<&WorthQueryProviderCheckpointReleaseEvidence> {
        match &self.resource {
            WorthQueryManagedGraphRestoreRecoveryResource::Released { checkpoint, .. } => {
                Some(checkpoint)
            }
            WorthQueryManagedGraphRestoreRecoveryResource::Retained(_)
            | WorthQueryManagedGraphRestoreRecoveryResource::RetainedAfterRestoredRelease {
                ..
            } => None,
        }
    }

    pub(super) const fn checkpoint_retained(&self) -> bool {
        !matches!(
            self.resource,
            WorthQueryManagedGraphRestoreRecoveryResource::Released { .. }
        )
    }

    pub(super) const fn restored_execution_release_evidence(
        &self,
    ) -> Option<&WorthQueryProviderExecutionReleaseEvidence> {
        match &self.resource {
            WorthQueryManagedGraphRestoreRecoveryResource::Retained(_) => None,
            WorthQueryManagedGraphRestoreRecoveryResource::RetainedAfterRestoredRelease {
                restored_execution,
                ..
            }
            | WorthQueryManagedGraphRestoreRecoveryResource::Released {
                restored_execution, ..
            } => Some(restored_execution),
        }
    }

    pub(super) fn into_retained(self) -> Result<WorthQueryRetainedManagedGraphExecution, Self> {
        let Self {
            kind,
            detail,
            resource,
        } = self;
        match resource {
            WorthQueryManagedGraphRestoreRecoveryResource::Retained(retained)
            | WorthQueryManagedGraphRestoreRecoveryResource::RetainedAfterRestoredRelease {
                retained,
                ..
            } => Ok(retained),
            resource @ WorthQueryManagedGraphRestoreRecoveryResource::Released { .. } => {
                Err(Self {
                    kind,
                    detail,
                    resource,
                })
            }
        }
    }
}
