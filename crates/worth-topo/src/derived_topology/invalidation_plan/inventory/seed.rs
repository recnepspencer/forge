use serde::Serialize;

use super::report::DerivedInvalidationAuthorityInventoryReport;
use super::row::digest_strings;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationPhaseTwoSeed {
    inventory_digest: String,
    required_product_category_count: usize,
    old_authority_row_count: usize,
    capped_residue_count: usize,
    seed_digest: String,
}

impl DerivedInvalidationPhaseTwoSeed {
    pub(crate) fn from_inventory_report(
        report: &DerivedInvalidationAuthorityInventoryReport,
    ) -> Self {
        let inventory_digest = report.report_digest().to_string();
        let required_product_category_count = report.required_ordinary_categories().len();
        let old_authority_row_count = report.counters().row_count();
        let capped_residue_count = report.counters().certification_bootstrap_residue_count();
        let seed_digest = digest_strings(vec![
            inventory_digest.clone(),
            format!("required:{required_product_category_count}"),
            format!("old_authority:{old_authority_row_count}"),
            format!("capped_residue:{capped_residue_count}"),
        ]);
        Self {
            inventory_digest,
            required_product_category_count,
            old_authority_row_count,
            capped_residue_count,
            seed_digest,
        }
    }

    pub fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }

    pub const fn capped_residue_count(&self) -> usize {
        self.capped_residue_count
    }
}
