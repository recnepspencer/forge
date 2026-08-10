use crate::{
    failure::{StoreError, StoreErrorKind},
    ExecutedSupportAction, PublishedSupportConsequence, SubscriptionSupportCounterSnapshot,
    SupportActionDurableRecord,
};

use super::super::core::verify_durable_barrier;
use super::super::{StateBackedStoreBackend, StatePersistence};
use super::record_family_verification::verify_subscription_support_record_family;

#[derive(Debug, Clone)]
pub(super) enum PendingSupportActionRecordPreparation {
    AlreadyDurable,
    Installed(PendingSupportActionRecordUpdate),
}

#[derive(Debug, Clone)]
pub(super) struct PendingSupportActionRecordUpdate {
    key: String,
    previous_record: Option<SupportActionDurableRecord>,
}

impl PendingSupportActionRecordUpdate {
    fn new(key: String) -> Self {
        Self {
            key,
            previous_record: None,
        }
    }

    fn rollback<P: StatePersistence>(self, backend: &mut StateBackedStoreBackend<P>) {
        match self.previous_record {
            Some(previous_record) => {
                backend
                    .state
                    .subscription_support_action_records
                    .insert(self.key, previous_record);
            }
            None => {
                backend
                    .state
                    .subscription_support_action_records
                    .remove(&self.key);
            }
        }
    }
}

pub(super) fn rollback_pending_support_action_record<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    update: PendingSupportActionRecordUpdate,
) {
    update.rollback(backend);
}

pub(super) fn prepare_pending_support_action_record<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    action: &ExecutedSupportAction,
) -> Result<PendingSupportActionRecordPreparation, StoreError> {
    let record = SupportActionDurableRecord::from_executed(action);
    let key = record.storage_key();
    match backend
        .state
        .subscription_support_action_records
        .get(&key)
    {
        Some(existing) if existing == &record => Ok(PendingSupportActionRecordPreparation::AlreadyDurable),
        Some(_) => Err(StoreError::new(
            StoreErrorKind::SubscriptionSupportPublicationViolation,
            "subscription-support action record collided with a different durable publication state",
        )),
        None => {
            backend
                .state
                .subscription_support_action_records
                .insert(key.clone(), record);
            Ok(PendingSupportActionRecordPreparation::Installed(
                PendingSupportActionRecordUpdate::new(key),
            ))
        }
    }
}

pub(super) fn verify_pending_support_action_record<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> Result<(), StoreError> {
    verify_subscription_support_record_family(backend)
}

pub(super) fn persist_pending_support_action_record_update<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    update: PendingSupportActionRecordUpdate,
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

#[derive(Debug, Clone)]
pub(super) struct PublishedSupportActionRecordUpdate {
    key: String,
    previous_record: SupportActionDurableRecord,
    previous_counter_snapshot: SubscriptionSupportCounterSnapshot,
}

impl PublishedSupportActionRecordUpdate {
    fn new(
        key: String,
        previous_record: SupportActionDurableRecord,
        previous_counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Self {
        Self {
            key,
            previous_record,
            previous_counter_snapshot,
        }
    }

    fn rollback<P: StatePersistence>(self, backend: &mut StateBackedStoreBackend<P>) {
        backend
            .state
            .subscription_support_action_records
            .insert(self.key, self.previous_record);
        backend.state.subscription_support_counter_snapshot = self.previous_counter_snapshot;
    }
}

pub(super) fn rollback_published_support_action_record<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    update: PublishedSupportActionRecordUpdate,
) {
    update.rollback(backend);
}

pub(super) fn install_published_support_action_record<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    published: &PublishedSupportConsequence,
) -> Result<PublishedSupportActionRecordUpdate, StoreError> {
    let key = published.envelope().action_id().as_str().to_string();
    let previous_record = backend
        .state
        .subscription_support_action_records
        .get(&key)
        .cloned()
        .ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                "subscription-support publication requires a persisted pending action record",
            )
        })?;
    let previous_counter_snapshot = backend.state.subscription_support_counter_snapshot.clone();
    let mut updated_record = previous_record.clone();
    updated_record.mark_published_consequence(published.envelope().clone())?;
    backend
        .state
        .subscription_support_action_records
        .insert(key.clone(), updated_record);
    backend
        .state
        .subscription_support_counter_snapshot
        .record_support_action_envelope_publication();
    Ok(PublishedSupportActionRecordUpdate::new(
        key,
        previous_record,
        previous_counter_snapshot,
    ))
}

pub(super) fn verify_published_support_action_record<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> Result<(), StoreError> {
    verify_subscription_support_record_family(backend)
}

pub(super) fn persist_published_support_action_record_update<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    update: PublishedSupportActionRecordUpdate,
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
