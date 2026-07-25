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
    pub(crate) fn initial_mounted(
        artifact_digest: u64,
        durable_resize_inputs: Vec<WorthUiDurableResizeInputDisposition>,
        counters: WorthUiDurableStateReconciliationCounters,
    ) -> Self {
        Self::new_with_durable_resize_inputs(
            artifact_digest,
            artifact_digest,
            Vec::new(),
            durable_resize_inputs,
            counters,
        )
    }

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
        let basis_digest =
            crate::runtime::replacement::reconciliation::basis_digest::reconciliation_basis_digest(
                active_artifact_digest,
                candidate_artifact_digest,
                &receipts,
                &durable_resize_dispositions,
            );
        let authority_generation = basis_digest.rotate_left(17) ^ 0x6475_7261_626c_6501;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::replacement::reconciliation::plan::WorthUiDurableResizeInputDispositionInput;

    #[test]
    fn durable_resize_shape_changes_reconciliation_basis_and_authority_generation() {
        let first = WorthUiDurableStateReconciliationPlan::initial_mounted(
            41,
            vec![resize_disposition(101)],
            WorthUiDurableStateReconciliationCounters::default(),
        );
        let second = WorthUiDurableStateReconciliationPlan::initial_mounted(
            41,
            vec![resize_disposition(202)],
            WorthUiDurableStateReconciliationCounters::default(),
        );

        assert_ne!(first.basis_digest(), second.basis_digest());
        assert_ne!(first.authority_generation(), second.authority_generation());
        assert_eq!(
            first.durable_resize_inputs()[0].authority_generation(),
            first.authority_generation()
        );
    }

    fn resize_disposition(resize_shape_digest: u64) -> WorthUiDurableResizeInputDisposition {
        WorthUiDurableResizeInputDisposition::new(WorthUiDurableResizeInputDispositionInput {
            identity_basis: "workspace.splitter".to_owned(),
            authored_provenance_digest: Some(17),
            family_id: WorthUiDurableStateFamilyId::SplitterPosition,
            transition: crate::runtime::WorthUiNodeLifecycleTransition::Create,
            resize_permission: crate::capability::MosaicResizePermission::UserResizable,
            resize_contract_id: crate::capability::MosaicSizingContractId::new(
                "workspace.sizing.splitter",
            )
            .expect("test sizing contract is valid"),
            resize_shape_digest,
            posture: WorthUiDurableResizeInputPosture::AdmittedPlanningTimeOnly,
        })
    }
}
