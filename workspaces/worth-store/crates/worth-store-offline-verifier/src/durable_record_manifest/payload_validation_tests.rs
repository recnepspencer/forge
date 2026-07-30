use sha2::Sha256;
use worth_store_physical_format::PhysicalRecordFormatDeclaration;

use super::{
    read_inline_payload, InlinePayloadExpectation, OfflineDurableManifestDenial,
    OfflineRecordIdentity, OfflineSegmentPageMembership, PayloadDigesters, FRAME_HEADER_BYTES,
};

#[test]
fn hostile_slot_directory_width_is_denied_before_indexing() {
    let parent = tempfile::tempdir().unwrap();
    let segment_directory = parent.path().join("families/records/segments");
    std::fs::create_dir_all(&segment_directory).unwrap();
    let format = PhysicalRecordFormatDeclaration::builder().admit().unwrap();
    let record = OfflineRecordIdentity::decode(&[1; 24]).unwrap();
    let membership = OfflineSegmentPageMembership {
        segment: 1,
        page: 1,
        page_generation: 1,
        data_generation: 1,
        data_page_count: 1,
        frame_index: 0,
    };
    let path = segment_directory.join("segment-0000000000000001-0000000000000001.pages");
    std::fs::write(&path, hostile_inline_page(format)).unwrap();

    let mut aggregate = Sha256::default();
    let mut digest = PayloadDigesters::new(&mut aggregate);
    let denial = read_inline_payload(
        parent.path(),
        format,
        membership,
        InlinePayloadExpectation {
            record,
            page_generation: 1,
            slot_generation: 1,
            payload_bytes: 1,
            slot: u16::MAX,
        },
        &mut digest,
    )
    .unwrap_err();

    assert_eq!(denial, OfflineDurableManifestDenial::MalformedPayloadFrame);
}

fn hostile_inline_page(format: PhysicalRecordFormatDeclaration) -> Vec<u8> {
    let page_bytes = format.page_size().bytes() as usize;
    let mut bytes = vec![0_u8; page_bytes];
    bytes[..8].copy_from_slice(b"WRC5FRM\0");
    bytes[8] = 3;
    bytes[9] = 1;
    bytes[10..20].copy_from_slice(&format.canonical_identity_bytes());
    bytes[20..22].copy_from_slice(&(FRAME_HEADER_BYTES as u16).to_le_bytes());
    bytes[24..28].copy_from_slice(&((page_bytes - FRAME_HEADER_BYTES) as u32).to_le_bytes());
    bytes[28..36].copy_from_slice(&1_u64.to_le_bytes());
    bytes[FRAME_HEADER_BYTES..FRAME_HEADER_BYTES + 8].copy_from_slice(&1_u64.to_le_bytes());
    bytes[FRAME_HEADER_BYTES + 8..FRAME_HEADER_BYTES + 16].copy_from_slice(&1_u64.to_le_bytes());
    bytes[FRAME_HEADER_BYTES + 16..FRAME_HEADER_BYTES + 18]
        .copy_from_slice(&u16::MAX.to_le_bytes());
    let checksum = crc32c(&[&bytes[..36], &bytes[FRAME_HEADER_BYTES..]]);
    bytes[36..FRAME_HEADER_BYTES].copy_from_slice(&checksum.to_le_bytes());
    bytes
}

fn crc32c(parts: &[&[u8]]) -> u32 {
    let mut crc = !0_u32;
    for part in parts {
        for byte in *part {
            crc ^= u32::from(*byte);
            for _ in 0..8 {
                let mask = 0_u32.wrapping_sub(crc & 1);
                crc = (crc >> 1) ^ (0x82f6_3b78 & mask);
            }
        }
    }
    !crc
}
