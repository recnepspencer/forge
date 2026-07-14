use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalSegmentId, SlotGenerationCell,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaselineBTreeCorruptionMarker {
    Header,
    CellPayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineBTreeLeafRecord {
    slots: [PhysicalRecordSlot; 2],
    sibling_links_present: bool,
    tombstones_present: bool,
}

impl BaselineBTreeLeafRecord {
    pub const fn slots(self) -> [PhysicalRecordSlot; 2] {
        self.slots
    }
    pub const fn sibling_links_present(self) -> bool {
        self.sibling_links_present
    }
    pub const fn tombstones_present(self) -> bool {
        self.tombstones_present
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineBTreeRootNode {
    corruption_marker: BaselineBTreeCorruptionMarker,
    separator_slot: PhysicalRecordSlot,
    left_child: SlotGenerationCell,
    right_child: SlotGenerationCell,
}

impl BaselineBTreeRootNode {
    pub const fn corruption_marker(self) -> BaselineBTreeCorruptionMarker {
        self.corruption_marker
    }
    pub const fn separator_slot(self) -> PhysicalRecordSlot {
        self.separator_slot
    }
    pub const fn left_child(self) -> SlotGenerationCell {
        self.left_child
    }
    pub const fn right_child(self) -> SlotGenerationCell {
        self.right_child
    }
}

pub fn encode_leaf_record(
    slots: [PhysicalRecordSlot; 2],
    sibling_links_present: bool,
    tombstones_present: bool,
) -> [u8; 6] {
    let [first_low, first_high] = slots[0].get().to_le_bytes();
    let [second_low, second_high] = slots[1].get().to_le_bytes();
    [
        b'L',
        sibling_links_present as u8 | ((tombstones_present as u8) << 1),
        first_low,
        first_high,
        second_low,
        second_high,
    ]
}

pub fn decode_leaf_record(bytes: &[u8]) -> Option<BaselineBTreeLeafRecord> {
    if bytes.len() != 6 || bytes[0] != b'L' {
        return None;
    }
    Some(BaselineBTreeLeafRecord {
        slots: [
            PhysicalRecordSlot::from_raw(u16::from_le_bytes([bytes[2], bytes[3]])).ok()?,
            PhysicalRecordSlot::from_raw(u16::from_le_bytes([bytes[4], bytes[5]])).ok()?,
        ],
        sibling_links_present: bytes[1] & 0b01 != 0,
        tombstones_present: bytes[1] & 0b10 != 0,
    })
}

pub fn encode_root_record(
    corruption_marker: BaselineBTreeCorruptionMarker,
    separator_slot: PhysicalRecordSlot,
    left_child: SlotGenerationCell,
    right_child: SlotGenerationCell,
) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(56);
    bytes.push(b'R');
    bytes.push(match corruption_marker {
        BaselineBTreeCorruptionMarker::Header => 0,
        BaselineBTreeCorruptionMarker::CellPayload => 1,
    });
    bytes.extend_from_slice(&separator_slot.get().to_le_bytes());
    encode_slot_cell(&mut bytes, left_child);
    encode_slot_cell(&mut bytes, right_child);
    bytes
}

pub fn decode_root_record(bytes: &[u8]) -> Option<BaselineBTreeRootNode> {
    if bytes.len() != 56 || bytes[0] != b'R' {
        return None;
    }
    Some(BaselineBTreeRootNode {
        corruption_marker: match bytes[1] {
            0 => BaselineBTreeCorruptionMarker::Header,
            1 => BaselineBTreeCorruptionMarker::CellPayload,
            _ => return None,
        },
        separator_slot: PhysicalRecordSlot::from_raw(u16::from_le_bytes([bytes[2], bytes[3]]))
            .ok()?,
        left_child: decode_slot_cell(&bytes[4..30])?,
        right_child: decode_slot_cell(&bytes[30..56])?,
    })
}

fn encode_slot_cell(bytes: &mut Vec<u8>, cell: SlotGenerationCell) {
    bytes.extend_from_slice(&cell.segment_id().get().to_le_bytes());
    bytes.extend_from_slice(&cell.page_id().get().to_le_bytes());
    bytes.extend_from_slice(&cell.slot().get().to_le_bytes());
    bytes.extend_from_slice(&cell.generation().get().to_le_bytes());
}

fn decode_slot_cell(bytes: &[u8]) -> Option<SlotGenerationCell> {
    if bytes.len() != 26 {
        return None;
    }
    Some(
        PhysicalGenerationAuthority::for_canonical_physical_format()
            .slot_cell(
                PhysicalSegmentId::from_raw(u64::from_le_bytes(bytes[0..8].try_into().ok()?))
                    .ok()?,
                PhysicalPageId::from_raw(u64::from_le_bytes(bytes[8..16].try_into().ok()?)).ok()?,
                PhysicalRecordSlot::from_raw(u16::from_le_bytes(bytes[16..18].try_into().ok()?))
                    .ok()?,
            )
            .with_slot_generation(
                PhysicalGeneration::from_raw(u64::from_le_bytes(bytes[18..26].try_into().ok()?))
                    .ok()?,
            ),
    )
}
