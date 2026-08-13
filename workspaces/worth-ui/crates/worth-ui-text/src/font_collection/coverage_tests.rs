use std::sync::Arc;

use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiFontSlant, UiTextOriginalRange, UiTextProfileGeneration,
    UiTextScaleGeneration,
};

use super::{
    application_test_world::{face, profile_collection_and_sources},
    UiApplicationFontPackDefinition, UiFontCollectionAdmissionDenial,
};
use crate::{
    UiAdmittedTextParagraph, UiAnalyzedTextParagraph, UiFallbackTextParagraph, UiFontFamilyStack,
    UiTextAlignment, UiTextBaseDirection, UiTextFaceRequest, UiTextOverflow,
    UiTextParagraphAdmissionInput, UiTextParagraphConstraints, UiTextParagraphConstraintsInput,
    UiTextStyle, UiTextStyleInput, UiTextStyleSpan, UiTextWrap,
};

#[test]
pub(super) fn admitted_coverage_index_skips_unsupported_faces_before_whole_cluster_shaping() {
    let (profile, sources) = profile_collection_and_sources();
    let definition = UiApplicationFontPackDefinition {
        name: Arc::from("Latin-only application family"),
        faces: Box::new([face(
            "Application Latin",
            sources["noto-sans-roman"].clone(),
            0,
            UiFontSlant::Upright,
        )]),
    };
    let (collection, receipt, cost) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
        .unwrap();
    let collection = Arc::new(collection);
    assert!(cost.coverage_ranges_built() > 0);
    let source = "漢";
    let constraints = UiTextParagraphConstraints::new(UiTextParagraphConstraintsInput {
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
    .unwrap();
    let style = UiTextStyle::new(UiTextStyleInput {
        language: Arc::from("und"),
        font_size_millipoints: 14_000,
        letter_spacing_millipoints: 0,
        word_spacing_millipoints: 0,
        family_stack: UiFontFamilyStack::new(Box::new([receipt
            .family("Application Latin")
            .unwrap()]))
        .unwrap(),
        face_request: UiTextFaceRequest::regular(),
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
    let fallback =
        UiFallbackTextParagraph::select(UiAnalyzedTextParagraph::analyze(admitted), collection)
            .unwrap();
    assert_eq!(fallback.clusters().len(), 1);
    assert!(fallback.cost().coverage_index_queries() > 1);
    assert_eq!(
        fallback.cost().face_shape_attempts(),
        1,
        "unsupported application/default faces must be rejected by coverage before shaping"
    );
}

#[test]
pub(super) fn application_pack_without_a_unicode_coverage_map_is_denied_atomically() {
    let (profile, sources) = profile_collection_and_sources();
    let mut bytes = sources["noto-sans-roman"].to_vec();
    remove_unicode_cmap_records(&mut bytes);
    let definition = UiApplicationFontPackDefinition {
        name: Arc::from("missing Unicode coverage"),
        faces: Box::new([face(
            "Broken coverage",
            Arc::from(bytes),
            0,
            UiFontSlant::Upright,
        )]),
    };
    let denial = match profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
    {
        Ok(_) => panic!("font without a Unicode coverage map was admitted"),
        Err(denial) => denial,
    };
    assert_eq!(
        denial,
        UiFontCollectionAdmissionDenial::MissingUnicodeCoverage
    );
    assert_eq!(profile.generation().get(), 1);
    assert!(profile.application_packs().is_empty());
}

fn remove_unicode_cmap_records(bytes: &mut [u8]) {
    let table_count = usize::from(be_u16(bytes, 4));
    let (directory_record, cmap) = (0..table_count)
        .find_map(|index| {
            let record = 12 + index * 16;
            (bytes[record..record + 4] == *b"cmap")
                .then(|| (record, usize::try_from(be_u32(bytes, record + 8)).unwrap()))
        })
        .expect("test font cmap table");
    let record_count = usize::from(be_u16(bytes, cmap + 2));
    for index in 0..record_count {
        let platform = cmap + 4 + index * 8;
        bytes[platform..platform + 2].copy_from_slice(&1u16.to_be_bytes());
    }
    let length = usize::try_from(be_u32(bytes, directory_record + 12)).unwrap();
    let checksum = table_checksum(&bytes[cmap..cmap + length]);
    bytes[directory_record + 4..directory_record + 8].copy_from_slice(&checksum.to_be_bytes());
}

fn table_checksum(bytes: &[u8]) -> u32 {
    bytes.chunks(4).fold(0u32, |sum, chunk| {
        let mut word = [0; 4];
        word[..chunk.len()].copy_from_slice(chunk);
        sum.wrapping_add(u32::from_be_bytes(word))
    })
}

fn be_u16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes(bytes[offset..offset + 2].try_into().unwrap())
}

fn be_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap())
}
