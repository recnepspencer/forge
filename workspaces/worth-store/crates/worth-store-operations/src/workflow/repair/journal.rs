use std::collections::HashMap;

use sha2::{Digest, Sha256};
use worth_store_authority::StoreCurrentAuthorityIdentity;

use crate::{
    IndeterminateRepairRecoveryHandle, OperationalControlAppendDenial, OperationalControlRecord,
    OperationalControlStore, OperationalControlStorePort, OperationalOperationId,
    OperationalTransitionId, OwnerPlanNodeIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairExecutionDisposition {
    Executed,
    Abandoned,
    Indeterminate,
    IsolatedForRecovery,
}

#[derive(Debug)]
pub enum RepairJournalDenial {
    Control(OperationalControlAppendDenial),
    Media,
    InvalidHistory,
    PlanMismatch,
    DuplicateOwnerReceipt,
    MissingOwnerReceipts { expected: u64, observed: u64 },
    TransitionIdentity,
}

pub(super) struct RepairExecutionJournal<'a> {
    control: &'a OperationalControlStore,
    authority: StoreCurrentAuthorityIdentity,
    operation: OperationalOperationId,
    plan_fingerprint: [u8; 32],
    expected_owner_nodes: u64,
    topology: crate::RepairRecoveryTopology,
    completed: HashMap<[u8; 32], ([u8; 32], u8)>,
    started: HashMap<[u8; 32], u8>,
    disposition: Option<RepairExecutionDisposition>,
}

impl<'a> RepairExecutionJournal<'a> {
    pub(super) const fn operation_id(&self) -> &OperationalOperationId {
        &self.operation
    }

    pub(super) fn open(
        control: &'a OperationalControlStore,
        authority: StoreCurrentAuthorityIdentity,
        operation: OperationalOperationId,
        authorization_identity: [u8; 32],
        plan_fingerprint: [u8; 32],
        expected_owner_nodes: u64,
        topology: crate::RepairRecoveryTopology,
    ) -> Result<Self, RepairJournalDenial> {
        let mut state = super::journal_replay::scan(control, &operation)?;
        match state.opened {
            Some((observed_authorization, observed_plan, observed_nodes, observed_topology))
                if observed_authorization == authorization_identity
                    && observed_plan == plan_fingerprint
                    && observed_nodes == expected_owner_nodes
                    && observed_topology == topology => {}
            Some(_) => return Err(RepairJournalDenial::PlanMismatch),
            None => {
                let transition = transition_id("repair-open", plan_fingerprint)?;
                control
                    .append(&OperationalControlRecord::repair_execution_opened(
                        authority,
                        operation.clone(),
                        transition,
                        authorization_identity,
                        plan_fingerprint,
                        expected_owner_nodes,
                        topology.tag(),
                    ))
                    .map_err(RepairJournalDenial::Control)?;
                state.opened = Some((
                    authorization_identity,
                    plan_fingerprint,
                    expected_owner_nodes,
                    topology,
                ));
            }
        }
        Ok(Self {
            control,
            authority,
            operation,
            plan_fingerprint,
            expected_owner_nodes,
            topology,
            completed: state.completed,
            started: state.started,
            disposition: state.disposition,
        })
    }

    pub(super) fn completed(&self, node: OwnerPlanNodeIdentity) -> Option<[u8; 32]> {
        self.completed
            .get(&node.fingerprint())
            .map(|(receipt, _)| *receipt)
    }

    pub(super) fn recover(
        control: &'a OperationalControlStore,
        authority: StoreCurrentAuthorityIdentity,
        handle: &IndeterminateRepairRecoveryHandle,
    ) -> Result<Self, RepairJournalDenial> {
        let journal = Self::open(
            control,
            authority,
            handle.operation_id().clone(),
            handle.authorization_identity(),
            handle.plan_fingerprint(),
            handle.expected_owner_nodes(),
            handle.topology(),
        )?;
        if journal.completed.len() != handle.durable_owner_receipts().len()
            || handle.durable_owner_receipts().iter().any(|receipt| {
                journal.completed.get(&receipt.node_fingerprint())
                    != Some(&(receipt.receipt_fingerprint(), receipt.owner_tag()))
            })
        {
            return Err(RepairJournalDenial::InvalidHistory);
        }
        if journal.started.len() != handle.started_owner_nodes().len()
            || handle.started_owner_nodes().iter().any(|started| {
                journal.started.get(&started.node_fingerprint()) != Some(&started.owner_tag())
            })
        {
            return Err(RepairJournalDenial::InvalidHistory);
        }
        Ok(journal)
    }

    pub(super) fn begin_owner_effect(
        &mut self,
        node: OwnerPlanNodeIdentity,
        owner_tag: u8,
    ) -> Result<(), RepairJournalDenial> {
        if let Some(observed) = self.started.get(&node.fingerprint()) {
            return if *observed == owner_tag {
                Ok(())
            } else {
                Err(RepairJournalDenial::InvalidHistory)
            };
        }
        if self.disposition.is_some() || owner_tag == 0 {
            return Err(RepairJournalDenial::InvalidHistory);
        }
        self.started
            .try_reserve(1)
            .map_err(|_| RepairJournalDenial::InvalidHistory)?;
        let transition = transition_id("repair-owner-start", node.fingerprint())?;
        self.control
            .append(&OperationalControlRecord::repair_owner_effect_started(
                self.authority,
                self.operation.clone(),
                transition,
                self.plan_fingerprint,
                node.fingerprint(),
                owner_tag,
            ))
            .map_err(RepairJournalDenial::Control)?;
        self.started.insert(node.fingerprint(), owner_tag);
        Ok(())
    }

