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
        let mut pending_recovery_publications = Vec::new();
        let mut prepared_recovery_publications = Vec::new();
        let mut terminal_recovery_fence_releases = Vec::new();
        pending_recovery_publications
            .try_reserve(self.recovery_publications.len())
            .map_err(|_| SelectedControlReplayDenial::AllocationFailed)?;
        prepared_recovery_publications
            .try_reserve(self.recovery_publications.len())
            .map_err(|_| SelectedControlReplayDenial::AllocationFailed)?;
        terminal_recovery_fence_releases
            .try_reserve(self.recovery_publications.len())
            .map_err(|_| SelectedControlReplayDenial::AllocationFailed)?;
        for (operation, publication) in self.recovery_publications {
            if let Some(handle) = publication.clone().pending_handle(operation.clone()) {
                pending_recovery_publications.push(handle);
            } else if let Some(handle) = publication.clone().prepared_handle(operation.clone()) {
                prepared_recovery_publications.push(handle);
            } else if let Some(handle) = publication.terminal_fence_release_handle(operation) {
                terminal_recovery_fence_releases.push(handle);
            }
        }
        pending_recovery_publications
            .sort_by(|left, right| left.operation_id().cmp(right.operation_id()));
        prepared_recovery_publications
            .sort_by(|left, right| left.operation_id().cmp(right.operation_id()));
        terminal_recovery_fence_releases
            .sort_by(|left, right| left.operation_id().cmp(right.operation_id()));
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
            pending_recovery_publications,
            prepared_recovery_publications,
            terminal_recovery_fence_releases,
            replica_bootstraps,
            replica_promotions,
        })
    }
}
