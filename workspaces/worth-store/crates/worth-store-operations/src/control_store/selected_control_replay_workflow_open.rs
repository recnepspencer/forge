use super::archived_workflow_index::ArchivedWorkflowKind;
use super::selected_control_replay::SelectedControlReplay;
use super::selected_control_replay_contract::{
    invalid, OperationalControlHistoryViolationKind, SelectedControlReplayDenial,
};
use super::selected_control_replay_state::ReplayedWorkflow;
use super::{OperationalControlReplayResource, OperationalOperationId, OperationalWorkflowKind};

impl SelectedControlReplay {
    pub(super) fn observe_workflow_open(
        &mut self,
        record_index: u64,
        operation: OperationalOperationId,
        workflow: OperationalWorkflowKind,
    ) -> Result<(), SelectedControlReplayDenial> {
        let archived = self
            .archived
            .lookup(&operation)
            .map_err(SelectedControlReplayDenial::DerivedIndex)?;
        if self.workflows.contains_key(&operation) || archived.is_some() {
            return invalid(
                record_index,
                operation,
                OperationalControlHistoryViolationKind::DuplicateWorkflowOpen,
            );
        }
        if workflow != OperationalWorkflowKind::Backup {
            return self
                .archived
                .insert(&operation, ArchivedWorkflowKind::NonBackup(workflow))
                .map_err(SelectedControlReplayDenial::DerivedIndex);
        }

        let required = self.workflows.len().saturating_add(1);
        if required > self.budget.max_active_workflows() {
            return Err(SelectedControlReplayDenial::BudgetExceeded {
                resource: OperationalControlReplayResource::ActiveWorkflows,
                required: u64::try_from(required).unwrap_or(u64::MAX),
                limit: u64::try_from(self.budget.max_active_workflows()).unwrap_or(u64::MAX),
            });
        }
        self.workflows
            .try_reserve(1)
            .map_err(|_| SelectedControlReplayDenial::AllocationFailed)?;
        self.workflows.insert(
            operation,
            ReplayedWorkflow::BackupAwaitingSourceLease {
                opened_record_index: record_index,
            },
        );
        Ok(())
    }
}
