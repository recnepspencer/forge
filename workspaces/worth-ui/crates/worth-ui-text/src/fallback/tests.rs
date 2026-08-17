use std::sync::Arc;

use super::*;
use crate::{
    font_collection::profile_inputs_from_repository, UiAdmittedTextParagraph, UiTextAlignment,
    UiTextBaseDirection, UiTextOverflow, UiTextParagraphAdmissionInput, UiTextParagraphConstraints,
    UiTextParagraphConstraintsInput, UiTextWrap,
};
use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiTextProfileGeneration, UiTextScaleGeneration,
};

const EMOJI_TEST: &str =
    include_str!("../../../../profiles/worth-ui-global-text-v2/unicode/emoji/emoji-test.txt");

#[test]
pub(crate) fn every_unicode_17_rgi_sequence_selects_one_complete_color_emoji_cluster() {
    let generation = UiFontCollectionGeneration::new(1).unwrap();
    let (fonts, _) =
        UiGlobalFontCollection::admit_profile(generation, profile_inputs_from_repository())
            .unwrap();
    let fonts = Arc::new(fonts);
    let mut cases = 0;
    for line in EMOJI_TEST.lines().filter(|line| {
        line.as_bytes().first().is_some_and(u8::is_ascii_hexdigit)
            && line.split(';').nth(1).is_some_and(|field| {
                matches!(
                    field.split_whitespace().next(),
                    Some("fully-qualified" | "component")
                )
            })
    }) {
        let source = line
            .split(';')
            .next()
            .unwrap()
            .split_whitespace()
            .map(|value| char::from_u32(u32::from_str_radix(value, 16).unwrap()).unwrap())
            .collect::<String>();
        let selected =
            UiFallbackTextParagraph::select(analyze(&source, generation), Arc::clone(&fonts))
                .unwrap();
        assert_eq!(selected.clusters().len(), 1, "split {line}");
        let cluster = selected.clusters()[0];
        assert!(cluster.is_rgi_emoji(), "RGI classification drifted: {line}");
        assert_eq!(cluster.coverage(), UiTextCoverageDisposition::QualifiedFace);
        assert_eq!(
            selected.cost().face_shape_attempts(),
            1,
            "fallback drifted: {line}"
        );
        cases += 1;
    }
    assert_eq!(cases, 3_953);
}

#[test]
pub(crate) fn repeated_clusters_reuse_one_exact_face_probe_inside_the_paragraph() {
    let generation = UiFontCollectionGeneration::new(1).unwrap();
    let (fonts, _) =
        UiGlobalFontCollection::admit_profile(generation, profile_inputs_from_repository())
            .unwrap();
    let selected =
        UiFallbackTextParagraph::select(analyze("xxxxxxxx", generation), Arc::new(fonts)).unwrap();
    assert_eq!(selected.clusters().len(), 8);
    assert_eq!(selected.cost().clusters_considered(), 8);
    assert_eq!(selected.cost().coverage_index_queries(), 1);
    assert_eq!(selected.cost().face_shape_attempts(), 1);
}

fn analyze(source: &str, generation: UiFontCollectionGeneration) -> UiAnalyzedTextParagraph {
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
    let styles = Box::new([
        crate::UiTextStyleSpan::whole_paragraph(source, &constraints).expect("nonempty source"),
    ]);
    let (admitted, _) = UiAdmittedTextParagraph::admit(UiTextParagraphAdmissionInput {
        source: Arc::from(source),
        constraints,
        profile_generation: UiTextProfileGeneration::new(1).unwrap(),
        font_collection_generation: generation,
        text_scale_generation: UiTextScaleGeneration::new(1).unwrap(),
        styles,
    })
    .unwrap();
    UiAnalyzedTextParagraph::analyze(admitted)
}
