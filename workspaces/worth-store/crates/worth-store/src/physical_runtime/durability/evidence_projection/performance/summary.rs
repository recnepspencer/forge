use worth_store_aspect_native::StorePhysicalBoundaryWitness;

use super::{PhysicalDurabilityPerformanceClaim, PhysicalDurabilityPerformanceContract};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalDurabilityPerformanceSummary {
    witness: StorePhysicalBoundaryWitness,
    contracts: [PhysicalDurabilityPerformanceContract; 5],
}

impl PhysicalDurabilityPerformanceSummary {
    pub(in crate::physical_runtime) fn from_observed_contracts(
        witness: StorePhysicalBoundaryWitness,
        contracts: [PhysicalDurabilityPerformanceContract; 5],
    ) -> Self {
        let expected = [
            PhysicalDurabilityPerformanceClaim::GroupCommitAmplification,
            PhysicalDurabilityPerformanceClaim::CheckpointBoundedness,
            PhysicalDurabilityPerformanceClaim::PageBasisBoundedness,
            PhysicalDurabilityPerformanceClaim::IdempotencyRetention,
            PhysicalDurabilityPerformanceClaim::TerminalCloseout,
        ];
        assert!(
            contracts
                .iter()
                .copied()
                .map(PhysicalDurabilityPerformanceContract::claim)
                .eq(expected),
            "closeout performance contracts must cover every governed claim exactly once"
        );
        Self { witness, contracts }
    }

    pub const fn physical_witness(self) -> StorePhysicalBoundaryWitness {
        self.witness
    }

    pub fn observed(
        self,
        claim: PhysicalDurabilityPerformanceClaim,
    ) -> PhysicalDurabilityPerformanceContract {
        self.contracts
            .into_iter()
            .find(|contract| contract.claim() == claim)
            .expect("a closeout summary covers every governed claim")
    }

    pub const fn contracts(self) -> [PhysicalDurabilityPerformanceContract; 5] {
        self.contracts
    }
}
