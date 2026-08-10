use crate::{
    failure::{StoreError, StoreErrorKind},
    RawSupportProgramAction, SupportActionBreadthBudget,
};

use super::super::{StateBackedStoreBackend, StatePersistence};
use super::action_record_publication::{
    install_published_support_action_record, persist_pending_support_action_record_update,
    persist_published_support_action_record_update, prepare_pending_support_action_record,
    rollback_pending_support_action_record, rollback_published_support_action_record,
    verify_pending_support_action_record, verify_published_support_action_record,
    PendingSupportActionRecordPreparation,
};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub(super) fn persist_pending_support_action_record(
        &mut self,
        action: &crate::ExecutedSupportAction,
    ) -> Result<(), StoreError> {
        match prepare_pending_support_action_record(self, action)? {
            PendingSupportActionRecordPreparation::AlreadyDurable => Ok(()),
            PendingSupportActionRecordPreparation::Installed(update) => {
                if let Err(error) = verify_pending_support_action_record(self) {
                    rollback_pending_support_action_record(self, update);
                    return Err(error);
                }
                persist_pending_support_action_record_update(self, update)
            }
        }
    }

    pub(super) fn publish_support_action_with_durable_recovery(
        &mut self,
        raw_action: RawSupportProgramAction,
        publication_budget: SupportActionBreadthBudget,
    ) -> Result<crate::CompletedSupportProgramAction, StoreError> {
        let executed = raw_action.plan().verify().execute();
        let envelope_header_bytes = executed.publication_envelope_header_bytes()?;
        if envelope_header_bytes > publication_budget.max_payload_header_bytes() {
            self.state
                .subscription_support_counter_snapshot
                .record_budget_denial();
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportClassificationViolation,
                "subscription-support action envelope exceeds publication budget before materialization",
            ));
        }
        self.persist_pending_support_action_record(&executed)?;
        let published = executed.publish();
        let update = install_published_support_action_record(self, &published)?;
        if let Err(error) = verify_published_support_action_record(self) {
            rollback_published_support_action_record(self, update);
            return Err(error);
        }
        persist_published_support_action_record_update(self, update)?;
        Ok(published.complete())
    }
}
