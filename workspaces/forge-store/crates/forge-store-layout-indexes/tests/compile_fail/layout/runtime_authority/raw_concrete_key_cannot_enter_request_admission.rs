use forge_store_layout_indexes::{
    AccessPlanSelector, AccessShapeContract, AdmittedPhysicalArtifactFamily,
    ConcretePhysicalKeyWitness,
};

fn bypass(
    family: AdmittedPhysicalArtifactFamily,
    key: ConcretePhysicalKeyWitness,
    shape: AccessShapeContract,
) {
    let _ = AccessPlanSelector.admit_request(family, key, shape);
}

fn main() {}
