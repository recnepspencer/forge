use crate::{
    failure::{StoreError, StoreErrorKind},
    PublishableSubscriptionSupportArtifact, PublishedSubscriptionSupportArtifact,
    SubscriptionSupportActionPublicationRecoveryReport, SupportActionId,
};

use super::super::{StateBackedStoreBackend, StatePersistence};
use super::action_recovery::{
    apply_support_action_recovery_transition, decide_support_action_recovery_transition,
    recovery_report_from_record,
};
use super::record_set_publication::{
    install_subscription_support_record_set, persist_subscription_support_record_set_update,
    record_malformed_subscription_support_record, record_set_update_requires_family_verification,
    rollback_subscription_support_record_set_update,
    verify_installed_subscription_support_record_set,
};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn publish_subscription_support(
        &mut self,
        publishable: PublishableSubscriptionSupportArtifact,
    ) -> Result<PublishedSubscriptionSupportArtifact, StoreError> {
        let published = PublishedSubscriptionSupportArtifact::new(publishable.clone())?;
        let update = install_subscription_support_record_set(self, &publishable, &published)?;
        if record_set_update_requires_family_verification(&update) {
            if let Err(error) = verify_installed_subscription_support_record_set(self) {
                rollback_subscription_support_record_set_update(self, update);
                record_malformed_subscription_support_record(self);
                return Err(error);
            }
        }
        persist_subscription_support_record_set_update(self, update)?;
        Ok(published)
    }

    pub fn persist_subscription_support_executed_action_for_publication(
        &mut self,
        action: crate::ExecutedSupportAction,
    ) -> Result<(), StoreError> {
        self.persist_pending_support_action_record(&action)
    }

    pub fn recover_subscription_support_action_publication(
        &mut self,
        action_id: SupportActionId,
    ) -> Result<SubscriptionSupportActionPublicationRecoveryReport, StoreError> {
        let key = action_id.as_str().to_string();
        let record = self
            .state
            .subscription_support_action_records
            .get(&key)
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::SubscriptionSupportPublicationViolation,
                    "subscription-support action recovery requires a persisted durable action record",
                )
            })?;
        let transition = decide_support_action_recovery_transition(&record);
        apply_support_action_recovery_transition(self, transition)?;
        let durable_record = self
            .state
            .subscription_support_action_records
            .get(action_id.as_str())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::SubscriptionSupportPublicationViolation,
                    "subscription-support action recovery lost its durable action record",
                )
            })?;
        recovery_report_from_record(durable_record)
    }

    pub fn reject_subscription_support_global_scan_recovery(&mut self) -> Result<(), StoreError> {
        self.state
            .subscription_support_counter_snapshot
            .record_support_global_scan_recovery_rejection();
        Err(StoreError::new(
            StoreErrorKind::SubscriptionSupportClassificationViolation,
            "subscription-support restart recovery must not scan backend residue outside durable action identity",
        ))
    }
}
