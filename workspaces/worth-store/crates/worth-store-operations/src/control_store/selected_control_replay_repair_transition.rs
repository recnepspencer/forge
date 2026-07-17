use super::{
    archived_workflow_index::ArchivedWorkflowKind,
    repair_control_replay::{observe_disposition, observe_open, observe_receipt, observe_start},
    selected_control_replay_contract::{
        invalid, wrong_workflow, OperationalControlHistoryViolationKind,
        SelectedControlReplayDenial,
    },
    OperationalControlHistoryViolation, OperationalOperationId, OperationalWorkflowKind,
    SelectedControlReplay,
};
use worth_store_authority::StoreCurrentAuthorityIdentity;

impl SelectedControlReplay {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn observe_repair_open(
        &mut self,
        record_index: u64,
        operation: &OperationalOperationId,
        authority_identity: StoreCurrentAuthorityIdentity,
        authorization_identity: [u8; 32],
        plan_fingerprint: [u8; 32],
        owner_node_count: u64,
        topology_tag: u8,
    ) -> Result<(), SelectedControlReplayDenial> {
        if self.consumed_authorizations.get(&authorization_identity)
            != Some(&(plan_fingerprint, operation.clone()))
        {
            return invalid(
                record_index,
                operation.clone(),
                OperationalControlHistoryViolationKind::RepairJournalAuthorizationMismatch,
            );
        }
        observe_open(
            &mut self.repair_journals,
            operation,
            authority_identity,
            authorization_identity,
            plan_fingerprint,
            owner_node_count,
            topology_tag,
        )
        .map_err(|kind| replay_denial(record_index, operation, kind))
    }

    pub(super) fn observe_repair_receipt(
        &mut self,
        record_index: u64,
        operation: &OperationalOperationId,
        plan_fingerprint: [u8; 32],
        node_fingerprint: [u8; 32],
        receipt_fingerprint: [u8; 32],
        owner_tag: u8,
    ) -> Result<(), SelectedControlReplayDenial> {
        observe_receipt(
            &mut self.repair_journals,
            operation,
            plan_fingerprint,
            node_fingerprint,
            receipt_fingerprint,
            owner_tag,
        )
        .map_err(|kind| replay_denial(record_index, operation, kind))
    }

    pub(super) fn observe_repair_start(
        &mut self,
        record_index: u64,
        operation: &OperationalOperationId,
        plan_fingerprint: [u8; 32],
        node_fingerprint: [u8; 32],
        owner_tag: u8,
    ) -> Result<(), SelectedControlReplayDenial> {
        observe_start(
            &mut self.repair_journals,
            operation,
            plan_fingerprint,
            node_fingerprint,
            owner_tag,
        )
        .map_err(|kind| replay_denial(record_index, operation, kind))
    }

    pub(super) fn observe_operational_owner_receipt(
        &mut self,
        record_index: u64,
        operation: &OperationalOperationId,
        workflow: OperationalWorkflowKind,
    ) -> Result<(), SelectedControlReplayDenial> {
        match self
            .archived
            .lookup(operation)
            .map_err(SelectedControlReplayDenial::DerivedIndex)?
        {
            Some(ArchivedWorkflowKind::NonBackup(observed)) if observed == workflow => Ok(()),
            Some(ArchivedWorkflowKind::NonBackup(observed)) => {
                invalid(record_index, operation.clone(), wrong_workflow(observed))
            }
            Some(ArchivedWorkflowKind::BackupTerminal) => invalid(
                record_index,
                operation.clone(),
                OperationalControlHistoryViolationKind::RecordAfterTerminal,
            ),
            None => invalid(
                record_index,
                operation.clone(),
                OperationalControlHistoryViolationKind::RecordBeforeWorkflowOpen,
            ),
        }
    }

    pub(super) fn observe_repair_disposition(
        &mut self,
        record_index: u64,
        operation: &OperationalOperationId,
        plan_fingerprint: [u8; 32],
        disposition_tag: u8,
        disposition_basis: [u8; 32],
    ) -> Result<(), SelectedControlReplayDenial> {
        observe_disposition(
            &mut self.repair_journals,
            operation,
            plan_fingerprint,
            disposition_tag,
            disposition_basis,
        )
        .map_err(|kind| replay_denial(record_index, operation, kind))
    }
}

fn replay_denial(
    record_index: u64,
    operation: &OperationalOperationId,
    kind: OperationalControlHistoryViolationKind,
) -> SelectedControlReplayDenial {
    SelectedControlReplayDenial::Invalid(OperationalControlHistoryViolation::new(
        record_index,
        operation.clone(),
        kind,
    ))
}
