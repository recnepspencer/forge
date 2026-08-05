use worth_foundational::facade::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisValue, CanonicalIntegerWidth,
};

use super::APPLICATION_QUERY_DOMAIN;

pub(super) fn text(locus: impl Into<String>, value: impl Into<String>) -> CanonicalBasisEntry {
    entry(locus, CanonicalBasisValue::ExactText(value.into().into()))
}

pub(super) fn unsigned_usize(locus: impl Into<String>, value: usize) -> CanonicalBasisEntry {
    unsigned_u64(
        locus,
        u64::try_from(value).expect("application-query structural counts fit in u64"),
    )
}

pub(super) fn unsigned_u64(locus: impl Into<String>, value: u64) -> CanonicalBasisEntry {
    entry(
        locus,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: value as u128,
        },
    )
}

pub(super) fn boolean(locus: impl Into<String>, value: bool) -> CanonicalBasisEntry {
    entry(locus, CanonicalBasisValue::Bool(value))
}

pub(super) fn null(locus: impl Into<String>) -> CanonicalBasisEntry {
    entry(locus, CanonicalBasisValue::Null)
}

fn entry(locus: impl Into<String>, value: CanonicalBasisValue) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Future(APPLICATION_QUERY_DOMAIN),
        CanonicalBasisLocus::Named(locus.into().into()),
        CanonicalBasisEntryKind::Field,
        value,
    )
}
