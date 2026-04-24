use crate::backend::records::StoreState;
use crate::failure::{StoreError, StoreErrorKind};

impl StoreState {
    pub(crate) fn verify_subscription_support_record_family(&self) -> Result<(), StoreError> {
        if self
            .subscription_support_counter_snapshot
            .artifacts_published()
            > self.subscription_support_record_sets.len() as u64
        {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                "subscription-support publication counters reference missing durable record sets",
            ));
        }
        for (storage_key, record_set) in &self.subscription_support_record_sets {
            let expected_key = record_set.key().storage_key();
            if storage_key != &expected_key {
                return Err(StoreError::new(
                    StoreErrorKind::SubscriptionSupportPublicationViolation,
                    format!(
                        "subscription-support durable record set key `{storage_key}` drifted from family/artifact key `{expected_key}`"
                    ),
                ));
            }
            record_set.validate()?;
        }
        if self
            .subscription_support_counter_snapshot
            .support_action_envelope_publications()
            < self
                .subscription_support_action_records
                .values()
                .filter(|record| {
                    matches!(
                        record.publication_state(),
                        crate::SupportActionPublicationState::PublishedConsequence
                    )
                })
                .count() as u64
        {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                "subscription-support action publication records exceed the published envelope counter",
            ));
        }
        for (action_id, record) in &self.subscription_support_action_records {
            if record.action_id().as_str() != action_id {
                return Err(StoreError::new(
                    StoreErrorKind::SubscriptionSupportPublicationViolation,
                    "subscription-support action record key drifted from its durable action id",
                ));
            }
            record.validate()?;
        }
        if self
            .subscription_support_counter_snapshot
            .support_maintenance_descriptor_count()
            < self
                .subscription_support_maintenance_descriptor_records
                .len() as u64
        {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                "subscription-support maintenance descriptor records exceed the published maintenance descriptor counter",
            ));
        }
        for (record_key, record) in &self.subscription_support_maintenance_descriptor_records {
            if record.record_key() != record_key {
                return Err(StoreError::new(
                    StoreErrorKind::SubscriptionSupportPublicationViolation,
                    "subscription-support maintenance descriptor record key drifted from its durable map key",
                ));
            }
            let declaration = self
                .maintenance_declaration_records
                .get(record.declaration_id())
                .ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::SubscriptionSupportPublicationViolation,
                        "subscription-support maintenance descriptor record references a missing maintenance declaration",
                    )
                })?;
            record.verify_persisted_descriptor(
                &declaration.declaration,
                &declaration.work_descriptor,
            )?;
        }
        if self
            .subscription_support_counter_snapshot
            .support_maintenance_delay_count()
            < self.subscription_support_maintenance_debt_records.len() as u64
        {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                "subscription-support maintenance debt records exceed the delayed-maintenance counter",
            ));
        }
        for (record_key, record) in &self.subscription_support_maintenance_debt_records {
            if record.record_key() != record_key {
                return Err(StoreError::new(
                    StoreErrorKind::SubscriptionSupportPublicationViolation,
                    "subscription-support maintenance debt record key drifted from its durable map key",
                ));
            }
            record.validate()?;
        }
        Ok(())
    }
}
