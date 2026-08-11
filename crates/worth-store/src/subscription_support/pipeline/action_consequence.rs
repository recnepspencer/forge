use super::super::{
    classification_error, ExecutedSupportAction, PublishedSupportConsequence,
    SupportActionBreadthBudget,
};
use super::SubscriptionSupportPublicationPipeline;
use crate::failure::StoreError;

impl SubscriptionSupportPublicationPipeline {
    pub fn publish_support_consequence(
        &mut self,
        action: ExecutedSupportAction,
        publication_budget: SupportActionBreadthBudget,
    ) -> Result<PublishedSupportConsequence, StoreError> {
        let envelope_header_bytes = action.publication_envelope_header_bytes()?;
        if envelope_header_bytes > publication_budget.max_payload_header_bytes() {
            self.counters.record_budget_denial();
            return Err(classification_error(
                "subscription-support action envelope exceeds publication budget before materialization",
            ));
        }
        self.counters.record_support_action_envelope_publication();
        Ok(action.publish())
    }

    pub fn reject_support_global_scan_recovery(&mut self) -> Result<(), StoreError> {
        self.counters
            .record_support_global_scan_recovery_rejection();
        Err(classification_error(
            "subscription-support restart recovery must not scan backend residue outside durable action identity",
        ))
    }
}
