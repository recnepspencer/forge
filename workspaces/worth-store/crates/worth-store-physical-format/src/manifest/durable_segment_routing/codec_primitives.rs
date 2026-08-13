use super::*;

pub(super) fn encode_entry(target: &mut [u8], entry: RecordSegmentPageManifestEntry) {
    target[..8].copy_from_slice(&entry.page_cell().segment_id().get().to_le_bytes());
    target[8..16].copy_from_slice(&entry.page().get().to_le_bytes());
    target[16..24].copy_from_slice(&entry.page_generation().to_le_bytes());
    target[24..32].copy_from_slice(&entry.data_generation().to_le_bytes());
    target[32..36].copy_from_slice(&entry.data_page_count().to_le_bytes());
    target[36..40].copy_from_slice(&entry.frame_index().to_le_bytes());
}

pub(super) fn decode_entry(bytes: &[u8]) -> Option<RecordSegmentPageManifestEntry> {
    let segment =
        PhysicalSegmentId::from_raw(u64::from_le_bytes(bytes[..8].try_into().ok()?)).ok()?;
    let page = PhysicalPageId::from_raw(u64::from_le_bytes(bytes[8..16].try_into().ok()?)).ok()?;
    let page_generation =
        PhysicalGeneration::from_raw(u64::from_le_bytes(bytes[16..24].try_into().ok()?)).ok()?;
    let data_generation =
        PhysicalGeneration::from_raw(u64::from_le_bytes(bytes[24..32].try_into().ok()?)).ok()?;
    let authority = PhysicalGenerationAuthority::for_canonical_physical_format();
    RecordSegmentPageManifestEntry::new(
        authority
            .page_cell(segment, page)
            .with_page_generation(page_generation),
        authority
            .segment_cell(segment)
            .with_segment_generation(data_generation),
        u32::from_le_bytes(bytes[32..36].try_into().ok()?),
        u32::from_le_bytes(bytes[36..40].try_into().ok()?),
    )
}

pub(crate) fn encode_reference(target: &mut [u8], reference: SegmentManifestBlockReference) {
    target[..8].copy_from_slice(&reference.generation().to_le_bytes());
    target[8..16].copy_from_slice(&reference.block().to_le_bytes());
    target[16..18].copy_from_slice(&reference.level().to_le_bytes());
    target[20..24].copy_from_slice(&reference.checksum().to_le_bytes());
    encode_key(&mut target[24..40], reference.first());
    encode_key(&mut target[40..56], reference.last());
}

pub(crate) fn decode_reference(bytes: &[u8]) -> Option<SegmentManifestBlockReference> {
    if bytes[18..20] != [0; 2] {
        return None;
    }
    SegmentManifestBlockReference::new(
        u64::from_le_bytes(bytes[..8].try_into().ok()?),
        u64::from_le_bytes(bytes[8..16].try_into().ok()?),
        u16::from_le_bytes(bytes[16..18].try_into().ok()?),
        u32::from_le_bytes(bytes[20..24].try_into().ok()?),
        decode_key(&bytes[24..40])?,
        decode_key(&bytes[40..56])?,
    )
}

fn encode_key(target: &mut [u8], key: SegmentPageKey) {
    target[..8].copy_from_slice(&key.segment().get().to_le_bytes());
    target[8..16].copy_from_slice(&key.page().get().to_le_bytes());
}

fn decode_key(bytes: &[u8]) -> Option<SegmentPageKey> {
    Some(SegmentPageKey::new(
        PhysicalSegmentId::from_raw(u64::from_le_bytes(bytes[..8].try_into().ok()?)).ok()?,
        PhysicalPageId::from_raw(u64::from_le_bytes(bytes[8..16].try_into().ok()?)).ok()?,
    ))
}
