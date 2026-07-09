use worth_store_contracts::StableArtifactId;
use worth_store_test_support::LargeStorePressureFixture;

use crate::{PhysicalProofOracleKind, PhysicalSubstrateLane, RoadmapLaneFamily};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScenarioLane {
    PhysicalSubstrate(PhysicalSubstrateLane),
    RoadmapFamily(RoadmapLaneFamily),
}

impl ScenarioLane {
    pub const fn family(self) -> RoadmapLaneFamily {
        match self {
            Self::PhysicalSubstrate(lane) => lane.family(),
            Self::RoadmapFamily(family) => family,
        }
    }

    pub const fn physical_substrate_lane(self) -> Option<PhysicalSubstrateLane> {
        match self {
            Self::PhysicalSubstrate(lane) => Some(lane),
            Self::RoadmapFamily(_) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalStoryStep {
    GivenCleanPhysicalStore,
    GivenLegacyBackendClaim,
    GivenHostilePhysicalReference,
    GivenHostilePhysicalBytes,
    WhenAuthoritativeRecordIsAppended,
    WhenStoreClosesAndReopensFromBytes,
    WhenOfflineVerifierReadsManifest,
    WhenLegacyClaimAsksForPlatformGrade,
    ThenRecordLocatesByPhysicalReference,
    ThenForbiddenClaimIsDenied,
    ThenRuntimeVerifierParityIsPreserved,
    GivenLargeStorePressureFixture,
    WhenBufferPoolPressureRuns,
    ThenMemoryPressureCountersMatchEnvelope,
    ThenShortcutCertificationFails,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalScenarioDefinition {
    name: StableArtifactId,
    lane: ScenarioLane,
    physical_law: String,
    steps: Vec<PhysicalStoryStep>,
    required_oracles: Vec<PhysicalProofOracleKind>,
    large_store_pressure_fixture: Option<LargeStorePressureFixture>,
}

impl PhysicalScenarioDefinition {
    pub fn story(name: impl Into<String>) -> PhysicalScenarioDefinitionBuilder {
        PhysicalScenarioDefinitionBuilder::new(name.into())
    }

    pub const fn name(&self) -> &StableArtifactId {
        &self.name
    }

    pub const fn lane(&self) -> ScenarioLane {
        self.lane
    }

    pub fn physical_law(&self) -> &str {
        &self.physical_law
    }

    pub fn steps(&self) -> &[PhysicalStoryStep] {
        &self.steps
    }

    pub fn required_oracles(&self) -> &[PhysicalProofOracleKind] {
        &self.required_oracles
    }

    pub const fn large_store_pressure_fixture(&self) -> Option<LargeStorePressureFixture> {
        self.large_store_pressure_fixture
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalScenarioDefinitionBuilder {
    name: String,
    lane: Option<ScenarioLane>,
    physical_law: Option<String>,
    steps: Vec<PhysicalStoryStep>,
    required_oracles: Vec<PhysicalProofOracleKind>,
    large_store_pressure_fixture: Option<LargeStorePressureFixture>,
}

impl PhysicalScenarioDefinitionBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            lane: None,
            physical_law: None,
            steps: Vec::new(),
            required_oracles: Vec::new(),
            large_store_pressure_fixture: None,
        }
    }

    pub const fn physical_substrate_lane(mut self, lane: PhysicalSubstrateLane) -> Self {
        self.lane = Some(ScenarioLane::PhysicalSubstrate(lane));
        self
    }

    pub const fn roadmap_lane_family(mut self, family: RoadmapLaneFamily) -> Self {
        self.lane = Some(ScenarioLane::RoadmapFamily(family));
        self
    }

    pub fn proves_law(mut self, physical_law: impl Into<String>) -> Self {
        self.physical_law = Some(physical_law.into());
        self
    }

    pub fn step(mut self, step: PhysicalStoryStep) -> Self {
        self.steps.push(step);
        self
    }

    pub fn requires_oracle(mut self, oracle: PhysicalProofOracleKind) -> Self {
        if !self.required_oracles.contains(&oracle) {
            self.required_oracles.push(oracle);
        }
        self
    }

    pub const fn large_store_pressure_fixture(
        mut self,
        fixture: LargeStorePressureFixture,
    ) -> Self {
        self.large_store_pressure_fixture = Some(fixture);
        self
    }

    pub fn define(self) -> Result<PhysicalScenarioDefinition, PhysicalScenarioDefinitionDenial> {
        let name = StableArtifactId::new(self.name)
            .map_err(|_| PhysicalScenarioDefinitionDenial::InvalidScenarioName)?;
        let lane = self
            .lane
            .ok_or(PhysicalScenarioDefinitionDenial::MissingPhysicalSubstrateLane)?;
        let physical_law = self
            .physical_law
            .filter(|law| !law.trim().is_empty())
            .ok_or(PhysicalScenarioDefinitionDenial::MissingPhysicalLaw)?;
        if self.steps.is_empty() {
            return Err(PhysicalScenarioDefinitionDenial::EmptyStory);
        }
        Ok(PhysicalScenarioDefinition {
            name,
            lane,
            physical_law,
            steps: self.steps,
            required_oracles: self.required_oracles,
            large_store_pressure_fixture: self.large_store_pressure_fixture,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalScenarioDefinitionDenial {
    EmptyStory,
    InvalidScenarioName,
    MissingPhysicalLaw,
    MissingPhysicalSubstrateLane,
}
