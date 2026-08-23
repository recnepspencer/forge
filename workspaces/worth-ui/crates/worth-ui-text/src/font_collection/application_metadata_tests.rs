use std::sync::Arc;

use worth_ui_host_contract::UiFontSlant;

use super::{
    application_test_world::{assert_pack_denial, face, profile_collection_and_sources},
    UiApplicationFontPackDefinition, UiFontCollectionAdmissionDenial,
};

#[test]
pub(super) fn malformed_localized_name_record_is_denied_before_pack_publication() {
    let (profile, sources) = profile_collection_and_sources();
    let bytes = corrupt_first_unicode_family_name(Arc::clone(&sources["noto-sans-roman"]));
    assert!(harfrust::FontRef::from_index(&bytes, 0).is_ok());
    assert_pack_denial(
        &profile,
        UiApplicationFontPackDefinition {
            name: Arc::from("malformed localized metadata"),
            faces: Box::new([face("Application", bytes, 0, UiFontSlant::Upright)]),
        },
        UiFontCollectionAdmissionDenial::FaceMetadataMismatch,
    );
}

fn corrupt_first_unicode_family_name(bytes: Arc<[u8]>) -> Arc<[u8]> {
    let mut mutated = bytes.to_vec();
    let table_count = usize::from(be_u16(&mutated, 4));
    let record = (0..table_count)
        .map(|index| 12 + index * 16)
        .find(|start| mutated[*start..*start + 4] == *b"name")
        .expect("fixture owns a name table");
    let table = usize::try_from(be_u32(&mutated, record + 8)).unwrap();
    let name_count = usize::from(be_u16(&mutated, table + 2));
    let string_base = table + usize::from(be_u16(&mutated, table + 4));
    let name = (0..name_count)
        .map(|index| table + 6 + index * 12)
        .find(|start| {
            let platform = be_u16(&mutated, *start);
            let name_id = be_u16(&mutated, *start + 6);
            matches!(platform, 0 | 3) && matches!(name_id, 1 | 16)
        })
        .expect("fixture owns a Unicode family name");
    let length = usize::from(be_u16(&mutated, name + 8));
    let offset = usize::from(be_u16(&mutated, name + 10));
    assert!(length >= 2);
    mutated[string_base + offset..string_base + offset + 2].copy_from_slice(&[0xD8, 0x00]);
    let table_length = usize::try_from(be_u32(&mutated, record + 12)).unwrap();
    let checksum = table_checksum(&mutated[table..table + table_length]);
    mutated[record + 4..record + 8].copy_from_slice(&checksum.to_be_bytes());
    Arc::from(mutated)
}

fn table_checksum(bytes: &[u8]) -> u32 {
    bytes.chunks(4).fold(0u32, |sum, chunk| {
        let mut word = [0; 4];
        word[..chunk.len()].copy_from_slice(chunk);
        sum.wrapping_add(u32::from_be_bytes(word))
    })
}

fn be_u16(bytes: &[u8], start: usize) -> u16 {
    u16::from_be_bytes(bytes[start..start + 2].try_into().unwrap())
}

fn be_u32(bytes: &[u8], start: usize) -> u32 {
    u32::from_be_bytes(bytes[start..start + 4].try_into().unwrap())
}
