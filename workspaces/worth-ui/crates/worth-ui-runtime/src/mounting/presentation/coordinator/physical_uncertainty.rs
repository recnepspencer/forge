use super::UiMountedPresentationCoordinator;

impl UiMountedPresentationCoordinator {
    pub(super) fn retain_semantic_uncertainty(
        &mut self,
        attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
        receipts: Vec<worth_ui_query_binding::WorthUiPresentationRecoveryReceipt>,
    ) {
        self.unresolved_semantic_recoveries
            .entry(attempt)
            .or_default()
            .extend(receipts);
    }

    pub(super) fn retire_resolved_semantic_recoveries(
        &mut self,
        reconstructed_bindings: &[worth_ui_host_contract::UiSurfaceBindingGeneration],
    ) {
        if reconstructed_bindings.is_empty() {
            return;
        }
        self.unresolved_semantic_receipts.retain(|_, receipts| {
            receipts.retain(|receipt| !reconstructed_bindings.contains(&receipt.binding()));
            !receipts.is_empty()
        });
        self.unresolved_semantic_recoveries.retain(|_, receipts| {
            receipts.retain(|receipt| !reconstructed_bindings.contains(&receipt.binding()));
            !receipts.is_empty()
        });
    }
}
