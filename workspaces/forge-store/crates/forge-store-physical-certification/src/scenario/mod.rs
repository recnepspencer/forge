mod authority;
mod canonical_basis;
mod certified;
mod collections;
mod definition;
mod denial;
mod identity;
mod proof_progression;
mod vocabulary;

pub use authority::PhysicalScenarioAuthorityWitness;
pub use certified::CertifiedPhysicalScenario;
pub use collections::{PhysicalScenarioActorSet, PhysicalScenarioFixtureSet};
pub use definition::PhysicalSimulationScenarioDefinition;
pub use denial::{
    JsonScenarioAuthorityDenied, PhysicalScenarioDefinitionDenial, TerminalProjectionScenarioDenied,
};
pub use identity::PhysicalScenarioCanonicalIdentity;
pub(crate) use proof_progression::certify_scenario_definition;
pub use proof_progression::reject_raw_json_scenario_authority_attempt;
pub use vocabulary::{
    PhysicalScenarioActor, PhysicalScenarioActorRole, PhysicalScenarioExpectation,
    PhysicalScenarioExpectationKind, PhysicalScenarioFault, PhysicalScenarioFaultKind,
    PhysicalScenarioIntent, PhysicalScenarioNonClaim, PhysicalScenarioSchedule,
    PhysicalSimulationScenarioFamily,
};
