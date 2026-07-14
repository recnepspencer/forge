#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalIsolationHarnessReadinessDenial {
    MissingDependency(PhysicalIsolationHarnessMaturityDependency),
    MissingInterleavingCapability,
    MissingMaintenanceActorCapability,
    MissingProductionBoundaryYieldpoint,
    MissingProductionDriverCapability,
    MissingReusableOracleFamily,
    MissingCounterContract,
    MissingReplayableTranscript,
    MissingShortcutDenialReport,
    MissingPhysicalIsolationCorrectnessNonClaim,
    WrongSequenceMaturityEvidence,
    UnsupportedProfileMaturityEvidence,
    CopiedReadinessFieldsDenied,
    GenericRunnerCannotSatisfyReadiness,
    FutureBehaviorSlotCannotSatisfyReadiness,
    FoundationalOrProofProjectionCannotSatisfyReadiness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PhysicalIsolationHarnessMaturityDependency {
    ScenarioDefinitions,
    DeterministicScheduler,
    ActorModel,
    ProductionDriverContracts,
    CertificationOracleFamilies,
    CounterStrengthContracts,
    ReplayableTranscripts,
    MutationValidation,
}

impl PhysicalIsolationHarnessMaturityDependency {
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

pub fn reject_missing_physical_isolation_correctness_non_claim(
) -> Result<(), PhysicalIsolationHarnessReadinessDenial> {
    Err(PhysicalIsolationHarnessReadinessDenial::MissingPhysicalIsolationCorrectnessNonClaim)
}

pub fn reject_copied_physical_isolation_simulation_harness_readiness_fields(
) -> Result<(), PhysicalIsolationHarnessReadinessDenial> {
    Err(PhysicalIsolationHarnessReadinessDenial::CopiedReadinessFieldsDenied)
}
