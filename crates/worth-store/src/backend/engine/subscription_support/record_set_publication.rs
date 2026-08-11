use crate::{
    failure::{StoreError, StoreErrorKind},
    PublishableSubscriptionSupportArtifact, PublishedSubscriptionSupportArtifact,
    SubscriptionSupportCounterSnapshot, SubscriptionSupportStoredRecordSet,
};

use super::super::core::verify_durable_barrier;
use super::super::{StateBackedStoreBackend, StatePersistence};
use super::record_family_verification::verify_subscription_support_record_family;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum SubscriptionSupportRecordSetPublicationDecision {
    DuplicateRetry,
    Install,
}

pub(super) fn decide_subscription_support_record_set_publication(
    existing: Option<&SubscriptionSupportStoredRecordSet>,
    candidate: &SubscriptionSupportStoredRecordSet,
) -> Result<SubscriptionSupportRecordSetPublicationDecision, StoreError> {
    match existing {
        Some(existing) if existing == candidate => {
            Ok(SubscriptionSupportRecordSetPublicationDecision::DuplicateRetry)
        }
        Some(_) => Err(StoreError::new(
            StoreErrorKind::SubscriptionSupportPublicationViolation,
            "subscription-support publication collided with a different durable record set",
        )),
        None => Ok(SubscriptionSupportRecordSetPublicationDecision::Install),
    }
}

#[derive(Debug, Clone)]
pub(super) struct SubscriptionSupportRecordSetPublicationUpdate {
    storage_key: String,
    previous_counter_snapshot: SubscriptionSupportCounterSnapshot,
    installed_record: bool,
}

impl SubscriptionSupportRecordSetPublicationUpdate {
    fn new(
        storage_key: String,
        previous_counter_snapshot: SubscriptionSupportCounterSnapshot,
        installed_record: bool,
    ) -> Self {
        Self {
            storage_key,
            previous_counter_snapshot,
            installed_record,
        }
    }

    fn rollback<P: StatePersistence>(self, backend: &mut StateBackedStoreBackend<P>) {
        if self.installed_record {
            backend
                .state
                .subscription_support_record_sets
                .remove(&self.storage_key);
        }
        backend.state.subscription_support_counter_snapshot = self.previous_counter_snapshot;
    }

    fn requires_family_verification(&self) -> bool {
        self.installed_record
    }
}

pub(super) fn rollback_subscription_support_record_set_update<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    update: SubscriptionSupportRecordSetPublicationUpdate,
) {
    update.rollback(backend);
}

pub(super) fn record_set_update_requires_family_verification(
    update: &SubscriptionSupportRecordSetPublicationUpdate,
) -> bool {
    update.requires_family_verification()
}

pub(super) fn install_subscription_support_record_set<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    publishable: &PublishableSubscriptionSupportArtifact,
    published: &PublishedSubscriptionSupportArtifact,
) -> Result<SubscriptionSupportRecordSetPublicationUpdate, StoreError> {
    let record_set =
        SubscriptionSupportStoredRecordSet::from_publishable_and_published(publishable, published)?;
    let storage_key = record_set.key().storage_key();
    let decision = match decide_subscription_support_record_set_publication(
        backend
            .state
            .subscription_support_record_sets
            .get(&storage_key),
        &record_set,
    ) {
        Ok(decision) => decision,
        Err(error) => {
            backend
                .state
                .subscription_support_counter_snapshot
                .record_identity_collision();
            return Err(error);
        }
    };
    let previous_counter_snapshot = backend.state.subscription_support_counter_snapshot.clone();
    match decision {
        SubscriptionSupportRecordSetPublicationDecision::DuplicateRetry => {
            backend
                .state
                .subscription_support_counter_snapshot
                .record_duplicate_retry();
            Ok(SubscriptionSupportRecordSetPublicationUpdate::new(
                storage_key,
                previous_counter_snapshot,
                false,
            ))
        }
        SubscriptionSupportRecordSetPublicationDecision::Install => {
            backend
                .state
                .subscription_support_record_sets
                .insert(storage_key.clone(), record_set);
            backend
                .state
                .subscription_support_counter_snapshot
                .record_published();
            backend
                .state
                .subscription_support_counter_snapshot
                .record_family_catalog_lookup();
            Ok(SubscriptionSupportRecordSetPublicationUpdate::new(
                storage_key,
                previous_counter_snapshot,
                true,
            ))
        }
    }
}

pub(super) fn verify_installed_subscription_support_record_set<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> Result<(), StoreError> {
    verify_subscription_support_record_family(backend)
}

pub(super) fn record_malformed_subscription_support_record<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
) {
    backend
        .state
        .subscription_support_counter_snapshot
        .record_malformed_support_record();
}

pub(super) fn persist_subscription_support_record_set_update<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    update: SubscriptionSupportRecordSetPublicationUpdate,
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