    pub(super) fn persist_owner_receipt(
        &mut self,
        node: OwnerPlanNodeIdentity,
        receipt_fingerprint: [u8; 32],
        owner_tag: u8,
    ) -> Result<(), RepairJournalDenial> {
        if let Some(existing) = self.completed(node) {
            return if existing == receipt_fingerprint {
                Ok(())
            } else {
                Err(RepairJournalDenial::DuplicateOwnerReceipt)
            };
        }
        if self.disposition.is_some() {
            return Err(RepairJournalDenial::InvalidHistory);
        }
        if self.started.get(&node.fingerprint()) != Some(&owner_tag) {
            return Err(RepairJournalDenial::InvalidHistory);
        }
        self.completed
            .try_reserve(1)
            .map_err(|_| RepairJournalDenial::InvalidHistory)?;
        let transition = transition_id("repair-owner", node.fingerprint())?;
        self.control
            .append(&OperationalControlRecord::repair_owner_receipt_persisted(
                self.authority,
                self.operation.clone(),
                transition,
                self.plan_fingerprint,
                node.fingerprint(),
                receipt_fingerprint,
                owner_tag,
            ))
            .map_err(RepairJournalDenial::Control)?;
        self.completed
            .insert(node.fingerprint(), (receipt_fingerprint, owner_tag));
        Ok(())
    }

    pub(super) fn close(
        &mut self,
        disposition: RepairExecutionDisposition,
        disposition_basis: [u8; 32],
    ) -> Result<(), RepairJournalDenial> {
        if disposition_basis == [0; 32] {
            return Err(RepairJournalDenial::InvalidHistory);
        }
        if self.disposition == Some(disposition) {
            return Ok(());
        }
        if self.disposition.is_some() {
            return Err(RepairJournalDenial::InvalidHistory);
        }
        if disposition == RepairExecutionDisposition::Executed
            && self.completed.len() as u64 != self.expected_owner_nodes
        {
            return Err(RepairJournalDenial::MissingOwnerReceipts {
                expected: self.expected_owner_nodes,
                observed: self.completed.len() as u64,
            });
        }
        let mutating_starts = self
            .started
            .values()
            .filter(|owner_tag| **owner_tag != 2)
            .count();
        if (disposition == RepairExecutionDisposition::Abandoned && mutating_starts != 0)
            || (disposition == RepairExecutionDisposition::IsolatedForRecovery
                && (self.topology != crate::RepairRecoveryTopology::NonCurrentAuthorityAffecting
                    || mutating_starts == 0))
        {
            return Err(RepairJournalDenial::InvalidHistory);
        }
        let transition = transition_id("repair-close", self.plan_fingerprint)?;
        self.control
            .append(&OperationalControlRecord::repair_disposition_recorded(
                self.authority,
                self.operation.clone(),
                transition,
                self.plan_fingerprint,
                disposition_tag(disposition),
                disposition_basis,
            ))
            .map_err(RepairJournalDenial::Control)?;
        self.disposition = Some(disposition);
        Ok(())
    }

    pub(super) fn completion_basis(&self) -> Result<[u8; 32], RepairJournalDenial> {
        let mut receipts = Vec::new();
        receipts
            .try_reserve_exact(self.completed.len())
            .map_err(|_| RepairJournalDenial::InvalidHistory)?;
        receipts.extend(
            self.completed
                .iter()
                .map(|(node, (receipt, owner))| (*node, *receipt, *owner)),
        );
        receipts.sort_unstable();
        let mut digest = Sha256::new();
        digest.update(b"worth-store-repair-execution-completion-v1");
        digest.update(self.plan_fingerprint);
        digest.update(self.expected_owner_nodes.to_be_bytes());
        digest.update([self.topology.tag()]);
        for (node, receipt, owner) in receipts {
            digest.update(node);
            digest.update(receipt);
            digest.update([owner]);
        }
        Ok(digest.finalize().into())
    }
}

fn transition_id(
    prefix: &str,
    fingerprint: [u8; 32],
) -> Result<OperationalTransitionId, RepairJournalDenial> {
    OperationalTransitionId::new(format!("{prefix}-{}", hex_prefix(fingerprint)))
        .map_err(|_| RepairJournalDenial::TransitionIdentity)
}
fn hex_prefix(value: [u8; 32]) -> String {
    value[..12]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
const fn disposition_tag(value: RepairExecutionDisposition) -> u8 {
    match value {
        RepairExecutionDisposition::Executed => 1,
        RepairExecutionDisposition::Abandoned => 2,
        RepairExecutionDisposition::Indeterminate => 3,
        RepairExecutionDisposition::IsolatedForRecovery => 4,
    }
}
