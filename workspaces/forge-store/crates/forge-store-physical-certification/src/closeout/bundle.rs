use crate::PhysicalIsolationCorrectnessNonClaimEvidence;

use crate::{
    accept_store_owned_physical_isolation_harness_readiness, GeneratedCoverageMatrix,
    HarnessCoverageStage, PhysicalIsolationHarnessReadinessReceipt,
    SyntheticHarnessShortcutRejectionReport,
};

use super::{
    FutureHarnessExtensionSlotInventory, PhysicalSimulationHarnessCloseoutDenial,
    PhysicalSimulationHarnessCloseoutReport, PhysicalSimulationHarnessCloseoutSuite,
    SimulationHarnessAcceptanceSuiteReceiptSet, SimulationHarnessCloseoutCoverageReport,
    SimulationHarnessDogfoodEvidence, SimulationHarnessDogfoodReport,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSimulationHarnessCertificationBundle {
    report: PhysicalSimulationHarnessCloseoutReport,
}

impl PhysicalSimulationHarnessCertificationBundle {
    #[allow(clippy::too_many_arguments)]
    pub fn from_simulation_harness_public_authoring_slices(
        suite: PhysicalSimulationHarnessCloseoutSuite,
        dogfood_evidence: SimulationHarnessDogfoodEvidence,
        acceptance_receipts: SimulationHarnessAcceptanceSuiteReceiptSet,
        shortcut_report: SyntheticHarnessShortcutRejectionReport,
        non_claim: PhysicalIsolationCorrectnessNonClaimEvidence,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        require_simulation_harness_suite(suite)?;
        require_shortcut_denials(&shortcut_report)?;
        require_mutation_coverage(dogfood_evidence.shortcut_rejection().coverage())?;
        require_mutation_coverage(
            dogfood_evidence
                .physical_isolation_readiness_shape_probe()
                .coverage(),
        )?;
        let coverage =
            SimulationHarnessCloseoutCoverageReport::from_dogfood_evidence(&dogfood_evidence);
        let acceptance = acceptance_receipts.into_suite_map_bound_to(&dogfood_evidence)?;
        let receipt = PhysicalIsolationHarnessReadinessReceipt::from_store_harness_evidence(
            dogfood_evidence
                .physical_isolation_readiness_shape_probe()
                .coverage(),
            dogfood_evidence
                .physical_isolation_readiness_shape_probe()
                .evidence(),
            &shortcut_report,
            non_claim,
        )?;
        let shortcut_denial_count = receipt.shortcut_denial_count();
        let physical_isolation_readiness =
            accept_store_owned_physical_isolation_harness_readiness(receipt);
        let dogfood = SimulationHarnessDogfoodReport::new(
            dogfood_evidence.recovery().scenario().clone(),
            dogfood_evidence.shortcut_rejection().scenario().clone(),
            dogfood_evidence
                .physical_isolation_readiness_shape_probe()
                .scenario()
                .clone(),
        );
        let report = PhysicalSimulationHarnessCloseoutReport::new(
            dogfood,
            dogfood_evidence,
            coverage,
            acceptance,
            physical_isolation_readiness,
            FutureHarnessExtensionSlotInventory::simulation_harness_reserved_future_slots(),
            shortcut_denial_count,
        );
        Ok(Self { report })
    }

    pub const fn closeout_report(&self) -> &PhysicalSimulationHarnessCloseoutReport {
        &self.report
    }
}

fn require_simulation_harness_suite(
    suite: PhysicalSimulationHarnessCloseoutSuite,
) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
    if suite.sequence() == HarnessCoverageStage::SimulationAdmission {
        Ok(())
    } else {
        Err(
            PhysicalSimulationHarnessCloseoutDenial::WrongCloseoutSuite {
                expected: HarnessCoverageStage::SimulationAdmission,
                actual: suite.sequence(),
            },
        )
    }
}

fn require_shortcut_denials(
    report: &SyntheticHarnessShortcutRejectionReport,
) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
    if report.all_required_shortcuts_denied() {
        Ok(())
    } else {
        Err(PhysicalSimulationHarnessCloseoutDenial::MissingShortcutDenialReport)
    }
}

fn require_mutation_coverage(
    matrix: &GeneratedCoverageMatrix,
) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
    if matrix
        .rows()
        .iter()
        .any(|row| row.surface() == crate::CoverageSurfaceKind::MutationResult)
    {
        Ok(())
    } else {
        Err(PhysicalSimulationHarnessCloseoutDenial::MissingMutationCoverage)
    }
}
