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
        Ok(())
    }
}
