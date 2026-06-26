use serde::Serialize;

use super::classification::{
    DerivedInvalidationAuthorityDisposition, DerivedInvalidationProductCategory,
};
use super::row::{digest_strings, DerivedInvalidationAuthorityInventoryRow};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationAuthorityInventoryCounters {
    row_count: usize,
    migrate_count: usize,
    delete_count: usize,
    certification_bootstrap_residue_count: usize,
    true_query_gap_count: usize,
    ordinary_path_count: usize,
    capped_residue_count: usize,
}

impl DerivedInvalidationAuthorityInventoryCounters {
    pub(crate) fn from_rows(rows: &[DerivedInvalidationAuthorityInventoryRow]) -> Self {
        Self {
            row_count: rows.len(),
            migrate_count: disposition_count(
                rows,
                DerivedInvalidationAuthorityDisposition::Migrate,
            ),
            delete_count: disposition_count(rows, DerivedInvalidationAuthorityDisposition::Delete),
            certification_bootstrap_residue_count: disposition_count(
                rows,
                DerivedInvalidationAuthorityDisposition::CertificationBootstrapResidue,
            ),
            true_query_gap_count: disposition_count(
                rows,
                DerivedInvalidationAuthorityDisposition::TrueQueryCapabilityGap,
            ),
            ordinary_path_count: rows.iter().filter(|row| row.ordinary_path()).count(),
            capped_residue_count: rows.iter().filter(|row| row.cap().is_some()).count(),
        }
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn migrate_count(&self) -> usize {
        self.migrate_count
    }

    pub fn delete_count(&self) -> usize {
        self.delete_count
    }

    pub fn certification_bootstrap_residue_count(&self) -> usize {
        self.certification_bootstrap_residue_count
    }

    pub fn true_query_gap_count(&self) -> usize {
        self.true_query_gap_count
    }

    pub fn ordinary_path_count(&self) -> usize {
        self.ordinary_path_count
    }

    pub fn capped_residue_count(&self) -> usize {
        self.capped_residue_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationAuthorityInventoryReport {
    rows: Vec<DerivedInvalidationAuthorityInventoryRow>,
    counters: DerivedInvalidationAuthorityInventoryCounters,
    required_ordinary_categories: Vec<DerivedInvalidationProductCategory>,
    report_digest: String,
}

impl DerivedInvalidationAuthorityInventoryReport {
    pub(crate) fn new(rows: Vec<DerivedInvalidationAuthorityInventoryRow>) -> Self {
        let counters = DerivedInvalidationAuthorityInventoryCounters::from_rows(&rows);
        let required_ordinary_categories =
            DerivedInvalidationProductCategory::COVERED_ORDINARY.to_vec();
        let report_digest = report_digest(&rows, &required_ordinary_categories);
        Self {
            rows,
            counters,
            required_ordinary_categories,
            report_digest,
        }
    }

    pub fn rows(&self) -> &[DerivedInvalidationAuthorityInventoryRow] {
        &self.rows
    }

    pub fn counters(&self) -> &DerivedInvalidationAuthorityInventoryCounters {
        &self.counters
    }

    pub fn required_ordinary_categories(&self) -> &[DerivedInvalidationProductCategory] {
        &self.required_ordinary_categories
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

fn disposition_count(
    rows: &[DerivedInvalidationAuthorityInventoryRow],
    disposition: DerivedInvalidationAuthorityDisposition,
) -> usize {
    rows.iter()
        .filter(|row| row.disposition() == disposition)
        .count()
}

fn report_digest(
    rows: &[DerivedInvalidationAuthorityInventoryRow],
    required_categories: &[DerivedInvalidationProductCategory],
) -> String {
    let mut parts = rows
        .iter()
        .map(|row| format!("row:{}", row.row_digest()))
        .collect::<Vec<_>>();
    parts.extend(
        required_categories
            .iter()
            .map(|category| format!("required:{}", category.as_str())),
    );
    digest_strings(parts)
}
