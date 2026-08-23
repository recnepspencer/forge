use std::sync::Arc;

use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiFontSlant, UiTextOriginalRange, UiTextProfileGeneration,
    UiTextScaleGeneration,
};

use crate::{
    font_collection::profile_inputs_from_repository, UiAdmittedTextParagraph,
    UiAnalyzedTextParagraph, UiFallbackTextParagraph, UiFontFamilyStack, UiGlobalFontCollection,
    UiQualifiedTextLayout, UiShapedTextParagraph, UiTextFaceRequest, UiTextParagraphAdmissionInput,
    UiTextParagraphConstraints, UiTextParagraphConstraintsInput, UiTextStyle, UiTextStyleInput,
    UiTextStyleSpan,
};

#[test]
fn italic_overhang_and_combining_mark_ink_survive_the_canonical_layout() {
    let source = "f\u{0301}";
    let layout = italic_layout(source);
    let line = layout.lines()[0];

    assert_eq!(layout.view().logical_bounds(), line.logical_bounds());
    assert_eq!(layout.view().ink_bounds(), line.ink_bounds());
    assert_eq!(layout.view().graphemes().len(), 1);
    assert!(layout.positioned_glyphs().len() >= 2);
    assert!(layout.glyphs().iter().all(|glyph| {
        glyph.original_range() == UiTextOriginalRange::new(0, source.len() as u32).unwrap()
    }));
    assert!(layout.positioned_glyphs().iter().any(|glyph| {
        let ink = glyph.ink_bounds();
        ink.left_millipoints() < glyph.origin_x_millipoints()
            || ink.right_millipoints()
                > glyph.origin_x_millipoints() + glyph.advance_x_millipoints().abs()
    }));
    assert!(layout
        .positioned_glyphs()
        .iter()
        .any(|glyph| glyph.advance_x_millipoints() == 0
            && glyph.ink_bounds().height_millipoints() > 0));
    assert_ne!(line.logical_bounds(), line.ink_bounds());
}

fn italic_layout(source: &str) -> UiQualifiedTextLayout {
    let generation = UiFontCollectionGeneration::new(1).unwrap();
    let (fonts, _) =
        UiGlobalFontCollection::admit_profile(generation, profile_inputs_from_repository())
            .unwrap();
    let constraints = UiTextParagraphConstraints::new(UiTextParagraphConstraintsInput {
        language: Arc::from("und"),
        base_direction: crate::UiTextBaseDirection::Auto,
        wrap: crate::UiTextWrap::UnicodeWord,
        alignment: crate::UiTextAlignment::Start,
        overflow: crate::UiTextOverflow::Clip,
        font_size_millipoints: 32_000,
        width_millipoints: 120_000,
        line_height_millipoints: 40_000,
        letter_spacing_millipoints: 0,
        word_spacing_millipoints: 0,
        tab_interval_millipoints: 64_000,
        maximum_lines: 1,
    })
    .unwrap();
    let style = UiTextStyle::new(UiTextStyleInput {
        language: Arc::from("und"),
        font_size_millipoints: 32_000,
        letter_spacing_millipoints: 0,
        word_spacing_millipoints: 0,
        family_stack: UiFontFamilyStack::profile_sans(),
        face_request: UiTextFaceRequest::new(400, 100_000, UiFontSlant::Italic).unwrap(),
        features: Box::new([]),
        variations: Box::new([]),
    })
    .unwrap();
    let (admitted, _) = UiAdmittedTextParagraph::admit(UiTextParagraphAdmissionInput {
        source: Arc::from(source),
        constraints,
        profile_generation: UiTextProfileGeneration::new(1).unwrap(),
        font_collection_generation: generation,
        text_scale_generation: UiTextScaleGeneration::new(1).unwrap(),
        styles: Box::new([UiTextStyleSpan::new(
            UiTextOriginalRange::new(0, source.len() as u32).unwrap(),
            style,
        )
        .unwrap()]),
    })
    .unwrap();
    let analyzed = UiAnalyzedTextParagraph::analyze(admitted);
    let fallback = UiFallbackTextParagraph::select(analyzed, Arc::new(fonts)).unwrap();
    UiQualifiedTextLayout::layout(UiShapedTextParagraph::shape(fallback).unwrap()).unwrap()
}
