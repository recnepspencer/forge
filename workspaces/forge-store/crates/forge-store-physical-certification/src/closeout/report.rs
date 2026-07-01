use crate::{AcceptedS5SimulationHarnessReadiness, GeneratedCoverageMatrix};

use super::{lanes_from_closeout_evidence, required_s45_lanes, S45AcceptanceSuiteMap};
use super::{
    FutureHarnessExtensionSlotInventory, S45HarnessDogfoodEvidence, S45HarnessDogfoodReport,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S45CloseoutCoverageReport {
    s4_recovery_matrix: GeneratedCoverageMatrix,
    shortcut_rejection_matrix: GeneratedCoverageMatrix,
    s5_readiness_shape_probe_matrix: GeneratedCoverageMatrix,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSimulationHarnessCloseoutReport {
    dogfood: S45HarnessDogfoodReport,
    dogfood_evidence: S45HarnessDogfoodEvidence,
    coverage: S45CloseoutCoverageReport,
    acceptance: S45AcceptanceSuiteMap,
    s5_readiness: AcceptedS5SimulationHarnessReadiness,
    future_extension_slots: FutureHarnessExtensionSlotInventory,
    shortcut_denial_count: usize,
}

impl S45CloseoutCoverageReport {
    pub fn from_dogfood_evidence(dogfood_evidence: &S45HarnessDogfoodEvidence) -> Self {
        Self {
            s4_recovery_matrix: dogfood_evidence.s4_recovery().coverage().clone(),
            shortcut_rejection_matrix: dogfood_evidence.shortcut_rejection().coverage().clone(),
            s5_readiness_shape_probe_matrix: dogfood_evidence
                .s5_readiness_shape_probe()
                .coverage()
                .clone(),
        }
    }

    pub const fn s4_recovery_matrix(&self) -> &GeneratedCoverageMatrix {
        &self.s4_recovery_matrix
    }

    pub const fn shortcut_rejection_matrix(&self) -> &GeneratedCoverageMatrix {
        &self.shortcut_rejection_matrix
    }

    pub const fn s5_readiness_shape_probe_matrix(&self) -> &GeneratedCoverageMatrix {
        &self.s5_readiness_shape_probe_matrix
    }

    pub fn all_required_s45_lanes_are_satisfied(&self) -> bool {
        required_s45_lanes()
            .into_iter()
            .all(|lane| lanes_from_closeout_evidence(self).contains(&lane))
    }

    pub(crate) fn matrices(&self) -> [&GeneratedCoverageMatrix; 3] {
        [
            &self.s4_recovery_matrix,
            &self.shortcut_rejection_matrix,
            &self.s5_readiness_shape_probe_matrix,
        ]
    }
}

impl PhysicalSimulationHarnessCloseoutReport {
    pub(crate) const fn new(
        dogfood: S45HarnessDogfoodReport,
        dogfood_evidence: S45HarnessDogfoodEvidence,
        coverage: S45CloseoutCoverageReport,
        acceptance: S45AcceptanceSuiteMap,
        s5_readiness: AcceptedS5SimulationHarnessReadiness,
        future_extension_slots: FutureHarnessExtensionSlotInventory,
        shortcut_denial_count: usize,
    ) -> Self {
        Self {
            dogfood,
            dogfood_evidence,
            coverage,
            acceptance,
            s5_readiness,
            future_extension_slots,
            shortcut_denial_count,
        }
    }

    pub const fn dogfood(&self) -> &S45HarnessDogfoodReport {
        &self.dogfood
    }

    pub const fn dogfood_evidence(&self) -> &S45HarnessDogfoodEvidence {
        &self.dogfood_evidence
    }

    pub const fn coverage(&self) -> &S45CloseoutCoverageReport {
        &self.coverage
    }

    pub const fn acceptance(&self) -> &S45AcceptanceSuiteMap {
        &self.acceptance
    }

    pub const fn s5_readiness(&self) -> &AcceptedS5SimulationHarnessReadiness {
        &self.s5_readiness
    }

    pub const fn future_extension_slots(&self) -> &FutureHarnessExtensionSlotInventory {
        &self.future_extension_slots
    }

    pub const fn shortcut_denial_count(&self) -> usize {
        self.shortcut_denial_count
    }
}
