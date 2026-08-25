use worth_ui_host_contract::UiTextOriginalRange;

#[test]
fn mixed_script_layout_preserves_exact_original_utf8_ranges() {
    let source = "a\u{301} \u{5e9}\u{5dc}\u{5d5}\u{5dd} \u{1f469}\u{1f3fd}\u{200d}\u{1f4bb}";
    let layout = super::tests::layout(source, 240_000, 8);
    let ranges = layout
        .glyphs()
        .iter()
        .map(|glyph| glyph.original_range())
        .collect::<Vec<_>>();

    assert!(matches_original_utf8_oracle(source, &ranges));
}

#[test]
fn normalized_offset_substitution_fails_the_original_utf8_oracle() {
    let source = "a\u{301} \u{5e9}\u{5dc}\u{5d5}\u{5dd} \u{1f469}\u{1f3fd}\u{200d}\u{1f4bb}";
    let layout = super::tests::layout(source, 240_000, 8);
    let mut substituted = layout
        .glyphs()
        .iter()
        .map(|glyph| glyph.original_range())
        .collect::<Vec<_>>();
    substituted[0] = UiTextOriginalRange::from_text_mechanics(0, 1).unwrap();

    assert!(source.is_char_boundary(1));
    assert!(!matches_original_utf8_oracle(source, &substituted));
}

fn matches_original_utf8_oracle(source: &str, ranges: &[UiTextOriginalRange]) -> bool {
    let expected = [
        (0, 3),
        (3, 4),
        (10, 12),
        (8, 10),
        (6, 8),
        (4, 6),
        (12, 13),
        (13, 28),
    ];
    ranges.len() == expected.len()
        && ranges.iter().zip(expected).all(|(range, (start, end))| {
            range.start() == start
                && range.end() == end
                && source.is_char_boundary(start as usize)
                && source.is_char_boundary(end as usize)
        })
}
