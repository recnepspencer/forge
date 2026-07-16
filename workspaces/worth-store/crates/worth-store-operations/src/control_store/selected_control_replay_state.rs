use super::archived_workflow_index::ArchivedWorkflowKind;
use super::selected_control_replay::{ReplayedBackup, ReplayedWorkflow, SelectedControlReplay};
use super::selected_control_replay_contract::{
    after_terminal, wrong_workflow, OperationalControlHistoryViolationKind, StateLookupDenial,
};
use super::OperationalOperationId;
use worth_store_physical_backend::ControlMediaFault;

impl SelectedControlReplay {
    pub(super) fn state(
        &mut self,
        operation: &OperationalOperationId,
    ) -> Result<&mut ReplayedWorkflow, StateLookupDenial> {
        if self.workflows.contains_key(operation) {
            return self
                .workflows
                .get_mut(operation)
                .ok_or(StateLookupDenial::DerivedIndex(
                    ControlMediaFault::DerivedTransitionIndexCorrupt,
                ));
        }
        match self
            .archived
            .lookup(operation)
            .map_err(StateLookupDenial::DerivedIndex)?
        {
            Some(ArchivedWorkflowKind::BackupTerminal) => {
                Err(StateLookupDenial::Semantic(after_terminal()))
            }
            Some(ArchivedWorkflowKind::NonBackup(workflow)) => {
                Err(StateLookupDenial::Semantic(wrong_workflow(workflow)))
            }
            None => Err(StateLookupDenial::Semantic(
                OperationalControlHistoryViolationKind::RecordBeforeWorkflowOpen,
            )),
        }
    }

    pub(super) fn active_backup(
        &mut self,
        operation: &OperationalOperationId,
        before_source: OperationalControlHistoryViolationKind,
    ) -> Result<&mut ReplayedBackup, StateLookupDenial> {
        match self.state(operation)? {
            ReplayedWorkflow::BackupAwaitingSourceLease { .. } => {
                Err(StateLookupDenial::Semantic(before_source))
            }
            ReplayedWorkflow::BackupActive(active) => Ok(active),
        }
    }
}
