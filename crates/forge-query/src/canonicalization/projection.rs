use std::collections::BTreeSet;

use crate::authoring::AspectFieldSelector;
use crate::diagnostics::{CanonicalizationCounters, CanonicalizationWarning, NormalizationEvent};

use super::artifacts::CanonicalProjectionEntry;

pub(super) fn canonicalize_projection(
    projection: &[AspectFieldSelector],
    warnings: &mut Vec<CanonicalizationWarning>,
    events: &mut Vec<NormalizationEvent>,
    counters: &mut CanonicalizationCounters,
) -> Vec<CanonicalProjectionEntry> {
    let mut seen = BTreeSet::new();
    let mut ordered = Vec::new();
    let mut duplicate_projection_entries = Vec::new();
    for entry in projection {
        let canonical = CanonicalProjectionEntry {
            field: entry.source_field_key().clone(),
        };
        if !seen.insert(canonical.clone()) {
            duplicate_projection_entries.push(terminal_field_pair(canonical.field_key()));
            counters.query_deduplication_count += 1;
            continue;
        }
        ordered.push(canonical);
    }
    ordered.sort();
    duplicate_projection_entries.sort();
    events.extend(
        ordered
            .iter()
            .map(|canonical| NormalizationEvent::ProjectionRetained {
                aspect: canonical.field_key().aspect().to_string(),
                field: canonical.field_key().field().to_string(),
            }),
    );
    for (aspect, field) in duplicate_projection_entries {
        warnings.push(CanonicalizationWarning::DuplicateProjectionCollapsed {
            aspect: aspect.clone(),
            field: field.clone(),
        });
        events.push(NormalizationEvent::ProjectionCollapsedDuplicate { aspect, field });
    }
    ordered
}

fn terminal_field_pair(field: &crate::authoring::AspectFieldKey) -> (String, String) {
    (field.aspect().to_string(), field.field().to_string())
}
