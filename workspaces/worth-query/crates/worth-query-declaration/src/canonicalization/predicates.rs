use crate::authoring::PredicateSelector;

use super::artifacts::CanonicalPredicateEntry;

pub(super) fn canonicalize_predicates(
    predicates: &[PredicateSelector],
) -> Vec<CanonicalPredicateEntry> {
    let mut normalized: Vec<_> = predicates
        .iter()
        .map(CanonicalPredicateEntry::from_authored)
        .collect();
    normalized.sort();
    normalized.dedup();
    normalized
}
