use super::*;

impl WorthUiPresentationAsyncOwner {
    pub fn admit_superseded_physical(
        &mut self,
        receipt: &WorthUiPresentationPendingReceipt,
        observation: WorthUiPresentationSupersededPhysicalObservation,
    ) -> Result<WorthUiPresentationPresentedReceipt, WorthUiPresentationSettlementDenial> {
        let (authority, receipt_authority, nonce, _) = observation.into_parts();
        if !correspondence::is_correspondence_authority(
            &self.correspondence_authority,
            &receipt.authority,
        ) {
            return Err(WorthUiPresentationSettlementDenial::ForeignPendingReceiptAuthority);
        }
        if !correspondence::is_correspondence_authority(&self.correspondence_authority, &authority)
        {
            return Err(WorthUiPresentationSettlementDenial::ForeignCompletionAuthority);
        }
        if !correspondence::is_correspondence_authority(&receipt.authority, &receipt_authority)
            || nonce != receipt.nonce
        {
            return Err(WorthUiPresentationSettlementDenial::CompletionReceiptMismatch);
        }
        let key = PresentationAdmissionKey {
            attempt: receipt.attempt,
            binding: receipt.binding,
        };
        if self
            .superseded_pending
            .get(&key)
            .is_some_and(|pending| pending.nonce == receipt.nonce)
        {
            return self.finish_superseded(key, true);
        }
        if let Some(completed) = self
            .superseded_awaiting_completion
            .remove(&(key, receipt.nonce))
        {
            self.record_transition(
                WorthUiPresentationTransitionKind::StaleCompletionRejected,
                key,
            );
            return Ok(completed);
        }
        Err(WorthUiPresentationSettlementDenial::InvalidPendingReceipt)
    }
}
