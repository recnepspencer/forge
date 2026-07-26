use std::sync::Arc;

use crate::domain_computation::provider_session::graph_provider::bounded_step::WorthQueryProviderCheckpointExportInvocation;

use super::super::{WorthQueryYieldedDirectRun, WorthQueryYieldedWorkflowRun};
use super::{
    WorthQueryCheckpointExportHandoff, WorthQueryCheckpointExportRecoveryKind,
    WorthQueryDirectCheckpointExportFailed, WorthQueryDirectCheckpointExportOutcome,
    WorthQueryDirectCheckpointExportRecoveryRequired, WorthQueryDirectCheckpointExported,
    WorthQueryWorkflowCheckpointExportFailed, WorthQueryWorkflowCheckpointExportOutcome,
    WorthQueryWorkflowCheckpointExportRecoveryRequired, WorthQueryWorkflowCheckpointExported,
};

pub(in crate::domain_computation::managed_run) fn export_direct_checkpoint(
    yielded: WorthQueryYieldedDirectRun,
) -> WorthQueryDirectCheckpointExportOutcome {
    match yielded.execution.checkpoint.invoke_export() {
        WorthQueryProviderCheckpointExportInvocation::Returned(Ok(provider)) => {
            WorthQueryDirectCheckpointExportOutcome::Exported(WorthQueryDirectCheckpointExported {
                handoff: WorthQueryCheckpointExportHandoff::bind_direct(&yielded, provider),
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
                    kind: WorthQueryCheckpointExportRecoveryKind::ProviderExportPanicked,
                    detail: Arc::from("provider checkpoint export panicked"),
                    yielded,
                },
            )
        }
    }
}

pub(in crate::domain_computation::managed_run) fn export_workflow_checkpoint(
    yielded: WorthQueryYieldedWorkflowRun,
) -> WorthQueryWorkflowCheckpointExportOutcome {
    match yielded.execution.checkpoint.invoke_export() {
        WorthQueryProviderCheckpointExportInvocation::Returned(Ok(provider)) => {
            WorthQueryWorkflowCheckpointExportOutcome::Exported(
                WorthQueryWorkflowCheckpointExported {
                    handoff: WorthQueryCheckpointExportHandoff::bind_workflow(&yielded, provider),
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
                    kind: WorthQueryCheckpointExportRecoveryKind::ProviderExportPanicked,
                    detail: Arc::from("provider checkpoint export panicked"),
                    yielded,
                },
            )
        }
    }
}
