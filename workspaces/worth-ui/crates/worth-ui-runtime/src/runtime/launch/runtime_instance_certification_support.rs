use super::runtime_instance::WorthUiRuntime;

impl WorthUiRuntime {
    pub(crate) fn detached_allocation_receipt_for_test(
        &self,
        candidate: &crate::runtime::UiAllocationCandidate,
    ) -> crate::runtime::UiAllocationReceipt {
        let candidate = crate::runtime::allocation_receipt::UiNonPortalReceiptLawCandidate::admit(
            candidate.clone(),
        )
        .expect("certification receipt fixture cannot carry portal allocation authority");
        match crate::runtime::allocation_receipt::detached_non_portal_receipt(candidate) {
            crate::runtime::UiAllocationReceiptCommitOutcome::Committed(receipt) => receipt,
            outcome => {
                panic!("certification input must carry admitted allocation planning: {outcome:?}")
            }
        }
    }
}
