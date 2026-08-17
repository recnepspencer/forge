use std::sync::Arc;

use worth_ui_host_contract::{UiFontCollectionGeneration, UiFontSlant};

use super::{
    application_selection_tests::{advance, selected_faces, shape},
    application_test_world::{face, profile_collection_and_sources},
    UiApplicationFontPackDefinition,
};
use crate::{UiFontFamilyStack, UiOpenTypeFeature, UiTextFaceRequest};

#[test]
pub(super) fn application_variable_axes_slant_features_and_metadata_drive_real_shaping() {
    let (profile, sources) = profile_collection_and_sources();
    let definition = UiApplicationFontPackDefinition {
        name: Arc::from("variable application family"),
        faces: Box::new([
            face(
                "Variable",
                sources["noto-sans-roman"].clone(),
                0,
                UiFontSlant::Upright,
            ),
            face(
                "Variable",
                sources["noto-sans-italic"].clone(),
                0,
                UiFontSlant::Italic,
            ),
        ]),
    };
    let (collection, receipt, _) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
        .unwrap();
    let collection = Arc::new(collection);
    let family = receipt.family("Variable").unwrap();
    let variable = receipt
        .faces()
        .iter()
        .find(|face| face.family() == family && face.slant() == UiFontSlant::Upright)
        .unwrap();
    assert!(variable.family_name_records() > 0);
    assert!(variable.style_name_records() > 0);
    assert!(variable.coverage_range_count() > 0);
    assert!(variable.axes().iter().any(|axis| axis.tag() == *b"wght"));
    assert!(variable.axes().iter().any(|axis| axis.tag() == *b"wdth"));
    assert!(variable.feature_tags().contains(b"liga"));
    let stack = UiFontFamilyStack::new(Box::new([family])).unwrap();
    let heavy = shape(
        &collection,
        stack.clone(),
        UiTextFaceRequest::new(700, 100_000, UiFontSlant::Upright).unwrap(),
        Box::new([]),
        "WORTH interface",
    );
    let narrow = shape(
        &collection,
        stack.clone(),
        UiTextFaceRequest::new(400, 62_500, UiFontSlant::Upright).unwrap(),
        Box::new([]),
        "WORTH interface",
    );
    let normal = shape(
        &collection,
        stack.clone(),
        UiTextFaceRequest::new(400, 100_000, UiFontSlant::Upright).unwrap(),
        Box::new([]),
        "WORTH interface",
    );
    assert_ne!(advance(&heavy), advance(&normal));
    assert_ne!(advance(&narrow), advance(&normal));
    assert_eq!(heavy.runs()[0].face(), normal.runs()[0].face());
    assert_eq!(narrow.runs()[0].face(), normal.runs()[0].face());

    let oblique = selected_faces(
        &collection,
        stack.clone(),
        UiTextFaceRequest::new(400, 100_000, UiFontSlant::Oblique).unwrap(),
        "office",
    );
    let italic = receipt
        .faces()
        .iter()
        .find(|face| face.slant() == UiFontSlant::Italic)
        .unwrap();
    assert_eq!(oblique[0], italic.identity());

    let ligatures = shape(
        &collection,
        stack.clone(),
        UiTextFaceRequest::regular(),
        Box::new([UiOpenTypeFeature::new(*b"liga", 1).unwrap()]),
        "office",
    );
    let separate = shape(
        &collection,
        stack,
        UiTextFaceRequest::regular(),
        Box::new([UiOpenTypeFeature::new(*b"liga", 0).unwrap()]),
        "office",
    );
    assert!(ligatures.glyphs().len() < separate.glyphs().len());
}

#[test]
pub(super) fn variable_slant_axis_is_selected_and_changes_real_shaping() {
    let (profile, sources) = profile_collection_and_sources();
    let slanted = with_renamed_axis_range(
        Arc::clone(&sources["noto-sans-roman"]),
        *b"wdth",
        *b"slnt",
        -20,
        0,
        0,
    );
    let definition = UiApplicationFontPackDefinition {
        name: Arc::from("variable slant application family"),
        faces: Box::new([super::UiApplicationFontFaceDefinition {
            slant: UiFontSlant::Oblique,
            ..face("Variable Slant", slanted, 0, UiFontSlant::Oblique)
        }]),
    };
    let (collection, receipt, _) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
        .unwrap();
    let collection = Arc::new(collection);
    let family = receipt.family("Variable Slant").unwrap();
    let selected = receipt.faces()[0].identity();
    assert!(receipt.faces()[0].axes().iter().any(|axis| {
        axis.tag() == *b"slnt"
            && axis.minimum_milli() == -20_000
            && axis.default_milli() == 0
            && axis.maximum_milli() == 0
    }));
    let stack = UiFontFamilyStack::new(Box::new([family])).unwrap();
    let upright = shape(
        &collection,
        stack.clone(),
        UiTextFaceRequest::new(400, 100_000, UiFontSlant::Upright).unwrap(),
        Box::new([]),
        "WORTH interface",
    );
    let oblique = shape(
        &collection,
        stack,
        UiTextFaceRequest::new(400, 100_000, UiFontSlant::Oblique).unwrap(),
        Box::new([]),
        "WORTH interface",
    );
    assert_eq!(upright.runs()[0].face(), selected);
    assert_eq!(oblique.runs()[0].face(), selected);
    assert_ne!(advance(&upright), advance(&oblique));
}

