use super::*;

impl ResidentFrameTable {
    pub fn open(
        entry: AdmittedBufferPoolEntry,
        capacity: ResidentFrameTableCapacity,
    ) -> Result<Self, ResidentFrameDenial> {
        let frame_count = capacity.as_frames() as usize;
        reject_metadata_budget(frame_count)?;
        let mut frames = Vec::new();
        let mut resident_source_index = HashMap::new();
        let mut resident_slots = Vec::new();
        let mut free_slots = Vec::new();
        let mut generations = Vec::new();
        let mut lease_epochs = Vec::new();
        frames
            .try_reserve_exact(frame_count)
            .map_err(table_allocation_failed)?;
        resident_source_index
            .try_reserve(frame_count)
            .map_err(table_allocation_failed)?;
        resident_slots
            .try_reserve_exact(frame_count)
            .map_err(table_allocation_failed)?;
        free_slots
            .try_reserve_exact(frame_count)
            .map_err(table_allocation_failed)?;
        generations
            .try_reserve_exact(frame_count)
            .map_err(table_allocation_failed)?;
        lease_epochs
            .try_reserve_exact(frame_count)
            .map_err(table_allocation_failed)?;
        frames.resize_with(frame_count, || None);
        free_slots.extend(
            (0..frame_count)
                .rev()
                .map(|index| ResidentFrameSlot::from_index(index as u32)),
        );
        generations.resize(frame_count, ResidentFrameGeneration::initial());
        lease_epochs.resize(frame_count, LeaseEpoch::initial());
        Ok(Self {
            entry,
            frames,
            resident_source_index,
            resident_slots,
            free_slots,
            generations,
            lease_epochs,
            counters: ResidentFrameCounterSnapshot::empty(),
            pin_counters: PinLifecycleCounterSnapshot::empty(),
            dirty_counters: DirtyPageCounterSnapshot::empty(),
            eviction_counters: EvictionCounterSnapshot::empty(),
            record_view_counters: RecordCopyCounterSnapshot::empty(),
        })
    }
}

fn reject_metadata_budget(frame_count: usize) -> Result<(), ResidentFrameDenial> {
    const MAXIMUM_FRAME_TABLE_ENTRIES: usize = 1_048_576;
    let per_frame = std::mem::size_of::<Option<ResidentFrameRecord>>()
        + std::mem::size_of::<ResidentFrameSlot>() * 3
        + std::mem::size_of::<ResidentFrameGeneration>()
        + std::mem::size_of::<LeaseEpoch>()
        + std::mem::size_of::<(ResidentFrameSourceKey, ResidentFrameSlot)>();
    frame_count.checked_mul(per_frame).ok_or_else(|| {
        ResidentFrameDenial::new(ResidentFrameDenialKind::TableMetadataBudgetExceeded)
    })?;
    if frame_count > MAXIMUM_FRAME_TABLE_ENTRIES {
        return Err(ResidentFrameDenial::new(
            ResidentFrameDenialKind::TableMetadataBudgetExceeded,
        ));
    }
    Ok(())
}

fn table_allocation_failed<T>(_: T) -> ResidentFrameDenial {
    ResidentFrameDenial::new(ResidentFrameDenialKind::TableAllocationFailed)
}
