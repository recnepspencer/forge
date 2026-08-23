use unicode_bidi::{BidiClass, BidiDataSource, Level, ParagraphBidiInfo};

use crate::bidi_data::UiUnicode17BidiData;

const BIDI_TEST: &str =
    include_str!("../../../profiles/worth-ui-global-text-v2/unicode/ucd/BidiTest.txt");

#[test]
pub(crate) fn abstract_bidi_classes_use_exact_unicode_17_representatives() {
    for name in CLASS_NAMES {
        let character = representative(name);
        assert_eq!(
            UiUnicode17BidiData.bidi_class(character),
            expected_class(name),
            "wrong abstract bidi representative for {name}"
        );
    }
}

#[test]
#[ignore = "Phase 4 closure: exhausts the 490,846-case Unicode 17 BidiTest corpus"]
pub(crate) fn every_unicode_17_abstract_bidi_case_matches_levels_and_visual_order() {
    let mut expected_levels: Box<[&str]> = Box::new([]);
    let mut expected_order: Box<[usize]> = Box::new([]);
    let mut cases = 0usize;
    for line in BIDI_TEST.lines() {
        if let Some(levels) = line.strip_prefix("@Levels:") {
            expected_levels = levels.split_whitespace().collect();
        } else if let Some(order) = line.strip_prefix("@Reorder:") {
            expected_order = order
                .split_whitespace()
                .map(|value| value.parse().expect("visual index"))
                .collect();
        } else if line.as_bytes().first().is_some_and(u8::is_ascii_alphabetic) {
            exercise_case(line, &expected_levels, &expected_order);
            cases += 1;
        }
    }
    assert_eq!(cases, 490_846, "Unicode 17 BidiTest inventory drifted");
}

fn exercise_case(line: &str, expected_levels: &[&str], expected_order: &[usize]) {
    let (classes, bitset) = line.split_once(';').expect("abstract bidi case");
    let source: String = classes.split_whitespace().map(representative).collect();
    let bitset = u8::from_str_radix(bitset.trim(), 16).expect("paragraph-level bitset");
    for (bit, paragraph_level) in [(1, None), (2, Some(Level::ltr())), (4, Some(Level::rtl()))] {
        if bitset & bit == 0 {
            continue;
        }
        let bidi =
            ParagraphBidiInfo::new_with_data_source(&UiUnicode17BidiData, &source, paragraph_level);
        let levels = bidi.reordered_levels_per_char(0..source.len());
        assert_eq!(levels.len(), expected_levels.len(), "{line}");
        for (observed, expected) in levels.iter().zip(expected_levels) {
            if *expected != "x" {
                assert_eq!(observed.number().to_string(), *expected, "{line}");
            }
        }
        let retained = expected_levels
            .iter()
            .enumerate()
            .filter_map(|(index, level)| (*level != "x").then_some(index))
            .collect::<Vec<_>>();
        let retained_levels = retained
            .iter()
            .map(|index| levels[*index])
            .collect::<Vec<_>>();
        let order = ParagraphBidiInfo::reorder_visual(&retained_levels)
            .into_iter()
            .map(|index| retained[index])
            .collect::<Vec<_>>();
        assert_eq!(order, expected_order, "{line}");
    }
}

const CLASS_NAMES: &[&str] = &[
    "AL", "AN", "B", "BN", "CS", "EN", "ES", "ET", "FSI", "L", "LRE", "LRI", "LRO", "NSM", "ON",
    "PDF", "PDI", "R", "RLE", "RLI", "RLO", "S", "WS",
];

fn representative(name: &str) -> char {
    match name {
        "AL" => '\u{0627}',
        "AN" => '\u{0660}',
        "B" => '\u{2029}',
        "BN" => '\u{00ad}',
        "CS" => ',',
        "EN" => '0',
        "ES" => '+',
        "ET" => '$',
        "FSI" => '\u{2068}',
        "L" => 'a',
        "LRE" => '\u{202a}',
        "LRI" => '\u{2066}',
        "LRO" => '\u{202d}',
        "NSM" => '\u{0301}',
        "ON" => '!',
        "PDF" => '\u{202c}',
        "PDI" => '\u{2069}',
        "R" => '\u{05d0}',
        "RLE" => '\u{202b}',
        "RLI" => '\u{2067}',
        "RLO" => '\u{202e}',
        "S" => '\t',
        "WS" => ' ',
        _ => panic!("unknown abstract bidi class {name}"),
    }
}

fn expected_class(name: &str) -> BidiClass {
    match name {
        "AL" => BidiClass::AL,
        "AN" => BidiClass::AN,
        "B" => BidiClass::B,
        "BN" => BidiClass::BN,
        "CS" => BidiClass::CS,
        "EN" => BidiClass::EN,
        "ES" => BidiClass::ES,
        "ET" => BidiClass::ET,
        "FSI" => BidiClass::FSI,
        "L" => BidiClass::L,
        "LRE" => BidiClass::LRE,
        "LRI" => BidiClass::LRI,
        "LRO" => BidiClass::LRO,
        "NSM" => BidiClass::NSM,
        "ON" => BidiClass::ON,
        "PDF" => BidiClass::PDF,
        "PDI" => BidiClass::PDI,
        "R" => BidiClass::R,
        "RLE" => BidiClass::RLE,
        "RLI" => BidiClass::RLI,
        "RLO" => BidiClass::RLO,
        "S" => BidiClass::S,
        "WS" => BidiClass::WS,
        _ => unreachable!(),
    }
}
