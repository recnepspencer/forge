#[path = "../src/topology_operators/overlap_region_blueprint/mod.rs"]
mod overlap_region_blueprint;

use overlap_region_blueprint::{
    PlanarBooleanOverlapBlueprintCloseoutDenial, PlanarBooleanOverlapBlueprintRegistry,
    PlanarBooleanOverlapOperatorClassification as Class, PlanarBooleanOverlapOperatorRow,
    PlanarBooleanOverlapOperatorTruthAuthority as Authority,
    PlanarBooleanOverlapRequiredQuerySurface as Surface,
};

#[test]
fn phase_two_overlap_blueprint_closes_out() {
    let registry = PlanarBooleanOverlapBlueprintRegistry::phase_2();
    let closeout = registry.closeout();
    assert!(closeout.prepared_spatial_operators() > 0);
    assert!(closeout.query_graph_composition_programs() > 0);
    assert!(closeout.runtime_facing_validator_count() > 0);
}

#[test]
fn unnamed_helper_operator_is_rejected() {
    let registry = PlanarBooleanOverlapBlueprintRegistry::phase_2();
    let (matrix, validators) = registry.into_classification_matrix_and_validator_plan();
    let mut operators = matrix.operators().to_vec();
    operators.push(PlanarBooleanOverlapOperatorRow::new(
        "LocalOverlapHelper",
        Class::PreparedSpatialOnly,
        Authority::WorthSpatialPrepared,
        Surface::None,
    ));
    let denial =
        PlanarBooleanOverlapBlueprintRegistry::try_from_rows(operators, validators.validators().to_vec())
            .unwrap_err();
    assert_eq!(denial, PlanarBooleanOverlapBlueprintCloseoutDenial::UnexpectedOperatorName);
}
