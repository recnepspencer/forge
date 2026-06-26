use std::collections::BTreeMap;

use crate::validator_invariant_catalog::milestone_nine_closeout::WorthTopologyMilestoneNineDeletionLedgerReport;

pub(in crate::validator_invariant_catalog::milestone_nine_closeout::authority_occurrence_inventory) fn observed_counts_by_source_path(
    sources: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    forbidden_patterns: &'static [&'static str],
) -> (Vec<String>, BTreeMap<String, BTreeMap<String, usize>>) {
    let mut scanned_source_paths = Vec::new();
    let mut observed_counts = BTreeMap::<String, BTreeMap<String, usize>>::new();
    for (path, source) in sources {
        let path = path.into();
        let source = source.into();
        scanned_source_paths.push(path.clone());
        let pattern_counts = observed_counts.entry(path).or_default();
        for pattern in forbidden_patterns {
            let count = source.match_indices(pattern).count();
            if count > 0 {
                pattern_counts.insert((*pattern).to_string(), count);
            }
        }
    }
    (scanned_source_paths, observed_counts)
}

pub(in crate::validator_invariant_catalog::milestone_nine_closeout::authority_occurrence_inventory) fn allowed_counts_by_source_path(
    deletion_ledger: &WorthTopologyMilestoneNineDeletionLedgerReport,
) -> BTreeMap<String, BTreeMap<String, usize>> {
    let mut allowed_counts = BTreeMap::<String, BTreeMap<String, usize>>::new();
    for row in deletion_ledger.rows() {
        let pattern_counts = allowed_counts
            .entry(row.source_path().to_string())
            .or_default();
        for (pattern, count) in row.allowed_forbidden_pattern_hits() {
            pattern_counts.insert(pattern.clone(), *count);
        }
    }
    allowed_counts
}
