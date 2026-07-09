use crate::canonicalization::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisValue, CanonicalIntegerWidth,
};
use crate::identities::CanonicalDigestId;

pub(super) fn text_entry(locus: &str, value: &str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Transition,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::TransitionArtifact,
        CanonicalBasisValue::ExactText(value.to_string().into()),
    )
}

pub(super) fn bool_entry(locus: &str, value: bool) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Transition,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::TransitionArtifact,
        CanonicalBasisValue::Bool(value),
    )
}

pub(super) fn u64_entry(locus: &str, value: u64) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Transition,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::TransitionArtifact,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: u128::from(value),
        },
    )
}

pub(super) fn digest_entry(locus: &str, value: CanonicalDigestId) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Transition,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::TransitionArtifact,
        CanonicalBasisValue::BytesDigest(value),
    )
}
