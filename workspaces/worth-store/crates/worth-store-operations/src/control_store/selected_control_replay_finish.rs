use super::selected_control_replay::SelectedControlReplay;
use super::selected_control_replay_contract::{
    invalid, OperationalControlHistoryViolationKind, ReplayedSelectedControlHistory,
    SelectedControlReplayDenial,
};
use super::selected_control_replay_state::ReplayedWorkflow;
use super::ActiveBackupRecoveryHandle;

impl SelectedControlReplay {
    pub(crate) fn finish(
        self,
    ) -> Result<ReplayedSelectedControlHistory, SelectedControlReplayDenial> {
        let mut active_backups = Vec::new();
        active_backups
            .try_reserve(self.workflows.len())
            .map_err(|_| SelectedControlReplayDenial::AllocationFailed)?;
        for (operation_id, state) in self.workflows {
            match state {
                ReplayedWorkflow::BackupAwaitingSourceLease {
                    opened_record_index,
                } => {
                    return invalid(
                        opened_record_index,
                        operation_id,
                        OperationalControlHistoryViolationKind::WorkflowOpenWithoutDurableSourceLease,
                    );
                }
                ReplayedWorkflow::BackupActive(active) => {
                    active_backups.push(ActiveBackupRecoveryHandle::new(
                        operation_id,
                        *active.recovery,
                        active.materialization_plan,
                    ));
                }
            }
        }
        active_backups.sort_by(|left, right| left.operation_id().cmp(right.operation_id()));
        let mut indeterminate_repairs = self
            .repair_journals
            .into_iter()
            .filter_map(|(operation, journal)| journal.pending_handle(operation))
            .collect::<Vec<_>>();
        indeterminate_repairs.sort_by(|left, right| left.operation_id().cmp(right.operation_id()));
        let mut indeterminate_recovery_staging = self
            .recovery_staging
            .into_iter()
            .map(|(operation, staging)| staging.pending_handle(operation))
            .collect::<Vec<_>>();
        indeterminate_recovery_staging
            .sort_by(|left, right| left.operation_id().cmp(right.operation_id()));
        let mut replica_bootstraps = self
            .replica_bootstraps
            .into_iter()
            .map(|(operation, state)| state.recovery_handle(operation))
            .collect::<Vec<_>>();
        replica_bootstraps.sort_by(|left, right| left.operation_id().cmp(right.operation_id()));
        let mut replica_promotions = self
            .replica_promotions
            .into_iter()
            .map(|(operation, state)| state.recovery_handle(operation))
            .collect::<Vec<_>>();
        replica_promotions.sort_by(|left, right| left.operation_id().cmp(right.operation_id()));
        Ok(ReplayedSelectedControlHistory {
            active_backups,
            completed_backups: self.completed_backups,
            abandoned_backups: self.abandoned_backups,
            indeterminate_repairs,
            indeterminate_recovery_staging,
            replica_bootstraps,
            replica_promotions,
        })
    }
}
