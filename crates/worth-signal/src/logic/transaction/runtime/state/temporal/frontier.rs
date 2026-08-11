use crate::data::temporal::{
    ReadyTemporalWake, ScheduledTemporalWake, TemporalWakeId, TemporalWakeOwner, WakeOrdinal,
};

use super::TemporalRuntimeState;

impl TemporalRuntimeState {
    pub fn active_wake_owner(&self, wake_id: TemporalWakeId) -> Option<TemporalWakeOwner> {
        self.scheduled_wakes
            .get(&wake_id)
            .map(ScheduledTemporalWake::owner)
            .or_else(|| self.ready_wakes.get(&wake_id).map(ReadyTemporalWake::owner))
    }

    pub fn active_wake_for_owner(&self, owner: TemporalWakeOwner) -> Option<TemporalWakeId> {
        self.owner_frontier
            .get(&owner)
            .and_then(|owned| owned.values().next().copied())
    }

    pub(in crate::logic::transaction::runtime) fn scheduled_wake(
        &self,
        wake_id: TemporalWakeId,
    ) -> Option<&ScheduledTemporalWake> {
        self.scheduled_wakes.get(&wake_id)
    }

    pub fn ready_wake_for_owner(&self, owner: TemporalWakeOwner) -> Option<ReadyTemporalWake> {
        self.owner_frontier.get(&owner).and_then(|owned| {
            owned
                .values()
                .find_map(|wake_id| self.ready_wakes.get(wake_id).cloned())
        })
    }

    pub(super) fn issue_wake_id(&mut self) -> TemporalWakeId {
        let id = self.next_wake_id;
        self.next_wake_id = TemporalWakeId::new(id.get().saturating_add(1));
        id
    }

    pub(super) fn issue_wake_ordinal(&mut self) -> WakeOrdinal {
        self.next_wake_ordinal = self.next_wake_ordinal.next();
        self.next_wake_ordinal
    }

    pub(super) fn issue_previous_value_revision(
        &mut self,
    ) -> crate::data::temporal::PreviousValueRevision {
        self.next_previous_value_revision = self.next_previous_value_revision.next();
        self.next_previous_value_revision
    }

    pub(super) fn insert_scheduled_frontier_entry(&mut self, wake: &ScheduledTemporalWake) {
        self.scheduled_frontier
            .entry(wake.due_tick())
            .or_default()
            .insert(wake.ordinal(), wake.id());
    }

    pub(super) fn remove_scheduled_frontier_entry(&mut self, wake: &ScheduledTemporalWake) {
        if let Some(bucket) = self.scheduled_frontier.get_mut(&wake.due_tick()) {
            bucket.remove(&wake.ordinal());
            if bucket.is_empty() {
                self.scheduled_frontier.remove(&wake.due_tick());
            }
        }
    }

    pub(super) fn insert_owner_frontier_entry(
        &mut self,
        owner: TemporalWakeOwner,
        ordinal: WakeOrdinal,
        wake_id: TemporalWakeId,
    ) {
        self.owner_frontier
            .entry(owner)
            .or_default()
            .insert(ordinal, wake_id);
    }

    pub(super) fn remove_owner_frontier_entry(
        &mut self,
        owner: TemporalWakeOwner,
        ordinal: WakeOrdinal,
        wake_id: TemporalWakeId,
    ) {
        if let Some(bucket) = self.owner_frontier.get_mut(&owner) {
            if bucket.get(&ordinal).copied() == Some(wake_id) {
                bucket.remove(&ordinal);
            }
            if bucket.is_empty() {
                self.owner_frontier.remove(&owner);
            }
        }
    }
}
