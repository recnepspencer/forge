use super::contract::S8AccessShapeContract;
use super::denial::S8AccessShapeUnsupportedDenial;
use super::detail::{S8AccessShapeDetail, S8DegradedExactScanBasis};
use super::lane::S8AccessLaneClassification;
use crate::materialization::S8LayoutCoverageWitness;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8DegradedExactScanRequest {
    coverage: S8LayoutCoverageWitness,
    budget_rows: u64,
}

impl S8DegradedExactScanRequest {
    pub const fn new(coverage: S8LayoutCoverageWitness) -> Self {
        Self {
            coverage,
            budget_rows: 0,
        }
    }

    pub const fn with_budget_rows(mut self, budget_rows: u64) -> Self {
        self.budget_rows = budget_rows;
        self
    }

    pub const fn coverage(self) -> S8LayoutCoverageWitness {
        self.coverage
    }

    pub const fn budget_rows(self) -> u64 {
        self.budget_rows
    }
}

pub(crate) fn explicit_degraded_exact_scan(
    request: S8DegradedExactScanRequest,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    if request.budget_rows() == 0 {
        return Err(S8AccessShapeUnsupportedDenial::DegradedExactScanBudgetRequired);
    }

    Ok(S8AccessShapeContract::explicit_degraded_exact_scan(
        S8AccessShapeDetail::DegradedExactScan(
            S8DegradedExactScanBasis::BudgetedCounterBoundedTraversal,
        ),
        S8AccessLaneClassification::Terminal,
        request
            .coverage()
            .require_exact()
            .map_err(S8AccessShapeUnsupportedDenial::MaterializationDenied)?,
        request.budget_rows(),
    ))
}
