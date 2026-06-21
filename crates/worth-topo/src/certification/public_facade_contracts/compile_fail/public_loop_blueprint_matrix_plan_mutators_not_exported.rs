use topology::facade::{
    PlanarBooleanLoopBlueprintRegistry, PlanarBooleanLoopOperatorClassification,
    PlanarBooleanLoopValidatorRuntimeLane,
};

fn main() {
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();

    let operator_matrix = registry.operator_classification_matrix();
    let _ = operator_matrix.without_operator_named("RegisterLoopReconstructionOperatorDeclarationFamily");
    let _ = operator_matrix.with_operator_classification(
        "RegisterLoopReconstructionOperatorDeclarationFamily",
        PlanarBooleanLoopOperatorClassification::PreparedSpatialOnly,
    );

    let validator_plan = registry.validator_registration_plan();
    let _ = validator_plan.without_validator_named("ValidateLoopValidatorRuntimeRegistration");
    let _ = validator_plan.with_validator_runtime_lane(
        "ValidateLoopValidatorRuntimeRegistration",
        PlanarBooleanLoopValidatorRuntimeLane::SpatialPreparedProductValidation,
    );
    let _ =
        validator_plan.with_validator_topology_legality("ValidateLoopValidatorRuntimeRegistration", false);
}
