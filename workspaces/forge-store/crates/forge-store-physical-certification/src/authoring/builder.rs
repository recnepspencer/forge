use forge_store_aspect_native::StoreAspectBoundaryFact;

use crate::scenario::{
    certify_scenario_definition, CertifiedPhysicalScenario, PhysicalScenarioActor,
    PhysicalScenarioDefinitionDenial, PhysicalScenarioExpectation, PhysicalScenarioFault,
    PhysicalScenarioIntent, PhysicalScenarioSchedule, PhysicalSimulationScenarioDefinition,
    PhysicalSimulationScenarioFamily,
};

use super::steps::ScenarioBuilderFixtureStep;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalScenarioBuilder {
    pub(crate) label: String,
    pub(crate) family: Option<PhysicalSimulationScenarioFamily>,
    pub(crate) intent: Option<PhysicalScenarioIntent>,
    pub(crate) fixtures: Vec<StoreAspectBoundaryFact>,
    pub(crate) actors: Vec<PhysicalScenarioActor>,
    pub(crate) schedule: Option<PhysicalScenarioSchedule>,
    pub(crate) fault: PhysicalScenarioFault,
    pub(crate) expectation: Option<PhysicalScenarioExpectation>,
}

impl PhysicalScenarioBuilder {
    pub(crate) fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            family: None,
            intent: None,
            fixtures: Vec::new(),
            actors: Vec::new(),
            schedule: None,
            fault: PhysicalScenarioFault::no_fault(),
            expectation: None,
        }
    }

    pub(crate) fn set_family(mut self, family: PhysicalSimulationScenarioFamily) -> Self {
        self.family = Some(family);
        self
    }

    pub(crate) fn set_intent(mut self, intent: PhysicalScenarioIntent) -> Self {
        self.intent = Some(intent);
        self
    }

    pub(crate) fn add_fixture(mut self, fixture: StoreAspectBoundaryFact) -> Self {
        self.fixtures.push(fixture);
        self
    }

    pub(crate) fn add_actor(mut self, actor: PhysicalScenarioActor) -> Self {
        self.actors.push(actor);
        self
    }

    pub(crate) fn set_schedule(mut self, schedule: PhysicalScenarioSchedule) -> Self {
        self.schedule = Some(schedule);
        self
    }

    pub(crate) fn set_fault(mut self, fault: PhysicalScenarioFault) -> Self {
        self.fault = fault;
        self
    }

    pub(crate) fn set_expectation(mut self, expectation: PhysicalScenarioExpectation) -> Self {
        self.expectation = Some(expectation);
        self
    }

    pub(crate) fn certify(
        self,
    ) -> Result<CertifiedPhysicalScenario, PhysicalScenarioDefinitionDenial> {
        let definition = PhysicalSimulationScenarioDefinition::from_builder(self)?;
        certify_scenario_definition(definition)
    }
}

pub fn physical_scenario(label: impl Into<String>) -> ScenarioBuilderFixtureStep {
    ScenarioBuilderFixtureStep::new(PhysicalScenarioBuilder::new(label))
}
