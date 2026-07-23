use crate::store_namespace::StoreNamespaceIdentityRecord;
use crate::*;

#[test]
fn bounded_arbitrary_bytes_never_escape_any_durable_decoder() {
    let mut state = 0x4d59_5df4_d0f3_3173_u64;
    for length in 0..=512 {
        let mut bytes = vec![0_u8; length];
        fill_deterministic(&mut bytes, &mut state);
        exercise_durable_decoders(&bytes);
    }
}

#[test]
fn bounded_checksum_valid_frames_never_escape_semantic_decoders() {
    let mut state = 0x94d0_49bb_1331_11eb_u64;
    for kind in 1..=10 {
        for payload_length in [0, 1, 7, 15, 23, 39, 55, 63, 87, 127, 255, 511] {
            let mut payload = vec![0_u8; payload_length];
            fill_deterministic(&mut payload, &mut state);
            exercise_durable_decoders(&checksum_valid_frame(kind, state, &payload));
        }
    }
}

fn exercise_durable_decoders(bytes: &[u8]) {
    let format = PhysicalRecordFormatDeclaration::builder().admit().unwrap();
    let record = PersistedRecordIdentity::new([0x51; 16], 1).unwrap();
    let generation = PhysicalGeneration::from_raw(1).unwrap();
    let authority = PhysicalGenerationAuthority::for_canonical_physical_format();
    let page = authority
        .page_cell(
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(1).unwrap(),
        )
        .with_page_generation(generation);
    let slot = authority
        .slot_cell(
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(1).unwrap(),
            PhysicalRecordSlot::from_raw(1).unwrap(),
        )
        .with_slot_generation(generation);
    let extent = authority
        .record_extent_cell(PhysicalExtentId::from_raw(1).unwrap())
        .with_extent_generation(generation);
    let coordinate = ExtentChunkCoordinate::new(record, extent, 1, 0, 1).unwrap();

    let _ = BootstrapCatalog::decode(bytes);
    let _ = StoreNamespaceIdentityRecord::decode(bytes);
    let _ = DurableExtentManifest::decode(bytes);
    let _ = DurableSegmentManifest::decode(bytes, u32::MAX);
    let _ = inspect_inline_page(format, bytes);
    let _ = decode_inline_record(bytes, record, page, slot);
    let _ = decode_extent_chunk(bytes, coordinate);
    let _ = SlotDirectory::decode(
        bytes,
        PhysicalByteOrder::LittleEndian,
        PageRecordCounterSnapshot::default(),
    );
    for capacity in [0, 1, 2, 64, u16::MAX] {
        let _ = DurablePhysicalRootManifest::decode(bytes, capacity);
        let _ = DurableFreeSpaceManifestHeader::decode(bytes, capacity);
        let _ = PhysicalRootRoutingBlock::decode(bytes, capacity);
        let _ = PhysicalSegmentMembershipBlock::decode(bytes, capacity);
        let _ = PhysicalFreeSpaceMembershipBlock::decode(bytes, capacity);
    }
}

fn fill_deterministic(bytes: &mut [u8], state: &mut u64) {
    for byte in bytes {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *byte = *state as u8;
    }
}

fn checksum_valid_frame(kind: u8, identity: u64, payload: &[u8]) -> Vec<u8> {
    let mut bytes = vec![0_u8; 40 + payload.len()];
    bytes[..8].copy_from_slice(b"WRC5FRM\0");
    bytes[8] = kind;
    bytes[9] = 1;
    bytes[10..20].copy_from_slice(&[1, 0, 0, 64, 0, 0, 1, 1, 1, 24]);
    bytes[20..22].copy_from_slice(&40_u16.to_le_bytes());
    bytes[24..28].copy_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes[28..36].copy_from_slice(&identity.to_le_bytes());
    bytes[40..].copy_from_slice(payload);
    let checksum = crate::record_framing::crc32c::checksum(&[&bytes[..36], payload]);
    bytes[36..40].copy_from_slice(&checksum.to_le_bytes());
    bytes
}
