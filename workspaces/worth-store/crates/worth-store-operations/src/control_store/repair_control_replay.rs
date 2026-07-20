use std::collections::HashMap;

use super::{
    IndeterminateRepairRecoveryHandle, OperationalControlHistoryViolationKind,
    OperationalOperationId, RecoveredRepairOwnerReceipt, RecoveredRepairOwnerStart,
};

pub(super) struct ReplayedRepairJournal {
    authorization_identity: [u8; 32],
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    plan_fingerprint: [u8; 32],
    expected_owner_nodes: u64,
    topology: super::RepairRecoveryTopology,
    receipts: HashMap<[u8; 32], ([u8; 32], u8)>,
    starts: HashMap<[u8; 32], u8>,
    terminal: bool,
}

impl ReplayedRepairJournal {
    pub(super) fn pending_handle(
        self,
        operation_id: OperationalOperationId,
    ) -> Option<IndeterminateRepairRecoveryHandle> {
        if self.terminal {
            return None;
        }
        let mut receipts = self
            .receipts
            .into_iter()
            .map(|(node, (receipt, owner_tag))| {
                RecoveredRepairOwnerReceipt::new(node, receipt, owner_tag)
            })
            .collect::<Vec<_>>();
        receipts.sort_by_key(|receipt| receipt.node_fingerprint());
        let mut starts = self
            .starts
            .into_iter()
            .map(|(node, owner)| RecoveredRepairOwnerStart::new(node, owner))
            .collect::<Vec<_>>();
        starts.sort_by_key(|started| started.node_fingerprint());
        Some(IndeterminateRepairRecoveryHandle::new(
            operation_id,
            self.authority_identity,
            self.authorization_identity,
            self.plan_fingerprint,
            self.expected_owner_nodes,
            self.topology,
            starts,
            receipts,
        ))
    }
}

pub(super) fn observe_open(
    journals: &mut HashMap<OperationalOperationId, ReplayedRepairJournal>,
    operation: &OperationalOperationId,
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    authorization_identity: [u8; 32],
    plan_fingerprint: [u8; 32],
    expected_owner_nodes: u64,
    topology_tag: u8,
) -> Result<(), OperationalControlHistoryViolationKind> {
    let topology = super::RepairRecoveryTopology::from_tag(topology_tag)
        .ok_or(OperationalControlHistoryViolationKind::RepairPlanFingerprintMismatch)?;
    if authorization_identity == [0; 32] || plan_fingerprint == [0; 32] || expected_owner_nodes == 0
    {
        return Err(OperationalControlHistoryViolationKind::RepairPlanFingerprintMismatch);
    }
    if journals.contains_key(operation) {
        return Err(OperationalControlHistoryViolationKind::DuplicateRepairJournalOpen);
    }
    journals
        .try_reserve(1)
        .map_err(|_| OperationalControlHistoryViolationKind::RepairPlanFingerprintMismatch)?;
    journals.insert(
        operation.clone(),
        ReplayedRepairJournal {
            authorization_identity,
            authority_identity,
            plan_fingerprint,
            expected_owner_nodes,
            topology,
            receipts: HashMap::new(),
            starts: HashMap::new(),
            terminal: false,
        },
    );
    Ok(())
}

pub(super) fn observe_start(
    journals: &mut HashMap<OperationalOperationId, ReplayedRepairJournal>,
    operation: &OperationalOperationId,
    plan_fingerprint: [u8; 32],
    node_fingerprint: [u8; 32],
    owner_tag: u8,
) -> Result<(), OperationalControlHistoryViolationKind> {
    let journal = journals
        .get_mut(operation)
        .ok_or(OperationalControlHistoryViolationKind::RepairRecordBeforeJournalOpen)?;
    validate_active(journal, plan_fingerprint)?;
    if node_fingerprint == [0; 32]
        || owner_tag == 0
        || journal.starts.contains_key(&node_fingerprint)
        || journal.starts.len() as u64 >= journal.expected_owner_nodes
    {
        return Err(OperationalControlHistoryViolationKind::DuplicateRepairOwnerStart);
    }
    journal
        .starts
        .try_reserve(1)
        .map_err(|_| OperationalControlHistoryViolationKind::DuplicateRepairOwnerStart)?;
    journal.starts.insert(node_fingerprint, owner_tag);
    Ok(())
}

