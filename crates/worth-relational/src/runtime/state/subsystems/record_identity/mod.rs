mod pending_allocations;
mod reclaimed;
mod record_ref;
mod reservation;

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

use crate::history::data::{
    CanonicalRecordAllocation, RecordAllocationClass, RecordAllocationOrigin,
};
use crate::identity::data::PartitionId;
use crate::transactions::data::RecordAllocationDenial;

pub(crate) use pending_allocations::PendingRecordAllocations;
pub(crate) use reclaimed::ReclaimedRecordSlot;
use reservation::{RecordSlotReservation, ReservationOrigin};

use super::RuntimeSubsystem;

type RecordSlotKey = (RecordAllocationClass, PartitionId, usize);
type RecordFrontierKey = (RecordAllocationClass, PartitionId);

#[derive(Debug, Clone, Default)]
struct RecordIdentityState {
    reusable: BTreeSet<RecordSlotKey>,
    next_slots: BTreeMap<RecordFrontierKey, usize>,
    generation_high_water: BTreeMap<RecordSlotKey, u32>,
}

#[derive(Debug, Default)]
pub(crate) struct RecordIdentitySubsystem {
    state: Arc<Mutex<RecordIdentityState>>,
    staged_replay_allocations: Option<Vec<CanonicalRecordAllocation>>,
}

impl Clone for RecordIdentitySubsystem {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
            staged_replay_allocations: None,
        }
    }
}

impl RecordIdentitySubsystem {
    pub(crate) fn begin_allocations(&mut self) -> PendingRecordAllocations {
        PendingRecordAllocations::new(self.clone(), self.staged_replay_allocations.take())
    }

