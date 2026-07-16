use crate::evidence::UiAllocationNeighborhoodScope;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct UiAllocationReceiptLedgerState {
    pub(super) committed_by_scope:
        BTreeMap<UiAllocationNeighborhoodScope, super::UiAllocationReceipt>,
    pub(super) completed_transactions: BTreeMap<u64, Vec<super::UiCommittedAllocationReplan>>,
    pub(super) denied_transactions: BTreeMap<
        u64,
        Vec<(
            super::UiAllocationReplanTransaction,
            super::UiAllocationReplanTransactionCommitDenial,
        )>,
    >,
    pub(super) latest_frame_epoch: Option<crate::runtime::UiAllocationFrameEpoch>,
    pub(super) next_transaction_generation: u64,
    pub(super) runtime_generation: u64,
    pub(super) durable_semantic_state: Option<super::UiAllocationDurableSemanticState>,
    pub(super) truth_revision: super::UiAllocationTruthRevision,
}

impl UiAllocationReceiptLedgerState {
    pub(super) fn initial(runtime_generation: u64) -> Self {
        Self {
            committed_by_scope: BTreeMap::new(),
            completed_transactions: BTreeMap::new(),
            denied_transactions: BTreeMap::new(),
            latest_frame_epoch: None,
            next_transaction_generation: 0,
            runtime_generation,
            durable_semantic_state: None,
            truth_revision: super::UiAllocationTruthRevision::initial(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct UiAllocationCatalogLedgerTransition {
    pub(super) predecessor: UiAllocationReceiptLedgerState,
    pub(super) successor: UiAllocationReceiptLedgerState,
    pub(super) outcome: super::UiCommittedAllocationReplan,
    pub(super) durable_reconciliation: crate::runtime::WorthUiDurableStateReconciliationPlan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiAllocationCatalogLedgerLineage {
    predecessor_truth: super::UiAllocationTruthRevision,
    successor_truth: super::UiAllocationTruthRevision,
    predecessor_transaction_generation: u64,
    successor_transaction_generation: u64,
    runtime_generation: u64,
}

impl UiAllocationCatalogLedgerLineage {
    pub(crate) fn identity_digest(&self) -> u64 {
        self.predecessor_truth.revision()
            ^ self.successor_truth.revision().rotate_left(7)
            ^ self.predecessor_transaction_generation.rotate_left(17)
            ^ self.successor_transaction_generation.rotate_left(29)
            ^ self.runtime_generation.rotate_left(43)
    }
}

impl UiAllocationCatalogLedgerTransition {
    pub(crate) fn committed_outcome(&self) -> &super::UiCommittedAllocationReplan {
        &self.outcome
    }
    pub(crate) fn structural_lineage(&self) -> UiAllocationCatalogLedgerLineage {
        UiAllocationCatalogLedgerLineage {
            predecessor_truth: self.predecessor.truth_revision,
            successor_truth: self.successor.truth_revision,
            predecessor_transaction_generation: self.predecessor.next_transaction_generation,
            successor_transaction_generation: self.successor.next_transaction_generation,
            runtime_generation: self.predecessor.runtime_generation,
        }
    }
    pub(crate) fn durable_reconciliation(
        &self,
    ) -> &crate::runtime::WorthUiDurableStateReconciliationPlan {
        &self.durable_reconciliation
    }
}
