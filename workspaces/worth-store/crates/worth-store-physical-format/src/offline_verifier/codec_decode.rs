use crate::offline_verifier::codec::{
    DecodedOfflineManifestSections, ALLOCATION_ROW_LENGTH, EXTENT_MAGIC, EXTENT_ROW_LENGTH,
    FREE_MAGIC, FREE_ROW_LENGTH, PAGE_SLOT_ROW_LENGTH, ROOT_BODY_LENGTH, ROOT_MAGIC, SEGMENT_MAGIC,
    SEGMENT_ROW_LENGTH,
};
use crate::offline_verifier::codec_decode_fields::{
    decode_allocation_class, physical_generation, physical_page_id, physical_segment_id, read_u64,
    reject_remaining, reject_section_magic, vocabulary_denial,
};
use crate::{
    AllocationClassKind, AllocationClassManifestEntry, ExtentManifestEntry, FreeSpaceManifestEntry,
    FreeSpaceReuseCell, OfflineVerifierCounterSnapshot, OfflineVerifierDenial,
    OfflineVerifierDenialKind, PhysicalByteOrder, PhysicalExtentId, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalRecordSlot, PhysicalRootReference, PhysicalSegmentId,
    RootPublicationCell, SegmentManifestEntry, SegmentPageManifestEntry,
};

pub(crate) fn decode(
    byte_order: PhysicalByteOrder,
    root: &[u8],
    segment_manifest: &[u8],
    extent_manifest: &[u8],
    free_space_map: &[u8],
    counters: OfflineVerifierCounterSnapshot,
) -> Result<DecodedOfflineManifestSections, OfflineVerifierDenial> {
    let root = decode_root(byte_order, root, counters)?;
    let (segments, page_slots, segment_rows) =
        decode_segment_manifest(byte_order, segment_manifest, counters)?;
    let (extents, allocation_classes, extent_rows) =
        decode_extent_manifest(byte_order, extent_manifest, counters)?;
    let (free_space, free_rows) = decode_free_space_map(byte_order, free_space_map, counters)?;
    Ok(DecodedOfflineManifestSections {
        root,
        segments,
        page_slots,
        extents,
        allocation_classes,
        free_space,
        decoded_rows: 1 + segment_rows + extent_rows + free_rows,
    })
}

pub(crate) fn decode_root(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    counters: OfflineVerifierCounterSnapshot,
) -> Result<RootPublicationCell, OfflineVerifierDenial> {
    reject_section_magic(
        bytes,
        ROOT_MAGIC,
        OfflineVerifierDenialKind::MalformedRootManifest,
        counters,
    )?;
    if bytes.len() != ROOT_MAGIC.len() + ROOT_BODY_LENGTH {
        return Err(OfflineVerifierDenial::new(
            OfflineVerifierDenialKind::MalformedRootManifest,
            counters,
        ));
    }
    let offset = ROOT_MAGIC.len();
    let root_reference = PhysicalRootReference::from_raw(read_u64(byte_order, bytes, offset))
        .map_err(|error| {
            OfflineVerifierDenial::new(OfflineVerifierDenialKind::MalformedRootManifest, counters)
                .with_vocabulary_error(error)
        })?;
    let generation = PhysicalGeneration::from_raw(read_u64(byte_order, bytes, offset + 8))
        .map_err(|error| {
            OfflineVerifierDenial::new(OfflineVerifierDenialKind::MalformedRootManifest, counters)
                .with_vocabulary_error(error)
        })?;
    Ok(PhysicalGenerationAuthority::for_canonical_physical_format()
        .root_publication_cell(root_reference)
        .with_root_publication_generation(generation))
}

fn decode_segment_manifest(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    counters: OfflineVerifierCounterSnapshot,
) -> Result<
    (
        Vec<SegmentManifestEntry>,
        Vec<SegmentPageManifestEntry>,
        u32,
    ),
    OfflineVerifierDenial,