    pub(crate) fn stage_replay_allocations(
        &mut self,
        allocations: Vec<CanonicalRecordAllocation>,
    ) -> Result<(), &'static str> {
        if self.staged_replay_allocations.is_some() {
            return Err("record allocation replay evidence is already staged");
        }
        self.staged_replay_allocations = Some(allocations);
        Ok(())
    }

    pub(crate) fn clear_staged_replay_allocations(&mut self) -> bool {
        self.staged_replay_allocations.take().is_some()
    }

    pub(crate) fn admit_reclaimed(&self, reclaimed: ReclaimedRecordSlot) {
        let mut state = self.lock();
        state
            .reusable
            .insert((reclaimed.class, reclaimed.partition_id, reclaimed.slot));
        if let Some(next) = reclaimed.slot.checked_add(1) {
            let frontier = state
                .next_slots
                .entry((reclaimed.class, reclaimed.partition_id))
                .or_default();
            *frontier = (*frontier).max(next);
        }
    }

    pub(crate) fn reusable_snapshot(&self) -> Vec<RecordSlotKey> {
        self.lock().reusable.iter().copied().collect()
    }

    pub(crate) fn frontier_snapshot(&self) -> Vec<(RecordAllocationClass, PartitionId, usize)> {
        self.lock()
            .next_slots
            .iter()
            .map(|(&(class, partition_id), &next_slot)| (class, partition_id, next_slot))
            .collect()
    }

    pub(crate) fn generation_snapshot(
        &self,
    ) -> Vec<(RecordAllocationClass, PartitionId, u64, u32)> {
        self.lock()
            .generation_high_water
            .iter()
            .map(|(&(class, partition_id, slot), &generation)| {
                (class, partition_id, slot as u64, generation)
            })
            .collect()
    }

    pub(crate) fn restore_reusable(
        &self,
        class: RecordAllocationClass,
        partition_id: PartitionId,
        slot: usize,
    ) {
        self.admit_reclaimed(ReclaimedRecordSlot::new(class, partition_id, slot));
    }

    pub(crate) fn restore_frontier(
        &self,
        class: RecordAllocationClass,
        partition_id: PartitionId,
        next_slot: usize,
    ) {
        let mut state = self.lock();
        let frontier = state.next_slots.entry((class, partition_id)).or_default();
        *frontier = (*frontier).max(next_slot);
    }

    pub(crate) fn restore_generation(
        &self,
        class: RecordAllocationClass,
        partition_id: PartitionId,
        slot: usize,
        generation: u32,
    ) {
        let mut state = self.lock();
        let high_water = state
            .generation_high_water
            .entry((class, partition_id, slot))
            .or_default();
        *high_water = (*high_water).max(generation);
    }

    fn reserve(
        &self,
        class: RecordAllocationClass,
        partition_id: PartitionId,
    ) -> Result<(RecordSlotReservation, u32, RecordAllocationOrigin), RecordAllocationDenial> {
        let mut state = self.lock();
        let reusable = state
            .reusable
            .range((class, partition_id, 0)..=(class, partition_id, usize::MAX))
            .next()
            .copied();
        let (slot, origin) = reusable
            .map(|key| (key.2, ReservationOrigin::Reclaimed))
            .unwrap_or_else(|| {
                (
                    state
                        .next_slots
                        .get(&(class, partition_id))
                        .copied()
                        .unwrap_or(0),
                    ReservationOrigin::AppendFrontier,
                )
            });
        let prior_generation = state
            .generation_high_water
            .get(&(class, partition_id, slot))
            .copied()
            .unwrap_or(0);
        let generation =
            prior_generation
                .checked_add(1)
                .ok_or(RecordAllocationDenial::GenerationExhausted {
                    class,
                    partition_id,
                    slot,
                })?;
        match origin {
            ReservationOrigin::Reclaimed => {
                state.reusable.remove(&(class, partition_id, slot));
            }
            ReservationOrigin::AppendFrontier => {
                let next =
                    slot.checked_add(1)
                        .ok_or(RecordAllocationDenial::SlotFrontierExhausted {
                            class,
                            partition_id,
                        })?;
                state.next_slots.insert((class, partition_id), next);
            }
        }
        state
            .generation_high_water
            .insert((class, partition_id, slot), generation);
        let canonical_origin = match origin {
            ReservationOrigin::AppendFrontier => RecordAllocationOrigin::AppendFrontier,
            ReservationOrigin::Reclaimed => RecordAllocationOrigin::Reclaimed { prior_generation },
        };
        Ok((
            RecordSlotReservation::new(Arc::clone(&self.state), class, partition_id, slot, origin),
            generation,
            canonical_origin,
        ))
    }

    fn reserve_exact(
        &self,
        ordinal: u64,
        class: RecordAllocationClass,
        partition_id: PartitionId,
        slot: usize,
        generation: u32,
        origin: RecordAllocationOrigin,
    ) -> Result<RecordSlotReservation, RecordAllocationDenial> {
        let mut state = self.lock();
        let prior_generation = match origin {
            RecordAllocationOrigin::AppendFrontier => 0,
            RecordAllocationOrigin::Reclaimed { prior_generation } => prior_generation,
        };
        let current_generation = state
            .generation_high_water
            .get(&(class, partition_id, slot))
            .copied()
            .unwrap_or(0);
        let expected_generation =
            prior_generation
                .checked_add(1)
                .ok_or(RecordAllocationDenial::GenerationExhausted {
                    class,
                    partition_id,
                    slot,
                })?;
        if current_generation != prior_generation || generation != expected_generation {
            return Err(RecordAllocationDenial::ReplayGenerationMismatch {
                ordinal,
                class,
                partition_id,
                slot,
                expected_generation,
                observed_generation: generation,
            });
        }
        let reservation_origin = match origin {
            RecordAllocationOrigin::Reclaimed { .. } => {
                if !state.reusable.remove(&(class, partition_id, slot)) {
                    return Err(RecordAllocationDenial::ReplaySlotUnavailable {
                        ordinal,
                        class,
                        partition_id,
                        slot,
                    });
                }
                ReservationOrigin::Reclaimed
            }
            RecordAllocationOrigin::AppendFrontier => {
                let expected_slot = state
                    .next_slots
                    .get(&(class, partition_id))
                    .copied()
                    .unwrap_or(0);
                if slot != expected_slot {
                    return Err(RecordAllocationDenial::ReplayAppendFrontierMismatch {
                        ordinal,
                        class,
                        partition_id,
                        expected_slot,
                        observed_slot: slot,
                    });
                }
                let next =
                    slot.checked_add(1)
                        .ok_or(RecordAllocationDenial::SlotFrontierExhausted {
                            class,
                            partition_id,
                        })?;
                state.next_slots.insert((class, partition_id), next);
                ReservationOrigin::AppendFrontier
            }
        };
        state
            .generation_high_water
            .insert((class, partition_id, slot), generation);
        Ok(RecordSlotReservation::new(
            Arc::clone(&self.state),
            class,
            partition_id,
            slot,
            reservation_origin,
        ))
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, RecordIdentityState> {
        self.state.lock().expect("record identity lock poisoned")
    }
}

impl RuntimeSubsystem for RecordIdentitySubsystem {
    type Config = ();

    fn new(_: &Self::Config) -> Self {
        Self::default()
    }

    fn fork(&self) -> Self {
        Self {
            state: Arc::new(Mutex::new(self.lock().clone())),
            staged_replay_allocations: None,
        }
    }
}
