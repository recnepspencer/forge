use crate::record_framing::decode_durable_frame;
use crate::{
    DurableFrameKind, PersistedRecordIdentity, PhysicalGeneration, PhysicalRecordFormatDeclaration,
    PhysicalRecordSlot,
};

use super::{
    inspect_inline_page, InlinePageDenial, DURABLE_INLINE_PAGE_PREFIX_BYTES,
    DURABLE_INLINE_SLOT_BYTES,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InlinePageRecordDescriptor {
    record: PersistedRecordIdentity,
    slot: PhysicalRecordSlot,
    slot_generation: PhysicalGeneration,
    payload_bytes: u32,
}

impl InlinePageRecordDescriptor {
    pub const fn record(self) -> PersistedRecordIdentity {
        self.record
    }
    pub const fn slot(self) -> PhysicalRecordSlot {
        self.slot
    }
    pub const fn slot_generation(self) -> PhysicalGeneration {
        self.slot_generation
    }
    pub const fn payload_bytes(self) -> u32 {
        self.payload_bytes
    }
}

pub fn inspect_inline_page_records(
    format: PhysicalRecordFormatDeclaration,
    bytes: &[u8],
) -> Result<Vec<InlinePageRecordDescriptor>, InlinePageDenial> {
    let geometry = inspect_inline_page(format, bytes)?;
    let (_, frame) = decode_durable_frame(bytes, DurableFrameKind::InlinePage)
        .map_err(InlinePageDenial::Frame)?;
    (0..geometry.slot_count())
        .map(|index| {
            let base =
                DURABLE_INLINE_PAGE_PREFIX_BYTES + usize::from(index) * DURABLE_INLINE_SLOT_BYTES;
            let record = PersistedRecordIdentity::new(
                frame.payload[base..base + 16].try_into().unwrap(),
                u64::from_le_bytes(frame.payload[base + 16..base + 24].try_into().unwrap()),
            )
            .ok_or(InlinePageDenial::InvalidRecordIdentity)?;
            let slot = PhysicalRecordSlot::from_raw(index + 1)
                .map_err(|_| InlinePageDenial::InvalidSlot)?;
            let slot_generation = PhysicalGeneration::from_raw(u64::from_le_bytes(
                frame.payload[base + 32..base + 40].try_into().unwrap(),
            ))
            .map_err(|_| InlinePageDenial::InvalidSlot)?;
            Ok(InlinePageRecordDescriptor {
                record,
                slot,
                slot_generation,
                payload_bytes: u32::from_le_bytes(
                    frame.payload[base + 28..base + 32].try_into().unwrap(),
                ),
            })
        })
        .collect()
}