> {
    reject_section_magic(
        bytes,
        SEGMENT_MAGIC,
        OfflineVerifierDenialKind::MalformedSegmentManifest,
        counters,
    )?;
    let mut segments = Vec::new();
    let mut page_slots = Vec::new();
    let mut offset = SEGMENT_MAGIC.len();
    while offset < bytes.len() {
        match bytes[offset] {
            0x01 => {
                reject_remaining(bytes, offset, SEGMENT_ROW_LENGTH, counters)?;
                segments.push(SegmentManifestEntry::new(decode_segment_row(
                    byte_order, bytes, offset, counters,
                )?));
                offset += SEGMENT_ROW_LENGTH;
            }
            0x02 => {
                reject_remaining(bytes, offset, PAGE_SLOT_ROW_LENGTH, counters)?;
                page_slots.push(SegmentPageManifestEntry::new(decode_page_slot_row(
                    byte_order, bytes, offset, counters,
                )?));
                offset += PAGE_SLOT_ROW_LENGTH;
            }
            _ => {
                return Err(OfflineVerifierDenial::new(
                    OfflineVerifierDenialKind::MalformedSegmentManifest,
                    counters,
                ));
            }
        }
    }
    let row_count = (segments.len() + page_slots.len()) as u32;
    Ok((segments, page_slots, row_count))
}

fn decode_extent_manifest(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    counters: OfflineVerifierCounterSnapshot,
) -> Result<
    (
        Vec<ExtentManifestEntry>,
        Vec<AllocationClassManifestEntry>,
        u32,
    ),
    OfflineVerifierDenial,
> {
    reject_section_magic(
        bytes,
        EXTENT_MAGIC,
        OfflineVerifierDenialKind::MalformedExtentManifest,
        counters,
    )?;
    let mut extents = Vec::new();
    let mut allocation_classes = Vec::new();
    let mut offset = EXTENT_MAGIC.len();
    while offset < bytes.len() {
        match bytes[offset] {
            0x01 => {
                reject_remaining(bytes, offset, EXTENT_ROW_LENGTH, counters)?;
                extents.push(ExtentManifestEntry::new(decode_extent_row(
                    byte_order, bytes, offset, counters,
                )?));
                offset += EXTENT_ROW_LENGTH;
            }
            0x02 => {
                reject_remaining(bytes, offset, 1 + ALLOCATION_ROW_LENGTH, counters)?;
                allocation_classes.push(AllocationClassManifestEntry::new(
                    decode_allocation_class(bytes[offset + 1], counters)?,
                ));
                offset += 1 + ALLOCATION_ROW_LENGTH;
            }
            _ => {
                return Err(OfflineVerifierDenial::new(
                    OfflineVerifierDenialKind::MalformedExtentManifest,
                    counters,
                ));
            }
        }
    }
    let row_count = (extents.len() + allocation_classes.len()) as u32;
    Ok((extents, allocation_classes, row_count))
}

fn decode_free_space_map(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    counters: OfflineVerifierCounterSnapshot,
) -> Result<(Vec<FreeSpaceManifestEntry>, u32), OfflineVerifierDenial> {
    reject_section_magic(
        bytes,
        FREE_MAGIC,
        OfflineVerifierDenialKind::MalformedFreeSpaceMap,
        counters,
    )?;
    let mut entries = Vec::new();
    let mut offset = FREE_MAGIC.len();
    while offset < bytes.len() {
        reject_remaining(bytes, offset, FREE_ROW_LENGTH, counters)?;
        entries.push(FreeSpaceManifestEntry::new(decode_free_space_row(
            byte_order, bytes, offset, counters,
        )?));
        offset += FREE_ROW_LENGTH;
    }
    let rows = entries.len() as u32;
    Ok((entries, rows))
}

