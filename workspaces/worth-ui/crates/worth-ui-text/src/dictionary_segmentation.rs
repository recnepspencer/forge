use icu_segmenter::{
    options::{LineBreakOptions, WordBreakInvariantOptions},
    LineSegmenter, WordSegmenter,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::line_break::LineUnit;

pub(super) fn word_boundaries(source: &str) -> Box<[u32]> {
    let mut boundaries = source
        .split_word_bound_indices()
        .map(|(offset, _)| admitted_offset(offset))
        .chain(core::iter::once(admitted_offset(source.len())))
        .collect::<Vec<_>>();
    for (start, end) in dictionary_script_runs(source) {
        boundaries.retain(|boundary| {
            let boundary = *boundary as usize;
            boundary <= start || boundary >= end
        });
        boundaries.extend(
            WordSegmenter::new_auto(WordBreakInvariantOptions::default())
                .segment_str(&source[start..end])
                .map(|offset| admitted_offset(start + offset)),
        );
    }
    boundaries.sort_unstable();
    boundaries.dedup();
    boundaries.into_boxed_slice()
}

fn dictionary_script_runs(source: &str) -> Vec<(usize, usize)> {
    let mut runs = Vec::new();
    let mut current: Option<(usize, ComplexScript)> = None;
    for (offset, character) in source.char_indices() {
        let script = complex_script(character as u32);
        if current.is_some_and(|(_, active)| Some(active) != script) {
            let (start, _) = current.take().expect("active dictionary run");
            runs.push((start, offset));
        }
        if current.is_none() {
            current = script.map(|script| (offset, script));
        }
    }
    if let Some((start, _)) = current {
        runs.push((start, source.len()));
    }
    runs
}

pub(super) fn complex_line_opportunities<'a>(
    source: &'a str,
    units: &'a [LineUnit],
) -> impl Iterator<Item = u32> + 'a {
    LineSegmenter::new_auto(LineBreakOptions::default())
        .segment_str(source)
        .filter(move |boundary| is_complex_boundary(*boundary, units))
        .map(admitted_offset)
}

fn is_complex_boundary(boundary: usize, units: &[LineUnit]) -> bool {
    let right = units.partition_point(|unit| unit.start() < boundary);
    if right == 0
        || right >= units.len()
        || units[right - 1].end() != boundary
        || units[right].start() != boundary
        || !units[right - 1].has_complex_context()
        || !units[right].has_complex_context()
    {
        return false;
    }
    let left_script = complex_script(units[right - 1].code());
    left_script.is_some() && left_script == complex_script(units[right].code())
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ComplexScript {
    Khmer,
    Lao,
    Myanmar,
    Thai,
}

const fn complex_script(code: u32) -> Option<ComplexScript> {
    match code {
        0x0E00..=0x0E7F => Some(ComplexScript::Thai),
        0x0E80..=0x0EFF => Some(ComplexScript::Lao),
        0x1000..=0x109F | 0xA9E0..=0xA9FF | 0xAA60..=0xAA7F => Some(ComplexScript::Myanmar),
        0x1780..=0x17FF | 0x19E0..=0x19FF => Some(ComplexScript::Khmer),
        _ => None,
    }
}

fn admitted_offset(offset: usize) -> u32 {
    u32::try_from(offset).expect("admitted text fits u32")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinned_auto_model_finds_thai_words_without_spaces() {
        let source = "\u{0E20}\u{0E32}\u{0E29}\u{0E32}\u{0E44}\u{0E17}\u{0E22}\u{0E20}\u{0E32}\u{0E29}\u{0E32}\u{0E44}\u{0E17}\u{0E22}";
        assert_eq!(word_boundaries(source).as_ref(), &[0, 12, 21, 33, 42]);

        let units = crate::line_break::units(source);
        assert_eq!(
            complex_line_opportunities(source, &units).collect::<Vec<_>>(),
            vec![12, 21, 33]
        );
    }

    #[test]
    fn dictionary_tailoring_does_not_join_distinct_complex_scripts() {
        let source = "\u{102C}\u{0E01}";
        assert_eq!(
            crate::line_break::opportunities(source).as_ref(),
            crate::line_break::unicode_opportunities(source).as_ref()
        );
    }
}
