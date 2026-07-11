use crate::{AcceptedPhysicalIsolationHarnessReadiness, GeneratedCoverageMatrix};

use super::{
    lanes_from_closeout_evidence, required_simulation_harness_lanes,
    SimulationHarnessAcceptanceSuiteMap,
};
use super::{
    FutureHarnessExtensionSlotInventory, SimulationHarnessDogfoodEvidence,
    SimulationHarnessDogfoodReport,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationHarnessCloseoutCoverageReport {
    recovery_matrix: GeneratedCoverageMatrix,
    shortcut_rejection_matrix: GeneratedCoverageMatrix,
    physical_isolation_readiness_shape_probe_matrix: GeneratedCoverageMatrix,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSimulationHarnessCloseoutReport {
    dogfood: SimulationHarnessDogfoodReport,
    dogfood_evidence: SimulationHarnessDogfoodEvidence,
    coverage: SimulationHarnessCloseoutCoverageReport,
    acceptance: SimulationHarnessAcceptanceSuiteMap,
    physical_isolation_readiness: AcceptedPhysicalIsolationHarnessReadiness,
    future_extension_slots: FutureHarnessExtensionSlotInventory,
    shortcut_denial_count: usize,
}

impl SimulationHarnessCloseoutCoverageReport {
    pub fn from_dogfood_evidence(dogfood_evidence: &SimulationHarnessDogfoodEvidence) -> Self {
        Self {
            recovery_matrix: dogfood_evidence.recovery().coverage().clone(),
            shortcut_rejection_matrix: dogfood_evidence.shortcut_rejection().coverage().clone(),
            physical_isolation_readiness_shape_probe_matrix: dogfood_evidence
                .physical_isolation_readiness_shape_probe()
                .coverage()
                .clone(),
        }
    }

    pub const fn recovery_matrix(&self) -> &GeneratedCoverageMatrix {
        &self.recovery_matrix
    }

    pub const fn shortcut_rejection_matrix(&self) -> &GeneratedCoverageMatrix {
        &self.shortcut_rejection_matrix
    }

    pub const fn physical_isolation_readiness_shape_probe_matrix(
        &self,
    ) -> &GeneratedCoverageMatrix {
        &self.physical_isolation_readiness_shape_probe_matrix
    }

    pub fn all_required_simulation_harness_lanes_are_satisfied(&self) -> bool {
        required_simulation_harness_lanes()
            .into_iter()
            .all(|lane| lanes_from_closeout_evidence(self).contains(&lane))
    }

    pub(crate) fn matrices(&self) -> [&GeneratedCoverageMatrix; 3] {
        [
            &self.recovery_matrix,
            &self.shortcut_rejection_matrix,
            &self.physical_isolation_readiness_shape_probe_matrix,
        ]
    }
}

impl PhysicalSimulationHarnessCloseoutReport {
    pub(crate) const fn new(
        dogfood: SimulationHarnessDogfoodReport,
        dogfood_evidence: SimulationHarnessDogfoodEvidence,
        coverage: SimulationHarnessCloseoutCoverageReport,
        acceptance: SimulationHarnessAcceptanceSuiteMap,
        physical_isolation_readiness: AcceptedPhysicalIsolationHarnessReadiness,
        future_extension_slots: FutureHarnessExtensionSlotInventory,
        shortcut_denial_count: usize,
    ) -> Self {
        Self {
            dogfood,
            dogfood_evidence,
            coverage,
            acceptance,
            physical_isolation_readiness,
            future_extension_slots,
            shortcut_denial_count,
        }
    }

    pub const fn dogfood(&self) -> &SimulationHarnessDogfoodReport {
        &self.dogfood
    }

    pub const fn dogfood_evidence(&self) -> &SimulationHarnessDogfoodEvidence {
        &self.dogfood_evidence
    }

    pub const fn coverage(&self) -> &SimulationHarnessCloseoutCoverageReport {
        &self.coverage
    }

    pub const fn acceptance(&self) -> &SimulationHarnessAcceptanceSuiteMap {
        &self.acceptance
    }

    pub const fn physical_isolation_readiness(&self) -> &AcceptedPhysicalIsolationHarnessReadiness {
        &self.physical_isolation_readiness
    }

    pub const fn future_extension_slots(&self) -> &FutureHarnessExtensionSlotInventory {
        &self.future_extension_slots
    }

    pub const fn shortcut_denial_count(&self) -> usize {
        self.shortcut_denial_count
    }
}