#[test]
pub(super) fn variable_face_matching_uses_the_requested_value_against_each_axis_range() {
    let (profile, sources) = profile_collection_and_sources();
    let lower = with_axis_range(
        Arc::clone(&sources["noto-sans-roman"]),
        *b"wght",
        100,
        400,
        500,
    );
    let upper = with_axis_range(
        Arc::clone(&sources["noto-sans-roman"]),
        *b"wght",
        600,
        700,
        900,
    );
    let definition = UiApplicationFontPackDefinition {
        name: Arc::from("disjoint variable ranges"),
        faces: Box::new([
            super::application_test_world::face("Variable Range", lower, 0, UiFontSlant::Upright),
            super::UiApplicationFontFaceDefinition {
                weight: 700,
                ..super::application_test_world::face(
                    "Variable Range",
                    upper,
                    0,
                    UiFontSlant::Upright,
                )
            },
        ]),
    };
    let (collection, receipt, _) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
        .unwrap();
    let family = receipt.family("Variable Range").unwrap();
    let expected = receipt
        .faces()
        .iter()
        .find(|face| face.weight() == 700)
        .unwrap()
        .identity();
    let selected = selected_faces(
        &Arc::new(collection),
        UiFontFamilyStack::new(Box::new([family])).unwrap(),
        UiTextFaceRequest::new(800, 100_000, UiFontSlant::Upright).unwrap(),
        "office",
    );
    assert!(selected.iter().all(|face| *face == expected));
}

fn with_axis_range(
    bytes: Arc<[u8]>,
    tag: [u8; 4],
    minimum: i32,
    default: i32,
    maximum: i32,
) -> Arc<[u8]> {
    let mut bytes = bytes.to_vec();
    let table_count = usize::from(be_u16(&bytes, 4));
    let record = (0..table_count)
        .map(|index| 12 + index * 16)
        .find(|start| bytes[*start..*start + 4] == *b"fvar")
        .unwrap();
    let table = usize::try_from(be_u32(&bytes, record + 8)).unwrap();
    let axis_offset = table + usize::from(be_u16(&bytes, table + 4));
    let axis_count = usize::from(be_u16(&bytes, table + 8));
    let axis_size = usize::from(be_u16(&bytes, table + 10));
    let axis = (0..axis_count)
        .map(|index| axis_offset + index * axis_size)
        .find(|start| bytes[*start..*start + 4] == tag)
        .unwrap();
    for (offset, value) in [(4, minimum), (8, default), (12, maximum)] {
        bytes[axis + offset..axis + offset + 4].copy_from_slice(&(value * 65_536).to_be_bytes());
    }
    let table_length = usize::try_from(be_u32(&bytes, record + 12)).unwrap();
    let checksum = table_checksum(&bytes[table..table + table_length]);
    bytes[record + 4..record + 8].copy_from_slice(&checksum.to_be_bytes());
    Arc::from(bytes)
}

fn with_renamed_axis_range(
    bytes: Arc<[u8]>,
    source_tag: [u8; 4],
    target_tag: [u8; 4],
    minimum: i32,
    default: i32,
    maximum: i32,
) -> Arc<[u8]> {
    let mut bytes = with_axis_range(bytes, source_tag, minimum, default, maximum).to_vec();
    let table_count = usize::from(be_u16(&bytes, 4));
    let record = (0..table_count)
        .map(|index| 12 + index * 16)
        .find(|start| bytes[*start..*start + 4] == *b"fvar")
        .unwrap();
    let table = usize::try_from(be_u32(&bytes, record + 8)).unwrap();
    let axis_offset = table + usize::from(be_u16(&bytes, table + 4));
    let axis_count = usize::from(be_u16(&bytes, table + 8));
    let axis_size = usize::from(be_u16(&bytes, table + 10));
    let axis = (0..axis_count)
        .map(|index| axis_offset + index * axis_size)
        .find(|start| bytes[*start..*start + 4] == source_tag)
        .unwrap();
    bytes[axis..axis + 4].copy_from_slice(&target_tag);
    let table_length = usize::try_from(be_u32(&bytes, record + 12)).unwrap();
    let checksum = table_checksum(&bytes[table..table + table_length]);
    bytes[record + 4..record + 8].copy_from_slice(&checksum.to_be_bytes());
    Arc::from(bytes)
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
