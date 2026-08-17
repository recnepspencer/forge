use super::units::{advance_at, LayoutUnit, UnitKind};
use crate::{UiShapedTextParagraph, UiTextWrap};

#[derive(Clone, Debug)]
pub(super) struct LinePlan {
    pub(super) unit_start: usize,
    pub(super) unit_end: usize,
    pub(super) width_millipoints: i64,
    pub(super) hard_break: bool,
    pub(super) overflowed: bool,
}

pub(super) fn fit(shaped: &UiShapedTextParagraph, units: &[LayoutUnit]) -> Vec<LinePlan> {
    let constraints = shaped.constraints();
    let maximum_width = i64::from(constraints.width_millipoints());
    let maximum_lines = constraints.maximum_lines() as usize;
    if units.is_empty() {
        return vec![LinePlan {
            unit_start: 0,
            unit_end: 0,
            width_millipoints: 0,
            hard_break: false,
            overflowed: false,
        }];
    }
    let mut lines = Vec::new();
    let mut start = 0usize;
    while start < units.len() && lines.len() < maximum_lines {
        let mut index = start;
        let mut width = 0i64;
        let mut last_break = None;
        let mut closed = false;
        while index < units.len() {
            if units[index].kind == UnitKind::HardBreak {
                lines.push(LinePlan {
                    unit_start: start,
                    unit_end: index + 1,
                    width_millipoints: width,
                    hard_break: true,
                    overflowed: width > maximum_width,
                });
                start = index + 1;
                closed = true;
                break;
            }
            let advance = advance_at(&units[index], width, constraints.tab_interval_millipoints());
            let wraps = constraints.wrap() != UiTextWrap::None;
            if wraps && width + advance > maximum_width && index > start {
                let end = last_break.filter(|end| *end > start).unwrap_or(index);
                let fitted_width = measure(shaped, &units[start..end]);
                lines.push(LinePlan {
                    unit_start: start,
                    unit_end: end,
                    width_millipoints: fitted_width,
                    hard_break: false,
                    overflowed: fitted_width > maximum_width,
                });
                start = end;
                closed = true;
                break;
            }
            width += advance;
            if break_allowed(shaped, &units[index]) {
                last_break = Some(index + 1);
            }
            index += 1;
        }
        if !closed {
            lines.push(LinePlan {
                unit_start: start,
                unit_end: units.len(),
                width_millipoints: width,
                hard_break: false,
                overflowed: width > maximum_width,
            });
            start = units.len();
        }
    }
    if start < units.len() {
        if let Some(last) = lines.last_mut() {
            last.overflowed = true;
        }
    } else if units
        .last()
        .is_some_and(|unit| unit.kind == UnitKind::HardBreak)
        && lines.len() < maximum_lines
    {
        lines.push(LinePlan {
            unit_start: units.len(),
            unit_end: units.len(),
            width_millipoints: 0,
            hard_break: false,
            overflowed: false,
        });
    }
    lines
}

fn break_allowed(shaped: &UiShapedTextParagraph, unit: &LayoutUnit) -> bool {
    match shaped.constraints().wrap() {
        UiTextWrap::None => false,
        UiTextWrap::Grapheme => true,
        UiTextWrap::UnicodeWord => shaped
            .line_opportunities()
            .binary_search(&unit.original_range.end())
            .is_ok(),
    }
}

pub(super) fn measure(shaped: &UiShapedTextParagraph, units: &[LayoutUnit]) -> i64 {
    units.iter().fold(0, |width, unit| {
        width + advance_at(unit, width, shaped.constraints().tab_interval_millipoints())
    })
}
