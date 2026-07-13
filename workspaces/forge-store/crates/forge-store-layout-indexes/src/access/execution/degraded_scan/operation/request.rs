use forge_store_budgets::PreExecutionBudgetEnvelope;
use forge_store_physical_format::{PhysicalPageId, PhysicalSegmentId};
use forge_store_security::StoreCurrentSecurityScopeWitnessSet;

#[derive(Debug)]
pub struct DegradedExactScanExecutionRequest<'a> {
    pub(super) catalog: &'a crate::BootstrapCatalogReadAdmission,
    pub(super) security: &'a StoreCurrentSecurityScopeWitnessSet,
    pub(super) segment: PhysicalSegmentId,
    pub(super) page: PhysicalPageId,
    pub(super) budget_rows: u64,
    pub(super) budget: PreExecutionBudgetEnvelope,
}

impl<'a> DegradedExactScanExecutionRequest<'a> {
    pub const fn new(
        catalog: &'a crate::BootstrapCatalogReadAdmission,
        security: &'a StoreCurrentSecurityScopeWitnessSet,
        segment: PhysicalSegmentId,
        page: PhysicalPageId,
        budget_rows: u64,
        budget: PreExecutionBudgetEnvelope,
    ) -> Self {
        Self {
            catalog,
            security,
            segment,
            page,
            budget_rows,
            budget,
        }
    }
}
