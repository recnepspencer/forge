#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HarnessCoverageStage {
    Recovery,
    SimulationAdmission,
    PhysicalIsolation,
    FutureSequence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HarnessSubsystem {
    ScenarioDefinitions,
    DeterministicScheduler,
    ActorModel,
    ProductionDriverContracts,
    CertificationOracleFamilies,
    CounterStrengthContracts,
    ReplayableTranscripts,
    MutationValidation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationReadinessDependencySet {
    required: Vec<HarnessSubsystem>,
}

impl PhysicalIsolationReadinessDependencySet {
    pub fn required_for_ci() -> Self {
        Self {
            required: vec![
                HarnessSubsystem::ScenarioDefinitions,
                HarnessSubsystem::DeterministicScheduler,
                HarnessSubsystem::ActorModel,
                HarnessSubsystem::ProductionDriverContracts,
                HarnessSubsystem::CertificationOracleFamilies,
                HarnessSubsystem::CounterStrengthContracts,
                HarnessSubsystem::ReplayableTranscripts,
                HarnessSubsystem::MutationValidation,
            ],
        }
    }

    pub fn required(&self) -> &[HarnessSubsystem] {
        &self.required
    }
}
