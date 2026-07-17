use super::layout::{OWNER_PRIMARY_OFFSET, OWNER_SECONDARY_OFFSET, OWNER_TERTIARY_OFFSET};
use crate::{
    ExtentGenerationCell, PageGenerationCell, PhysicalByteOrder, PhysicalCellReuseDomain,
    PhysicalFormatVersion, PhysicalFrameKind, PhysicalGenerationOwner,
    PhysicalHeaderDecodeCounterSnapshot, PhysicalHeaderDecodeDenial,
    PhysicalHeaderDecodeDenialKind, PhysicalPageKind, PhysicalPublicationState, SlotGenerationCell,
    PHYSICAL_HEADER_LENGTH,
};

pub(crate) fn encode_page_header(
    byte_order: PhysicalByteOrder,
    kind: PhysicalPageKind,
    owner: PageGenerationCell,
    payload_length: u32,
) -> [u8; PHYSICAL_HEADER_LENGTH as usize] {
    encode_header(
        byte_order,
        kind.tag(),
        owner.generation().get(),
        payload_length,
        owner.segment_id().get(),
        owner.page_id().get(),
        0,
    )
}

pub(crate) fn encode_record_frame_header(
    byte_order: PhysicalByteOrder,
    owner: SlotGenerationCell,
    payload_length: u32,
) -> [u8; PHYSICAL_HEADER_LENGTH as usize] {
    encode_header(
        byte_order,
        PhysicalFrameKind::RecordFrame.tag(),
        owner.generation().get(),
        payload_length,
        owner.segment_id().get(),
        owner.page_id().get(),
        owner.slot().get(),
    )
}

pub(crate) fn encode_extent_frame_header(
    byte_order: PhysicalByteOrder,
    owner: ExtentGenerationCell,
    payload_length: u32,
) -> [u8; PHYSICAL_HEADER_LENGTH as usize] {
    encode_header(
        byte_order,
        PhysicalFrameKind::ExtentRecordFrame.tag(),
        owner.generation().get(),
        payload_length,
        owner.segment_id().get(),
        owner.extent_id().get(),
        0,
    )
}

pub(crate) fn reject_page_owner_coordinates(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    expected: PageGenerationCell,
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> Result<(), PhysicalHeaderDecodeDenial> {
    reject_generation_owner_coordinates(byte_order, bytes, expected.owner(), counters)
}

pub(crate) fn reject_frame_owner_coordinates(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    expected: PhysicalGenerationOwner,
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> Result<(), PhysicalHeaderDecodeDenial> {
    reject_generation_owner_coordinates(byte_order, bytes, expected, counters)
}

pub(crate) fn reject_generation_owner_coordinates(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    expected: PhysicalGenerationOwner,
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> Result<(), PhysicalHeaderDecodeDenial> {
    let (primary, secondary, tertiary) = match expected.domain() {
        PhysicalCellReuseDomain::Page => (
            expected.segment_id().map(|value| value.get()),
            expected.page_id().map(|value| value.get()),
            Some(0),
        ),
        PhysicalCellReuseDomain::SlotAllocation => (
            expected.segment_id().map(|value| value.get()),
            expected.page_id().map(|value| value.get()),
            expected.slot().map(|value| value.get()),
        ),
        PhysicalCellReuseDomain::ExtentAllocation => (
            expected.segment_id().map(|value| value.get()),
            expected.extent_id().map(|value| value.get()),
            Some(0),
        ),
        _ => (None, None, None),
    };
    let (Some(primary), Some(secondary), Some(tertiary)) = (primary, secondary, tertiary) else {
        return Err(owner_mismatch(counters));
    };
    reject_owner_coordinates(byte_order, bytes, primary, secondary, tertiary, counters)
}

#[allow(clippy::too_many_arguments)]
fn encode_header(
    byte_order: PhysicalByteOrder,
    tag: u8,
    generation: u64,
    payload_length: u32,
    owner_primary: u64,
    owner_secondary: u64,
    owner_tertiary: u16,
) -> [u8; PHYSICAL_HEADER_LENGTH as usize] {
    let mut bytes = [0_u8; PHYSICAL_HEADER_LENGTH as usize];
    bytes[0] = tag;
    bytes[1..3].copy_from_slice(
        &byte_order.write_u16(PhysicalFormatVersion::initial_format_version().value()),
    );
    bytes[3..5].copy_from_slice(&byte_order.write_u16(PHYSICAL_HEADER_LENGTH));
    bytes[5..9].copy_from_slice(&byte_order.write_u32(payload_length));
    bytes[9..17].copy_from_slice(&byte_order.write_u64(generation));
    bytes[17] = PhysicalPublicationState::Published.code();
    bytes[OWNER_PRIMARY_OFFSET..OWNER_PRIMARY_OFFSET + 8]
        .copy_from_slice(&byte_order.write_u64(owner_primary));
    bytes[OWNER_SECONDARY_OFFSET..OWNER_SECONDARY_OFFSET + 8]
        .copy_from_slice(&byte_order.write_u64(owner_secondary));
    bytes[OWNER_TERTIARY_OFFSET..OWNER_TERTIARY_OFFSET + 2]
        .copy_from_slice(&byte_order.write_u16(owner_tertiary));
    bytes
}

fn reject_owner_coordinates(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    expected_primary: u64,
    expected_secondary: u64,
    expected_tertiary: u16,
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> Result<(), PhysicalHeaderDecodeDenial> {
    let primary = read_u64(byte_order, bytes, OWNER_PRIMARY_OFFSET);
    let secondary = read_u64(byte_order, bytes, OWNER_SECONDARY_OFFSET);
    let tertiary = byte_order.read_u16([
        bytes[OWNER_TERTIARY_OFFSET],
        bytes[OWNER_TERTIARY_OFFSET + 1],
    ]);
    if (primary, secondary, tertiary) != (expected_primary, expected_secondary, expected_tertiary) {
        Err(owner_mismatch(counters))
    } else {
        Ok(())
    }
}

fn read_u64(byte_order: PhysicalByteOrder, bytes: &[u8], offset: usize) -> u64 {
    byte_order.read_u64(
        bytes[offset..offset + 8]
            .try_into()
            .expect("fixed owner coordinate"),
    )
}

const fn owner_mismatch(
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> PhysicalHeaderDecodeDenial {
    PhysicalHeaderDecodeDenial::new(
        PhysicalHeaderDecodeDenialKind::OwnerCoordinateMismatch,
        counters,
    )
}
