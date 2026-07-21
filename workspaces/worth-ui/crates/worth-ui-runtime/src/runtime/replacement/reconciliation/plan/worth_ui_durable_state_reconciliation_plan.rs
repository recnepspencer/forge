use crate::runtime::{
    WorthUiAdmittedDurableResizeInput, WorthUiDurableResizeInputDisposition,
    WorthUiDurableResizeInputPosture, WorthUiDurableStateFamilyId,
    WorthUiDurableStateReconciliationCounters, WorthUiDurableStateReconciliationReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDurableStateReconciliationPlan {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    receipts: Vec<WorthUiDurableStateReconciliationReceipt>,
    durable_resize_dispositions: Vec<WorthUiDurableResizeInputDisposition>,
    admitted_durable_resize_inputs: Vec<WorthUiAdmittedDurableResizeInput>,
    authority_generation: u64,
    basis_digest: u64,
    counters: WorthUiDurableStateReconciliationCounters,
}

impl WorthUiDurableStateReconciliationPlan {
    pub fn allocation_durable_semantic_state(
        &self,
    ) -> crate::runtime::UiAllocationDurableSemanticState {
        crate::runtime::UiAllocationDurableSemanticState::from_reconciliation(
            self.clone(),
            crate::runtime::replacement::reconciliation::UiAllocationDurableSemanticStateMintAuthority::new(),
        )
    }

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
        mut durable_resize_dispositions: Vec<WorthUiDurableResizeInputDisposition>,
        counters: WorthUiDurableStateReconciliationCounters,
    ) -> Self {
        receipts.sort_by(|left, right| {
            left.identity_basis()
                .cmp(right.identity_basis())
                .then_with(|| left.family_id().cmp(right.family_id()))
                .then_with(|| left.outcome().cmp(&right.outcome()))
        });
        durable_resize_dispositions.sort_by(|left, right| {
            left.identity_basis()
                .cmp(right.identity_basis())
                .then_with(|| left.family_id().cmp(right.family_id()))
                .then_with(|| left.identity_digest().cmp(&right.identity_digest()))
        });
        let authority_generation = durable_resize_dispositions.iter().fold(
            active_artifact_digest.rotate_left(7) ^ candidate_artifact_digest.rotate_left(19),
            |digest, input| digest ^ input.identity_digest().rotate_left(23),
        );
        let admitted_durable_resize_inputs = durable_resize_dispositions
            .iter()
            .filter(|input| {
                input.posture() == WorthUiDurableResizeInputPosture::AdmittedPlanningTimeOnly
            })
            .cloned()
            .map(|input| {
                WorthUiAdmittedDurableResizeInput::from_reconciliation(input, authority_generation)
            })
            .collect();
        let basis_digest =
            crate::runtime::replacement::reconciliation::basis_digest::reconciliation_basis_digest(
                active_artifact_digest,
                candidate_artifact_digest,
                &receipts,
            );
        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            receipts,
            durable_resize_dispositions,
            admitted_durable_resize_inputs,
            authority_generation,
            basis_digest,
            counters,
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }
    pub fn authority_generation(&self) -> u64 {
        self.authority_generation
    }

    pub fn receipts(&self) -> &[WorthUiDurableStateReconciliationReceipt] {
        &self.receipts
    }

    pub(crate) fn basis_digest(&self) -> u64 {
        self.basis_digest
    }

    pub fn durable_resize_inputs(&self) -> &[WorthUiAdmittedDurableResizeInput] {
        &self.admitted_durable_resize_inputs
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
    ) -> Option<&WorthUiDurableResizeInputDisposition> {
        self.durable_resize_dispositions
            .iter()
            .find(|input| input.identity_basis() == identity_basis)
    }

    pub fn admitted_durable_resize_input(
        &self,
        identity_basis: &str,
    ) -> Option<&WorthUiAdmittedDurableResizeInput> {
        self.admitted_durable_resize_inputs
            .iter()
            .find(|input| input.identity_basis() == identity_basis)
    }

    pub fn counters(&self) -> WorthUiDurableStateReconciliationCounters {
        self.counters
    }
}
