use crate::runtime::{
    WorthUiDurableStateFamilyId, WorthUiDurableStateReconciliationCounters,
    WorthUiDurableStateReconciliationReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDurableStateReconciliationPlan {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    receipts: Vec<WorthUiDurableStateReconciliationReceipt>,
    counters: WorthUiDurableStateReconciliationCounters,
}

impl WorthUiDurableStateReconciliationPlan {
    pub(crate) fn new(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        mut receipts: Vec<WorthUiDurableStateReconciliationReceipt>,
        counters: WorthUiDurableStateReconciliationCounters,
    ) -> Self {
        receipts.sort_by(|left, right| {
            left.identity_basis()
                .cmp(right.identity_basis())
                .then_with(|| left.family_id().cmp(right.family_id()))
                .then_with(|| left.outcome().cmp(&right.outcome()))
        });
        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            receipts,
            counters,
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn receipts(&self) -> &[WorthUiDurableStateReconciliationReceipt] {
        &self.receipts
    }

    pub fn receipt_for(
        &self,
        identity_basis: &str,
        family_id: &WorthUiDurableStateFamilyId,
    ) -> Option<&WorthUiDurableStateReconciliationReceipt> {
        self.receipts
            .binary_search_by(|receipt| {
                receipt
                    .identity_basis()
                    .cmp(identity_basis)
                    .then_with(|| receipt.family_id().cmp(family_id))
            })
            .ok()
            .and_then(|index| self.receipts.get(index))
    }

    pub fn counters(&self) -> WorthUiDurableStateReconciliationCounters {
        self.counters
    }
}
