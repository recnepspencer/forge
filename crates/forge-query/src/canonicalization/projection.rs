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
            aspect: entry.aspect_name().clone(),
            field: entry.field_name().clone(),
        };
        if !seen.insert(canonical.clone()) {
            duplicate_projection_entries
                .push((canonical.aspect.to_string(), canonical.field.to_string()));
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
                aspect: canonical.aspect.to_string(),
                field: canonical.field.to_string(),
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