pub(super) fn observe_receipt(
    journals: &mut HashMap<OperationalOperationId, ReplayedRepairJournal>,
    operation: &OperationalOperationId,
    plan_fingerprint: [u8; 32],
    node_fingerprint: [u8; 32],
    receipt_fingerprint: [u8; 32],
    owner_tag: u8,
) -> Result<(), OperationalControlHistoryViolationKind> {
    let journal = journals
        .get_mut(operation)
        .ok_or(OperationalControlHistoryViolationKind::RepairRecordBeforeJournalOpen)?;
    validate_active(journal, plan_fingerprint)?;
    if journal.starts.get(&node_fingerprint) != Some(&owner_tag) {
        return Err(OperationalControlHistoryViolationKind::RepairReceiptBeforeOwnerStart);
    }
    if node_fingerprint == [0; 32]
        || receipt_fingerprint == [0; 32]
        || owner_tag == 0
        || journal.receipts.contains_key(&node_fingerprint)
        || journal.receipts.len() as u64 >= journal.expected_owner_nodes
    {
        return Err(OperationalControlHistoryViolationKind::DuplicateRepairOwnerReceipt);
    }
    journal
        .receipts
        .try_reserve(1)
        .map_err(|_| OperationalControlHistoryViolationKind::DuplicateRepairOwnerReceipt)?;
    journal
        .receipts
        .insert(node_fingerprint, (receipt_fingerprint, owner_tag));
    Ok(())
}

pub(super) fn observe_disposition(
    journals: &mut HashMap<OperationalOperationId, ReplayedRepairJournal>,
    operation: &OperationalOperationId,
    plan_fingerprint: [u8; 32],
    disposition_tag: u8,
    disposition_basis: [u8; 32],
) -> Result<(), OperationalControlHistoryViolationKind> {
    let journal = journals
        .get_mut(operation)
        .ok_or(OperationalControlHistoryViolationKind::RepairRecordBeforeJournalOpen)?;
    validate_active(journal, plan_fingerprint)?;
    let mutating_starts = journal
        .starts
        .values()
        .filter(|owner_tag| **owner_tag != 2)
        .count();
    if disposition_tag == 1 && journal.receipts.len() as u64 != journal.expected_owner_nodes {
        return Err(OperationalControlHistoryViolationKind::RepairCompletedBeforeAllOwnerReceipts);
    }
    if (disposition_tag == 2 && mutating_starts != 0)
        || (disposition_tag == 4
            && (journal.topology != super::RepairRecoveryTopology::NonCurrentAuthorityAffecting
                || mutating_starts == 0))
    {
        return Err(OperationalControlHistoryViolationKind::RepairPlanFingerprintMismatch);
    }
    if !(1..=4).contains(&disposition_tag) || disposition_basis == [0; 32] {
        return Err(OperationalControlHistoryViolationKind::RepairPlanFingerprintMismatch);
    }
    journal.terminal = true;
    Ok(())
}

