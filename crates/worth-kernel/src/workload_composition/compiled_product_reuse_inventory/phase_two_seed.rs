use super::classification::CompiledProductReuseSemanticCategory;
use super::report::CompiledProductReuseInventoryReport;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompiledProductReusePhaseTwoSeed {
    inventory_digest: String,
    ordinary_surface_count: usize,
    covered_category_count: usize,
}

impl CompiledProductReusePhaseTwoSeed {
    pub(crate) fn from_inventory(report: &CompiledProductReuseInventoryReport) -> Self {
        let ordinary_surface_count = report.ordinary_rows().count();
        let covered_category_count = CompiledProductReuseSemanticCategory::REQUIRED_COVERED.len();
        let inventory_digest = format!(
            "compiled-product-reuse-phase-two:{}:{}:{}",
            report.rows().len(),
            ordinary_surface_count,
            covered_category_count
        );
        Self {
            inventory_digest,
            ordinary_surface_count,
            covered_category_count,
        }
    }

    pub fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }
}
