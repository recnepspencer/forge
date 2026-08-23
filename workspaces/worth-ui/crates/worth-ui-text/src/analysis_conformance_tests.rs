use unicode_bidi::{BidiInfo, Level};
use unicode_segmentation::UnicodeSegmentation;

use crate::{bidi_data::UiUnicode17BidiData, dictionary_segmentation, line_break};

const GRAPHEME_BREAK_TEST: &str = include_str!(
    "../../../profiles/worth-ui-global-text-v2/unicode/ucd/auxiliary/GraphemeBreakTest.txt"
);
const EMOJI_TEST: &str =
    include_str!("../../../profiles/worth-ui-global-text-v2/unicode/emoji/emoji-test.txt");
const WORD_BREAK_TEST: &str = include_str!(
    "../../../profiles/worth-ui-global-text-v2/unicode/ucd/auxiliary/WordBreakTest.txt"
);
const LINE_BREAK_TEST: &str = include_str!(
    "../../../profiles/worth-ui-global-text-v2/unicode/ucd/auxiliary/LineBreakTest.txt"
);
const BIDI_CHARACTER_TEST: &str =
    include_str!("../../../profiles/worth-ui-global-text-v2/unicode/ucd/BidiCharacterTest.txt");

#[test]
pub(crate) fn every_unicode_17_grapheme_break_case_matches_the_frozen_corpus() {
    let mut cases = 0usize;
    for line in GRAPHEME_BREAK_TEST
        .lines()
        .filter(|line| line.starts_with('÷'))
    {
        let (source, expected) = boundary_case(line);
        let mut observed = source
            .grapheme_indices(true)
            .map(|(offset, _)| offset)
            .collect::<Vec<_>>();
        observed.push(source.len());
        assert_eq!(observed, expected, "grapheme case drifted: {line}");
        cases += 1;
    }
    assert!(cases > 500, "Unicode grapheme corpus was not exercised");
}

#[test]
pub(crate) fn every_unicode_17_word_break_case_matches_the_frozen_corpus() {
    let mut cases = 0usize;
    for line in WORD_BREAK_TEST.lines().filter(|line| line.starts_with('÷')) {
        let (source, expected) = boundary_case(line);
        let observed = dictionary_segmentation::word_boundaries(&source)
            .iter()
            .map(|offset| *offset as usize)
            .collect::<Vec<_>>();
        assert_eq!(observed, expected, "word case drifted: {line}");
        cases += 1;
    }
    assert!(cases > 1_000, "Unicode word corpus was not exercised");
}

#[test]
pub(crate) fn every_unicode_17_line_break_case_matches_the_frozen_corpus() {
    let mut cases = 0usize;
    let mut mismatches = Vec::new();
    for line in LINE_BREAK_TEST
        .lines()
        .filter(|line| matches!(line.chars().next(), Some('÷' | '×')))
    {
        let (source, expected) = boundary_case(line);
        let observed = line_break::unicode_opportunities(&source)
            .iter()
            .map(|offset| *offset as usize)
            .collect::<Vec<_>>();
        if observed != expected && mismatches.len() < 20 {
            mismatches.push(format!(
                "{line}\nobserved={observed:?} expected={expected:?}"
            ));
        }
        cases += 1;
    }
    assert!(cases > 5_000, "Unicode line corpus was not exercised");
    assert!(
        mismatches.is_empty(),
        "Unicode line corpus drifted:\n{}",
        mismatches.join("\n")
    );
}

#[test]
pub(crate) fn every_unicode_17_rgi_emoji_is_one_extended_grapheme_cluster() {
    let mut cases = 0usize;
    for line in EMOJI_TEST.lines().filter(|line| {
        line.as_bytes().first().is_some_and(u8::is_ascii_hexdigit)
            && line.split(';').nth(1).is_some_and(|field| {
                matches!(
                    field.split_whitespace().next(),
                    Some("fully-qualified" | "component")
                )
            })
    }) {
        let sequence = line.split(';').next().expect("emoji sequence");
        let source = scalars(sequence);
        assert_eq!(
            source.graphemes(true).count(),
            1,
            "RGI emoji sequence split: {sequence}"
        );
        cases += 1;
    }
    assert_eq!(cases, 3_953, "Unicode 17 RGI inventory drifted");
}

#[test]
pub(crate) fn every_unicode_17_bidi_character_case_uses_repository_data_and_visual_order() {
    let mut cases = 0usize;
    for line in BIDI_CHARACTER_TEST
        .lines()
        .filter(|line| line.as_bytes().first().is_some_and(u8::is_ascii_hexdigit))
    {
        let fields = line.split(';').map(str::trim).collect::<Vec<_>>();
        assert_eq!(fields.len(), 5, "malformed bidi case");
        let source = scalars(fields[0]);
        let paragraph_level = match fields[1] {
            "0" => Some(Level::ltr()),
            "1" => Some(Level::rtl()),
            "2" => None,
            value => panic!("unknown paragraph direction {value}"),
        };
        let bidi = BidiInfo::new_with_data_source(&UiUnicode17BidiData, &source, paragraph_level);
        let paragraph = bidi.paragraphs.first().expect("one bidi paragraph");
        assert_eq!(paragraph.level.number().to_string(), fields[2], "{line}");
        let observed_levels = bidi.reordered_levels_per_char(paragraph, paragraph.range.clone());
        let expected_levels = fields[3].split_whitespace().collect::<Vec<_>>();
        assert_eq!(observed_levels.len(), expected_levels.len(), "{line}");
        let retained = expected_levels
            .iter()
            .enumerate()
            .filter_map(|(index, expected)| (*expected != "x").then_some(index))
            .collect::<Vec<_>>();
        let retained_levels = retained
            .iter()
            .map(|index| observed_levels[*index])
            .collect::<Vec<_>>();
        for (observed, expected) in observed_levels.iter().zip(&expected_levels) {
            if *expected != "x" {
                assert_eq!(observed.number().to_string(), *expected, "{line}");
            }
        }
        let observed_order = BidiInfo::reorder_visual(&retained_levels)
            .into_iter()
            .map(|index| retained[index])
            .collect::<Vec<_>>();
        let expected_order = fields[4]
            .split_whitespace()
            .map(|value| value.parse::<usize>().expect("bidi visual index"))
            .collect::<Vec<_>>();
        assert_eq!(observed_order, expected_order, "{line}");
        cases += 1;
    }
    assert!(
        cases > 90_000,
        "Unicode bidi character corpus was not exercised"
    );
}

fn boundary_case(line: &str) -> (String, Vec<usize>) {
    let data = line.split('#').next().expect("grapheme data");
    let mut source = String::new();
    let mut expected = Vec::new();
    let mut next_break = false;
    for token in data.split_whitespace() {
        match token {
            "÷" => next_break = true,
            "×" => next_break = false,
            scalar => {
                if next_break {
                    expected.push(source.len());
                }
                source.push(parse_scalar(scalar));
                next_break = false;
            }
        }
    }
    if next_break {
        expected.push(source.len());
    }
    (source, expected)
}

fn scalars(field: &str) -> String {
    field.split_whitespace().map(parse_scalar).collect()
}

fn parse_scalar(value: &str) -> char {
    let scalar = u32::from_str_radix(value, 16).expect("hex Unicode scalar");
    char::from_u32(scalar).expect("valid Unicode scalar")
}
