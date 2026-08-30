mod properties;
mod rules;

use properties::{LineClass, LineProperties};

#[derive(Clone, Copy)]
pub(super) struct LineUnit {
    start: usize,
    end: usize,
    code: u32,
    class: LineClass,
    properties: LineProperties,
    ends_with_zwj: bool,
}

pub(super) fn opportunities(source: &str) -> Box<[u32]> {
    let units = units(source);
    let mut boundaries = unicode_opportunities_for_units(source, &units).into_vec();
    boundaries.extend(crate::dictionary_segmentation::complex_line_opportunities(
        source, &units,
    ));
    boundaries.sort_unstable();
    boundaries.dedup();
    boundaries.into_boxed_slice()
}

#[cfg(test)]
pub(super) fn unicode_opportunities(source: &str) -> Box<[u32]> {
    let units = units(source);
    unicode_opportunities_for_units(source, &units)
}

fn unicode_opportunities_for_units(source: &str, units: &[LineUnit]) -> Box<[u32]> {
    let mut boundaries = (1..units.len())
        .filter(|right| rules::break_before(units, *right))
        .map(|right| u32::try_from(units[right].start).expect("admitted text fits u32"))
        .collect::<Vec<_>>();
    boundaries.push(u32::try_from(source.len()).expect("admitted text fits u32"));
    boundaries.into_boxed_slice()
}

pub(super) fn units(source: &str) -> Vec<LineUnit> {
    let mut units: Vec<LineUnit> = Vec::new();
    for (start, character) in source.char_indices() {
        let properties = LineProperties::for_character(character);
        let raw = properties.resolved_class();
        let end = start + character.len_utf8();
        let combining = matches!(raw, LineClass::CM | LineClass::ZWJ);
        let can_attach = units.last().is_some_and(|unit| {
            !matches!(
                unit.class,
                LineClass::BK
                    | LineClass::CR
                    | LineClass::LF
                    | LineClass::NL
                    | LineClass::SP
                    | LineClass::ZW
            )
        });
        if combining && can_attach {
            let unit = units.last_mut().expect("attachment has a base");
            unit.end = end;
            unit.ends_with_zwj = raw == LineClass::ZWJ;
            continue;
        }
        units.push(LineUnit {
            start,
            end,
            code: character as u32,
            class: if combining { LineClass::AL } else { raw },
            properties,
            ends_with_zwj: raw == LineClass::ZWJ,
        });
    }
    units
}

impl LineUnit {
    const fn class(self) -> LineClass {
        self.class
    }

    pub(super) const fn code(self) -> u32 {
        self.code
    }

    const fn is_east_asian(self) -> bool {
        self.properties.east_asian()
    }

    fn is_initial_quote(self) -> bool {
        self.class == LineClass::QU && self.properties.is_initial_punctuation()
    }

    fn is_final_quote(self) -> bool {
        self.class == LineClass::QU && self.properties.is_final_punctuation()
    }

    const fn is_unassigned_extended_pictographic(self) -> bool {
        self.properties.is_unassigned() && self.properties.is_extended_pictographic()
    }

    pub(super) const fn has_complex_context(self) -> bool {
        self.properties.complex_context()
    }

    pub(super) const fn start(self) -> usize {
        self.start
    }

    pub(super) const fn end(self) -> usize {
        self.end
    }
}
