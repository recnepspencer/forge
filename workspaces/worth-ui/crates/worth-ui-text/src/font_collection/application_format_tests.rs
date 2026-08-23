use std::sync::Arc;

use sha2::Digest;
use worth_ui_host_contract::{UiFontCollectionGeneration, UiFontSlant};

use super::{
    application_test_world::{face, profile_collection_and_sources, selected_face},
    UiApplicationFontPackDefinition, UiFontCollectionAdmissionDenial,
};
use crate::{UiFontFamilyStack, UiTextFaceRequest};

#[test]
pub(super) fn owned_ttf_otf_ttc_and_otc_bytes_cross_one_public_pack_transition() {
    let (profile, sources) = profile_collection_and_sources();
    let ttf: Arc<[u8]> = Arc::from(
        include_bytes!("../../../worth-ui-host-native/assets/fonts/NotoSans-Regular.ttf")
            .as_slice(),
    );
    let ttc: Arc<[u8]> = ttc_from_ttf_faces(&ttf, 2).into();
    let otc = Arc::clone(&sources["noto-sans-cjk-jp"]);
    let otf: Arc<[u8]> = standalone_face(&otc, 1).into();
    assert_eq!(&ttf[..4], &[0, 1, 0, 0]);
    assert_eq!(&ttc[..4], b"ttcf");
    assert_eq!(&otf[..4], b"OTTO");
    assert_eq!(&otc[..4], b"ttcf");

    let definition = UiApplicationFontPackDefinition {
        name: Arc::from("all owned OpenType containers"),
        faces: Box::new([
            face("Owned TTF", Arc::clone(&ttf), 0, UiFontSlant::Upright),
            face("Owned TTC First", Arc::clone(&ttc), 0, UiFontSlant::Upright),
            face(
                "Owned TTC Second",
                Arc::clone(&ttc),
                1,
                UiFontSlant::Upright,
            ),
            face("Owned OTF", Arc::clone(&otf), 0, UiFontSlant::Upright),
            face(
                "Owned OTC Japanese",
                Arc::clone(&otc),
                0,
                UiFontSlant::Upright,
            ),
            face(
                "Owned OTC Korean",
                Arc::clone(&otc),
                1,
                UiFontSlant::Upright,
            ),
        ]),
    };
    let (collection, receipt, _) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
        .unwrap();
    let collection = Arc::new(collection);
    assert_eq!(receipt.faces().len(), 6);
    for (family_name, expected_index, expected_bytes) in [
        ("Owned TTF", 0, &ttf),
        ("Owned TTC First", 0, &ttc),
        ("Owned TTC Second", 1, &ttc),
        ("Owned OTF", 0, &otf),
        ("Owned OTC Japanese", 0, &otc),
        ("Owned OTC Korean", 1, &otc),
    ] {
        let family = receipt.family(family_name).unwrap();
        let expected = receipt
            .faces()
            .iter()
            .find(|face| face.family() == family)
            .unwrap()
            .identity();
        assert_eq!(expected.face_index(), expected_index);
        assert_eq!(expected.font_bytes_digest(), font_digest(expected_bytes));
        assert_eq!(
            selected_face(
                Arc::clone(&collection),
                UiFontFamilyStack::new(Box::new([family])).unwrap(),
                UiTextFaceRequest::regular(),
            ),
            expected
        );
    }
    assert_ne!(
        receipt.family("Owned TTC First"),
        receipt.family("Owned TTC Second")
    );
    assert_ne!(
        receipt.family("Owned OTC Japanese"),
        receipt.family("Owned OTC Korean")
    );
}

#[test]
pub(super) fn woff_and_woff2_are_typed_unsupported_containers_before_font_parsing() {
    let (profile, _) = profile_collection_and_sources();
    for signature in [*b"wOFF", *b"wOF2"] {
        let definition = UiApplicationFontPackDefinition {
            name: Arc::from("unsupported web font"),
            faces: Box::new([face(
                "Web Font",
                Arc::from(signature),
                0,
                UiFontSlant::Upright,
            )]),
        };
        let denial = match profile
            .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
        {
            Ok(_) => panic!("web font container was admitted"),
            Err(denial) => denial,
        };
        assert_eq!(
            denial,
            UiFontCollectionAdmissionDenial::UnsupportedFontContainer
        );
        assert!(profile.is_current_for_admission());
    }
}

