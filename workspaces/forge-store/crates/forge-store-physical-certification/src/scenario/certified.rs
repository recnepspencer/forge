use super::authority::PhysicalScenarioAuthorityWitness;
use super::definition::PhysicalSimulationScenarioDefinition;
use super::identity::PhysicalScenarioCanonicalIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertifiedPhysicalScenario {
    definition: PhysicalSimulationScenarioDefinition,
    identity: PhysicalScenarioCanonicalIdentity,
    authority_witness: PhysicalScenarioAuthorityWitness,
}

impl CertifiedPhysicalScenario {
    pub(crate) fn from_admitted_definition(
        definition: PhysicalSimulationScenarioDefinition,
        identity: PhysicalScenarioCanonicalIdentity,
        authority_witness: PhysicalScenarioAuthorityWitness,
    ) -> Self {
        Self {
            definition,
            identity,
            authority_witness,
        }
    }

    pub const fn definition(&self) -> &PhysicalSimulationScenarioDefinition {
        &self.definition
    }

    pub const fn identity(&self) -> &PhysicalScenarioCanonicalIdentity {
        &self.identity
    }

    pub const fn authority_witness(&self) -> &PhysicalScenarioAuthorityWitness {
        &self.authority_witness
    }
}
