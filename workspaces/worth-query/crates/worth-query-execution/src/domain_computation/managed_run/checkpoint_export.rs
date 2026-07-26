use std::sync::Arc;

use crate::domain_computation::provider_session::graph_provider::bounded_step::{
    WorthQueryProviderCheckpointExport, WorthQueryProviderCheckpointExportInvocation,
};

use super::{
    WorthQueryDirectYieldCleanupOutcome, WorthQueryWorkflowYieldCleanupOutcome,
    WorthQueryYieldedDirectRun, WorthQueryYieldedWorkflowRun,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCheckpointExportHandoff {
    logical_run_identity: Arc<str>,
    yielded_attempt_identity: Arc<str>,
    operation_binding_identity: Arc<str>,
    installed_operation_identity: Arc<str>,
    installation_generation: u64,
    semantic_basis_identity: Arc<str>,
    provider_generation: u64,
    checkpoint_occurrence_identity: Arc<str>,
    artifact_run_identity: Option<Arc<str>>,
    artifact_production_generation: Option<u64>,
    provider: WorthQueryProviderCheckpointExport,
}

pub enum WorthQueryDirectCheckpointExportOutcome {
    Exported(WorthQueryDirectCheckpointExported),
    Failed(WorthQueryDirectCheckpointExportFailed),
    RecoveryRequired(WorthQueryDirectCheckpointExportRecoveryRequired),
}

pub struct WorthQueryDirectCheckpointExported {
    handoff: WorthQueryCheckpointExportHandoff,
    yielded: WorthQueryYieldedDirectRun,
}

pub struct WorthQueryDirectCheckpointExportFailed {
    detail: Arc<str>,
    yielded: WorthQueryYieldedDirectRun,
}

pub struct WorthQueryDirectCheckpointExportRecoveryRequired {
    detail: Arc<str>,
    yielded: WorthQueryYieldedDirectRun,
}

pub enum WorthQueryWorkflowCheckpointExportOutcome {
    Exported(WorthQueryWorkflowCheckpointExported),
    Failed(WorthQueryWorkflowCheckpointExportFailed),
    RecoveryRequired(WorthQueryWorkflowCheckpointExportRecoveryRequired),
}

pub struct WorthQueryWorkflowCheckpointExported {
    handoff: WorthQueryCheckpointExportHandoff,
    yielded: WorthQueryYieldedWorkflowRun,
}

pub struct WorthQueryWorkflowCheckpointExportFailed {
    detail: Arc<str>,
    yielded: WorthQueryYieldedWorkflowRun,
}

pub struct WorthQueryWorkflowCheckpointExportRecoveryRequired {
    detail: Arc<str>,
    yielded: WorthQueryYieldedWorkflowRun,
}

impl WorthQueryCheckpointExportHandoff {
    pub fn logical_run_identity(&self) -> &str {
        &self.logical_run_identity
    }

    pub fn yielded_attempt_identity(&self) -> &str {
        &self.yielded_attempt_identity
    }

    pub fn operation_binding_identity(&self) -> &str {
        &self.operation_binding_identity
    }

    pub fn installed_operation_identity(&self) -> &str {
        &self.installed_operation_identity
    }

    pub const fn installation_generation(&self) -> u64 {
        self.installation_generation
    }

    pub fn semantic_basis_identity(&self) -> &str {
        &self.semantic_basis_identity
    }

    pub const fn provider_generation(&self) -> u64 {
        self.provider_generation
    }

    pub fn checkpoint_occurrence_identity(&self) -> &str {
        &self.checkpoint_occurrence_identity
    }

    pub fn artifact_run_identity(&self) -> Option<&str> {
        self.artifact_run_identity.as_deref()
    }

    pub const fn artifact_production_generation(&self) -> Option<u64> {
        self.artifact_production_generation
    }

    pub fn provider_export(&self) -> &WorthQueryProviderCheckpointExport {
        &self.provider
    }
}

impl WorthQueryDirectCheckpointExported {
    pub fn handoff(&self) -> &WorthQueryCheckpointExportHandoff {
        &self.handoff
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthQueryCheckpointExportHandoff,
        WorthQueryYieldedDirectRun,
    ) {
        (self.handoff, self.yielded)
    }
}

impl WorthQueryDirectCheckpointExportFailed {
    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn into_yielded(self) -> WorthQueryYieldedDirectRun {
        self.yielded
    }
}

impl WorthQueryDirectCheckpointExportRecoveryRequired {
    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn retained_authority_count(&self) -> usize {
        let _ = (
            self.yielded.checkpoint(),
            self.yielded.resource_attempt_identity(),
            self.yielded.bridge(),
            self.yielded.relational_basis_identity(),
        );
        4
    }

    pub fn cleanup(self) -> WorthQueryDirectYieldCleanupOutcome {
        self.yielded.cleanup()
    }
}

impl WorthQueryWorkflowCheckpointExported {
    pub fn handoff(&self) -> &WorthQueryCheckpointExportHandoff {
        &self.handoff
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthQueryCheckpointExportHandoff,
        WorthQueryYieldedWorkflowRun,
    ) {
        (self.handoff, self.yielded)
    }
}

impl WorthQueryWorkflowCheckpointExportFailed {
    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn into_yielded(self) -> WorthQueryYieldedWorkflowRun {
        self.yielded
    }
}

impl WorthQueryWorkflowCheckpointExportRecoveryRequired {
    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn retained_authority_count(&self) -> usize {
        let _ = (
            self.yielded.checkpoint(),
            self.yielded.resource_attempt_identity(),
            self.yielded.bridge(),
            self.yielded.relational_basis_identity(),
            self.yielded.artifact_run_identity(),
        );
        5
    }

    pub fn cleanup(self) -> WorthQueryWorkflowYieldCleanupOutcome {
        self.yielded.cleanup()
    }
}

pub(super) fn export_direct_checkpoint(
    yielded: WorthQueryYieldedDirectRun,
) -> WorthQueryDirectCheckpointExportOutcome {
    match yielded.execution.checkpoint.invoke_export() {
        WorthQueryProviderCheckpointExportInvocation::Returned(Ok(provider)) => {
            WorthQueryDirectCheckpointExportOutcome::Exported(WorthQueryDirectCheckpointExported {
                handoff: direct_handoff(&yielded, provider),
                yielded,
            })
        }
        WorthQueryProviderCheckpointExportInvocation::Returned(Err(failure)) => {
            WorthQueryDirectCheckpointExportOutcome::Failed(
                WorthQueryDirectCheckpointExportFailed {
                    detail: Arc::from(failure.detail()),
                    yielded,
                },
            )
        }
        WorthQueryProviderCheckpointExportInvocation::Panicked => {
            WorthQueryDirectCheckpointExportOutcome::RecoveryRequired(
                WorthQueryDirectCheckpointExportRecoveryRequired {
                    detail: Arc::from("provider checkpoint export panicked"),
                    yielded,
                },
            )
        }
    }
}

pub(super) fn export_workflow_checkpoint(
    yielded: WorthQueryYieldedWorkflowRun,
) -> WorthQueryWorkflowCheckpointExportOutcome {
    match yielded.execution.checkpoint.invoke_export() {
        WorthQueryProviderCheckpointExportInvocation::Returned(Ok(provider)) => {
            WorthQueryWorkflowCheckpointExportOutcome::Exported(
                WorthQueryWorkflowCheckpointExported {
                    handoff: workflow_handoff(&yielded, provider),
                    yielded,
                },
            )
        }
        WorthQueryProviderCheckpointExportInvocation::Returned(Err(failure)) => {
            WorthQueryWorkflowCheckpointExportOutcome::Failed(
                WorthQueryWorkflowCheckpointExportFailed {
                    detail: Arc::from(failure.detail()),
                    yielded,
                },
            )
        }
        WorthQueryProviderCheckpointExportInvocation::Panicked => {
            WorthQueryWorkflowCheckpointExportOutcome::RecoveryRequired(
                WorthQueryWorkflowCheckpointExportRecoveryRequired {
                    detail: Arc::from("provider checkpoint export panicked"),
                    yielded,
                },
            )
        }
    }
}

fn direct_handoff(
    yielded: &WorthQueryYieldedDirectRun,
    provider: WorthQueryProviderCheckpointExport,
) -> WorthQueryCheckpointExportHandoff {
    WorthQueryCheckpointExportHandoff {
        logical_run_identity: Arc::from(yielded.logical_run_identity()),
        yielded_attempt_identity: Arc::from(yielded.yielded_attempt_identity()),
        operation_binding_identity: Arc::from(yielded.operation_binding_identity()),
        installed_operation_identity: Arc::from(yielded.installed_operation_identity()),
        installation_generation: yielded.installation_generation().ordinal(),
        semantic_basis_identity: Arc::from(yielded.semantic_basis_identity()),
        provider_generation: yielded.checkpoint().provider_generation(),
        checkpoint_occurrence_identity: Arc::from(yielded.checkpoint().identity()),
        artifact_run_identity: None,
        artifact_production_generation: None,
        provider,
    }
}

fn workflow_handoff(
    yielded: &WorthQueryYieldedWorkflowRun,
    provider: WorthQueryProviderCheckpointExport,
) -> WorthQueryCheckpointExportHandoff {
    WorthQueryCheckpointExportHandoff {
        logical_run_identity: Arc::from(yielded.logical_run_identity()),
        yielded_attempt_identity: Arc::from(yielded.yielded_attempt_identity()),
        operation_binding_identity: Arc::from(yielded.operation_binding_identity()),
        installed_operation_identity: Arc::from(yielded.installed_operation_identity()),
        installation_generation: yielded.installation_generation().ordinal(),
        semantic_basis_identity: Arc::from(yielded.semantic_basis_identity()),
        provider_generation: yielded.checkpoint().provider_generation(),
        checkpoint_occurrence_identity: Arc::from(yielded.checkpoint().identity()),
        artifact_run_identity: Some(Arc::from(yielded.artifact_run_identity())),
        artifact_production_generation: Some(yielded.artifact_evidence().production_generation()),
        provider,
    }
}
