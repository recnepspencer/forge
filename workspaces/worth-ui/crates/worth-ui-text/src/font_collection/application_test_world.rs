use std::{collections::BTreeMap, sync::Arc};

use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiFontSlant, UiQualifiedFontFaceIdentity,
    UiQualifiedFontFamilyIdentity, UiTextOriginalRange, UiTextProfileGeneration,
    UiTextScaleGeneration,
};

use super::{
    profile_inputs_from_repository, UiApplicationFontFaceDefinition,
    UiApplicationFontLicenseRecord, UiApplicationFontPackDefinition,
    UiFontCollectionAdmissionDenial, UiGlobalFontCollection,
};
use crate::{
    UiAdmittedTextParagraph, UiAnalyzedTextParagraph, UiFallbackTextParagraph, UiFontFamilyStack,
    UiQualifiedTextLayout, UiShapedTextParagraph, UiTextAlignment, UiTextBaseDirection,
    UiTextFaceRequest, UiTextOverflow, UiTextParagraphAdmissionInput, UiTextParagraphConstraints,
    UiTextParagraphConstraintsInput, UiTextStyle, UiTextStyleInput, UiTextStyleSpan, UiTextWrap,
};

pub(super) fn profile_collection_and_sources(
) -> (UiGlobalFontCollection, BTreeMap<Arc<str>, Arc<[u8]>>) {
    let inputs = profile_inputs_from_repository();
    let sources = inputs
        .iter()
        .map(|input| (input.id.clone(), input.bytes.clone()))
        .collect();
    let (collection, _) =
        UiGlobalFontCollection::admit_profile(UiFontCollectionGeneration::new(1).unwrap(), inputs)
            .unwrap();
    (collection, sources)
}

pub(super) fn face(
    family: &str,
    bytes: Arc<[u8]>,
    face_index: u32,
    slant: UiFontSlant,
) -> UiApplicationFontFaceDefinition {
    UiApplicationFontFaceDefinition {
        family: Arc::from(family),
        bytes,
        face_index,
        weight: 400,
        width_milli_percent: 100_000,
        slant,
        license: UiApplicationFontLicenseRecord {
            identifier: Arc::from("OFL-1.1"),
            notice: Arc::from("Owned test font bytes under the repository-pinned OFL record."),
        },
    }
}

pub(super) fn static_face_bytes(bytes: &[u8], weight: u16, slant: UiFontSlant) -> Arc<[u8]> {
    let mut bytes = bytes.to_vec();
    let table_count = usize::from(be_u16(&bytes, 4));
    let record = (0..table_count)
        .map(|index| 12 + index * 16)
        .find(|start| bytes[*start..*start + 4] == *b"OS/2")
        .expect("fixture owns OS/2 metadata");
    let table = usize::try_from(be_u32(&bytes, record + 8)).unwrap();
    bytes[table + 4..table + 6].copy_from_slice(&weight.to_be_bytes());
    let mut selection = be_u16(&bytes, table + 62);
    selection &= !((1 << 0) | (1 << 9));
    selection |= match slant {
        UiFontSlant::Upright => 0,
        UiFontSlant::Italic => 1 << 0,
        UiFontSlant::Oblique => 1 << 9,
    };
    bytes[table + 62..table + 64].copy_from_slice(&selection.to_be_bytes());
    let length = usize::try_from(be_u32(&bytes, record + 12)).unwrap();
    let checksum = table_checksum(&bytes[table..table + length]);
    bytes[record + 4..record + 8].copy_from_slice(&checksum.to_be_bytes());
    Arc::from(bytes)
}

pub(super) fn assert_pack_denial(
    collection: &UiGlobalFontCollection,
    definition: UiApplicationFontPackDefinition,
    expected: UiFontCollectionAdmissionDenial,
) {
    let denial = match collection.register_application_pack(
        UiFontCollectionGeneration::new(collection.generation().get() + 1).unwrap(),
        definition,
    ) {
        Ok(_) => panic!("hostile application font pack was admitted"),
        Err(denial) => denial,
    };
    assert_eq!(denial, expected);
    assert!(collection.application_packs().is_empty());
    assert_eq!(collection.application_font_bytes(), 0);
}

pub(super) fn selected_face(
    collection: Arc<UiGlobalFontCollection>,
    family_stack: UiFontFamilyStack,
    face_request: UiTextFaceRequest,
) -> UiQualifiedFontFaceIdentity {
    fallback(collection, family_stack, face_request, "office").clusters()[0]
        .face()
        .unwrap()
}

pub(super) fn layout(
    collection: Arc<UiGlobalFontCollection>,
    family: UiQualifiedFontFamilyIdentity,
    source: &str,
) -> UiQualifiedTextLayout {
    let fallback = fallback(
        collection,
        UiFontFamilyStack::new(Box::new([family])).unwrap(),
        UiTextFaceRequest::regular(),
        source,
    );
    let shaped = UiShapedTextParagraph::shape(fallback).unwrap();
    UiQualifiedTextLayout::layout(shaped).unwrap()
}

fn fallback(
    collection: Arc<UiGlobalFontCollection>,
    family_stack: UiFontFamilyStack,
    face_request: UiTextFaceRequest,
    source: &str,
) -> UiFallbackTextParagraph {
    fallback_result(collection, family_stack, face_request, source).unwrap()
}

pub(super) fn fallback_result(
    collection: Arc<UiGlobalFontCollection>,
    family_stack: UiFontFamilyStack,
    face_request: UiTextFaceRequest,
    source: &str,
) -> Result<UiFallbackTextParagraph, crate::UiTextFallbackDenial> {
    let constraints = constraints();
    let style = UiTextStyle::new(UiTextStyleInput {
        language: Arc::from("und"),
        font_size_millipoints: 14_000,
        letter_spacing_millipoints: 0,
        word_spacing_millipoints: 0,
        family_stack,
        face_request,
        features: Box::new([]),
        variations: Box::new([]),
    })
    .unwrap();
    let styles = Box::new([UiTextStyleSpan::new(
        UiTextOriginalRange::from_text_mechanics(0, source.len() as u32).unwrap(),
        style,
    )
    .unwrap()]);
    let (admitted, _) = UiAdmittedTextParagraph::admit(UiTextParagraphAdmissionInput {
        source: Arc::from(source),
        constraints,
        profile_generation: UiTextProfileGeneration::new(1).unwrap(),
        font_collection_generation: collection.generation(),
        text_scale_generation: UiTextScaleGeneration::new(1).unwrap(),
        styles,
    })
    .unwrap();
    UiFallbackTextParagraph::select(UiAnalyzedTextParagraph::analyze(admitted), collection)
}

pub(super) fn constraints() -> UiTextParagraphConstraints {
    UiTextParagraphConstraints::new(UiTextParagraphConstraintsInput {
        language: Arc::from("und"),
        base_direction: UiTextBaseDirection::Auto,
        wrap: UiTextWrap::UnicodeWord,
        alignment: UiTextAlignment::Start,
        overflow: UiTextOverflow::Clip,
        font_size_millipoints: 14_000,
        width_millipoints: 320_000,
        line_height_millipoints: 18_000,
        letter_spacing_millipoints: 0,
        word_spacing_millipoints: 0,
        tab_interval_millipoints: 56_000,
        maximum_lines: 16,
    })
    .unwrap()
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
