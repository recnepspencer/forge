use crate::{
    PageRecordCounterSnapshot, PageRecordDenial, PageRecordDenialKind, PhysicalByteOrder,
    PhysicalGeneration, PhysicalRecordSlot, SlotDirectoryEntryState,
};

pub const SLOT_DIRECTORY_PREFIX_LENGTH: usize = 4;
pub const SLOT_DIRECTORY_ENTRY_LENGTH: usize = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotDirectoryEntry {
    slot: PhysicalRecordSlot,
    state: SlotDirectoryEntryState,
    offset: u32,
    frame_length: u32,
    generation: PhysicalGeneration,
}

impl SlotDirectoryEntry {
    pub(crate) const fn occupied(
        slot: PhysicalRecordSlot,
        offset: u32,
        frame_length: u32,
        generation: PhysicalGeneration,
    ) -> Self {
        Self {
            slot,
            state: SlotDirectoryEntryState::Occupied,
            offset,
            frame_length,
            generation,
        }
    }

    pub const fn state(self) -> SlotDirectoryEntryState {
        self.state
    }

    pub const fn slot(self) -> PhysicalRecordSlot {
        self.slot
    }

    pub const fn offset(self) -> u32 {
        self.offset
    }

    pub const fn frame_length(self) -> u32 {
        self.frame_length
    }

    pub const fn generation(self) -> PhysicalGeneration {
        self.generation
    }

    const fn with_offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotDirectory<'a> {
    bytes: &'a [u8],
    slot_count: u16,
}

impl<'a> SlotDirectory<'a> {
    pub fn decode(
        bytes: &'a [u8],
        byte_order: PhysicalByteOrder,
        counters: PageRecordCounterSnapshot,
    ) -> Result<Self, PageRecordDenial> {
        if bytes.is_empty() {
            return Ok(Self {
                bytes,
                slot_count: 0,
            });
        }
        if bytes.len() < SLOT_DIRECTORY_PREFIX_LENGTH {
            return Err(PageRecordDenial::new(
                PageRecordDenialKind::SlotDirectoryTooShort,
                counters,
            )
            .with_lengths(SLOT_DIRECTORY_PREFIX_LENGTH, bytes.len()));
        }
        let slot_count = byte_order.read_u16([bytes[0], bytes[1]]);
        let directory_len = directory_length(slot_count);
        if bytes.len() < directory_len {
            return Err(PageRecordDenial::new(
                PageRecordDenialKind::SlotDirectoryLengthMismatch,
                counters,
            )
            .with_lengths(directory_len, bytes.len()));
        }
        Ok(Self { bytes, slot_count })
    }

    pub const fn slot_count(self) -> u16 {
        self.slot_count
    }

    pub fn locate(
        self,
        slot: PhysicalRecordSlot,
        byte_order: PhysicalByteOrder,
        counters: PageRecordCounterSnapshot,
    ) -> Result<SlotDirectoryEntry, PageRecordDenial> {
        let index = slot.get();
        if index == 0 || index > self.slot_count {
            return Err(
                PageRecordDenial::new(PageRecordDenialKind::SlotOutOfRange, counters)
                    .with_slot(slot),
            );
        }
        let offset =
            SLOT_DIRECTORY_PREFIX_LENGTH + ((index as usize - 1) * SLOT_DIRECTORY_ENTRY_LENGTH);
        let entry = decode_entry(
            &self.bytes[offset..offset + SLOT_DIRECTORY_ENTRY_LENGTH],
            byte_order,
            counters,
        )?;
        if entry.slot() != slot {
            return Err(
                PageRecordDenial::new(PageRecordDenialKind::SlotEntryMismatch, counters)
                    .with_slot(slot),
            );
        }
        Ok(entry)
    }
}

pub(crate) fn append_occupied_entry(
    page_payload: &[u8],
    byte_order: PhysicalByteOrder,
    slot: PhysicalRecordSlot,
    generation: PhysicalGeneration,
    frame_bytes: &[u8],
) -> Result<Vec<u8>, PageRecordDenial> {
    let counters = PageRecordCounterSnapshot::for_append(1);
    let directory = SlotDirectory::decode(page_payload, byte_order, counters)?;
    if slot.get() <= directory.slot_count() {
        return Err(
            PageRecordDenial::new(PageRecordDenialKind::SlotEntryMismatch, counters)
                .with_slot(slot),
        );
    }
    let new_count = directory.slot_count().max(slot.get());
    let old_directory_len = directory_length(directory.slot_count());
    let new_directory_len = directory_length(new_count);
    let frame_offset = new_directory_len + record_area_len(page_payload, old_directory_len);
    let entry = SlotDirectoryEntry::occupied(
        slot,
        frame_offset as u32,
        frame_bytes.len() as u32,
        generation,
    );

    let mut next = Vec::with_capacity(
        new_directory_len + record_area_len(page_payload, old_directory_len) + frame_bytes.len(),
    );
    next.extend_from_slice(&byte_order.write_u16(new_count));
    next.extend_from_slice(&[0, 0]);
    copy_rebased_existing_entries(
        page_payload,
        directory.slot_count(),
        byte_order,
        (new_directory_len - old_directory_len) as u32,
        counters,
        &mut next,
    )?;
    fill_missing_entries(directory.slot_count(), new_count, byte_order, &mut next);
    write_entry(entry, byte_order, &mut next);
    if next.len() > new_directory_len {
        return Err(PageRecordDenial::new(
            PageRecordDenialKind::SlotDirectoryLengthMismatch,
            counters,
        ));
    }
    while next.len() < new_directory_len {
        next.push(0);
    }
    if page_payload.len() > old_directory_len {
        next.extend_from_slice(&page_payload[old_directory_len..]);
    }
    next.extend_from_slice(frame_bytes);
    Ok(next)
}

