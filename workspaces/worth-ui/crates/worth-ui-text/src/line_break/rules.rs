use super::{properties::LineClass, LineUnit};

pub(super) fn break_before(units: &[LineUnit], right: usize) -> bool {
    use LineClass::*;
    let left = units[right - 1];
    let current = units[right];
    let l = left.class();
    let r = current.class();

    if l == BK || (matches!(l, CR | LF | NL) && !(l == CR && r == LF)) {
        return true;
    }
    if matches!(r, BK | CR | LF | NL) || matches!(r, SP | ZW) {
        return false;
    }
    if zero_width_break(units, right) {
        return true;
    }
    if left.ends_with_zwj || l == WJ || r == WJ || l == GL {
        return false;
    }
    if r == GL && !matches!(l, SP | BA | HY | HH) {
        return false;
    }
    if matches!(r, CL | CP | EX | SY) || opening_before_spaces(units, right) {
        return false;
    }
    if initial_quote_before_spaces(units, right) || final_quote_context(units, right) {
        return false;
    }
    if l == SP && r == IS && next_class(units, right) == Some(NU) {
        return true;
    }
    if r == IS || closing_nonstarter(units, right) || paired_break_symbol(units, right) {
        return false;
    }
    if l == SP {
        return true;
    }
    if unresolved_quote_context(units, right) {
        return false;
    }
    if l == CB || r == CB {
        return true;
    }
    if word_initial_hyphen(units, right)
        || matches!(r, BA | HH | HY | NS)
        || l == BB
        || hebrew_hyphen(units, right)
        || (l == SY && r == HL)
        || r == IN
    {
        return false;
    }
    if alphanumeric_pair(l, r) || numeric_pair(units, right) || hangul_pair(l, r) {
        return false;
    }
    if brahmic_pair(units, right)
        || (l == IS && matches!(r, AL | HL))
        || parenthetic_pair(left, current)
        || regional_indicator_pair(units, right)
        || (r == EM && (l == EB || left.is_unassigned_extended_pictographic()))
    {
        return false;
    }
    true
}

fn zero_width_break(units: &[LineUnit], right: usize) -> bool {
    previous_non_space(units, right).is_some_and(|index| units[index].class() == LineClass::ZW)
}

fn opening_before_spaces(units: &[LineUnit], right: usize) -> bool {
    previous_non_space(units, right).is_some_and(|index| units[index].class() == LineClass::OP)
}

fn initial_quote_before_spaces(units: &[LineUnit], right: usize) -> bool {
    let Some(quote) = previous_non_space(units, right) else {
        return false;
    };
    if !units[quote].is_initial_quote() {
        return false;
    }
    quote == 0
        || matches!(
            units[quote - 1].class(),
            LineClass::BK
                | LineClass::CR
                | LineClass::LF
                | LineClass::NL
                | LineClass::OP
                | LineClass::QU
                | LineClass::GL
                | LineClass::SP
                | LineClass::ZW
        )
}

fn final_quote_context(units: &[LineUnit], right: usize) -> bool {
    if !units[right].is_final_quote() {
        return false;
    }
    matches!(
        next_class(units, right),
        None | Some(
            LineClass::SP
                | LineClass::GL
                | LineClass::WJ
                | LineClass::CL
                | LineClass::QU
                | LineClass::CP
                | LineClass::EX
                | LineClass::IS
                | LineClass::SY
                | LineClass::BK
                | LineClass::CR
                | LineClass::LF
                | LineClass::NL
                | LineClass::ZW
        )
    )
}

fn closing_nonstarter(units: &[LineUnit], right: usize) -> bool {
    units[right].class() == LineClass::NS
        && previous_non_space(units, right)
            .is_some_and(|index| matches!(units[index].class(), LineClass::CL | LineClass::CP))
}

fn paired_break_symbol(units: &[LineUnit], right: usize) -> bool {
    units[right].class() == LineClass::B2
        && previous_non_space(units, right)
            .is_some_and(|index| units[index].class() == LineClass::B2)
}

fn unresolved_quote_context(units: &[LineUnit], right: usize) -> bool {
    let left = units[right - 1];
    let current = units[right];
    if current.class() == LineClass::QU && !current.is_initial_quote() {
        return true;
    }
    if left.class() == LineClass::QU && !left.is_final_quote() {
        return true;
    }
    if current.class() == LineClass::QU {
        let after = units.get(right + 1);
        if !left.is_east_asian() || after.is_none_or(|unit| !unit.is_east_asian()) {
            return true;
        }
    }
    left.class() == LineClass::QU
        && (!current.is_east_asian() || right == 1 || !units[right - 2].is_east_asian())
}

