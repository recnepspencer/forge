use crate::{
    ContainerIntegrityCounters, PhysicalBoundaryLocalization, PhysicalContainerIntegrityDenial,
    PhysicalContainerIntegrityDenialKind,
};
use worth_store_physical_format::{
    PhysicalByteOrder, PhysicalFormatVersion, PhysicalFrameKind, PhysicalGeneration,
    PhysicalPublicationState, SlotDirectoryEntry, PHYSICAL_HEADER_LENGTH,
};

pub(crate) fn reject_page_local_frame_header_mismatch(
    frame: &[u8],
    entry: SlotDirectoryEntry,
    counters: ContainerIntegrityCounters,
) -> Result<(), PhysicalContainerIntegrityDenial> {
    let byte_order = PhysicalByteOrder::LittleEndian;
    if PhysicalFrameKind::from_tag(frame[0]) != Some(PhysicalFrameKind::RecordFrame) {
        return Err(header_mismatch(entry, counters));
    }
    let version = byte_order.read_u16([frame[1], frame[2]]);
    if version != PhysicalFormatVersion::initial_format_version().value() {
        return Err(header_mismatch(entry, counters));
    }
    let header_len = byte_order.read_u16([frame[3], frame[4]]);
    if header_len != PHYSICAL_HEADER_LENGTH {
        return Err(PhysicalContainerIntegrityDenial::new(
            PhysicalContainerIntegrityDenialKind::HeaderWitnessMismatch,
            PhysicalBoundaryLocalization::LengthField,
            counters,
        )
        .with_slot(entry.slot())
        .with_lengths(PHYSICAL_HEADER_LENGTH as usize, header_len as usize));
    }
    let generation = byte_order.read_u64([
        frame[9], frame[10], frame[11], frame[12], frame[13], frame[14], frame[15], frame[16],
    ]);
    if PhysicalGeneration::from_raw(generation).ok() != Some(entry.generation()) {
        return Err(header_mismatch(entry, counters));
    }
    if PhysicalPublicationState::from_code(frame[17]).is_none() {
        return Err(header_mismatch(entry, counters));
    }
    let checksum_slot = byte_order.read_u32([frame[18], frame[19], frame[20], frame[21]]);
    let recovery_lsn = byte_order.read_u64([
        frame[22], frame[23], frame[24], frame[25], frame[26], frame[27], frame[28], frame[29],
    ]);
    if checksum_slot != 0 || recovery_lsn != 0 {
        return Err(header_mismatch(entry, counters));
    }
    Ok(())
}

fn header_mismatch(
    entry: SlotDirectoryEntry,
    counters: ContainerIntegrityCounters,
) -> PhysicalContainerIntegrityDenial {
    PhysicalContainerIntegrityDenial::new(
        PhysicalContainerIntegrityDenialKind::HeaderWitnessMismatch,
        PhysicalBoundaryLocalization::FrameHeader,
        counters,
    )
    .with_slot(entry.slot())
}
