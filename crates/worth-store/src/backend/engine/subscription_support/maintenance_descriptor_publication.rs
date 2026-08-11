use crate::{
    failure::{StoreError, StoreErrorKind},
    SubscriptionSupportCounterSnapshot, SubscriptionSupportMaintenanceReport,
};

use super::super::core::verify_durable_barrier;
use super::super::{StateBackedStoreBackend, StatePersistence};

#[derive(Debug, Clone)]
pub(super) struct MaintenanceDescriptorPublicationUpdate {
    inserted_keys: Vec<String>,
    previous_counter_snapshot: SubscriptionSupportCounterSnapshot,
}

impl MaintenanceDescriptorPublicationUpdate {
    fn new(previous_counter_snapshot: SubscriptionSupportCounterSnapshot) -> Self {
        Self {
            inserted_keys: Vec::new(),
            previous_counter_snapshot,
        }
    }

    fn rollback<P: StatePersistence>(self, backend: &mut StateBackedStoreBackend<P>) {
        for key in self.inserted_keys {
            backend
                .state
                .subscription_support_maintenance_descriptor_records
                .remove(&key);
        }
        backend.state.subscription_support_counter_snapshot = self.previous_counter_snapshot;
    }
}

pub(super) fn install_subscription_support_maintenance_descriptors<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    report: &SubscriptionSupportMaintenanceReport,
) -> Result<MaintenanceDescriptorPublicationUpdate, StoreError> {
    let mut update = MaintenanceDescriptorPublicationUpdate::new(
        backend.state.subscription_support_counter_snapshot.clone(),
    );
    for record in report.descriptor_records() {
        let key = record.record_key().to_string();
        match backend
            .state
            .subscription_support_maintenance_descriptor_records
            .get(&key)
        {
            Some(existing) if existing == record => {}
            Some(_) => {
                update.rollback(backend);
                return Err(StoreError::new(
                    StoreErrorKind::SubscriptionSupportPublicationViolation,
                    "subscription-support maintenance descriptor record collided with a different durable descriptor row",
                ));
            }
            None => {
                backend
                    .state
                    .subscription_support_maintenance_descriptor_records
                    .insert(key.clone(), record.clone());
                update.inserted_keys.push(key);
            }
        }
    }
    Ok(update)
}

pub(super) fn persist_subscription_support_maintenance_descriptors<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    update: MaintenanceDescriptorPublicationUpdate,
) -> Result<(), StoreError> {
    let persist_report = match backend.persistence.persist_state(&backend.state) {
        Ok(report) => report,
        Err(error) => {
            update.rollback(backend);
            return Err(error);
        }
    };
    verify_durable_barrier(&mut backend.counters, &persist_report)?;
    Ok(())
}
