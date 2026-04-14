use crate::authoring::OrderingSelector;

use super::artifacts::CanonicalOrderingEntry;

pub(super) fn canonicalize_ordering(ordering: &[OrderingSelector]) -> Vec<CanonicalOrderingEntry> {
    let mut normalized: Vec<_> = ordering
        .iter()
        .map(|entry| CanonicalOrderingEntry {
            aspect: entry.aspect_name().clone(),
            field: entry.field_name().clone(),
            direction: entry.direction(),
        })
        .collect();
    normalized.sort();
    normalized.dedup();
    normalized
}
