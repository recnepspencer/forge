use crate::{
    failure::StoreError, SubscriptionSupportActionPublicationRecoveryReport,
    SubscriptionSupportCounterSnapshot, SupportActionDurableRecord, SupportActionId,
    SupportActionPublicationState,
};

use super::super::core::verify_durable_barrier;
use super::super::{StateBackedStoreBackend, StatePersistence};
use super::record_family_verification::verify_subscription_support_record_family;

#[derive(Debug, Clone)]
pub(super) enum SupportActionRecoveryTransition {
    AlreadySettled,
    InterruptedBeforePublication {
        update: SupportActionPublicationRecoveryUpdate,
    },
}

#[derive(Debug, Clone)]
struct SupportActionPublicationRecoveryUpdate {
    action_id: SupportActionId,
    previous_record: SupportActionDurableRecord,
    updated_record: SupportActionDurableRecord,
}

impl SupportActionPublicationRecoveryUpdate {
    fn from_pending_record(record: &SupportActionDurableRecord) -> Self {
        let mut updated_record = record.clone();
        updated_record.mark_interrupted_before_publication();
        Self {
            action_id: record.action_id().clone(),
            previous_record: record.clone(),
            updated_record,
        }
    }

    fn snapshot<P: StatePersistence>(
        &self,
        backend: &StateBackedStoreBackend<P>,
    ) -> SupportActionPublicationRecoverySnapshot {
        SupportActionPublicationRecoverySnapshot {
            action_id: self.action_id.clone(),
            previous_record: self.previous_record.clone(),
            previous_counter_snapshot: backend.state.subscription_support_counter_snapshot.clone(),
        }
    }

    fn install<P: StatePersistence>(&self, backend: &mut StateBackedStoreBackend<P>) {
        backend.state.subscription_support_action_records.insert(
            self.action_id.as_str().to_string(),
            self.updated_record.clone(),
        );
    }
}

#[derive(Debug, Clone)]
struct SupportActionPublicationRecoverySnapshot {
    action_id: SupportActionId,
    previous_record: SupportActionDurableRecord,
    previous_counter_snapshot: SubscriptionSupportCounterSnapshot,
}

impl SupportActionPublicationRecoverySnapshot {
    fn restore<P: StatePersistence>(self, backend: &mut StateBackedStoreBackend<P>) {
        backend
            .state
            .subscription_support_action_records
            .insert(self.action_id.as_str().to_string(), self.previous_record);
        backend.state.subscription_support_counter_snapshot = self.previous_counter_snapshot;
    }
}

pub(super) fn decide_support_action_recovery_transition(
    record: &SupportActionDurableRecord,
) -> SupportActionRecoveryTransition {
    match record.publication_state() {
        SupportActionPublicationState::PendingPublication => {
            SupportActionRecoveryTransition::InterruptedBeforePublication {
                update: SupportActionPublicationRecoveryUpdate::from_pending_record(record),
            }
        }
        SupportActionPublicationState::InterruptedBeforePublication
        | SupportActionPublicationState::PublishedConsequence => {
            SupportActionRecoveryTransition::AlreadySettled
        }
    }
}

pub(super) fn apply_support_action_recovery_transition<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    transition: SupportActionRecoveryTransition,
) -> Result<(), StoreError> {
    let SupportActionRecoveryTransition::InterruptedBeforePublication { update } = transition
    else {
        return Ok(());
    };
    let rollback_snapshot = update.snapshot(backend);
    update.install(backend);
    backend
        .state
        .subscription_support_counter_snapshot
        .record_support_action_recovery();
    if let Err(error) = verify_subscription_support_record_family(backend) {
        rollback_snapshot.restore(backend);
        return Err(error);
    }
    let persist_report = match backend.persistence.persist_state(&backend.state) {
        Ok(report) => report,
        Err(error) => {
            rollback_snapshot.restore(backend);
            return Err(error);
        }
    };
    verify_durable_barrier(&mut backend.counters, &persist_report)
}

pub(super) fn recovery_report_from_record(
    record: &SupportActionDurableRecord,
) -> Result<SubscriptionSupportActionPublicationRecoveryReport, StoreError> {
    SubscriptionSupportActionPublicationRecoveryReport::from_durable_record(record)
}
