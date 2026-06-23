use super::super::{
    PlanarBooleanLoopBlueprintRegistry, PlanarBooleanLoopOperatorClassification as Class,
    PlanarBooleanLoopRequiredQuerySurface as Surface,
    PlanarBooleanLoopValidatorRuntimeLane as Lane,
};

#[test]
fn phase_15_consumption_surfaces_are_frozen_by_phase_2_artifacts() {
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let plan = registry.validator_registration_plan();

    assert_eq!(matrix.registry_identity(), registry.identity());
    assert_eq!(plan.registry_identity(), registry.identity());

    for operator_name in [
        "RequireBooleanLoopReconstructionEvidence",
        "RegisterBooleanLoopReconstructionStageRequirement",
        "ReplayPlanarBooleanLoopReconstruction",
        "CompareLoopReconstructionReplayParity",
        "CompareLoopReconstructionCheckpointParity",
    ] {
        let operator = matrix
            .operator(operator_name)
            .expect("phase 15 loop evidence and replay operator should be frozen in phase 2");
        assert_eq!(
            operator.classification(),
            Class::QueryGraphCompositionProgram
        );
    }

    assert_eq!(
        matrix
            .operator("RegisterBooleanLoopReconstructionStageRequirement")
            .expect("stage requirement registration operator should be frozen")
            .required_query_surface(),
        Surface::QueryInvariantRegistration
    );

    for validator_name in [
        "ValidatePlanarBooleanLoopReplayParity",
        "ValidatePlanarBooleanLoopCheckpointParity",
        "ValidateLoopValidatorRuntimeRegistration",
        "ValidateLoopGraphInvariantPackRegistration",
    ] {
        let validator = plan
            .validator(validator_name)
            .expect("phase 15 validator lane should be frozen in phase 2");
        assert_eq!(validator.runtime_lane(), Lane::QueryGraphInvariantPack);
        assert!(validator.governs_topology_legality());
    }
}
