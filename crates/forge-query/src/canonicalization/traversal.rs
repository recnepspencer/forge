use std::collections::BTreeSet;

use crate::authoring::TraversalSelector;
use crate::diagnostics::{CanonicalizationCounters, CanonicalizationWarning, NormalizationEvent};

use super::artifacts::CanonicalTraversalEntry;

pub(super) fn canonicalize_traversal(
    traversal: &[TraversalSelector],
    warnings: &mut Vec<CanonicalizationWarning>,
    events: &mut Vec<NormalizationEvent>,
    counters: &mut CanonicalizationCounters,
) -> Vec<CanonicalTraversalEntry> {
    let mut seen = BTreeSet::new();
    let mut ordered = Vec::new();
    let mut duplicate_traversal_entries = Vec::new();
    for entry in traversal {
        let canonical = CanonicalTraversalEntry {
            relation: entry.relation_name().clone(),
            depth: entry.depth(),
        };
        if !seen.insert(canonical.clone()) {
            duplicate_traversal_entries.push((canonical.relation.to_string(), canonical.depth));
            counters.query_deduplication_count += 1;
            continue;
        }
        ordered.push(canonical);
    }
    ordered.sort();
    duplicate_traversal_entries.sort();
    events.extend(
        ordered
            .iter()
            .map(|canonical| NormalizationEvent::TraversalRetained {
                relation: canonical.relation.to_string(),
                depth: canonical.depth,
            }),
    );
    for (relation, depth) in duplicate_traversal_entries {
        warnings.push(CanonicalizationWarning::DuplicateTraversalCollapsed {
            relation: relation.clone(),
            depth,
        });
        events.push(NormalizationEvent::TraversalCollapsedDuplicate { relation, depth });
    }
    ordered
}
