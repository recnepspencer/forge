use super::{
    resident_frame_record::ResidentFrameRecord, resident_frame_source::ResidentFrameSourceKey,
};
use crate::{
    AdmittedBufferPoolEntry, DirtyPageCounterSnapshot, EvictionCounterSnapshot, LeaseEpoch,
    PinLifecycleCounterSnapshot, RecordCopyCounterSnapshot, ResidentFrameAdmission,
    ResidentFrameBytes, ResidentFrameCounterSnapshot, ResidentFrameDenial, ResidentFrameDenialKind,
    ResidentFrameGeneration, ResidentFrameHitMissReport, ResidentFrameIdentity,
    ResidentFrameLoadRequest, ResidentFrameResidence, ResidentFrameSlot, ResidentFrameToken,
    ResidentGenerationSeparationProof,
};
use forge_store_physical_format::{PhysicalPayloadViewAdmission, PhysicalReference};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResidentFrameTableCapacity {
    frames: u32,
}

impl ResidentFrameTableCapacity {
    pub fn frames(frames: u32) -> Result<Self, ResidentFrameDenial> {
        if frames == 0 {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::TableCapacityIsZero,
            ));
        }
        Ok(Self { frames })
    }

    pub const fn as_frames(self) -> u32 {
        self.frames
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ResidentFrameTable {
    pub(crate) entry: AdmittedBufferPoolEntry,
    pub(crate) frames: Vec<Option<ResidentFrameRecord>>,
    pub(crate) resident_source_index: HashMap<ResidentFrameSourceKey, ResidentFrameSlot>,
    pub(crate) resident_slots: Vec<ResidentFrameSlot>,
    pub(crate) free_slots: Vec<ResidentFrameSlot>,
    pub(crate) generations: Vec<ResidentFrameGeneration>,
    pub(crate) lease_epochs: Vec<LeaseEpoch>,
    pub(crate) counters: ResidentFrameCounterSnapshot,
    pub(crate) pin_counters: PinLifecycleCounterSnapshot,
    pub(crate) dirty_counters: DirtyPageCounterSnapshot,
    pub(crate) eviction_counters: EvictionCounterSnapshot,
    pub(crate) record_view_counters: RecordCopyCounterSnapshot,
}

impl ResidentFrameTable {
    pub fn open(entry: AdmittedBufferPoolEntry, capacity: ResidentFrameTableCapacity) -> Self {
        let frame_count = capacity.as_frames() as usize;
        Self {
            entry,
            frames: vec![None; frame_count],
            resident_source_index: HashMap::with_capacity(frame_count),
            resident_slots: Vec::with_capacity(frame_count),
            free_slots: free_slots(frame_count),
            generations: vec![ResidentFrameGeneration::initial(); frame_count],
            lease_epochs: vec![LeaseEpoch::initial(); frame_count],
            counters: ResidentFrameCounterSnapshot::empty(),
            pin_counters: PinLifecycleCounterSnapshot::empty(),
            dirty_counters: DirtyPageCounterSnapshot::empty(),
            eviction_counters: EvictionCounterSnapshot::empty(),
            record_view_counters: RecordCopyCounterSnapshot::empty(),
        }
    }

    pub fn admit_frame(
        &mut self,
        request: ResidentFrameLoadRequest,
    ) -> Result<ResidentFrameAdmission, ResidentFrameDenial> {
        if let Some(slot) = self.find_resident_slot(request) {
            let identity = self.record_at_slot(slot)?.identity();
            self.counters = self.counters.with_hit();
            return Ok(self.admission_report(identity, request));
        }

        self.counters = self.counters.with_miss();
        self.reject_resident_budget_overflow(request.frame_size().as_bytes())?;
        let slot = self.first_empty_slot()?;
        let identity = self.install_record(slot, request, None);
        Ok(self.admission_report(identity, request))
    }

    pub fn admit_resident_frame_bytes(
        &mut self,
        request: ResidentFrameLoadRequest,
        payload_admission: PhysicalPayloadViewAdmission<'_>,
    ) -> Result<ResidentFrameAdmission, ResidentFrameDenial> {
        let resident_bytes =
            ResidentFrameBytes::from_s1_payload_admission(request, payload_admission)?;
        if let Some(slot) = self.find_resident_slot(request) {
            self.counters = self.counters.with_hit();
            let identity = {
                let record = self.record_at_slot_mut(slot)?;
                if record.bytes().is_none() {
                    record.attach_resident_bytes(resident_bytes);
                }
                record.identity()
            };
            return Ok(self.admission_report(identity, request));
        }

        self.counters = self.counters.with_miss();
        self.reject_resident_budget_overflow(request.frame_size().as_bytes())?;
        let slot = self.first_empty_slot()?;
        let identity = self.install_record(slot, request, Some(resident_bytes));
        Ok(self.admission_report(identity, request))
    }

    pub fn reuse_frame_slot(
        &mut self,
        slot: ResidentFrameSlot,
        request: ResidentFrameLoadRequest,
    ) -> Result<ResidentFrameAdmission, ResidentFrameDenial> {
        self.reject_slot_out_of_range(slot)?;
        let (previous_request, previous_bytes, previous_lease_epoch, previous_has_active_pin) = {
            let previous_record = self.record_at_slot(slot)?;
            (
                previous_record.request(),
                previous_record.request().frame_size().as_bytes(),
                previous_record.lease_epoch(),
                previous_record.has_active_pin(),
            )
        };
        if previous_has_active_pin {
            self.record_protected_mutation_denial();
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::ResidentFramePinned,
            ));
        }
        if self.record_at_slot(slot)?.has_unflushed_dirty_state() {
            self.dirty_counters = self.dirty_counters.with_dirty_discard_denial();
            self.publish_dirty_counters();
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::DirtyFrameUnpublished,
            ));
        }
        self.counters = self.counters.with_miss();
        let replacement_bytes = self.resident_bytes_after_reuse(previous_bytes, request)?;
        let next_generation = self.next_generation_for_slot(slot)?;
        self.resident_source_index
            .remove(&previous_request.source_key());
        self.generations[slot.index() as usize] = next_generation;
        self.lease_epochs[slot.index() as usize] = next_lease_epoch(previous_lease_epoch)?;
        let identity = ResidentFrameIdentity::new(slot, next_generation);
        let replacement = ResidentFrameRecord::new(identity, request, None);
        self.frames[slot.index() as usize] = Some(replacement);
        self.resident_source_index
            .insert(request.source_key(), slot);
        self.counters = self.counters.with_resident_bytes(replacement_bytes);
        Ok(self.admission_report(identity, request))
    }

    pub fn reuse_frame_slot_with_generation_separation(
        &mut self,
        slot: ResidentFrameSlot,
        request: ResidentFrameLoadRequest,
    ) -> Result<ResidentGenerationSeparationProof, ResidentFrameDenial> {
        self.reject_slot_out_of_range(slot)?;
        let previous_identity = self.record_at_slot(slot)?.identity();
        let stale_token = previous_identity.token();
        let replacement = self.reuse_frame_slot(slot, request)?;
        let stale_denial = self.observe_stale_token_after_reuse(stale_token)?;
        Ok(ResidentGenerationSeparationProof::new(
            previous_identity,
            replacement.identity(),
            stale_token,
            stale_denial,
            self.counters,
        ))
    }

    pub fn resident_frame(
        &mut self,
        token: ResidentFrameToken,
    ) -> Result<ResidentFrameResidence, ResidentFrameDenial> {
        self.reject_slot_out_of_range(token.slot())?;
        self.counters = self.counters.with_lookup();
        let record = self.record_at_slot(token.slot())?;
        if record.identity().generation() != token.resident_generation() {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::StaleResidentGeneration,
            ));
        }
        Ok(ResidentFrameResidence::new(
            record.identity(),
            record.request(),
        ))
    }

    pub const fn entry(&self) -> AdmittedBufferPoolEntry {
        self.entry
    }

    pub const fn counters(&self) -> ResidentFrameCounterSnapshot {
        self.counters
    }

    fn find_resident_slot(&self, request: ResidentFrameLoadRequest) -> Option<ResidentFrameSlot> {
        self.resident_source_index
            .get(&request.source_key())
            .copied()
    }

    fn reject_resident_budget_overflow(
        &self,
        incoming_bytes: u64,
    ) -> Result<(), ResidentFrameDenial> {
        let current_bytes = self.counters.resident_bytes().as_bytes();
        let Some(next_bytes) = current_bytes.checked_add(incoming_bytes) else {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::ResidentMemoryBudgetExceeded,
            ));
        };
        if next_bytes > self.entry.admission().budget().resident_memory().as_bytes() {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::ResidentMemoryBudgetExceeded,
            ));
        }
        Ok(())
    }

    fn first_empty_slot(&self) -> Result<ResidentFrameSlot, ResidentFrameDenial> {
        self.free_slots.last().copied().ok_or_else(|| {
            ResidentFrameDenial::new(ResidentFrameDenialKind::ResidentFrameTableFull)
        })
    }

    fn install_record(
        &mut self,
        slot: ResidentFrameSlot,
        request: ResidentFrameLoadRequest,
        bytes: Option<ResidentFrameBytes>,
    ) -> ResidentFrameIdentity {
        let identity = ResidentFrameIdentity::new(slot, self.generations[slot.index() as usize]);
        let record = ResidentFrameRecord::new(identity, request, bytes);
        let resident_bytes =
            self.counters.resident_bytes().as_bytes() + request.frame_size().as_bytes();
        self.frames[slot.index() as usize] = Some(record);
        self.free_slots.pop();
        self.resident_source_index
            .insert(request.source_key(), slot);
        self.track_resident_slot(slot);
        self.counters = self.counters.with_resident_bytes(resident_bytes);
        identity
    }

    pub(crate) fn reject_slot_out_of_range(
        &self,
        slot: ResidentFrameSlot,
    ) -> Result<(), ResidentFrameDenial> {
        if slot.index() as usize >= self.frames.len() {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::ResidentFrameSlotOutOfRange,
            ));
        }
        Ok(())
    }

    pub(crate) fn record_at_slot(
        &self,
        slot: ResidentFrameSlot,
    ) -> Result<&ResidentFrameRecord, ResidentFrameDenial> {
        self.frames[slot.index() as usize].as_ref().ok_or_else(|| {
            ResidentFrameDenial::new(ResidentFrameDenialKind::ResidentFrameSlotNotResident)
        })
    }

    pub(crate) fn record_at_slot_mut(
        &mut self,
        slot: ResidentFrameSlot,
    ) -> Result<&mut ResidentFrameRecord, ResidentFrameDenial> {
        self.frames[slot.index() as usize].as_mut().ok_or_else(|| {
            ResidentFrameDenial::new(ResidentFrameDenialKind::ResidentFrameSlotNotResident)
        })
    }

    pub(crate) fn resident_physical_reference(
        &self,
        identity: ResidentFrameIdentity,
    ) -> Result<PhysicalReference, ResidentFrameDenial> {
        let record = self.record_at_slot(identity.slot())?;
        if record.identity().generation() != identity.generation() {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::StaleResidentGeneration,
            ));
        }
        Ok(record.request().reference().reference())
    }

    fn resident_bytes_after_reuse(
        &self,
        previous_bytes: u64,
        request: ResidentFrameLoadRequest,
    ) -> Result<u64, ResidentFrameDenial> {
        let current_bytes = self.counters.resident_bytes().as_bytes();
        let next_without_previous = current_bytes - previous_bytes;
        let Some(next_bytes) = next_without_previous.checked_add(request.frame_size().as_bytes())
        else {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::ResidentMemoryBudgetExceeded,
            ));
        };
        if next_bytes > self.entry.admission().budget().resident_memory().as_bytes() {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::ResidentMemoryBudgetExceeded,
            ));
        }
        Ok(next_bytes)
    }

    pub(crate) fn next_generation_for_slot(
        &self,
        slot: ResidentFrameSlot,
    ) -> Result<ResidentFrameGeneration, ResidentFrameDenial> {
        self.generations[slot.index() as usize]
            .next()
            .ok_or_else(|| {
                ResidentFrameDenial::new(ResidentFrameDenialKind::ResidentGenerationOverflow)
            })
    }

    fn admission_report(
        &self,
        identity: ResidentFrameIdentity,
        request: ResidentFrameLoadRequest,
    ) -> ResidentFrameAdmission {
        ResidentFrameAdmission::new(
            identity,
            request,
            ResidentFrameHitMissReport::new(self.counters),
        )
    }

    fn observe_stale_token_after_reuse(
        &mut self,
        stale_token: ResidentFrameToken,
    ) -> Result<ResidentFrameDenial, ResidentFrameDenial> {
        match self.resident_frame(stale_token) {
            Ok(_) => Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::GenerationSeparationNotObserved,
            )),
            Err(denial) if denial.is_stale_resident_generation() => Ok(denial),
            Err(denial) => Err(denial),
        }
    }

    pub(crate) fn publish_dirty_counters(&mut self) {
        self.counters = self.counters.with_dirty_state(self.dirty_counters);
    }

    pub(crate) fn track_resident_slot(&mut self, slot: ResidentFrameSlot) {
        match self
            .resident_slots
            .binary_search_by_key(&slot.index(), |resident_slot| resident_slot.index())
        {
            Ok(_) => {}
            Err(index) => self.resident_slots.insert(index, slot),
        }
    }

    pub(crate) fn untrack_resident_slot(&mut self, slot: ResidentFrameSlot) {
        if let Ok(index) = self
            .resident_slots
            .binary_search_by_key(&slot.index(), |resident_slot| resident_slot.index())
        {
            self.resident_slots.remove(index);
        }
    }
}

fn free_slots(frame_count: usize) -> Vec<ResidentFrameSlot> {
    (0..frame_count)
        .rev()
        .map(|index| ResidentFrameSlot::from_index(index as u32))
        .collect()
}

pub(crate) fn next_lease_epoch(current: LeaseEpoch) -> Result<LeaseEpoch, ResidentFrameDenial> {
    current
        .next()
        .ok_or_else(|| ResidentFrameDenial::new(ResidentFrameDenialKind::PageLeaseStale))
}