pub(crate) const fn directory_length(slot_count: u16) -> usize {
    SLOT_DIRECTORY_PREFIX_LENGTH + (slot_count as usize * SLOT_DIRECTORY_ENTRY_LENGTH)
}

fn record_area_len(page_payload: &[u8], directory_len: usize) -> usize {
    page_payload.len().saturating_sub(directory_len)
}

fn copy_rebased_existing_entries(
    page_payload: &[u8],
    slot_count: u16,
    byte_order: PhysicalByteOrder,
    offset_delta: u32,
    counters: PageRecordCounterSnapshot,
    next: &mut Vec<u8>,
) -> Result<(), PageRecordDenial> {
    for slot_index in 0..slot_count as usize {
        let entry_offset =
            SLOT_DIRECTORY_PREFIX_LENGTH + (slot_index * SLOT_DIRECTORY_ENTRY_LENGTH);
        let entry = decode_entry(
            &page_payload[entry_offset..entry_offset + SLOT_DIRECTORY_ENTRY_LENGTH],
            byte_order,
            counters,
        )?;
        let rebased = if entry.offset() == 0 {
            entry
        } else {
            entry.with_offset(entry.offset() + offset_delta)
        };
        write_entry(rebased, byte_order, next);
    }
    Ok(())
}

fn fill_missing_entries(
    old_count: u16,
    new_count: u16,
    byte_order: PhysicalByteOrder,
    next: &mut Vec<u8>,
) {
    for slot_index in old_count + 1..new_count {
        let slot = PhysicalRecordSlot::from_raw(slot_index).expect("range excludes zero slot");
        write_entry(free_entry(slot), byte_order, next);
    }
}

fn free_entry(slot: PhysicalRecordSlot) -> SlotDirectoryEntry {
    SlotDirectoryEntry {
        slot,
        state: SlotDirectoryEntryState::Free,
        offset: 0,
        frame_length: 0,
        generation: PhysicalGeneration::from_raw(1).expect("static non-zero generation"),
    }
}

fn decode_entry(
    bytes: &[u8],
    byte_order: PhysicalByteOrder,
    counters: PageRecordCounterSnapshot,
) -> Result<SlotDirectoryEntry, PageRecordDenial> {
    let slot = PhysicalRecordSlot::from_raw(byte_order.read_u16([bytes[2], bytes[3]]))
        .map_err(|_| PageRecordDenial::new(PageRecordDenialKind::SlotEntryMismatch, counters))?;
    let generation = PhysicalGeneration::from_raw(byte_order.read_u64([
        bytes[12], bytes[13], bytes[14], bytes[15], bytes[16], bytes[17], bytes[18], bytes[19],
    ]))
    .map_err(|_| PageRecordDenial::new(PageRecordDenialKind::SlotGenerationMismatch, counters))?;
    Ok(SlotDirectoryEntry {
        state: SlotDirectoryEntryState::from_code(bytes[0])
            .ok_or_else(|| PageRecordDenial::new(PageRecordDenialKind::ReservedSlot, counters))?,
        slot,
        offset: byte_order.read_u32([bytes[4], bytes[5], bytes[6], bytes[7]]),
        frame_length: byte_order.read_u32([bytes[8], bytes[9], bytes[10], bytes[11]]),
        generation,
    })
}

fn write_entry(entry: SlotDirectoryEntry, byte_order: PhysicalByteOrder, bytes: &mut Vec<u8>) {
    bytes.push(entry.state().code());
    bytes.push(0);
    bytes.extend_from_slice(&byte_order.write_u16(entry.slot().get()));
    bytes.extend_from_slice(&byte_order.write_u32(entry.offset()));
    bytes.extend_from_slice(&byte_order.write_u32(entry.frame_length()));
    bytes.extend_from_slice(&byte_order.write_u64(entry.generation().get()));
    bytes.extend_from_slice(&[0, 0, 0, 0]);
}
