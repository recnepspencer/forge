use worth_store_aspect_native::StoreAspectBoundaryFact;

use crate::scenario::{
    CertifiedPhysicalScenario, PhysicalScenarioActor, PhysicalScenarioDefinitionDenial,
    PhysicalScenarioExpectation, PhysicalScenarioFault, PhysicalScenarioIntent,
    PhysicalScenarioSchedule, PhysicalSimulationScenarioFamily,
};

use super::builder::PhysicalScenarioBuilder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioBuilderFixtureStep {
    builder: PhysicalScenarioBuilder,
}

impl ScenarioBuilderFixtureStep {
    pub(crate) const fn new(builder: PhysicalScenarioBuilder) -> Self {
        Self { builder }
    }

    pub fn family(self, family: PhysicalSimulationScenarioFamily) -> Self {
        Self {
            builder: self.builder.set_family(family),
        }
    }

    pub fn intent(self, intent: PhysicalScenarioIntent) -> Self {
        Self {
            builder: self.builder.set_intent(intent),
        }
    }

    pub fn fixture(self, fixture: StoreAspectBoundaryFact) -> ScenarioBuilderActorStep {
        ScenarioBuilderActorStep::new(self.builder.add_fixture(fixture))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioBuilderActorStep {
    builder: PhysicalScenarioBuilder,
}

impl ScenarioBuilderActorStep {
    pub(crate) const fn new(builder: PhysicalScenarioBuilder) -> Self {
        Self { builder }
    }

    pub fn fixture(self, fixture: StoreAspectBoundaryFact) -> Self {
        Self {
            builder: self.builder.add_fixture(fixture),
        }
    }

    pub fn actor(self, actor: PhysicalScenarioActor) -> Self {
        Self {
            builder: self.builder.add_actor(actor),
        }
    }

    pub fn fault(self, fault: PhysicalScenarioFault) -> Self {
        Self {
            builder: self.builder.set_fault(fault),
        }
    }

    pub fn schedule(self, schedule: PhysicalScenarioSchedule) -> ScenarioBuilderScheduleStep {
        ScenarioBuilderScheduleStep::new(self.builder.set_schedule(schedule))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioBuilderScheduleStep {
    builder: PhysicalScenarioBuilder,
}

impl ScenarioBuilderScheduleStep {
    pub(crate) const fn new(builder: PhysicalScenarioBuilder) -> Self {
        Self { builder }
    }

    pub fn fault(self, fault: PhysicalScenarioFault) -> Self {
        Self {
            builder: self.builder.set_fault(fault),
        }
    }

    pub fn expectation(
        self,
        expectation: PhysicalScenarioExpectation,
    ) -> ScenarioBuilderExpectationStep {
        ScenarioBuilderExpectationStep::new(self.builder.set_expectation(expectation))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioBuilderExpectationStep {
    builder: PhysicalScenarioBuilder,
}

impl ScenarioBuilderExpectationStep {
    pub(crate) const fn new(builder: PhysicalScenarioBuilder) -> Self {
        Self { builder }
    }

    pub fn certify_definition(
        self,
    ) -> Result<CertifiedPhysicalScenario, PhysicalScenarioDefinitionDenial> {
        self.builder.certify()
    }
}
