#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Roadmap2HarnessSequence {
    S4,
    S45,
    S5,
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
pub struct S5ReadinessDependencySet {
    required: Vec<HarnessSubsystem>,
}

impl S5ReadinessDependencySet {
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
