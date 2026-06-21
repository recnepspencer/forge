use std::collections::BTreeSet;

use super::BasisDigestPart;

pub(super) fn canonical_digest_parts<T>(values: &[T]) -> Vec<String>
where
    T: BasisDigestPart,
{
    values
        .iter()
        .map(BasisDigestPart::digest_part)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn canonical_values<T>(values: Vec<T>) -> Vec<T>
where
    T: Ord,
{
    values
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
