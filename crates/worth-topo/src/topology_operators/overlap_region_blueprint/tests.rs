use super::classification::{
    PlanarBooleanOverlapOperatorClassification as Class,
    PlanarBooleanOverlapOperatorTruthAuthority as Authority,
    PlanarBooleanOverlapRequiredQuerySurface as Surface,
    PlanarBooleanOverlapValidatorRuntimeLane as Lane,
};
use super::closeout::PlanarBooleanOverlapBlueprintCloseoutDenial as Denial;
use super::operator_row::PlanarBooleanOverlapOperatorRow;
use super::registry::PlanarBooleanOverlapBlueprintRegistry;

#[test]
fn phase_two_overlap_blueprint_closes_out() {
    let registry = PlanarBooleanOverlapBlueprintRegistry::phase_2();
    assert!(registry.closeout().prepared_spatial_operators() > 0);
    assert!(registry.closeout().query_graph_composition_programs() > 0);
    assert!(registry.closeout().runtime_facing_validator_count() > 0);
}

#[test]
fn missing_required_operator_is_rejected() {
    let registry = PlanarBooleanOverlapBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let validators = registry.validator_registration_plan();
    let operators = matrix
        .without_operator_named("ConsumePlanarBooleanLoopReconstructionLedger")
        .operators()
        .to_vec();
    let denial = PlanarBooleanOverlapBlueprintRegistry::try_from_rows(
        operators,
        validators.validators().to_vec(),
    )
    .unwrap_err();
    assert_eq!(denial, Denial::MissingRequiredOperator);
}

#[test]
fn unnamed_helper_operator_is_rejected() {
    let registry = PlanarBooleanOverlapBlueprintRegistry::phase_2();
    let mut operators = registry
        .operator_classification_matrix()
        .operators()
        .to_vec();
    operators.push(PlanarBooleanOverlapOperatorRow::new(
        "LocalOverlapHelper",
        Class::PreparedSpatialOnly,
        Authority::WorthSpatialPrepared,
        Surface::None,
    ));
    let denial = PlanarBooleanOverlapBlueprintRegistry::try_from_rows(
        operators,
        registry.validator_registration_plan().validators().to_vec(),
    )
    .unwrap_err();
    assert_eq!(denial, Denial::UnexpectedOperatorName);
}

#[test]
fn phase_fifteen_overlap_replay_and_checkpoint_validators_stay_on_query_graph_invariant_pack() {
    let registry = PlanarBooleanOverlapBlueprintRegistry::phase_2();
    let plan = registry.validator_registration_plan();
    for validator_name in [
        "ValidatePlanarBooleanOverlapRegionReplayParity",
        "ValidatePlanarBooleanOverlapRegionCheckpointParity",
    ] {
        let validator = plan
            .validator(validator_name)
            .expect("phase 15 overlap validator should be registered");
        assert_eq!(validator.runtime_lane(), Lane::QueryGraphInvariantPack);
        assert!(validator.governs_topology_legality());
    }
}
