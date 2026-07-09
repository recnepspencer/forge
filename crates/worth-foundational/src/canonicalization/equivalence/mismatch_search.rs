use super::readiness::CanonicalComparisonInput;
use crate::canonicalization::{CanonicalBasisEntry, CanonicalMismatchBasis, CanonicalMismatchKind};

pub(super) fn first_mismatch(input: &CanonicalComparisonInput) -> Option<CanonicalMismatchBasis> {
    let mut left_iter = input.left().payload().entries().iter().peekable();
    let mut right_iter = input.right().payload().entries().iter().peekable();

    loop {
        match (left_iter.peek(), right_iter.peek()) {
            (Some(left), Some(right))
                if same_entry_locus(left, right) && left.kind() != right.kind() =>
            {
                return Some(CanonicalMismatchBasis::from_input(
                    input,
                    CanonicalMismatchKind::EntryKindMismatch,
                    left_iter.next(),
                    right_iter.next(),
                ));
            }
            (Some(left), Some(right)) => match compare_entry_keys(left, right) {
                std::cmp::Ordering::Equal => {
                    let left = left_iter.next().expect("peeked left");
                    let right = right_iter.next().expect("peeked right");
                    if left.value() != right.value() {
                        return Some(CanonicalMismatchBasis::from_input(
                            input,
                            CanonicalMismatchKind::ValueMismatch,
                            Some(left),
                            Some(right),
                        ));
                    }
                }
                std::cmp::Ordering::Less => {
                    return Some(CanonicalMismatchBasis::from_input(
                        input,
                        CanonicalMismatchKind::AdditionalEntry,
                        left_iter.next(),
                        None,
                    ));
                }
                std::cmp::Ordering::Greater => {
                    return Some(CanonicalMismatchBasis::from_input(
                        input,
                        CanonicalMismatchKind::MissingEntry,
                        None,
                        right_iter.next(),
                    ));
                }
            },
            (Some(_), None) => {
                return Some(CanonicalMismatchBasis::from_input(
                    input,
                    CanonicalMismatchKind::AdditionalEntry,
                    left_iter.next(),
                    None,
                ));
            }
            (None, Some(_)) => {
                return Some(CanonicalMismatchBasis::from_input(
                    input,
                    CanonicalMismatchKind::MissingEntry,
                    None,
                    right_iter.next(),
                ));
            }
            (None, None) => return None,
        }
    }
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
