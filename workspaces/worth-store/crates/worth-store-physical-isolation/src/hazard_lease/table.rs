use crate::CurrentPhysicalRoot;

use super::epoch_index::{HazardLeaseEpochIndex, HazardLeaseEpochIndexEntry};
use super::{
    HazardLeaseCounterSnapshot, HazardLeaseDenial, HazardLeaseEpochIndexSnapshot,
    HazardLeaseGeneration, HazardLeaseReleaseReceipt, HazardLeaseSlot, OwnedCopyStableReadReceipt,
    ProtectedReferenceLease, ReadHandleRevocationReceipt,
};

#[derive(Debug, Clone)]
pub struct HazardLeaseTable {
    slots: Vec<HazardLeaseSlotEntry>,
    free_slots: Vec<usize>,
    index: HazardLeaseEpochIndex,
    counters: HazardLeaseCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HazardLeaseTableCapacity {
    slots: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActiveHazardLease {
    slot: HazardLeaseSlot,
    generation: HazardLeaseGeneration,
}

#[derive(Debug, Clone)]
struct HazardLeaseSlotEntry {
    generation: HazardLeaseGeneration,
    lease: Option<ProtectedReferenceLease>,
    root: Option<CurrentPhysicalRoot>,
}

impl HazardLeaseTable {
    pub fn with_capacity(capacity: HazardLeaseTableCapacity) -> Self {
        let slots = (0..capacity.slots)
            .map(|_| HazardLeaseSlotEntry {
                generation: HazardLeaseGeneration::initial(),
                lease: None,
                root: None,
            })
            .collect();
        Self {
            free_slots: (0..capacity.slots).rev().collect(),
            slots,
            index: HazardLeaseEpochIndex::default(),
            counters: HazardLeaseCounterSnapshot::default(),
        }
    }

    pub fn acquire(
        &mut self,
        root: CurrentPhysicalRoot,
        lease: ProtectedReferenceLease,
    ) -> Result<ActiveHazardLease, HazardLeaseDenial> {
        let Some(index) = self.free_slots.pop() else {
            return Err(HazardLeaseDenial::TableFull);
        };
        let entry = &mut self.slots[index];
        let slot = HazardLeaseSlot::from_index(index);
        let generation = entry.generation;
        let index_entry = HazardLeaseEpochIndexEntry::new(
            slot,
            generation,
            lease.kind(),
            root.epoch(),
            lease.footprint().ranges().ranges().to_vec(),
        );
        entry.root = Some(root);
        entry.lease = Some(lease);
        self.index.insert(index_entry);
        self.counters = self.counters.with_acquire();
        Ok(ActiveHazardLease { slot, generation })
    }

    pub fn release(
        &mut self,
        active: ActiveHazardLease,
    ) -> Result<HazardLeaseReleaseReceipt, HazardLeaseDenial> {
        self.remove_indexed_lease(active)?;
        let receipt = {
            let (entry, lease) = self.take_matching_lease(active)?;
            let receipt = HazardLeaseReleaseReceipt::new(
                active.slot,
                active.generation,
                lease.kind(),
                lease.barrier().footprint_basis(),
            );
            entry.generation = entry.generation.next();
            entry.root = None;
            receipt
        };
        self.free_slots.push(active.slot.get() as usize);
        self.counters = self.counters.with_release();
        Ok(receipt)
    }

    pub fn revoke(
        &mut self,
        active: ActiveHazardLease,
    ) -> Result<ReadHandleRevocationReceipt, HazardLeaseDenial> {
        self.remove_indexed_lease(active)?;
        let receipt = {
            let (entry, lease) = self.take_matching_lease(active)?;
            let receipt = ReadHandleRevocationReceipt::new(
                active.slot,
                active.generation,
                lease.barrier().footprint_basis(),
            );
            entry.generation = entry.generation.next();
            entry.root = None;
            receipt
        };
        self.free_slots.push(active.slot.get() as usize);
        self.counters = self.counters.with_revocation();
        Ok(receipt)
    }

    pub fn convert_to_owned_copy(
        &mut self,
        active: ActiveHazardLease,
    ) -> Result<OwnedCopyStableReadReceipt, HazardLeaseDenial> {
        self.remove_indexed_lease(active)?;
        let receipt = {
            let (entry, lease) = self.take_matching_lease(active)?;
            let receipt = OwnedCopyStableReadReceipt::new(
                active.slot,
                active.generation,
                lease.barrier().footprint_basis(),
            );
            entry.generation = entry.generation.next();
            entry.root = None;
            receipt
        };
        self.free_slots.push(active.slot.get() as usize);
        self.counters = self.counters.with_owned_copy();
        Ok(receipt)
    }

    pub fn live_index_snapshot(&self) -> HazardLeaseEpochIndexSnapshot {
        self.index.snapshot(self.counters)
    }

    pub const fn counters(&self) -> HazardLeaseCounterSnapshot {
        self.counters
    }

    fn take_matching_lease(
        &mut self,
        active: ActiveHazardLease,
    ) -> Result<(&mut HazardLeaseSlotEntry, ProtectedReferenceLease), HazardLeaseDenial> {
        let Some(entry) = self.slots.get_mut(active.slot.get() as usize) else {
            self.counters = self.counters.with_stale_release_denial();
            return Err(HazardLeaseDenial::UnknownLeaseSlot { slot: active.slot });
        };
        if entry.generation != active.generation {
            self.counters = self.counters.with_stale_release_denial();
            return Err(HazardLeaseDenial::StaleLeaseGeneration {
                slot: active.slot,
                expected: entry.generation,
                observed: active.generation,
            });
        }
        let Some(lease) = entry.lease.take() else {
            self.counters = self.counters.with_stale_release_denial();
            return Err(HazardLeaseDenial::LeaseAlreadyReleased { slot: active.slot });
        };
        Ok((entry, lease))
    }

    fn remove_indexed_lease(&mut self, active: ActiveHazardLease) -> Result<(), HazardLeaseDenial> {
        let root_epoch = {
            let entry = self.matching_slot_entry(active)?;
            entry
                .root
                .expect("an active lease always retains its admitted root")
                .epoch()
        };
        if self
            .index
            .remove(root_epoch, active.slot, active.generation)
        {
            Ok(())
        } else {
            Err(HazardLeaseDenial::InternalIndexInconsistency { slot: active.slot })
        }
    }

    fn matching_slot_entry(
        &mut self,
        active: ActiveHazardLease,
    ) -> Result<&HazardLeaseSlotEntry, HazardLeaseDenial> {
        let Some(entry) = self.slots.get(active.slot.get() as usize) else {
            self.counters = self.counters.with_stale_release_denial();
            return Err(HazardLeaseDenial::UnknownLeaseSlot { slot: active.slot });
        };
        if entry.generation != active.generation {
            self.counters = self.counters.with_stale_release_denial();
            return Err(HazardLeaseDenial::StaleLeaseGeneration {
                slot: active.slot,
                expected: entry.generation,
                observed: active.generation,
            });
        }
        if entry.lease.is_none() {
            self.counters = self.counters.with_stale_release_denial();
            return Err(HazardLeaseDenial::LeaseAlreadyReleased { slot: active.slot });
        }
        Ok(entry)
    }
}

impl HazardLeaseTableCapacity {
    pub const fn bounded_slots(slots: usize) -> Result<Self, HazardLeaseDenial> {
        if slots == 0 {
            return Err(HazardLeaseDenial::EmptyCapacity);
        }
        Ok(Self { slots })
    }

    pub const fn slots(self) -> usize {
        self.slots
    }
}

impl ActiveHazardLease {
    pub const fn slot(self) -> HazardLeaseSlot {
        self.slot
    }

    pub const fn generation(self) -> HazardLeaseGeneration {
        self.generation
    }
}
