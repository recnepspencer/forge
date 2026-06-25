use std::collections::BTreeMap;

use super::inventory_row::WorthTopologyQueryNativeRuntimeBoundaryInventoryRow;
use super::residue_status::WorthTopologyQueryNativeRuntimeBoundaryResidueStatus;
use super::source_scan::current_stale_symbol_rows;
use super::stale_symbol::WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthTopologyQueryNativeRuntimeBoundaryInventoryError {
    SourceScanFailed(String),
    UnclassifiedResidue {
        source_path: String,
        stale_symbol: WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol,
    },
    MissingOwner {
        source_path: String,
    },
    MissingRemovalTrigger {
        source_path: String,
    },
    CompatibilityShimResidue {
        source_path: String,
        stale_symbol: WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyQueryNativeRuntimeBoundaryInventory {
    rows: Vec<WorthTopologyQueryNativeRuntimeBoundaryInventoryRow>,
    report_digest: String,
}

impl WorthTopologyQueryNativeRuntimeBoundaryInventory {
    pub fn from_current_sources(
    ) -> Result<Self, WorthTopologyQueryNativeRuntimeBoundaryInventoryError> {
        let rows = current_stale_symbol_rows()
            .map_err(WorthTopologyQueryNativeRuntimeBoundaryInventoryError::SourceScanFailed)?;
        Self::from_rows_for_validation(rows)
    }

    pub(crate) fn from_rows_for_validation(
        rows: Vec<WorthTopologyQueryNativeRuntimeBoundaryInventoryRow>,
    ) -> Result<Self, WorthTopologyQueryNativeRuntimeBoundaryInventoryError> {
        validate_rows(&rows)?;
        let report_digest = report_digest_for_rows(&rows);
        Ok(Self {
            rows,
            report_digest,
        })
    }

    pub fn rows(&self) -> &[WorthTopologyQueryNativeRuntimeBoundaryInventoryRow] {
        &self.rows
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub fn total_observed_occurrence_count(&self) -> usize {
        self.rows().iter().map(|row| row.observed_count()).sum()
    }

    pub fn unclassified_count(&self) -> usize {
        self.rows()
            .iter()
            .filter(|row| !row.status().is_classified())
            .count()
    }

    pub fn ordinary_runtime_migration_row_count(&self) -> usize {
        self.rows()
            .iter()
            .filter(|row| row.status().is_ordinary_runtime_migration())
            .count()
    }

    pub fn row_count_by_status(
        &self,
    ) -> BTreeMap<WorthTopologyQueryNativeRuntimeBoundaryResidueStatus, usize> {
        let mut counts = BTreeMap::new();
        for row in self.rows() {
            *counts.entry(row.status()).or_insert(0) += 1;
        }
        counts
    }

    pub fn row_count_by_stale_symbol(
        &self,
    ) -> BTreeMap<WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol, usize> {
        let mut counts = BTreeMap::new();
        for row in self.rows() {
            *counts.entry(row.stale_symbol()).or_insert(0) += 1;
        }
        counts
    }
}

fn validate_rows(
    rows: &[WorthTopologyQueryNativeRuntimeBoundaryInventoryRow],
) -> Result<(), WorthTopologyQueryNativeRuntimeBoundaryInventoryError> {
    for row in rows {
        if !row.status().is_classified() {
            return Err(
                WorthTopologyQueryNativeRuntimeBoundaryInventoryError::UnclassifiedResidue {
                    source_path: row.source_path().to_string(),
                    stale_symbol: row.stale_symbol(),
                },
            );
        }
        if row.owner().trim().is_empty() {
            return Err(
                WorthTopologyQueryNativeRuntimeBoundaryInventoryError::MissingOwner {
                    source_path: row.source_path().to_string(),
                },
            );
        }
        if row.removal_trigger().trim().is_empty() {
            return Err(
                WorthTopologyQueryNativeRuntimeBoundaryInventoryError::MissingRemovalTrigger {
                    source_path: row.source_path().to_string(),
                },
            );
        }
        if row.looks_like_compatibility_shim()
            && row.status()
                != WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::FirewallPatternOnly
        {
            return Err(
                WorthTopologyQueryNativeRuntimeBoundaryInventoryError::CompatibilityShimResidue {
                    source_path: row.source_path().to_string(),
                    stale_symbol: row.stale_symbol(),
                },
            );
        }
    }
    Ok(())
}

fn report_digest_for_rows(rows: &[WorthTopologyQueryNativeRuntimeBoundaryInventoryRow]) -> String {
    let mut digest_parts = vec![
        "worth-topo-query-native-runtime-boundary-inventory-v1".to_string(),
        format!("row-count:{}", rows.len()),
    ];
    digest_parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
    digest_parts.join("|")
}
