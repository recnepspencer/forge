use super::{
    PhysicalSimulationHarnessCloseoutDenial, SimulationHarnessAcceptanceSuiteEvidenceSource,
    SimulationHarnessAcceptanceSuiteExecutionProof, SimulationHarnessAcceptanceSuiteName,
    SimulationHarnessCloseoutCoverageReport, SimulationHarnessDogfoodEvidence,
};

impl SimulationHarnessAcceptanceSuiteExecutionProof {
    pub fn entry_boundary_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::EntryBoundary,
            dogfood,
            coverage,
        )
    }

    pub fn aspect_native_scenario_definition_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::AspectNativeScenarioDefinition,
            dogfood,
            coverage,
        )
    }

    pub fn simulation_plan_lowering_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::SimulationPlanLowering,
            dogfood,
            coverage,
        )
    }

    pub fn golden_path_authoring_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::GoldenPathAuthoring,
            dogfood,
            coverage,
        )
    }

    pub fn production_driver_contract_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::ProductionDriverContract,
            dogfood,
            coverage,
        )
    }

    pub fn yieldpoint_control_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::YieldpointControl,
            dogfood,
            coverage,
        )
    }

    pub fn deterministic_schedule_replay_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::DeterministicScheduleReplay,
            dogfood,
            coverage,
        )
    }

    pub fn fault_delivery_boundary_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::FaultDeliveryBoundary,
            dogfood,
            coverage,
        )
    }

    pub fn observer_oracle_separation_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::ObserverOracleSeparation,
            dogfood,
            coverage,
        )
    }

    pub fn oracle_library_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::OracleLibrary,
            dogfood,
            coverage,
        )
    }

    pub fn counter_contract_profile_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::CounterContractProfile,
            dogfood,
            coverage,
        )
    }

    pub fn counter_strength_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::CounterStrength,
            dogfood,
            coverage,
        )
    }

    pub fn production_backed_fixture_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::ProductionBackedFixture,
            dogfood,
            coverage,
        )
    }

    pub fn transcript_evidence_bundle_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::TranscriptEvidenceBundle,
            dogfood,
            coverage,
        )
    }

    pub fn coverage_maturity_ladder_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::CoverageMaturityLadder,
            dogfood,
            coverage,
        )
    }

    pub fn generated_coverage_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::GeneratedCoverage,
            dogfood,
            coverage,
        )
    }

    pub fn forbidden_shortcut_rejection_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::ForbiddenShortcutRejection,
            dogfood,
            coverage,
        )
    }

    pub fn harness_dogfood_vertical_slice_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::HarnessDogfoodVerticalSlice,
            dogfood,
            coverage,
        )
    }

    pub fn extension_slot_containment_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::ExtensionSlotContainment,
            dogfood,
            coverage,
        )
    }

    pub fn foundational_proof_simulation_evidence_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::FoundationalProofSimulationEvidence,
            dogfood,
            coverage,
        )
    }

    pub fn physical_isolation_simulation_harness_readiness_suite_run(
        dogfood: &SimulationHarnessDogfoodEvidence,
        coverage: &SimulationHarnessCloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        from_named_suite_run(
            SimulationHarnessAcceptanceSuiteName::PhysicalIsolationHarnessReadiness,
            dogfood,
            coverage,
        )
    }
}

fn from_named_suite_run(
    suite: SimulationHarnessAcceptanceSuiteName,
    dogfood: &SimulationHarnessDogfoodEvidence,
    coverage: &SimulationHarnessCloseoutCoverageReport,
) -> Result<SimulationHarnessAcceptanceSuiteExecutionProof, PhysicalSimulationHarnessCloseoutDenial>
{
    SimulationHarnessAcceptanceSuiteExecutionProof::from_closeout_suite_run(
        SimulationHarnessAcceptanceSuiteEvidenceSource::for_suite(suite),
        dogfood,
        coverage,
    )
}
