#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S5SimulationHarnessReadinessDenial {
    MissingDependency(S5HarnessMaturityDependency),
    MissingInterleavingCapability,
    MissingMaintenanceActorCapability,
    MissingProductionBoundaryYieldpoint,
    MissingProductionDriverCapability,
    MissingReusableOracleFamily,
    MissingCounterContract,
    MissingReplayableTranscript,
    MissingShortcutDenialReport,
    MissingS5CorrectnessNonClaim,
    WrongSequenceMaturityEvidence,
    UnsupportedProfileMaturityEvidence,
    CopiedReadinessFieldsDenied,
    GenericRunnerCannotSatisfyReadiness,
    FutureBehaviorSlotCannotSatisfyReadiness,
    FoundationalOrProofProjectionCannotSatisfyReadiness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum S5HarnessMaturityDependency {
    ScenarioDefinitions,
    DeterministicScheduler,
    ActorModel,
    ProductionDriverContracts,
    CertificationOracleFamilies,
    CounterStrengthContracts,
    ReplayableTranscripts,
    MutationValidation,
}

impl S5HarnessMaturityDependency {
    pub const fn required_for_ci() -> [Self; 8] {
        [
            Self::ScenarioDefinitions,
            Self::DeterministicScheduler,
            Self::ActorModel,
            Self::ProductionDriverContracts,
            Self::CertificationOracleFamilies,
            Self::CounterStrengthContracts,
            Self::ReplayableTranscripts,
            Self::MutationValidation,
        ]
    }
}

pub fn reject_missing_s5_correctness_non_claim() -> Result<(), S5SimulationHarnessReadinessDenial> {
    Err(S5SimulationHarnessReadinessDenial::MissingS5CorrectnessNonClaim)
}

pub fn reject_copied_s5_simulation_harness_readiness_fields(
) -> Result<(), S5SimulationHarnessReadinessDenial> {
    Err(S5SimulationHarnessReadinessDenial::CopiedReadinessFieldsDenied)
}