fn word_initial_hyphen(units: &[LineUnit], right: usize) -> bool {
    use LineClass::*;
    if !matches!(units[right - 1].class(), HY | HH) || !matches!(units[right].class(), AL | HL) {
        return false;
    }
    right == 1
        || matches!(
            units[right - 2].class(),
            BK | CR | LF | NL | SP | ZW | CB | GL
        )
}

fn hebrew_hyphen(units: &[LineUnit], right: usize) -> bool {
    right >= 2
        && units[right - 2].class() == LineClass::HL
        && matches!(units[right - 1].class(), LineClass::HY | LineClass::HH)
        && units[right].class() != LineClass::HL
}

fn alphanumeric_pair(left: LineClass, right: LineClass) -> bool {
    use LineClass::*;
    (matches!(left, AL | HL) && matches!(right, AL | HL | NU | PR | PO))
        || (left == NU && matches!(right, AL | HL))
        || (matches!(left, PR | PO) && matches!(right, AL | HL))
        || (left == PR && matches!(right, ID | EB | EM))
        || (matches!(left, ID | EB | EM) && right == PO)
}

fn numeric_pair(units: &[LineUnit], right: usize) -> bool {
    use LineClass::*;
    let l = units[right - 1].class();
    let r = units[right].class();
    if r == NU && matches!(l, PO | PR | HY | IS) {
        return true;
    }
    if matches!(r, PO | PR | NU) && preceding_numeric(units, right) {
        return true;
    }
    if matches!(l, PO | PR) {
        return r == NU
            || (r == OP && matches!(next_class(units, right), Some(NU))
                || (r == OP
                    && next_class(units, right) == Some(IS)
                    && units.get(right + 2).is_some_and(|unit| unit.class() == NU)));
    }
    false
}

fn preceding_numeric(units: &[LineUnit], right: usize) -> bool {
    let mut index = right;
    if index > 0 && matches!(units[index - 1].class(), LineClass::CL | LineClass::CP) {
        index -= 1;
    }
    while index > 0 && matches!(units[index - 1].class(), LineClass::SY | LineClass::IS) {
        index -= 1;
    }
    index > 0 && units[index - 1].class() == LineClass::NU
}

fn hangul_pair(left: LineClass, right: LineClass) -> bool {
    use LineClass::*;
    (left == JL && matches!(right, JL | JV | H2 | H3))
        || (matches!(left, JV | H2) && matches!(right, JV | JT))
        || (matches!(left, JT | H3) && right == JT)
        || (matches!(left, JL | JV | JT | H2 | H3) && right == PO)
        || (left == PR && matches!(right, JL | JV | JT | H2 | H3))
}

fn brahmic_pair(units: &[LineUnit], right: usize) -> bool {
    use LineClass::*;
    let starter = |unit: LineUnit| matches!(unit.class(), AK | AS) || unit.code() == 0x25CC;
    let left = units[right - 1];
    let current = units[right];
    (left.class() == AP && starter(current))
        || (starter(left) && matches!(current.class(), VF | VI))
        || (right >= 2
            && starter(units[right - 2])
            && left.class() == VI
            && (current.class() == AK || current.code() == 0x25CC))
        || (starter(left) && starter(current) && next_class(units, right) == Some(VF))
}

fn parenthetic_pair(left: LineUnit, right: LineUnit) -> bool {
    use LineClass::*;
    (matches!(left.class(), AL | HL | NU) && right.class() == OP && !right.is_east_asian())
        || (left.class() == CP && !left.is_east_asian() && matches!(right.class(), AL | HL | NU))
}

fn regional_indicator_pair(units: &[LineUnit], right: usize) -> bool {
    if units[right - 1].class() != LineClass::RI || units[right].class() != LineClass::RI {
        return false;
    }
    let mut count = 0;
    let mut index = right;
    while index > 0 && units[index - 1].class() == LineClass::RI {
        count += 1;
        index -= 1;
    }
    count % 2 == 1
}

fn previous_non_space(units: &[LineUnit], right: usize) -> Option<usize> {
    (0..right)
        .rev()
        .find(|index| units[*index].class() != LineClass::SP)
}

fn next_class(units: &[LineUnit], right: usize) -> Option<LineClass> {
    units.get(right + 1).map(|unit| unit.class())
}
