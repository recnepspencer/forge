use forge_store_physical_certification::{
    CertifiedPhysicalScenario, PhysicalScenarioAuthorityWitness, PhysicalScenarioCanonicalIdentity,
    PhysicalSimulationScenarioDefinition,
};

fn main() {
    let definition: PhysicalSimulationScenarioDefinition = todo!();
    let identity: PhysicalScenarioCanonicalIdentity = todo!();
    let authority_witness: PhysicalScenarioAuthorityWitness = todo!();
    let _scenario = CertifiedPhysicalScenario {
        definition,
        identity,
        authority_witness,
    };
}
