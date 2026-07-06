use crate::runtime::{
    WorthUiAdmittedDurableResizeInput, WorthUiDurableStateFamilyId,
    WorthUiDurableStateReconciliationCounters, WorthUiDurableStateReconciliationReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDurableStateReconciliationPlan {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    receipts: Vec<WorthUiDurableStateReconciliationReceipt>,
    durable_resize_inputs: Vec<WorthUiAdmittedDurableResizeInput>,
    counters: WorthUiDurableStateReconciliationCounters,
}

impl WorthUiDurableStateReconciliationPlan {
    #[cfg(test)]
    pub(crate) fn new(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        receipts: Vec<WorthUiDurableStateReconciliationReceipt>,
        counters: WorthUiDurableStateReconciliationCounters,
    ) -> Self {
        Self::new_with_durable_resize_inputs(
            active_artifact_digest,
            candidate_artifact_digest,
            receipts,
            Vec::new(),
            counters,
        )
    }

    pub(crate) fn new_with_durable_resize_inputs(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        mut receipts: Vec<WorthUiDurableStateReconciliationReceipt>,
        mut durable_resize_inputs: Vec<WorthUiAdmittedDurableResizeInput>,
        counters: WorthUiDurableStateReconciliationCounters,
    ) -> Self {
        receipts.sort_by(|left, right| {
            left.identity_basis()
                .cmp(right.identity_basis())
                .then_with(|| left.family_id().cmp(right.family_id()))
                .then_with(|| left.outcome().cmp(&right.outcome()))
        });
        durable_resize_inputs.sort_by(|left, right| {
            left.identity_basis()
                .cmp(right.identity_basis())
                .then_with(|| left.family_id().cmp(right.family_id()))
                .then_with(|| left.identity_digest().cmp(&right.identity_digest()))
        });
        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            receipts,
            durable_resize_inputs,
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

    pub fn durable_resize_inputs(&self) -> &[WorthUiAdmittedDurableResizeInput] {
        &self.durable_resize_inputs
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

    pub fn durable_resize_input(
        &self,
        identity_basis: &str,
    ) -> Option<&WorthUiAdmittedDurableResizeInput> {
        self.durable_resize_inputs
            .iter()
            .find(|input| input.identity_basis() == identity_basis)
    }

    pub fn admitted_durable_resize_input(
        &self,
        identity_basis: &str,
    ) -> Option<&WorthUiAdmittedDurableResizeInput> {
        self.durable_resize_input(identity_basis)
            .filter(|input| input.is_admitted())
    }

    pub fn counters(&self) -> WorthUiDurableStateReconciliationCounters {
        self.counters
    }
}
