use worth_store_physical_backend::ControlMediaFault;

use super::archived_workflow_index::ArchivedWorkflowKind;
use super::selected_control_replay::SelectedControlReplay;
use super::selected_control_replay_contract::{
    OperationalControlHistoryViolationKind, StateLookupDenial,
};
use super::selected_control_replay_state::ReplayedWorkflow;
use super::OperationalOperationId;

impl SelectedControlReplay {
    pub(super) fn finish_backup(
        &mut self,
        operation: &OperationalOperationId,
        before_source: OperationalControlHistoryViolationKind,
        released_cut: [u8; 32],
        require_materialization: bool,
    ) -> Result<(), StateLookupDenial> {
        {
            let state = self.state(operation)?;
            let active = match state {
                ReplayedWorkflow::BackupAwaitingSourceLease { .. } => {
                    return Err(StateLookupDenial::Semantic(before_source));
                }
                ReplayedWorkflow::BackupActive(active) => active,
            };
            if require_materialization && !active.materialized {
                return Err(StateLookupDenial::Semantic(
                    OperationalControlHistoryViolationKind::VerificationBeforeMaterialization,
                ));
            }
            if active.recovery.cut_identity() != released_cut {
                return Err(StateLookupDenial::Semantic(
                    OperationalControlHistoryViolationKind::TerminalReleaseCutMismatch,
                ));
            }
        }
        let removed = self.workflows.remove(operation).ok_or({
            StateLookupDenial::DerivedIndex(ControlMediaFault::DerivedTransitionIndexCorrupt)
        })?;
        let ReplayedWorkflow::BackupActive(removed) = removed else {
            return Err(StateLookupDenial::DerivedIndex(
                ControlMediaFault::DerivedTransitionIndexCorrupt,
            ));
        };
        self.active_recovery_object_bytes = self
            .active_recovery_object_bytes
            .checked_sub(removed.recovery_object_bytes)
            .ok_or(StateLookupDenial::DerivedIndex(
                ControlMediaFault::DerivedTransitionIndexCorrupt,
            ))?;
        self.archived
            .insert(operation, ArchivedWorkflowKind::BackupTerminal)
            .map_err(StateLookupDenial::DerivedIndex)?;
        Ok(())
    }
}
