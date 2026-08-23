use core::ops::Range;

use worth_ui_host_contract::UiTextOriginalRange;

use crate::{UiShapedTextParagraph, UiTextStyle};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum UnitKind {
    Glyphs,
    Tab,
    HardBreak,
}

#[derive(Clone, Debug)]
pub(super) struct LayoutUnit {
    pub(super) original_range: UiTextOriginalRange,
    pub(super) glyph_range: Range<u32>,
    pub(super) logical_run_index: u32,
    pub(super) bidi_level: u8,
    pub(super) style_index: u16,
    pub(super) advance_millipoints: i64,
    pub(super) kind: UnitKind,
}

pub(super) fn build(shaped: &UiShapedTextParagraph) -> Vec<LayoutUnit> {
    let mut units = Vec::new();
    for (run_index, run) in shaped.runs().iter().copied().enumerate() {
        let glyph_range = run.glyph_range();
        let mut index = glyph_range.start;
        while index < glyph_range.end {
            let range = shaped.glyphs()[index as usize].original_range();
            let start = index;
            let mut raw_advance = 0i64;
            while index < glyph_range.end
                && shaped.glyphs()[index as usize].original_range() == range
            {
                raw_advance += i64::from(shaped.glyphs()[index as usize].x_advance_font_units());
                index += 1;
            }
            let style = shaped.styles()[run.style_index() as usize].style();
            let source = &shaped.source()[range.start() as usize..range.end() as usize];
            let advance = scale_advance(raw_advance.unsigned_abs(), style, run.units_per_em())
                + i64::from(style.letter_spacing_millipoints())
                + word_spacing(source, style);
            units.push(LayoutUnit {
                original_range: range,
                glyph_range: start..index,
                logical_run_index: u32::try_from(run_index).expect("run cap fits u32"),
                bidi_level: run.bidi_level(),
                style_index: run.style_index(),
                advance_millipoints: advance.max(0),
                kind: UnitKind::Glyphs,
            });
        }
    }
    for cluster in shaped
        .selected_clusters()
        .iter()
        .copied()
        .filter(|cluster| cluster.face().is_none())
    {
        let range = cluster.original_range();
        let text = &shaped.source()[range.start() as usize..range.end() as usize];
        units.push(LayoutUnit {
            original_range: range,
            glyph_range: 0..0,
            logical_run_index: u32::MAX,
            bidi_level: cluster.bidi_level(),
            style_index: u16::try_from(cluster.style_index()).expect("style cap fits u16"),
            advance_millipoints: 0,
            kind: if text == "\t" {
                UnitKind::Tab
            } else {
                UnitKind::HardBreak
            },
        });
    }
    units.sort_by_key(|unit| (unit.original_range.start(), unit.original_range.end()));
    units
}

pub(super) fn advance_at(
    unit: &LayoutUnit,
    x_millipoints: i64,
    tab_interval_millipoints: u32,
) -> i64 {
    if unit.kind != UnitKind::Tab {
        return unit.advance_millipoints;
    }
    let interval = i64::from(tab_interval_millipoints);
    interval - x_millipoints.rem_euclid(interval)
}

pub(super) fn scale_advance(raw_advance: u64, style: &UiTextStyle, units_per_em: u16) -> i64 {
    let numerator = u128::from(raw_advance) * u128::from(style.font_size_millipoints());
    let denominator = u128::from(units_per_em);
    i64::try_from((numerator + denominator / 2) / denominator)
        .expect("qualified text advance fits i64")
}

fn word_spacing(source: &str, style: &UiTextStyle) -> i64 {
    if source
        .chars()
        .all(|character| matches!(character, ' ' | '\u{3000}'))
    {
        i64::from(style.word_spacing_millipoints())
    } else {
        0
    }
}
