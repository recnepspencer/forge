use crate::{
    failure::{StoreError, StoreErrorKind},
    SubscriptionSupportCounterSnapshot, SupportMaintenanceDebtRecord,
};

use super::super::core::verify_durable_barrier;
use super::super::{StateBackedStoreBackend, StatePersistence};
use super::record_family_verification::verify_subscription_support_record_family;

#[derive(Debug, Clone)]
pub(super) enum MaintenanceDebtPublicationDecision {
    AlreadyDurable,
    Install(MaintenanceDebtPublicationUpdate),
}

#[derive(Debug, Clone)]
pub(super) struct MaintenanceDebtPublicationUpdate {
    key: String,
    previous_counter_snapshot: SubscriptionSupportCounterSnapshot,
}

impl MaintenanceDebtPublicationUpdate {
    fn new(key: String, previous_counter_snapshot: SubscriptionSupportCounterSnapshot) -> Self {
        Self {
            key,
            previous_counter_snapshot,
        }
    }

    fn rollback<P: StatePersistence>(self, backend: &mut StateBackedStoreBackend<P>) {
        backend
            .state
            .subscription_support_maintenance_debt_records
            .remove(&self.key);
        backend.state.subscription_support_counter_snapshot = self.previous_counter_snapshot;
    }
}

pub(super) fn rollback_subscription_support_maintenance_debt<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    update: MaintenanceDebtPublicationUpdate,
) {
    update.rollback(backend);
}

pub(super) fn install_subscription_support_maintenance_debt<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    record: SupportMaintenanceDebtRecord,
) -> Result<MaintenanceDebtPublicationDecision, StoreError> {
    let key = record.record_key().to_string();
    match backend
        .state
        .subscription_support_maintenance_debt_records
        .get(&key)
    {
        Some(existing) if existing == &record => Ok(MaintenanceDebtPublicationDecision::AlreadyDurable),
        Some(_) => Err(StoreError::new(
            StoreErrorKind::SubscriptionSupportPublicationViolation,
            "subscription-support maintenance debt record collided with a different durable operator report",
        )),
        None => {
            let previous_counter_snapshot =
                backend.state.subscription_support_counter_snapshot.clone();
            backend
                .state
                .subscription_support_maintenance_debt_records
                .insert(key.clone(), record);
            backend
                .state
                .subscription_support_counter_snapshot
                .record_support_maintenance_delay_report();
            Ok(MaintenanceDebtPublicationDecision::Install(
                MaintenanceDebtPublicationUpdate::new(key, previous_counter_snapshot),
            ))
        }
    }
}

pub(super) fn persist_subscription_support_maintenance_debt<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    update: MaintenanceDebtPublicationUpdate,
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

pub(super) fn verify_subscription_support_maintenance_debt<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> Result<(), StoreError> {
    verify_subscription_support_record_family(backend)
}
