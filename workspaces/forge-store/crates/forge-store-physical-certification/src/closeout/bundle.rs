use forge_store_readiness::S5CorrectnessNonClaimEvidence;

use crate::{
    accept_store_owned_s5_harness_readiness, GeneratedCoverageMatrix, Roadmap2HarnessSequence,
    S5HarnessReadinessReceipt, SyntheticHarnessShortcutRejectionReport,
};

use super::{
    FutureHarnessExtensionSlotInventory, PhysicalSimulationHarnessCloseoutDenial,
    PhysicalSimulationHarnessCloseoutReport, PhysicalSimulationHarnessCloseoutSuite,
    S45AcceptanceSuiteReceiptSet, S45CloseoutCoverageReport, S45HarnessDogfoodEvidence,
    S45HarnessDogfoodReport,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSimulationHarnessCertificationBundle {
    report: PhysicalSimulationHarnessCloseoutReport,
}

impl PhysicalSimulationHarnessCertificationBundle {
    #[allow(clippy::too_many_arguments)]
    pub fn from_s45_public_authoring_slices(
        suite: PhysicalSimulationHarnessCloseoutSuite,
        dogfood_evidence: S45HarnessDogfoodEvidence,
        acceptance_receipts: S45AcceptanceSuiteReceiptSet,
        shortcut_report: SyntheticHarnessShortcutRejectionReport,
        non_claim: S5CorrectnessNonClaimEvidence,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        require_s45_suite(suite)?;
        require_shortcut_denials(&shortcut_report)?;
        require_mutation_coverage(dogfood_evidence.shortcut_rejection().coverage())?;
        require_mutation_coverage(dogfood_evidence.s5_readiness_shape_probe().coverage())?;
        let coverage = S45CloseoutCoverageReport::from_dogfood_evidence(&dogfood_evidence);
        let acceptance = acceptance_receipts.into_suite_map_bound_to(&dogfood_evidence)?;
        let receipt = S5HarnessReadinessReceipt::from_store_harness_evidence(
            dogfood_evidence.s5_readiness_shape_probe().coverage(),
            dogfood_evidence.s5_readiness_shape_probe().evidence(),
            &shortcut_report,
            non_claim,
        )?;
        let shortcut_denial_count = receipt.shortcut_denial_count();
        let s5_readiness = accept_store_owned_s5_harness_readiness(
            receipt,
            forge_store_physical_isolation::s5_simulation_harness_readiness_requirement(),
        );
        let dogfood = S45HarnessDogfoodReport::new(
            dogfood_evidence.s4_recovery().scenario().clone(),
            dogfood_evidence.shortcut_rejection().scenario().clone(),
            dogfood_evidence
                .s5_readiness_shape_probe()
                .scenario()
                .clone(),
        );
        let report = PhysicalSimulationHarnessCloseoutReport::new(
            dogfood,
            dogfood_evidence,
            coverage,
            acceptance,
            s5_readiness,
            FutureHarnessExtensionSlotInventory::s45_reserved_future_slots(),
            shortcut_denial_count,
        );
        Ok(Self { report })
    }

    pub const fn closeout_report(&self) -> &PhysicalSimulationHarnessCloseoutReport {
        &self.report
    }
}

fn require_s45_suite(
    suite: PhysicalSimulationHarnessCloseoutSuite,
) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
    if suite.sequence() == Roadmap2HarnessSequence::S45 {
        Ok(())
    } else {
        Err(
            PhysicalSimulationHarnessCloseoutDenial::WrongCloseoutSuite {
                expected: Roadmap2HarnessSequence::S45,
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
