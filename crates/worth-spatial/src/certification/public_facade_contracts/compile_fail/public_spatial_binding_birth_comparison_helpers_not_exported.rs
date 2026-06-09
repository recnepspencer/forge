use worth_spatial::facade::bindings::{
    evaluate_primitive_construction_birth_consequence,
    plan_primitive_construction_birth, AdmittedPrimitiveConstructionBirthConsequence,
    RejectedPrimitiveConstructionBirthConsequence, SpatialConstructionBirthConsequence,
    SpatialConstructionBirthMappingKind, SpatialConstructionBirthPlan,
    SpatialConstructionBirthRejectionKind,
};

fn main() {
    let _ = plan_primitive_construction_birth;
    let _ = evaluate_primitive_construction_birth_consequence;
    let _: Option<SpatialConstructionBirthPlan> = None;
    let _: Option<AdmittedPrimitiveConstructionBirthConsequence> = None;
    let _: Option<RejectedPrimitiveConstructionBirthConsequence> = None;
    let _: Option<SpatialConstructionBirthConsequence> = None;
    let _: Option<SpatialConstructionBirthMappingKind> = None;
    let _: Option<SpatialConstructionBirthRejectionKind> = None;
}
