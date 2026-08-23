use std::sync::{Arc, Mutex};

use crate::history::data::RecordAllocationClass;
use crate::identity::data::PartitionId;

use super::RecordIdentityState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ReservationOrigin {
    AppendFrontier,
    Reclaimed,
}

#[derive(Debug)]
pub(super) struct RecordSlotReservation {
    authority: Arc<Mutex<RecordIdentityState>>,
    class: RecordAllocationClass,
    partition_id: PartitionId,
    pub(super) slot: usize,
    pub(super) origin: ReservationOrigin,
    consumed: bool,
}

impl RecordSlotReservation {
    pub(super) fn new(
        authority: Arc<Mutex<RecordIdentityState>>,
        class: RecordAllocationClass,
        partition_id: PartitionId,
        slot: usize,
        origin: ReservationOrigin,
    ) -> Self {
        Self {
            authority,
            class,
            partition_id,
            slot,
            origin,
            consumed: false,
        }
    }

    pub(super) fn consume(&mut self) {
        self.consumed = true;
    }
}

impl Drop for RecordSlotReservation {
    fn drop(&mut self) {
        if self.consumed || self.origin == ReservationOrigin::AppendFrontier {
            return;
        }
        self.authority
            .lock()
            .expect("record identity lock poisoned")
            .reusable
            .insert((self.class, self.partition_id, self.slot));
    }
}
