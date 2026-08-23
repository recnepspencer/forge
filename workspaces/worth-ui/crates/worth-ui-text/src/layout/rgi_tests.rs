use std::sync::Arc;

use worth_ui_host_contract::{UiFontCollectionGeneration, UiTextOriginalRange, UiTextPoint};

use super::tests::layout_with_fonts;
use crate::{
    font_collection::profile_inputs_from_repository, UiGlobalFontCollection, UiTextOverflow,
};

const EMOJI_TEST: &str =
    include_str!("../../../../profiles/worth-ui-global-text-v2/unicode/emoji/emoji-test.txt");

#[test]
pub(crate) fn every_unicode_17_rgi_sequence_remains_atomic_through_layout_and_ellipsis() {
    let generation = UiFontCollectionGeneration::new(1).unwrap();
    let (fonts, _) =
        UiGlobalFontCollection::admit_profile(generation, profile_inputs_from_repository())
            .unwrap();
    let fonts = Arc::new(fonts);
    let mut cases = 0;
    for source in rgi_emoji_sources() {
        let layout = layout_with_fonts(
            &source,
            320_000,
            1,
            UiTextOverflow::Clip,
            generation,
            &fonts,
        );
        let end = u32::try_from(source.len()).unwrap();
        assert_eq!(layout.view().graphemes().len(), 1, "split {source:?}");
        assert!(layout.glyphs().iter().all(|glyph| {
            let range = glyph.original_range();
            range.start() == 0 && range.end() == end
        }));
        let boundaries = layout
            .carets()
            .iter()
            .map(|caret| caret.position().original_boundary().start())
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(boundaries, std::collections::BTreeSet::from([0, end]));
        assert_canonical_interaction_geometry(&layout, end);

        let clipped = layout_with_fonts(
            &format!("{source} trailing"),
            1,
            1,
            UiTextOverflow::Ellipsis,
            generation,
            &fonts,
        );
        assert!(clipped.lines()[0].overflowed());
        assert!(clipped.positioned_glyphs().iter().all(|positioned| {
            let range = clipped.glyphs()[positioned.source_glyph_index() as usize].original_range();
            range.is_empty() || range.start() >= end || (range.start() == 0 && range.end() == end)
        }));
        cases += 1;
    }
    assert_eq!(cases, 3_953);
}

fn assert_canonical_interaction_geometry(layout: &crate::UiQualifiedTextLayout, end: u32) {
    let left = layout
        .carets()
        .iter()
        .map(|caret| caret.x_millipoints())
        .min()
        .expect("RGI layout has a leading caret");
    let right = layout
        .carets()
        .iter()
        .map(|caret| caret.x_millipoints())
        .max()
        .expect("RGI layout has a trailing caret");
    let first_caret = layout.carets()[0];
    let hit = layout
        .hit_test(UiTextPoint::from_text_mechanics(
            (left + right) / 2,
            (first_caret.top_millipoints() + first_caret.bottom_millipoints()) / 2,
        ))
        .expect("RGI cluster is hit-testable");
    assert_eq!(
        (hit.cluster_range().start(), hit.cluster_range().end()),
        (0, end)
    );
    let selection = layout
        .selection_rects(
            UiTextOriginalRange::from_text_mechanics(0, end).expect("nonempty RGI sequence"),
        )
        .unwrap();
    assert!(!selection.is_empty());
    assert!(selection
        .iter()
        .all(|rect| { rect.selected_range().start() == 0 && rect.selected_range().end() == end }));
}

fn rgi_emoji_sources() -> impl Iterator<Item = String> {
    EMOJI_TEST
        .lines()
        .filter(|line| {
            line.as_bytes().first().is_some_and(u8::is_ascii_hexdigit)
                && line.split(';').nth(1).is_some_and(|field| {
                    matches!(
                        field.split_whitespace().next(),
                        Some("fully-qualified" | "component")
                    )
                })
        })
        .map(|line| {
            line.split(';')
                .next()
                .expect("emoji sequence")
                .split_whitespace()
                .map(|value| {
                    char::from_u32(u32::from_str_radix(value, 16).expect("hex scalar"))
                        .expect("valid Unicode scalar")
                })
                .collect()
        })
}
