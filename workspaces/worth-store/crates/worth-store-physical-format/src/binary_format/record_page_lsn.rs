use crate::store_namespace::{ProposedStoreIdentity, StableStoreIdentity};
use crate::*;

fn format() -> PhysicalRecordFormatDeclaration {
    PhysicalRecordFormatDeclaration::builder().admit().unwrap()
}

fn generation(raw: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(raw).unwrap()
}

fn page_cell(page: u64, generation_raw: u64) -> PageGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .page_cell(
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(page).unwrap(),
        )
        .with_page_generation(generation(generation_raw))
}

fn slot_cell(page: u64, slot: u16, generation_raw: u64) -> SlotGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(page).unwrap(),
            PhysicalRecordSlot::from_raw(slot).unwrap(),
        )
        .with_slot_generation(generation(generation_raw))
}

fn extent_cell(extent: u64, generation_raw: u64) -> RecordExtentGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .record_extent_cell(PhysicalExtentId::from_raw(extent).unwrap())
        .with_extent_generation(generation(generation_raw))
}

#[test]
fn inline_page_lsn_round_trips_without_changing_record_bytes() {
    let record = PersistedRecordIdentity::new([0x41; 16], 1).unwrap();
    let page = page_cell(1, 3);
    let slot = slot_cell(1, 1, 3);
    let mut bytes = encode_inline_page(
        format(),
        page,
        &[InlineRecordAppend::new(record, slot, b"payload")],
    )
    .unwrap();

    assert_eq!(
        decode_data_frame_page_lsn(&bytes, DurableFrameKind::InlinePage),
        Ok(PhysicalPageLsn::GENESIS)
    );
    encode_data_frame_page_lsn(
        &mut bytes,
        DurableFrameKind::InlinePage,
        PhysicalPageLsn::new(71),
    )
    .unwrap();

    assert_eq!(
        decode_data_frame_page_lsn(&bytes, DurableFrameKind::InlinePage),
        Ok(PhysicalPageLsn::new(71))
    );
    let range = decode_inline_record(&bytes, record, page, slot)
        .unwrap()
        .0
        .range();
    assert_eq!(&bytes[range], b"payload");
}

#[test]
fn extent_page_lsn_round_trips_without_changing_chunk_bytes() {
    let record = PersistedRecordIdentity::new([0x52; 16], 2).unwrap();
    let coordinate = ExtentChunkCoordinate::new(record, extent_cell(3, 5), 5, 0, 1).unwrap();
    let mut bytes = encode_extent_chunk(format(), coordinate, b"chunk").unwrap();

    encode_data_frame_page_lsn(
        &mut bytes,
        DurableFrameKind::Extent,
        PhysicalPageLsn::new(83),
    )
    .unwrap();

    assert_eq!(
        decode_data_frame_page_lsn(&bytes, DurableFrameKind::Extent),
        Ok(PhysicalPageLsn::new(83))
    );
    assert_eq!(decode_extent_chunk(&bytes, coordinate).unwrap().0, b"chunk");
}

#[test]
fn page_lsn_is_covered_by_the_durable_frame_checksum() {
    let mut bytes = encode_inline_page(format(), page_cell(1, 1), &[]).unwrap();
    encode_data_frame_page_lsn(
        &mut bytes,
        DurableFrameKind::InlinePage,
        PhysicalPageLsn::new(97),
    )
    .unwrap();
    bytes[36] ^= 1;

    assert_eq!(
        decode_data_frame_page_lsn(&bytes, DurableFrameKind::InlinePage),
        Err(DurableFrameDenial::IntegrityMismatch)
    );
}

#[test]
fn non_data_artifacts_cannot_receive_or_expose_page_lsn() {
    let store = StableStoreIdentity::from_published_record(
        ProposedStoreIdentity::from_nonzero_bytes([0x63; 16]).unwrap(),
    );
    let root = CurrentRootCatalogEntry::new(CurrentRootCatalogGeneration::new(1).unwrap());
    let mut bytes = BootstrapCatalog::new(store, format(), root).encode();

    assert_eq!(
        encode_data_frame_page_lsn(
            &mut bytes,
            DurableFrameKind::BootstrapCatalog,
            PhysicalPageLsn::new(1),
        ),
        Err(DurableFrameDenial::IllegalKind(
            DurableFrameKind::BootstrapCatalog as u8
        ))
    );
    assert_eq!(
        decode_data_frame_page_lsn(&bytes, DurableFrameKind::BootstrapCatalog),
        Err(DurableFrameDenial::IllegalKind(
            DurableFrameKind::BootstrapCatalog as u8
        ))
    );
}
