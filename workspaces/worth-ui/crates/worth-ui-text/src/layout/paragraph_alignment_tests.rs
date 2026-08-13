use crate::UiTextAlignment;

#[test]
pub(crate) fn adjacent_bidi_paragraphs_own_their_half_open_alignment_boundary() {
    for (source, alignment, expect_second_at_left) in [
        ("abc\nשלום ", UiTextAlignment::Start, false),
        ("abc\nשלום ", UiTextAlignment::End, true),
        ("שלום\nabc ", UiTextAlignment::Start, true),
        ("שלום\nabc ", UiTextAlignment::End, false),
    ] {
        let layout = super::tests::layout_with_alignment(source, alignment);
        assert_eq!(layout.lines().len(), 2, "{source:?} {alignment:?}");
        let second_left = layout.lines()[1].logical_bounds().left_millipoints();
        assert_eq!(
            second_left == 0,
            expect_second_at_left,
            "{source:?} {alignment:?}"
        );
    }
}

#[test]
pub(crate) fn trailing_empty_line_inherits_the_last_bidi_paragraph_alignment() {
    for (source, expect_trailing_at_left) in [("שלום\nabc\n", true), ("abc\nשלום\n", false)]
    {
        let layout = super::tests::layout_with_alignment(source, UiTextAlignment::Start);
        assert_eq!(layout.lines().len(), 3, "{source:?}");
        let trailing = layout.lines()[2].logical_bounds();
        assert_eq!(
            trailing.left_millipoints() == 0,
            expect_trailing_at_left,
            "{source:?}"
        );
        assert_eq!(
            layout.carets().last().unwrap().x_millipoints(),
            trailing.left_millipoints()
        );
    }
}
