use worth_ui_host_contract::{UiTextOriginalRange, UiTextPoint, UiTextVisualEdge};

#[test]
pub(crate) fn empty_paragraph_has_one_hit_testable_line_boundary_without_paint() {
    let layout = super::tests::layout("", 80_000, 4);

    assert_eq!(layout.lines().len(), 1);
    assert_eq!(layout.visual_runs().len(), 1);
    assert!(layout.glyphs().is_empty());
    assert!(layout.positioned_glyphs().is_empty());
    assert_eq!(layout.carets().len(), 1);
    assert_line_anchor(&layout, 0, 0);
}

#[test]
pub(crate) fn rtl_empty_line_anchor_uses_the_visual_start_and_line_wide_hit_geometry() {
    let layout = layout_with_direction(crate::UiTextBaseDirection::RightToLeft);
    let line = layout.lines()[0];
    let caret = layout.carets()[0];

    assert_eq!(line.bounds().left_millipoints(), 80_000);
    assert_eq!(line.bounds().right_millipoints(), 80_000);
    assert_eq!(caret.x_millipoints(), 80_000);
    assert_eq!(caret.x_millipoints(), line.bounds().right_millipoints());
    let far_left_hit = layout
        .hit_test(UiTextPoint::from_text_mechanics(
            0,
            line.bounds().height_millipoints() / 2,
        ))
        .expect("whole empty RTL line is hit testable");
    assert_eq!(far_left_hit.caret(), caret);
    assert!(layout.positioned_glyphs().is_empty());
}

#[test]
pub(crate) fn consecutive_hard_breaks_and_trailing_empty_line_have_distinct_anchors() {
    let layout = super::tests::layout("\n\n", 80_000, 4);

    assert_eq!(layout.lines().len(), 3);
    assert_eq!(layout.visual_runs().len(), 3);
    assert!(layout.lines()[0].hard_break());
    assert!(layout.lines()[1].hard_break());
    assert!(!layout.lines()[2].hard_break());
    assert!(layout.glyphs().is_empty());
    assert!(layout.positioned_glyphs().is_empty());
    assert_eq!(layout.carets().len(), 3);
    assert_line_anchor(&layout, 0, 0);
    assert_line_anchor(&layout, 1, 1);
    assert_line_anchor(&layout, 2, 2);
}

#[test]
pub(crate) fn painted_text_before_a_hard_break_keeps_the_trailing_empty_line_hit_testable() {
    let layout = super::tests::layout("a\n", 80_000, 4);

    assert_eq!(layout.lines().len(), 2);
    assert_eq!(layout.positioned_glyphs().len(), 1);
    assert!(layout
        .glyphs()
        .iter()
        .all(|glyph| glyph.original_range().end() <= 1));
    assert_line_anchor(&layout, 1, 2);
}

fn assert_line_anchor(layout: &super::UiQualifiedTextLayout, line_index: u32, boundary: u32) {
    let line = layout.lines()[line_index as usize];
    let caret = layout
        .carets()
        .iter()
        .copied()
        .find(|caret| {
            caret.line_index() == line_index
                && caret.position().original_boundary()
                    == UiTextOriginalRange::from_text_mechanics(boundary, boundary).unwrap()
        })
        .expect("line boundary caret");
    let hit = layout
        .hit_test(UiTextPoint::from_text_mechanics(
            caret.x_millipoints(),
            (line.bounds().top_millipoints() + line.bounds().bottom_millipoints()) / 2,
        ))
        .expect("empty line remains hit testable");
    assert_eq!(hit.caret(), caret);
    assert_eq!(hit.cluster_range().start(), boundary);
    assert_eq!(hit.cluster_range().end(), boundary);
    assert_eq!(hit.visual_edge(), UiTextVisualEdge::Leading);
    assert!(line.visual_run_range().contains(&caret.visual_run_index()));
}

fn layout_with_direction(direction: crate::UiTextBaseDirection) -> super::UiQualifiedTextLayout {
    use std::sync::Arc;

    use worth_ui_host_contract::{
        UiFontCollectionGeneration, UiTextProfileGeneration, UiTextScaleGeneration,
    };

    let generation = UiFontCollectionGeneration::new(1).unwrap();
    let (fonts, _) = crate::UiGlobalFontCollection::admit_profile(
        generation,
        crate::font_collection::profile_inputs_from_repository(),
    )
    .unwrap();
    let constraints =
        crate::UiTextParagraphConstraints::new(crate::UiTextParagraphConstraintsInput {
            language: Arc::from("und"),
            base_direction: direction,
            wrap: crate::UiTextWrap::UnicodeWord,
            alignment: crate::UiTextAlignment::Start,
            overflow: crate::UiTextOverflow::Clip,
            font_size_millipoints: 14_000,
            width_millipoints: 80_000,
            line_height_millipoints: 18_000,
            letter_spacing_millipoints: 0,
            word_spacing_millipoints: 0,
            tab_interval_millipoints: 56_000,
            maximum_lines: 4,
        })
        .unwrap();
    let input = crate::UiTextParagraphAdmissionInput {
        source: Arc::from(""),
        constraints,
        profile_generation: UiTextProfileGeneration::new(1).unwrap(),
        font_collection_generation: generation,
        text_scale_generation: UiTextScaleGeneration::new(1).unwrap(),
        styles: Box::new([]),
    };
    let admitted = crate::UiAdmittedTextParagraph::admit(input).unwrap().0;
    let analyzed = crate::UiAnalyzedTextParagraph::analyze(admitted);
    let fallback = crate::UiFallbackTextParagraph::select(analyzed, Arc::new(fonts)).unwrap();
    super::UiQualifiedTextLayout::layout(crate::UiShapedTextParagraph::shape(fallback).unwrap())
        .unwrap()
}
