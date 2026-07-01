use super::{
    PhysicalSimulationHarnessCloseoutDenial, S45AcceptanceSuiteEvidenceSource,
    S45AcceptanceSuiteExecutionProof, S45AcceptanceSuiteName, S45CloseoutCoverageReport,
    S45HarnessDogfoodEvidence,
};

impl S45AcceptanceSuiteExecutionProof {
    pub fn entry_boundary_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(S45AcceptanceSuiteName::EntryBoundary, dogfood, coverage)
    }

    pub fn aspect_native_scenario_definition_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            S45AcceptanceSuiteName::AspectNativeScenarioDefinition,
            dogfood,
            coverage,
        )
    }

    pub fn simulation_plan_lowering_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            S45AcceptanceSuiteName::SimulationPlanLowering,
            dogfood,
            coverage,
        )
    }

    pub fn golden_path_authoring_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            S45AcceptanceSuiteName::GoldenPathAuthoring,
            dogfood,
            coverage,
        )
    }

    pub fn production_driver_contract_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            S45AcceptanceSuiteName::ProductionDriverContract,
            dogfood,
            coverage,
        )
    }

    pub fn yieldpoint_control_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(S45AcceptanceSuiteName::YieldpointControl, dogfood, coverage)
    }

    pub fn deterministic_schedule_replay_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            S45AcceptanceSuiteName::DeterministicScheduleReplay,
            dogfood,
            coverage,
        )
    }

    pub fn fault_delivery_boundary_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            S45AcceptanceSuiteName::FaultDeliveryBoundary,
            dogfood,
            coverage,
        )
    }

    pub fn observer_oracle_separation_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            S45AcceptanceSuiteName::ObserverOracleSeparation,
            dogfood,
            coverage,
        )
    }

    pub fn oracle_library_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(S45AcceptanceSuiteName::OracleLibrary, dogfood, coverage)
    }

    pub fn counter_contract_profile_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            S45AcceptanceSuiteName::CounterContractProfile,
            dogfood,
            coverage,
        )
    }

    pub fn counter_strength_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(S45AcceptanceSuiteName::CounterStrength, dogfood, coverage)
    }

    pub fn production_backed_fixture_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            S45AcceptanceSuiteName::ProductionBackedFixture,
            dogfood,
            coverage,
        )
    }

    pub fn transcript_evidence_bundle_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            S45AcceptanceSuiteName::TranscriptEvidenceBundle,
            dogfood,
            coverage,
        )
    }

    pub fn coverage_maturity_ladder_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            S45AcceptanceSuiteName::CoverageMaturityLadder,
            dogfood,
            coverage,
        )
    }

    pub fn generated_coverage_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(S45AcceptanceSuiteName::GeneratedCoverage, dogfood, coverage)
    }

    pub fn forbidden_shortcut_rejection_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            S45AcceptanceSuiteName::ForbiddenShortcutRejection,
            dogfood,
            coverage,
        )
    }

    pub fn harness_dogfood_vertical_slice_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            S45AcceptanceSuiteName::HarnessDogfoodVerticalSlice,
            dogfood,
            coverage,
        )
    }

    pub fn extension_slot_containment_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            S45AcceptanceSuiteName::ExtensionSlotContainment,
            dogfood,
            coverage,
        )
    }

    pub fn foundational_proof_simulation_evidence_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            S45AcceptanceSuiteName::FoundationalProofSimulationEvidence,
            dogfood,
            coverage,
        )
    }

    pub fn s5_simulation_harness_readiness_suite_run(
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            S45AcceptanceSuiteName::S5SimulationHarnessReadiness,
            dogfood,
            coverage,
        )
    }
}

fn from_named_suite_run(
    suite: S45AcceptanceSuiteName,
    dogfood: &S45HarnessDogfoodEvidence,
    coverage: &S45CloseoutCoverageReport,
) -> Result<S45AcceptanceSuiteExecutionProof, PhysicalSimulationHarnessCloseoutDenial> {
    S45AcceptanceSuiteExecutionProof::from_closeout_suite_run(
        S45AcceptanceSuiteEvidenceSource::for_suite(suite),
        dogfood,
        coverage,
    )
}
