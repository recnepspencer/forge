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

pub(crate) struct NativeScenarioDefinitionParts {
    pub(crate) label: String,
    pub(crate) family: PhysicalSimulationScenarioFamily,
    pub(crate) intent: PhysicalScenarioIntent,
    pub(crate) fixtures: Vec<StoreAspectBoundaryFact>,
    pub(crate) actors: Vec<PhysicalScenarioActor>,
    pub(crate) schedule: PhysicalScenarioSchedule,
    pub(crate) fault: PhysicalScenarioFault,
    pub(crate) expectation: PhysicalScenarioExpectation,
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
        Self::from_native_parts(NativeScenarioDefinitionParts {
            label: builder.label,
            family,
            intent,
            fixtures: builder.fixtures,
            actors: builder.actors,
            schedule,
            fault: builder.fault,
            expectation,
        })
    }

    pub(crate) fn from_native_parts(
        parts: NativeScenarioDefinitionParts,
    ) -> Result<Self, PhysicalScenarioDefinitionDenial> {
        require_named_production_boundary_yieldpoint(&parts.schedule)?;
        let actors = PhysicalScenarioActorSet::from_actors(parts.actors)?;
        let fixtures = PhysicalScenarioFixtureSet::from_fixtures(parts.fixtures)?;
        Ok(Self {
            label: parts.label,
            family: parts.family,
            intent: parts.intent,
            fixtures,
            actors,
            schedule: parts.schedule,
            fault: parts.fault,
            expectation: parts.expectation,
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