fn decode_segment_row(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    offset: usize,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<crate::SegmentGenerationCell, OfflineVerifierDenial> {
    let segment_id = physical_segment_id(byte_order, bytes, offset + 1, counters)?;
    let generation = physical_generation(byte_order, bytes, offset + 9, counters)?;
    Ok(PhysicalGenerationAuthority::for_canonical_physical_format()
        .segment_cell(segment_id)
        .with_segment_generation(generation))
}

fn decode_page_slot_row(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    offset: usize,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<crate::SlotGenerationCell, OfflineVerifierDenial> {
    let segment_id = physical_segment_id(byte_order, bytes, offset + 1, counters)?;
    let page_id = physical_page_id(byte_order, bytes, offset + 9, counters)?;
    let slot =
        PhysicalRecordSlot::from_raw(byte_order.read_u16([bytes[offset + 17], bytes[offset + 18]]))
            .map_err(|error| vocabulary_denial(error, counters))?;
    let generation = physical_generation(byte_order, bytes, offset + 19, counters)?;
    Ok(PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(segment_id, page_id, slot)
        .with_slot_generation(generation))
}

fn decode_extent_row(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    offset: usize,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<crate::ExtentGenerationCell, OfflineVerifierDenial> {
    let segment_id = physical_segment_id(byte_order, bytes, offset + 1, counters)?;
    let extent_id = PhysicalExtentId::from_raw(read_u64(byte_order, bytes, offset + 9))
        .map_err(|error| vocabulary_denial(error, counters))?;
    let generation = physical_generation(byte_order, bytes, offset + 17, counters)?;
    Ok(PhysicalGenerationAuthority::for_canonical_physical_format()
        .extent_cell(segment_id, extent_id)
        .with_extent_generation(generation))
}

fn decode_free_space_row(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    offset: usize,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<FreeSpaceReuseCell, OfflineVerifierDenial> {
    let allocation_class = decode_allocation_class(bytes[offset + 1], counters)?;
    let segment_id = physical_segment_id(byte_order, bytes, offset + 2, counters)?;
    let generation = physical_generation(byte_order, bytes, offset + 20, counters)?;
    match bytes[offset] {
        0x01 => decode_free_space_slot_cell(
            byte_order,
            bytes,
            offset,
            counters,
            allocation_class,
            segment_id,
            generation,
        ),
        0x02 => decode_free_space_extent_cell(
            byte_order,
            bytes,
            offset,
            counters,
            allocation_class,
            segment_id,
            generation,
        ),
        _ => Err(OfflineVerifierDenial::new(
            OfflineVerifierDenialKind::MalformedFreeSpaceMap,
            counters,
        )),
    }
}

fn decode_free_space_slot_cell(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    offset: usize,
    counters: OfflineVerifierCounterSnapshot,
    allocation_class: AllocationClassKind,
    segment_id: PhysicalSegmentId,
    generation: PhysicalGeneration,
) -> Result<FreeSpaceReuseCell, OfflineVerifierDenial> {
    let page_id = physical_page_id(byte_order, bytes, offset + 10, counters)?;
    let slot =
        PhysicalRecordSlot::from_raw(byte_order.read_u16([bytes[offset + 18], bytes[offset + 19]]))
            .map_err(|error| vocabulary_denial(error, counters))?;
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .free_space_slot_cell(segment_id, page_id, slot, allocation_class)
        .map_err(|error| vocabulary_denial(error, counters))
        .map(|builder| builder.with_free_space_generation(generation))
}

fn decode_free_space_extent_cell(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    offset: usize,
    counters: OfflineVerifierCounterSnapshot,
    allocation_class: AllocationClassKind,
    segment_id: PhysicalSegmentId,
    generation: PhysicalGeneration,
) -> Result<FreeSpaceReuseCell, OfflineVerifierDenial> {
    let extent_id = PhysicalExtentId::from_raw(read_u64(byte_order, bytes, offset + 10))
        .map_err(|error| vocabulary_denial(error, counters))?;
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .free_space_extent_cell(segment_id, extent_id, allocation_class)
        .map_err(|error| vocabulary_denial(error, counters))
        .map(|builder| builder.with_free_space_generation(generation))
}
