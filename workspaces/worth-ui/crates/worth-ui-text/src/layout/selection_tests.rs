use worth_ui_host_contract::UiTextOriginalRange;

use super::{tests::layout, UiTextSelectionDenial};

#[test]
fn selection_admits_only_utf8_cluster_boundaries_for_combining_emoji_and_bidi_text() {
    let combining = layout("e\u{301}", 200_000, 2);
    assert_valid_selection(&combining, 0, 3);
    assert_denied_selection(&combining, 1, 2, UiTextSelectionDenial::NotUtf8Boundary);
    assert_denied_selection(&combining, 1, 3, UiTextSelectionDenial::NotClusterBoundary);
    assert_denied_selection(&combining, 0, 4, UiTextSelectionDenial::RangeOutOfBounds);

    let emoji_source = "👨‍👩‍👧‍👦";
    let emoji = layout(emoji_source, 200_000, 2);
    assert_valid_selection(&emoji, 0, emoji_source.len() as u32);
    let interior = emoji_source.char_indices().nth(1).unwrap().0 as u32;
    assert_denied_selection(
        &emoji,
        interior,
        emoji_source.len() as u32,
        UiTextSelectionDenial::NotClusterBoundary,
    );

    let bidi_source = "abc אבג";
    let bidi = layout(bidi_source, 200_000, 2);
    assert_valid_selection(&bidi, 4, 6);
    assert_denied_selection(&bidi, 5, 6, UiTextSelectionDenial::NotUtf8Boundary);
}

fn assert_valid_selection(layout: &super::UiQualifiedTextLayout, start: u32, end: u32) {
    let rects = layout
        .selection_rects(UiTextOriginalRange::new(start, end).unwrap())
        .expect("selection endpoints are qualified boundaries");
    assert!(!rects.is_empty());
    assert!(rects.iter().all(|rect| {
        rect.selected_range().start() >= start && rect.selected_range().end() <= end
    }));
}

fn assert_denied_selection(
    layout: &super::UiQualifiedTextLayout,
    start: u32,
    end: u32,
    expected: UiTextSelectionDenial,
) {
    assert_eq!(
        layout.selection_rects(UiTextOriginalRange::new(start, end).unwrap()),
        Err(expected)
    );
}
