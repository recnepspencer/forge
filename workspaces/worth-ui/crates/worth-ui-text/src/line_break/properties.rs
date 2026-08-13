#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::upper_case_acronyms)] // Unicode normative Line_Break aliases are uppercase.
pub(super) enum LineClass {
    AI,
    AK,
    AL,
    AP,
    AS,
    B2,
    BA,
    BB,
    BK,
    CB,
    CJ,
    CL,
    CM,
    CP,
    CR,
    EB,
    EM,
    EX,
    GL,
    H2,
    H3,
    HH,
    HL,
    HY,
    ID,
    IN,
    IS,
    JL,
    JT,
    JV,
    LF,
    NL,
    NS,
    NU,
    OP,
    PO,
    PR,
    QU,
    RI,
    SA,
    SG,
    SP,
    SY,
    VF,
    VI,
    WJ,
    XX,
    ZW,
    ZWJ,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GeneralCategory {
    Mn,
    Mc,
    Pi,
    Pf,
    Other,
}

#[derive(Clone, Copy)]
pub(super) struct LineProperties {
    class: LineClass,
    category: GeneralCategory,
    east_asian: bool,
    extended_pictographic: bool,
    assigned: bool,
}

include!(concat!(env!("OUT_DIR"), "/unicode_17_line.rs"));

impl LineProperties {
    pub(super) fn for_character(character: char) -> Self {
        let code = character as u32;
        Self {
            class: value_at(LINE_CLASSES, code).unwrap_or(LineClass::XX),
            category: value_at(GENERAL_CATEGORIES, code).unwrap_or(GeneralCategory::Other),
            east_asian: contains(EAST_ASIAN_RANGES, code),
            extended_pictographic: contains(EXTENDED_PICTOGRAPHIC_RANGES, code),
            assigned: contains(ASSIGNED_RANGES, code),
        }
    }

    pub(super) const fn resolved_class(self) -> LineClass {
        match self.class {
            LineClass::AI | LineClass::SG | LineClass::XX => LineClass::AL,
            LineClass::CJ => LineClass::NS,
            LineClass::SA if matches!(self.category, GeneralCategory::Mn | GeneralCategory::Mc) => {
                LineClass::CM
            }
            LineClass::SA => LineClass::AL,
            class => class,
        }
    }

    pub(super) const fn complex_context(self) -> bool {
        matches!(self.class, LineClass::SA)
    }

    pub(super) const fn east_asian(self) -> bool {
        self.east_asian
    }

    pub(super) const fn is_initial_punctuation(self) -> bool {
        matches!(self.category, GeneralCategory::Pi)
    }

    pub(super) const fn is_final_punctuation(self) -> bool {
        matches!(self.category, GeneralCategory::Pf)
    }

    pub(super) const fn is_unassigned(self) -> bool {
        !self.assigned
    }

    pub(super) const fn is_extended_pictographic(self) -> bool {
        self.extended_pictographic
    }
}

fn value_at<T: Copy>(ranges: &[(u32, u32, T)], code: u32) -> Option<T> {
    let index = ranges.partition_point(|(_, end, _)| *end < code);
    ranges
        .get(index)
        .filter(|(start, _, _)| *start <= code)
        .map(|(_, _, value)| *value)
}

fn contains(ranges: &[(u32, u32)], code: u32) -> bool {
    let index = ranges.partition_point(|(_, end)| *end < code);
    ranges.get(index).is_some_and(|(start, _)| *start <= code)
}
