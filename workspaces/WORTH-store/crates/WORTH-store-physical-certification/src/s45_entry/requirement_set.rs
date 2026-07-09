#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S45RoadmapHarnessRequirement {
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
    S4RecoveryDogfoodSlice,
    S5ReadinessShapeProbeNonClaim,
    FutureExtensionSlotContainment,
    MutationStyleHarnessValidation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S45RoadmapHarnessRequirementSet {
    requirements: Vec<S45RoadmapHarnessRequirement>,
}

impl S45RoadmapHarnessRequirementSet {
    pub fn roadmap2_required() -> Self {
        Self::from_requirements(REQUIRED_S45_ROADMAP_HARNESS_REQUIREMENTS.to_vec())
    }

    pub fn from_requirements(requirements: Vec<S45RoadmapHarnessRequirement>) -> Self {
        Self {
            requirements: REQUIRED_S45_ROADMAP_HARNESS_REQUIREMENTS
                .iter()
                .copied()
                .filter(|requirement| requirements.contains(requirement))
                .collect(),
        }
    }

    pub fn requirements(&self) -> &[S45RoadmapHarnessRequirement] {
        &self.requirements
    }

    pub fn contains(&self, requirement: S45RoadmapHarnessRequirement) -> bool {
        self.requirements.contains(&requirement)
    }

    pub fn missing_required(&self) -> Option<S45RoadmapHarnessRequirement> {
        REQUIRED_S45_ROADMAP_HARNESS_REQUIREMENTS
            .iter()
            .copied()
            .find(|requirement| !self.contains(*requirement))
    }

    pub fn is_complete(&self) -> bool {
        self.missing_required().is_none()
    }

    pub(crate) fn canonical_identity_requirements(&self) -> Vec<S45RoadmapHarnessRequirement> {
        self.requirements.clone()
    }
}

pub(crate) const REQUIRED_S45_ROADMAP_HARNESS_REQUIREMENTS: [S45RoadmapHarnessRequirement; 18] = [
    S45RoadmapHarnessRequirement::GoldenPathAuthoringApi,
    S45RoadmapHarnessRequirement::AspectNativeScenarioDefinitions,
    S45RoadmapHarnessRequirement::DeterministicScheduler,
    S45RoadmapHarnessRequirement::NamedProductionBoundaryYieldpoints,
    S45RoadmapHarnessRequirement::ProductionFacingDriverContracts,
    S45RoadmapHarnessRequirement::ActorFaultCrashVocabulary,
    S45RoadmapHarnessRequirement::ObserverOracleSeparation,
    S45RoadmapHarnessRequirement::CertificationOwnedOracleFamilies,
    S45RoadmapHarnessRequirement::CounterStrengthContracts,
    S45RoadmapHarnessRequirement::ProductionBackedFixtureManifests,
    S45RoadmapHarnessRequirement::ReplayableTranscriptsAndEvidence,
    S45RoadmapHarnessRequirement::GeneratedCoverageMatrix,
    S45RoadmapHarnessRequirement::HarnessMaturityLadder,
    S45RoadmapHarnessRequirement::ForbiddenShortcutRejection,
    S45RoadmapHarnessRequirement::S4RecoveryDogfoodSlice,
    S45RoadmapHarnessRequirement::S5ReadinessShapeProbeNonClaim,
    S45RoadmapHarnessRequirement::FutureExtensionSlotContainment,
    S45RoadmapHarnessRequirement::MutationStyleHarnessValidation,
];
