use worth_store_test_support::{LargeStorePressureClass, LargeStorePressureFixture};

use crate::{
    PhysicalCounterExpectationKind, PhysicalScenarioCostClass, PhysicalScenarioPlan,
    PhysicalScenarioPlanIdentity, RoadmapLaneFamily, ScenarioCounterExpectation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferPoolScenarioPlan<'a> {
    plan: &'a PhysicalScenarioPlan,
    fixture: LargeStorePressureFixture,
}

impl<'a> BufferPoolScenarioPlan<'a> {
    pub fn admit(plan: &'a PhysicalScenarioPlan) -> Result<Self, BufferPoolScenarioPlanDenial> {
        if plan.identity().lane_family() != RoadmapLaneFamily::BufferPool {
            return Err(BufferPoolScenarioPlanDenial::NotBufferPoolLane);
        }
        if plan.cost_class() != PhysicalScenarioCostClass::LargeStoreMemoryPressure {
            return Err(BufferPoolScenarioPlanDenial::NotLargeStorePressurePlan);
        }
        let fixture = plan
            .large_store_pressure_fixture()
            .ok_or(BufferPoolScenarioPlanDenial::MissingPressureFixture)?;
        if !fixture.persisted_exceeds_budget() {
            return Err(BufferPoolScenarioPlanDenial::FixtureFitsInResidentBudget);
        }
        Ok(Self { plan, fixture })
    }

    pub const fn pressure_class(&self) -> LargeStorePressureClass {
        self.fixture.class()
    }

    pub const fn fixture(&self) -> LargeStorePressureFixture {
        self.fixture
    }

    pub const fn plan_identity(&self) -> &PhysicalScenarioPlanIdentity {
        self.plan.identity()
    }

    pub fn expected_counter(&self, counter: PhysicalCounterExpectationKind) -> Option<u64> {
        self.plan
            .expected_counters()
            .iter()
            .find(|expectation| expectation.counter() == counter)
            .map(ScenarioCounterExpectation::expected)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferPoolScenarioPlanDenial {
    FixtureFitsInResidentBudget,
    MissingPressureFixture,
    NotBufferPoolLane,
    NotLargeStorePressurePlan,
}
