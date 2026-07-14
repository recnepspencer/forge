#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SimulationHarnessRoadmapRequirement {
    GoldenPathAuthoringApi,
    AspectNativeScenarioDefinitions,
    DeterministicScheduler,
    NamedProductionBoundaryYieldpoints,
    ProductionFacingDriverContracts,
    ActorFaultCrashVocabulary,
    ObserverOracleSeparation,
    CertificationOwnedOracleFamilies,
    CounterStrengthContracts,
    ProductionBackedFixtureManifests,
    ReplayableTranscriptsAndEvidence,
    GeneratedCoverageMatrix,
    HarnessMaturityLadder,
    ForbiddenShortcutRejection,
    RecoveryDogfoodSlice,
    PhysicalIsolationReadinessShapeProbeNonClaim,
    FutureExtensionSlotContainment,
    MutationStyleHarnessValidation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationHarnessRoadmapRequirementSet {
    requirements: Vec<SimulationHarnessRoadmapRequirement>,
}

impl SimulationHarnessRoadmapRequirementSet {
    pub fn certification_required() -> Self {
        Self::from_requirements(REQUIRED_S45_ROADMAP_HARNESS_REQUIREMENTS.to_vec())
    }

    pub fn from_requirements(requirements: Vec<SimulationHarnessRoadmapRequirement>) -> Self {
        Self {
            requirements: REQUIRED_S45_ROADMAP_HARNESS_REQUIREMENTS
                .iter()
                .copied()
                .filter(|requirement| requirements.contains(requirement))
                .collect(),
        }
    }

    pub fn requirements(&self) -> &[SimulationHarnessRoadmapRequirement] {
        &self.requirements
    }

    pub fn contains(&self, requirement: SimulationHarnessRoadmapRequirement) -> bool {
        self.requirements.contains(&requirement)
    }

    pub fn missing_required(&self) -> Option<SimulationHarnessRoadmapRequirement> {
        REQUIRED_S45_ROADMAP_HARNESS_REQUIREMENTS
            .iter()
            .copied()
            .find(|requirement| !self.contains(*requirement))
    }

    pub fn is_complete(&self) -> bool {
        self.missing_required().is_none()
    }

    pub(crate) fn canonical_identity_requirements(
        &self,
    ) -> Vec<SimulationHarnessRoadmapRequirement> {
        self.requirements.clone()
    }
}

pub(crate) const REQUIRED_S45_ROADMAP_HARNESS_REQUIREMENTS: [SimulationHarnessRoadmapRequirement;
    18] = [
    SimulationHarnessRoadmapRequirement::GoldenPathAuthoringApi,
    SimulationHarnessRoadmapRequirement::AspectNativeScenarioDefinitions,
    SimulationHarnessRoadmapRequirement::DeterministicScheduler,
    SimulationHarnessRoadmapRequirement::NamedProductionBoundaryYieldpoints,
    SimulationHarnessRoadmapRequirement::ProductionFacingDriverContracts,
    SimulationHarnessRoadmapRequirement::ActorFaultCrashVocabulary,
    SimulationHarnessRoadmapRequirement::ObserverOracleSeparation,
    SimulationHarnessRoadmapRequirement::CertificationOwnedOracleFamilies,
    SimulationHarnessRoadmapRequirement::CounterStrengthContracts,
    SimulationHarnessRoadmapRequirement::ProductionBackedFixtureManifests,
    SimulationHarnessRoadmapRequirement::ReplayableTranscriptsAndEvidence,
    SimulationHarnessRoadmapRequirement::GeneratedCoverageMatrix,
    SimulationHarnessRoadmapRequirement::HarnessMaturityLadder,
    SimulationHarnessRoadmapRequirement::ForbiddenShortcutRejection,
    SimulationHarnessRoadmapRequirement::RecoveryDogfoodSlice,
    SimulationHarnessRoadmapRequirement::PhysicalIsolationReadinessShapeProbeNonClaim,
    SimulationHarnessRoadmapRequirement::FutureExtensionSlotContainment,
    SimulationHarnessRoadmapRequirement::MutationStyleHarnessValidation,
];