#[test]
fn malformed_face_index_directory_order_and_table_checksum_deny_atomically() {
    let (profile, _) = profile_collection_and_sources();
    let ttf = include_bytes!("../../../worth-ui-host-native/assets/fonts/NotoSans-Regular.ttf");
    let mut unsorted = ttf.to_vec();
    unsorted[12..44].rotate_left(16);
    let mut corrupt_table = ttf.to_vec();
    let table_offset = usize::try_from(be_u32(&corrupt_table, 12 + 8)).unwrap();
    corrupt_table[table_offset] ^= 1;
    for (bytes, face_index) in [
        (Arc::from(ttf.as_slice()), 1),
        (Arc::from(unsorted), 0),
        (Arc::from(corrupt_table), 0),
    ] {
        let definition = UiApplicationFontPackDefinition {
            name: Arc::from("malformed owned container"),
            faces: Box::new([face(
                "Rejected Face",
                bytes,
                face_index,
                UiFontSlant::Upright,
            )]),
        };
        let denial = match profile
            .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
        {
            Ok(_) => panic!("malformed SFNT container was admitted"),
            Err(denial) => denial,
        };
        assert_eq!(denial, UiFontCollectionAdmissionDenial::MalformedFont);
        assert!(profile.is_current_for_admission());
    }
}

#[test]
pub(super) fn aat_substitution_tables_are_typed_unsupported_before_metadata_admission() {
    let (profile, _) = profile_collection_and_sources();
    let source = include_bytes!("../../../worth-ui-host-native/assets/fonts/NotoSans-Regular.ttf");
    for tag in [*b"morx", *b"mort"] {
        let mut bytes = source.to_vec();
        let table_count = usize::from(u16::from_be_bytes([bytes[4], bytes[5]]));
        let record = (0..table_count)
            .map(|index| 12 + index * 16)
            .find(|start| bytes[*start..*start + 4] == *b"name")
            .expect("fixture has a name table between maxp and post");
        bytes[record..record + 4].copy_from_slice(&tag);
        let definition = UiApplicationFontPackDefinition {
            name: Arc::from("unsupported AAT shaping authority"),
            faces: Box::new([face("AAT Face", Arc::from(bytes), 0, UiFontSlant::Upright)]),
        };
        let denial = match profile
            .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
        {
            Ok(_) => panic!("AAT shaping table bypassed the qualified OpenType path"),
            Err(denial) => denial,
        };
        assert_eq!(
            denial,
            UiFontCollectionAdmissionDenial::UnsupportedShapingTable
        );
    }
}

fn ttc_from_ttf_faces(ttf: &[u8], face_count: usize) -> Vec<u8> {
    let header_len = 12 + face_count * 4;
    let mut collection = Vec::with_capacity(header_len + face_count * ttf.len());
    collection.extend_from_slice(b"ttcf");
    collection.extend_from_slice(&0x0001_0000_u32.to_be_bytes());
    collection.extend_from_slice(&u32::try_from(face_count).unwrap().to_be_bytes());
    collection.resize(header_len, 0);
    for face in 0..face_count {
        while !collection.len().is_multiple_of(4) {
            collection.push(0);
        }
        let face_start = collection.len();
        collection[12 + face * 4..16 + face * 4]
            .copy_from_slice(&u32::try_from(face_start).unwrap().to_be_bytes());
        collection.extend_from_slice(ttf);
        let table_count = usize::from(u16::from_be_bytes([ttf[4], ttf[5]]));
        for index in 0..table_count {
            let offset_field = face_start + 12 + index * 16 + 8;
            let offset = be_u32(&collection, offset_field);
            collection[offset_field..offset_field + 4]
                .copy_from_slice(&(offset + u32::try_from(face_start).unwrap()).to_be_bytes());
        }
    }
    collection
}

fn standalone_face(collection: &[u8], face_index: usize) -> Vec<u8> {
    assert_eq!(&collection[..4], b"ttcf");
    let face_offset_field = 12 + face_index * 4;
    let face_offset = usize::try_from(be_u32(collection, face_offset_field)).unwrap();
    let table_count = usize::from(u16::from_be_bytes([
        collection[face_offset + 4],
        collection[face_offset + 5],
    ]));
    let directory_len = 12 + table_count * 16;
    let mut face = collection[face_offset..face_offset + directory_len].to_vec();
    for index in 0..table_count {
        let source_record = face_offset + 12 + index * 16;
        let output_record = 12 + index * 16;
        let source_offset = usize::try_from(be_u32(collection, source_record + 8)).unwrap();
        let length = usize::try_from(be_u32(collection, source_record + 12)).unwrap();
        while !face.len().is_multiple_of(4) {
            face.push(0);
        }
        let output_offset = face.len();
        face[output_record + 8..output_record + 12]
            .copy_from_slice(&u32::try_from(output_offset).unwrap().to_be_bytes());
        face.extend_from_slice(&collection[source_offset..source_offset + length]);
    }
    face
}

fn be_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn font_digest(bytes: &[u8]) -> [u8; 32] {
    sha2::Sha256::digest(bytes).into()
}
