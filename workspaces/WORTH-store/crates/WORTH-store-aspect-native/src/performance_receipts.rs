use worth_foundational::{
    FoundationalCounterBackedPerformanceReceipt, FoundationalPerformanceClaimSurface,
};

use crate::StorePhysicalBoundaryWitness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorePerformanceReceiptEvidence<Claim>
where
    Claim: FoundationalPerformanceClaimSurface,
{
    receipt: FoundationalCounterBackedPerformanceReceipt<Claim>,
    physical_witness: StorePhysicalBoundaryWitness,
}

impl<Claim> StorePerformanceReceiptEvidence<Claim>
where
    Claim: FoundationalPerformanceClaimSurface,
{
    pub const fn new(
        receipt: FoundationalCounterBackedPerformanceReceipt<Claim>,
        physical_witness: StorePhysicalBoundaryWitness,
    ) -> Self {
        Self {
            receipt,
            physical_witness,
        }
    }

    pub const fn receipt(&self) -> &FoundationalCounterBackedPerformanceReceipt<Claim> {
        &self.receipt
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.physical_witness
    }
}
