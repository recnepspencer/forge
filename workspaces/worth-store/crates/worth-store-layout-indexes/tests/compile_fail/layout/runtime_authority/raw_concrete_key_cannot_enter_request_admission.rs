use worth_store_layout_indexes::{
    AccessPlanSelector, AccessShapeContract, AdmittedLayoutMaterialization,
    AdmittedPhysicalArtifactFamily,
};
use worth_store_layout_indexes::strategy_declarations::ConcretePhysicalKeyWitness;

fn bypass(
    family: AdmittedPhysicalArtifactFamily,
    key: ConcretePhysicalKeyWitness,
    materialization: AdmittedLayoutMaterialization,
    shape: AccessShapeContract,
) {
    let _ = AccessPlanSelector.admit_read_request(family, key, materialization, shape);
}

fn main() {}
