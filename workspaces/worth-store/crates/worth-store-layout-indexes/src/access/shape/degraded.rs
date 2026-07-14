use super::contract::AccessShapeContract;
use super::denial::AccessShapeUnsupportedDenial;
use super::detail::{AccessShapeDetail, DegradedExactScanBasis};
use super::lane::AccessLaneClassification;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DegradedExactScanRequest {
    budget_rows: u64,
}

impl DegradedExactScanRequest {
    pub const fn new() -> Self {
        Self { budget_rows: 0 }
    }

    pub const fn with_budget_rows(mut self, budget_rows: u64) -> Self {
        self.budget_rows = budget_rows;
        self
    }

    pub const fn budget_rows(self) -> u64 {
        self.budget_rows
    }
}

impl Default for DegradedExactScanRequest {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) fn explicit_degraded_exact_scan(
    request: DegradedExactScanRequest,
) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
    if request.budget_rows() == 0 {
        return Err(AccessShapeUnsupportedDenial::DegradedExactScanBudgetRequired);
    }

    Ok(AccessShapeContract::explicit_degraded_exact_scan(
        AccessShapeDetail::DegradedExactScan(
            DegradedExactScanBasis::BudgetedCounterBoundedTraversal,
        ),
        AccessLaneClassification::Terminal,
        request.budget_rows(),
    ))
}
