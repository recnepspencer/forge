use worth_store_aspect_native::StoreAspectBoundaryFact;

use crate::authoring::PhysicalScenarioBuilder;

use super::collections::{PhysicalScenarioActorSet, PhysicalScenarioFixtureSet};
use super::denial::PhysicalScenarioDefinitionDenial;
use super::expectation::PhysicalScenarioExpectation;
use super::identity::PhysicalScenarioCanonicalIdentity;
use super::vocabulary::{
    PhysicalScenarioActor, PhysicalScenarioFault, PhysicalScenarioIntent, PhysicalScenarioSchedule,
    PhysicalSimulationScenarioFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSimulationScenarioDefinition {
    label: String,
    family: PhysicalSimulationScenarioFamily,
    intent: PhysicalScenarioIntent,
    fixtures: PhysicalScenarioFixtureSet,
    actors: PhysicalScenarioActorSet,
    schedule: PhysicalScenarioSchedule,
    fault: PhysicalScenarioFault,
    expectation: PhysicalScenarioExpectation,
}

impl PhysicalSimulationScenarioDefinition {
    pub(crate) fn from_builder(
        builder: PhysicalScenarioBuilder,
    ) -> Result<Self, PhysicalScenarioDefinitionDenial> {
        let family = builder
            .family
            .ok_or(PhysicalScenarioDefinitionDenial::MissingScenarioFamily)?;
        let intent = builder
            .intent
            .ok_or(PhysicalScenarioDefinitionDenial::MissingScenarioIntent)?;
        let schedule = builder
            .schedule
            .ok_or(PhysicalScenarioDefinitionDenial::MissingSchedule)?;
        let expectation = builder
            .expectation
            .ok_or(PhysicalScenarioDefinitionDenial::MissingExpectation)?;
        Self::from_native_parts(
            builder.label,
            family,
            intent,
            builder.fixtures,
            builder.actors,
            schedule,
            builder.fault,
            expectation,
        )
    }

    pub(crate) fn from_native_parts(
        label: String,
        family: PhysicalSimulationScenarioFamily,
        intent: PhysicalScenarioIntent,
        fixtures: Vec<StoreAspectBoundaryFact>,
        actors: Vec<PhysicalScenarioActor>,
        schedule: PhysicalScenarioSchedule,
        fault: PhysicalScenarioFault,
        expectation: PhysicalScenarioExpectation,
    ) -> Result<Self, PhysicalScenarioDefinitionDenial> {
        require_named_production_boundary_yieldpoint(&schedule)?;
        let actors = PhysicalScenarioActorSet::from_actors(actors)?;
        let fixtures = PhysicalScenarioFixtureSet::from_fixtures(fixtures)?;
        Ok(Self {
            label,
            family,
            intent,
            fixtures,
            actors,
            schedule,
            fault,
            expectation,
        })
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub const fn family(&self) -> PhysicalSimulationScenarioFamily {
        self.family
    }

    pub const fn intent(&self) -> PhysicalScenarioIntent {
        self.intent
    }

    pub fn fixtures(&self) -> &[StoreAspectBoundaryFact] {
        self.fixtures.fixtures()
    }

    pub fn actors(&self) -> &[PhysicalScenarioActor] {
        self.actors.actors()
    }

    pub const fn fixture_set(&self) -> &PhysicalScenarioFixtureSet {
        &self.fixtures
    }

    pub const fn actor_set(&self) -> &PhysicalScenarioActorSet {
        &self.actors
    }

    pub const fn schedule(&self) -> &PhysicalScenarioSchedule {
        &self.schedule
    }

    pub const fn fault(&self) -> &PhysicalScenarioFault {
        &self.fault
    }

    pub const fn expectation(&self) -> &PhysicalScenarioExpectation {
        &self.expectation
    }

    pub fn canonical_identity(
        &self,
    ) -> Result<PhysicalScenarioCanonicalIdentity, PhysicalScenarioDefinitionDenial> {
        PhysicalScenarioCanonicalIdentity::from_definition(self)
    }
}

fn require_named_production_boundary_yieldpoint(
    schedule: &PhysicalScenarioSchedule,
) -> Result<(), PhysicalScenarioDefinitionDenial> {
    if schedule.production_boundary_yieldpoint().trim().is_empty() {
        return Err(PhysicalScenarioDefinitionDenial::UnnamedProductionBoundaryYieldpoint);
    }
    Ok(())
}