fn validate_active(
    journal: &ReplayedRepairJournal,
    plan_fingerprint: [u8; 32],
) -> Result<(), OperationalControlHistoryViolationKind> {
    if journal.terminal {
        Err(OperationalControlHistoryViolationKind::RepairRecordAfterDisposition)
    } else if journal.plan_fingerprint != plan_fingerprint {
        Err(OperationalControlHistoryViolationKind::RepairPlanFingerprintMismatch)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_journal_becomes_an_exact_sorted_recovery_handle() {
        let operation = OperationalOperationId::new("repair-recovery-handle").unwrap();
        let mut journals = HashMap::new();
        observe_open(
            &mut journals,
            &operation,
            authority(),
            [7; 32],
            [1; 32],
            3,
            1,
        )
        .unwrap();
        observe_start(&mut journals, &operation, [1; 32], [8; 32], 5).unwrap();
        observe_receipt(&mut journals, &operation, [1; 32], [8; 32], [9; 32], 5).unwrap();
        observe_start(&mut journals, &operation, [1; 32], [2; 32], 2).unwrap();
        observe_receipt(&mut journals, &operation, [1; 32], [2; 32], [3; 32], 2).unwrap();

        let handle = journals
            .remove(&operation)
            .unwrap()
            .pending_handle(operation.clone())
            .unwrap();
        assert_eq!(handle.operation_id(), &operation);
        assert_eq!(handle.authorization_identity(), [7; 32]);
        assert_eq!(handle.plan_fingerprint(), [1; 32]);
        assert_eq!(handle.expected_owner_nodes(), 3);
        assert_eq!(handle.unapplied_owner_nodes(), 1);
        assert_eq!(
            handle.durable_owner_receipts()[0].node_fingerprint(),
            [2; 32]
        );
        assert_eq!(handle.durable_owner_receipts()[1].owner_tag(), 5);
    }

    #[test]
    fn terminal_journal_cannot_masquerade_as_indeterminate_recovery() {
        let operation = OperationalOperationId::new("terminal-repair").unwrap();
        let mut journals = HashMap::new();
        observe_open(
            &mut journals,
            &operation,
            authority(),
            [7; 32],
            [1; 32],
            1,
            1,
        )
        .unwrap();
        observe_start(&mut journals, &operation, [1; 32], [2; 32], 5).unwrap();
        observe_receipt(&mut journals, &operation, [1; 32], [2; 32], [3; 32], 5).unwrap();
        observe_disposition(&mut journals, &operation, [1; 32], 1, [5; 32]).unwrap();
        assert!(journals
            .remove(&operation)
            .unwrap()
            .pending_handle(operation)
            .is_none());
    }

    #[test]
    fn receipt_before_start_and_duplicate_start_are_rejected_without_state_rewrite() {
        let operation = OperationalOperationId::new("hostile-repair-order").unwrap();
        let mut journals = HashMap::new();
        observe_open(
            &mut journals,
            &operation,
            authority(),
            [7; 32],
            [1; 32],
            1,
            1,
        )
        .unwrap();
        assert_eq!(
            observe_receipt(&mut journals, &operation, [1; 32], [2; 32], [3; 32], 5),
            Err(OperationalControlHistoryViolationKind::RepairReceiptBeforeOwnerStart)
        );
        observe_start(&mut journals, &operation, [1; 32], [2; 32], 5).unwrap();
        assert_eq!(
            observe_start(&mut journals, &operation, [1; 32], [2; 32], 6),
            Err(OperationalControlHistoryViolationKind::DuplicateRepairOwnerStart)
        );
        observe_receipt(&mut journals, &operation, [1; 32], [2; 32], [3; 32], 5)
            .expect("duplicate denial must not rewrite the durable owner binding");
    }

    #[test]
    fn plan_node_and_owner_receipt_mutants_fail_closed_without_poisoning_retry() {
        let operation = OperationalOperationId::new("repair-receipt-mutants").unwrap();
        let mut journals = HashMap::new();
        observe_open(
            &mut journals,
            &operation,
            authority(),
            [7; 32],
            [1; 32],
            1,
            1,
        )
        .unwrap();
        observe_start(&mut journals, &operation, [1; 32], [2; 32], 5).unwrap();
        for (plan, node, owner) in [
            ([9; 32], [2; 32], 5),
            ([1; 32], [8; 32], 5),
            ([1; 32], [2; 32], 6),
        ] {
            assert!(
                observe_receipt(&mut journals, &operation, plan, node, [3; 32], owner,).is_err()
            );
        }
        observe_receipt(&mut journals, &operation, [1; 32], [2; 32], [3; 32], 5)
            .expect("mutant denials cannot rewrite the exact durable start binding");
    }

    #[test]
    fn terminal_disposition_must_match_the_mutation_topology() {
        let operation = OperationalOperationId::new("current-repair-abandon").unwrap();
        let mut current = HashMap::new();
        observe_open(
            &mut current,
            &operation,
            authority(),
            [7; 32],
            [1; 32],
            1,
            1,
        )
        .unwrap();
        observe_start(&mut current, &operation, [1; 32], [2; 32], 5).unwrap();
        assert!(observe_disposition(&mut current, &operation, [1; 32], 2, [4; 32]).is_err());
        assert!(observe_disposition(&mut current, &operation, [1; 32], 4, [4; 32]).is_err());

        let operation = OperationalOperationId::new("isolated-repair-retain").unwrap();
        let mut isolated = HashMap::new();
        observe_open(
            &mut isolated,
            &operation,
            authority(),
            [7; 32],
            [1; 32],
            1,
            2,
        )
        .unwrap();
        observe_start(&mut isolated, &operation, [1; 32], [2; 32], 5).unwrap();
        observe_disposition(&mut isolated, &operation, [1; 32], 4, [4; 32])
            .expect("non-current mutating residue has an explicit isolation disposition");
    }

    fn authority() -> worth_store_authority::StoreCurrentAuthorityIdentity {
        worth_store_authority::StoreCurrentAuthorityIdentity::from_persisted_fingerprint([4; 32])
    }
}
