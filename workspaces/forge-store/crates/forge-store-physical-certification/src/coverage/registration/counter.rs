use super::Roadmap2CoverageRegistry;
use crate::PhysicalCounterEvidenceReceipt;

use super::super::{
    CoverageGapDenial, CoverageRowDimension, CoverageSurfaceKind, PhysicalCoverageMatrixRow,
};

impl Roadmap2CoverageRegistry {
    pub fn register_counter_receipt(
        mut self,
        receipt: &PhysicalCounterEvidenceReceipt,
    ) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::Counter)?;
        if receipt.rows().is_empty() {
            return Err(CoverageGapDenial::EmptyCounterReceiptRegistration);
        }
        let plan =
            self.plan
                .as_ref()
                .ok_or(CoverageGapDenial::MissingPlanBeforeDependentSurface {
                    surface: CoverageSurfaceKind::Counter,
                })?;
        if receipt.plan_identity() != plan.identity() {
            return Err(CoverageGapDenial::CounterReceiptPlanMismatch);
        }
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::Counter,
            *plan.identity().digest_bytes(),
            plan.counter_contracts()
                .iter()
                .map(|contract| CoverageRowDimension::CounterContract(contract.kind())),
        ));
        Ok(self)
    }

    pub fn register_counter_contracts_from_plan(mut self) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::Counter)?;
        let plan =
            self.plan
                .as_ref()
                .ok_or(CoverageGapDenial::MissingPlanBeforeDependentSurface {
                    surface: CoverageSurfaceKind::Counter,
                })?;
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::Counter,
            *plan.identity().digest_bytes(),
            plan.counter_contracts()
                .iter()
                .map(|contract| CoverageRowDimension::CounterContract(contract.kind())),
        ));
        Ok(self)
    }
}
