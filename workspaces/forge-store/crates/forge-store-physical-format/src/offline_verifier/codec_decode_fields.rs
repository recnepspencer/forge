use crate::{
    AllocationClassKind, OfflineVerifierCounterSnapshot, OfflineVerifierDenial,
    OfflineVerifierDenialKind, PhysicalByteOrder, PhysicalGeneration, PhysicalPageId,
    PhysicalSegmentId,
};

pub(crate) fn reject_section_magic(
    bytes: &[u8],
    magic: &[u8; 4],
    kind: OfflineVerifierDenialKind,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<(), OfflineVerifierDenial> {
    if bytes.len() < magic.len() || &bytes[..magic.len()] != magic {
        return Err(OfflineVerifierDenial::new(kind, counters));
    }
    Ok(())
}

pub(crate) fn reject_remaining(
    bytes: &[u8],
    offset: usize,
    row_len: usize,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<(), OfflineVerifierDenial> {
    if bytes.len().saturating_sub(offset) < row_len {
        return Err(OfflineVerifierDenial::new(
            OfflineVerifierDenialKind::MalformedManifestMembership,
            counters,
        ));
    }
    Ok(())
}

pub(crate) fn physical_segment_id(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    offset: usize,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<PhysicalSegmentId, OfflineVerifierDenial> {
    PhysicalSegmentId::from_raw(read_u64(byte_order, bytes, offset))
        .map_err(|error| vocabulary_denial(error, counters))
}

pub(crate) fn physical_page_id(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    offset: usize,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<PhysicalPageId, OfflineVerifierDenial> {
    PhysicalPageId::from_raw(read_u64(byte_order, bytes, offset))
        .map_err(|error| vocabulary_denial(error, counters))
}

pub(crate) fn physical_generation(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    offset: usize,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<PhysicalGeneration, OfflineVerifierDenial> {
    PhysicalGeneration::from_raw(read_u64(byte_order, bytes, offset))
        .map_err(|error| vocabulary_denial(error, counters))
}

pub(crate) fn read_u64(byte_order: PhysicalByteOrder, bytes: &[u8], offset: usize) -> u64 {
    byte_order.read_u64([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
        bytes[offset + 4],
        bytes[offset + 5],
        bytes[offset + 6],
        bytes[offset + 7],
    ])
}

pub(crate) fn decode_allocation_class(
    code: u8,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<AllocationClassKind, OfflineVerifierDenial> {
    match code {
        1 => Ok(AllocationClassKind::OrdinaryRecordPage),
        2 => Ok(AllocationClassKind::LargeRecordExtent),
        3 => Ok(AllocationClassKind::RootManifest),
        4 => Ok(AllocationClassKind::SegmentManifest),
        5 => Ok(AllocationClassKind::ExtentManifest),
        6 => Ok(AllocationClassKind::FreeSpaceMap),
        _ => Err(OfflineVerifierDenial::new(
            OfflineVerifierDenialKind::MalformedManifestMembership,
            counters,
        )),
    }
}

pub(crate) fn vocabulary_denial(
    error: crate::PhysicalVocabularyError,
    counters: OfflineVerifierCounterSnapshot,
) -> OfflineVerifierDenial {
    OfflineVerifierDenial::new(
        OfflineVerifierDenialKind::MalformedManifestMembership,
        counters,
    )
    .with_vocabulary_error(error)
}
