#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SimulationHarnessAcceptanceSuiteName {
    EntryBoundary,
    AspectNativeScenarioDefinition,
    SimulationPlanLowering,
    GoldenPathAuthoring,
    ProductionDriverContract,
    YieldpointControl,
    DeterministicScheduleReplay,
    FaultDeliveryBoundary,
    ObserverOracleSeparation,
    OracleLibrary,
    CounterContractProfile,
    CounterStrength,
    ProductionBackedFixture,
    TranscriptEvidenceBundle,
    CoverageMaturityLadder,
    GeneratedCoverage,
    ForbiddenShortcutRejection,
    HarnessDogfoodVerticalSlice,
    ExtensionSlotContainment,
    FoundationalProofSimulationEvidence,
    PhysicalIsolationHarnessReadiness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationHarnessAcceptanceSuiteEvidenceSource {
    suite: SimulationHarnessAcceptanceSuiteName,
}

impl SimulationHarnessAcceptanceSuiteName {
    pub const fn ordinal(&self) -> u8 {
        match self {
            Self::EntryBoundary => 0,
            Self::AspectNativeScenarioDefinition => 1,
            Self::SimulationPlanLowering => 2,
            Self::GoldenPathAuthoring => 3,
            Self::ProductionDriverContract => 4,
            Self::YieldpointControl => 5,
            Self::DeterministicScheduleReplay => 6,
            Self::FaultDeliveryBoundary => 7,
            Self::ObserverOracleSeparation => 8,
            Self::OracleLibrary => 9,
            Self::CounterContractProfile => 10,
            Self::CounterStrength => 11,
            Self::ProductionBackedFixture => 12,
            Self::TranscriptEvidenceBundle => 13,
            Self::CoverageMaturityLadder => 14,
            Self::GeneratedCoverage => 15,
            Self::ForbiddenShortcutRejection => 16,
            Self::HarnessDogfoodVerticalSlice => 17,
            Self::ExtensionSlotContainment => 18,
            Self::FoundationalProofSimulationEvidence => 19,
            Self::PhysicalIsolationHarnessReadiness => 20,
        }
    }

    pub const fn required_simulation_harness() -> [Self; 21] {
        [
            Self::EntryBoundary,
            Self::AspectNativeScenarioDefinition,
            Self::SimulationPlanLowering,
            Self::GoldenPathAuthoring,
            Self::ProductionDriverContract,
            Self::YieldpointControl,
            Self::DeterministicScheduleReplay,
            Self::FaultDeliveryBoundary,
            Self::ObserverOracleSeparation,
            Self::OracleLibrary,
            Self::CounterContractProfile,
            Self::CounterStrength,
            Self::ProductionBackedFixture,
            Self::TranscriptEvidenceBundle,
            Self::CoverageMaturityLadder,
            Self::GeneratedCoverage,
            Self::ForbiddenShortcutRejection,
            Self::HarnessDogfoodVerticalSlice,
            Self::ExtensionSlotContainment,
            Self::FoundationalProofSimulationEvidence,
            Self::PhysicalIsolationHarnessReadiness,
        ]
    }
}

impl SimulationHarnessAcceptanceSuiteEvidenceSource {
    pub const fn suite(&self) -> SimulationHarnessAcceptanceSuiteName {
        self.suite
    }

    pub(crate) const fn for_suite(suite: SimulationHarnessAcceptanceSuiteName) -> Self {
        Self { suite }
    }
}
