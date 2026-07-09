use crate::canonicalization::export::bundle::CanonicalExportBasisBundle;
use crate::canonicalization::mismatch::CanonicalExportMismatchContext;
use crate::canonicalization::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalEquivalenceBasis, CanonicalMismatchBasis,
    CanonicalMismatchKind, CanonicalizationRuleVersion,
};

pub(super) fn first_bundle_mismatch(
    left: &CanonicalExportBasisBundle,
    right: &CanonicalExportBasisBundle,
) -> Option<CanonicalMismatchBasis> {
    let mut left_sequences = left.sequences().iter();
    let mut right_sequences = right.sequences().iter();

    loop {
        match (left_sequences.next(), right_sequences.next()) {
            (Some(left_sequence), Some(right_sequence)) => {
                if let Some(mismatch) = first_sequence_mismatch(
                    left.version().clone(),
                    right.version().clone(),
                    left_sequence.domain(),
                    right_sequence.domain(),
                    left_sequence.entries(),
                    right_sequence.entries(),
                ) {
                    return Some(mismatch);
                }
            }
            (Some(left_sequence), None) => {
                return Some(CanonicalMismatchBasis::from_export_entries(
                    CanonicalMismatchKind::AdditionalEntry,
                    CanonicalExportMismatchContext::new(
                        CanonicalEquivalenceBasis::ExactCanonicalBasis,
                        left.version().clone(),
                        right.version().clone(),
                        left_sequence.domain(),
                        left_sequence.domain(),
                    ),
                    left_sequence.entries().first(),
                    None,
                ));
            }
            (None, Some(right_sequence)) => {
                return Some(CanonicalMismatchBasis::from_export_entries(
                    CanonicalMismatchKind::MissingEntry,
                    CanonicalExportMismatchContext::new(
                        CanonicalEquivalenceBasis::ExactCanonicalBasis,
                        left.version().clone(),
                        right.version().clone(),
                        right_sequence.domain(),
                        right_sequence.domain(),
                    ),
                    None,
                    right_sequence.entries().first(),
                ));
            }
            (None, None) => return None,
        }
    }
}

fn first_sequence_mismatch(
    left_version: CanonicalizationRuleVersion,
    right_version: CanonicalizationRuleVersion,
    left_domain: CanonicalBasisDomain,
    right_domain: CanonicalBasisDomain,
    left_entries: &[CanonicalBasisEntry],
    right_entries: &[CanonicalBasisEntry],
) -> Option<CanonicalMismatchBasis> {
    let mut left_iter = left_entries.iter().peekable();
    let mut right_iter = right_entries.iter().peekable();

    loop {
        match (left_iter.peek(), right_iter.peek()) {
            (Some(left), Some(right))
                if same_entry_locus(left, right) && left.kind() != right.kind() =>
            {
                return Some(export_entry_mismatch(
                    CanonicalMismatchKind::EntryKindMismatch,
                    left_version,
                    right_version,
                    left_domain,
                    right_domain,
                    left_iter.next(),
                    right_iter.next(),
                ));
            }
            (Some(left), Some(right)) => match compare_entry_keys(left, right) {
                std::cmp::Ordering::Equal => {
                    let left = left_iter.next().expect("peeked left");
                    let right = right_iter.next().expect("peeked right");
                    if left.value() != right.value() {
                        return Some(export_entry_mismatch(
                            CanonicalMismatchKind::ValueMismatch,
                            left_version,
                            right_version,
                            left_domain,
                            right_domain,
                            Some(left),
                            Some(right),
                        ));
                    }
                }
                std::cmp::Ordering::Less => {
                    return Some(export_entry_mismatch(
                        CanonicalMismatchKind::AdditionalEntry,
                        left_version,
                        right_version,
                        left_domain,
                        right_domain,
                        left_iter.next(),
                        None,
                    ));
                }
                std::cmp::Ordering::Greater => {
                    return Some(export_entry_mismatch(
                        CanonicalMismatchKind::MissingEntry,
                        left_version,
                        right_version,
                        left_domain,
                        right_domain,
                        None,
                        right_iter.next(),
                    ));
                }
            },
            (Some(_), None) => {
                return Some(export_entry_mismatch(
                    CanonicalMismatchKind::AdditionalEntry,
                    left_version,
                    right_version,
                    left_domain,
                    right_domain,
                    left_iter.next(),
                    None,
                ));
            }
            (None, Some(_)) => {
                return Some(export_entry_mismatch(
                    CanonicalMismatchKind::MissingEntry,
                    left_version,
                    right_version,
                    left_domain,
                    right_domain,
                    None,
                    right_iter.next(),
                ));
            }
            (None, None) => return None,
        }
    }
}

fn export_entry_mismatch(
    kind: CanonicalMismatchKind,
    left_version: CanonicalizationRuleVersion,
    right_version: CanonicalizationRuleVersion,
    left_domain: CanonicalBasisDomain,
    right_domain: CanonicalBasisDomain,
    left: Option<&CanonicalBasisEntry>,
    right: Option<&CanonicalBasisEntry>,
) -> CanonicalMismatchBasis {
    CanonicalMismatchBasis::from_export_entries(
        kind,
        CanonicalExportMismatchContext::new(
            CanonicalEquivalenceBasis::ExactCanonicalBasis,
            left_version,
            right_version,
            left_domain,
            right_domain,
        ),
        left,
        right,
    )
}

fn same_entry_locus(left: &CanonicalBasisEntry, right: &CanonicalBasisEntry) -> bool {
    left.domain() == right.domain() && left.locus() == right.locus()
}

fn compare_entry_keys(
    left: &CanonicalBasisEntry,
    right: &CanonicalBasisEntry,
) -> std::cmp::Ordering {
    (left.domain(), left.locus(), left.kind()).cmp(&(right.domain(), right.locus(), right.kind()))
}
