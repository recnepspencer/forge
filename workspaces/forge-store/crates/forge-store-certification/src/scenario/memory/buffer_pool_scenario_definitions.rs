use forge_store_test_support::{LargeStorePressureClass, LargeStorePressureFixture};

use crate::{
    PhysicalProofOracleKind, PhysicalScenarioDefinition, PhysicalStoryStep, RoadmapLaneFamily,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LargeStoreMemoryPressureScenario {
    fixture: LargeStorePressureFixture,
}

impl LargeStoreMemoryPressureScenario {
    pub fn for_class(class: LargeStorePressureClass) -> Self {
        Self {
            fixture: LargeStorePressureFixture::for_class(class),
        }
    }

    pub const fn fixture(&self) -> LargeStorePressureFixture {
        self.fixture
    }

    pub fn definition(&self) -> Result<PhysicalScenarioDefinition, LargeStoreScenarioDenial> {
        if !self.fixture.persisted_exceeds_budget() {
            return Err(LargeStoreScenarioDenial::FixtureFitsInResidentBudget);
        }
        PhysicalScenarioDefinition::story(format!(
            "{}_large_store_pressure",
            self.fixture.class().as_str()
        ))
        .roadmap_lane_family(RoadmapLaneFamily::BufferPool)
        .proves_law("S.2 large-store pressure is admitted by bounded BufferPool plans")
        .step(PhysicalStoryStep::GivenLargeStorePressureFixture)
        .step(PhysicalStoryStep::WhenBufferPoolPressureRuns)
        .step(PhysicalStoryStep::ThenMemoryPressureCountersMatchEnvelope)
        .step(PhysicalStoryStep::ThenShortcutCertificationFails)
        .requires_oracle(PhysicalProofOracleKind::LargeStorePressureBounded)
        .requires_oracle(PhysicalProofOracleKind::OomAvoidanceBeforeMaterialization)
        .requires_oracle(PhysicalProofOracleKind::PressureTranscriptReplayStable)
        .requires_oracle(PhysicalProofOracleKind::ShortcutCertificationRejected)
        .large_store_pressure_fixture(self.fixture)
        .define()
        .map_err(|_| LargeStoreScenarioDenial::InvalidScenarioDefinition)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LargeStoreScenarioDenial {
    FixtureFitsInResidentBudget,
    InvalidScenarioDefinition,
}
