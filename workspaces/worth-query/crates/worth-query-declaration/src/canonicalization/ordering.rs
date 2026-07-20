use crate::authoring::OrderingSelector;

use super::artifacts::CanonicalOrderingEntry;

pub(super) fn canonicalize_ordering(ordering: &[OrderingSelector]) -> Vec<CanonicalOrderingEntry> {
    let mut normalized: Vec<_> = ordering
        .iter()
        .map(|entry| CanonicalOrderingEntry {
            field: entry.source_field_key().clone(),
            direction: entry.direction(),
        })
        .collect();
    normalized.sort();
    normalized.dedup();
    normalized
}
