use std::collections::BTreeMap;

use crate::validator_invariant_catalog::milestone_nine_closeout::authority_occurrence_inventory::{
    allowed_counts_by_source_path, current_source_pairs, observed_counts_by_source_path,
    occurrence_status, WorthTopologyMilestoneNineAuthorityOccurrenceStatus,
    FORBIDDEN_AUTHORITY_PATTERNS,
};
use crate::validator_invariant_catalog::milestone_nine_closeout::WorthTopologyMilestoneNineDeletionLedgerReport;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyMilestoneNineAuthorityOccurrenceInventoryRow {
    source_path: String,
    forbidden_pattern: String,
    observed_count: usize,
    ledger_allowed_count: usize,
    status: WorthTopologyMilestoneNineAuthorityOccurrenceStatus,
    row_digest: String,
}

impl WorthTopologyMilestoneNineAuthorityOccurrenceInventoryRow {
    fn new(
        source_path: impl Into<String>,
        forbidden_pattern: impl Into<String>,
        observed_count: usize,
        ledger_allowed_count: usize,
    ) -> Self {
        let source_path = source_path.into();
        let forbidden_pattern = forbidden_pattern.into();
        let status = occurrence_status(observed_count, ledger_allowed_count);
        let row_digest = [
            "worth-topo-milestone-nine-authority-occurrence-row-v1",
            source_path.as_str(),
            forbidden_pattern.as_str(),
            &observed_count.to_string(),
            &ledger_allowed_count.to_string(),
            status.as_str(),
        ]
        .join("|");
        Self {
            source_path,
            forbidden_pattern,
            observed_count,
            ledger_allowed_count,
            status,
            row_digest,
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn forbidden_pattern(&self) -> &str {
        &self.forbidden_pattern
    }

    pub const fn observed_count(&self) -> usize {
        self.observed_count
    }

    pub const fn ledger_allowed_count(&self) -> usize {
        self.ledger_allowed_count
    }

    pub const fn status(&self) -> WorthTopologyMilestoneNineAuthorityOccurrenceStatus {
        self.status
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyMilestoneNineAuthorityOccurrenceInventory {
    scanned_source_paths: Vec<String>,
    rows: Vec<WorthTopologyMilestoneNineAuthorityOccurrenceInventoryRow>,
    inventory_digest: String,
}

impl WorthTopologyMilestoneNineAuthorityOccurrenceInventory {
    pub(in crate::validator_invariant_catalog) fn current_from_deletion_ledger(
        deletion_ledger: &WorthTopologyMilestoneNineDeletionLedgerReport,
    ) -> Self {
        Self::from_source_pairs_and_deletion_ledger(
            current_source_pairs(),
            deletion_ledger,
            FORBIDDEN_AUTHORITY_PATTERNS,
        )
    }

    pub(in crate::validator_invariant_catalog) fn from_source_pairs_and_deletion_ledger(
        sources: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
        deletion_ledger: &WorthTopologyMilestoneNineDeletionLedgerReport,
        forbidden_patterns: &'static [&'static str],
    ) -> Self {
        let (scanned_source_paths, observed_counts) =
            observed_counts_by_source_path(sources, forbidden_patterns);
        let allowed_counts = allowed_counts_by_source_path(deletion_ledger);
        let row_keys = authority_occurrence_row_keys(&observed_counts, &allowed_counts);
        let rows = row_keys
            .into_keys()
            .map(|(source_path, pattern)| {
                let observed_count = observed_count_for(&observed_counts, &source_path, &pattern);
                let ledger_allowed_count =
                    ledger_allowed_count_for(&allowed_counts, &source_path, &pattern);
                WorthTopologyMilestoneNineAuthorityOccurrenceInventoryRow::new(
                    source_path,
                    pattern,
                    observed_count,
                    ledger_allowed_count,
                )
            })
            .collect::<Vec<_>>();
        Self::from_rows(scanned_source_paths, rows)
    }

    fn from_rows(
        mut scanned_source_paths: Vec<String>,
        rows: impl IntoIterator<Item = WorthTopologyMilestoneNineAuthorityOccurrenceInventoryRow>,
    ) -> Self {
        scanned_source_paths.sort();
        scanned_source_paths.dedup();
        let rows = rows.into_iter().collect::<Vec<_>>();
        let mut digest_parts = vec![
            "worth-topo-milestone-nine-authority-occurrence-inventory-v1".to_string(),
            format!("scanned-source-count:{}", scanned_source_paths.len()),
            format!("row-count:{}", rows.len()),
        ];
        digest_parts.extend(
            scanned_source_paths
                .iter()
                .map(|path| format!("scanned:{path}")),
        );
        digest_parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
        Self {
            scanned_source_paths,
            rows,
            inventory_digest: digest_parts.join("|"),
        }
    }

    pub fn scanned_source_paths(&self) -> &[String] {
        &self.scanned_source_paths
    }

    pub fn rows(&self) -> &[WorthTopologyMilestoneNineAuthorityOccurrenceInventoryRow] {
        &self.rows
    }

    pub fn violation_rows(
        &self,
    ) -> Vec<&WorthTopologyMilestoneNineAuthorityOccurrenceInventoryRow> {
        self.rows
            .iter()
            .filter(|row| row.status().is_violation())
            .collect()
    }

    pub fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }
}

fn authority_occurrence_row_keys(
    observed_counts: &BTreeMap<String, BTreeMap<String, usize>>,
    allowed_counts: &BTreeMap<String, BTreeMap<String, usize>>,
) -> BTreeMap<(String, String), ()> {
    let mut row_keys = BTreeMap::<(String, String), ()>::new();
    insert_occurrence_keys(&mut row_keys, observed_counts);
    insert_occurrence_keys(&mut row_keys, allowed_counts);
    row_keys
}

fn insert_occurrence_keys(
    row_keys: &mut BTreeMap<(String, String), ()>,
    counts_by_source_path: &BTreeMap<String, BTreeMap<String, usize>>,
) {
    for (source_path, pattern_counts) in counts_by_source_path {
        for pattern in pattern_counts.keys() {
            row_keys.insert((source_path.clone(), pattern.clone()), ());
        }
    }
}

fn observed_count_for(
    observed_counts: &BTreeMap<String, BTreeMap<String, usize>>,
    source_path: &str,
    pattern: &str,
) -> usize {
    observed_counts
        .get(source_path)
        .and_then(|patterns| patterns.get(pattern))
        .copied()
        .unwrap_or(0)
}

fn ledger_allowed_count_for(
    allowed_counts: &BTreeMap<String, BTreeMap<String, usize>>,
    source_path: &str,
    pattern: &str,
) -> usize {
    allowed_counts
        .get(source_path)
        .and_then(|patterns| patterns.get(pattern))
        .copied()
        .unwrap_or(0)
}
