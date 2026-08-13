use std::sync::Arc;

use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiTextProfileGeneration, UiTextScaleGeneration,
};

use crate::{
    qualify_text_layout, UiGlobalFontCollection, UiTextAlignment, UiTextBaseDirection,
    UiTextOverflow, UiTextParagraphAdmissionInput, UiTextParagraphConstraints,
    UiTextParagraphConstraintsInput, UiTextStyleSpan, UiTextWrap,
};

#[test]
pub(crate) fn qualified_layout_retains_pinned_dictionary_boundaries_in_original_utf8() {
    let (fonts, _) = UiGlobalFontCollection::admit_qualified_profile().unwrap();
    let fonts = Arc::new(fonts);
    for (source, expected) in [
        ("ภาษาไทยภาษาไทย", &[0, 12, 21, 33, 42][..]),
        ("ພາສາລາວພາສາລາວ", &[0, 12, 21, 33, 42][..]),
        ("မြန်မာဘာသာမြန်မာဘာသာ", &[0, 12, 30, 42, 48, 60][..]),
        ("ភាសាខ្មែរភាសាខ្មែរ", &[0, 12, 27, 54][..]),
    ] {
        let layout = qualify_text_layout(input(source), Arc::clone(&fonts)).unwrap();
        let observed = layout
            .view()
            .word_boundaries()
            .iter()
            .map(|record| record.original_boundary().start())
            .collect::<Vec<_>>();
        assert_eq!(observed, expected, "dictionary boundaries drifted");
    }
}

fn input(source: &str) -> UiTextParagraphAdmissionInput {
    let constraints = UiTextParagraphConstraints::new(UiTextParagraphConstraintsInput {
        language: Arc::from("und"),
        base_direction: UiTextBaseDirection::Auto,
        wrap: UiTextWrap::UnicodeWord,
        alignment: UiTextAlignment::Start,
        overflow: UiTextOverflow::Clip,
        font_size_millipoints: 14_000,
        width_millipoints: 160_000,
        line_height_millipoints: 18_000,
        letter_spacing_millipoints: 0,
        word_spacing_millipoints: 0,
        tab_interval_millipoints: 56_000,
        maximum_lines: 8,
    })
    .unwrap();
    UiTextParagraphAdmissionInput {
        source: Arc::from(source),
        styles: Box::new([UiTextStyleSpan::whole_paragraph(source, &constraints).unwrap()]),
        constraints,
        profile_generation: UiTextProfileGeneration::new(1).unwrap(),
        font_collection_generation: UiFontCollectionGeneration::new(1).unwrap(),
        text_scale_generation: UiTextScaleGeneration::new(1).unwrap(),
    }
}
