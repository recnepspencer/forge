use super::{
    PlanarBooleanLoopBlueprintCloseoutDenial, PlanarBooleanLoopBlueprintRegistry,
    PlanarBooleanLoopOperatorClassification as Class,
    PlanarBooleanLoopOperatorProofObligation as OperatorProof, PlanarBooleanLoopOperatorRow,
    PlanarBooleanLoopOperatorTruthAuthority as Authority,
    PlanarBooleanLoopRequiredQuerySurface as Surface,
    PlanarBooleanLoopValidatorProofObligation as ValidatorProof, PlanarBooleanLoopValidatorRow,
    PlanarBooleanLoopValidatorRuntimeLane as Lane,
};
use std::collections::BTreeSet;

#[path = "tests_phase_15.rs"]
mod tests_phase_15;

#[test]
fn phase_2_registry_closes_exact_required_inventory_and_derived_views() {
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let plan = registry.validator_registration_plan();
    let operator_names = names(
        registry
            .operators()
            .iter()
            .map(|operator| operator.operator_name()),
    );
    let required_operator_names = names(registry.required_operator_names());
    let validator_names = names(
        registry
            .validators()
            .iter()
            .map(|validator| validator.validator_name()),
    );
    let required_validator_names = names(registry.required_validator_names());

    assert_eq!(operator_names, required_operator_names);
    assert_eq!(validator_names, required_validator_names);
    assert_eq!(matrix.registry_identity(), registry.identity());
    assert_eq!(plan.registry_identity(), registry.identity());
    assert!(registry
        .closeout()
        .certified_phase_2_required_rows_present());
    assert_eq!(registry.closeout().support_gated_future_operators(), 0);
    assert_eq!(
        matrix
            .operator("RecoverBooleanLoopSourceCarriers")
            .expect("prepared source-carrier operator should be registered")
            .classification(),
        Class::PreparedSpatialOnly
    );
    assert_eq!(
        matrix
            .operator("RegisterLoopReconstructionGroupedOperatorFamily")
            .expect("grouped registration operator should be registered")
            .classification(),
        Class::TopologyGroupedDeclarationFamily
    );
    assert_eq!(
        matrix
            .operator("PropagatePersistentNamesThroughLoopReconstruction")
            .expect("loop naming contribution should be registered")
            .classification(),
        Class::TopologyContributionWorkflow
    );
    assert_eq!(
        matrix
            .operator("AssemblePlanarBooleanLoopReconstructionLedger")
            .expect("loop ledger assembly operator should be registered")
            .required_query_surface(),
        Surface::QueryGraphComposition
    );

    assert!(plan
        .validator("ValidateLoopCarrierCoverage")
        .expect("carrier coverage validator should be registered")
        .requires_runtime_lane(Lane::SpatialPreparedProductValidation));
    assert!(plan
        .validator("ValidateLoopIslandPartitionConsistency")
        .expect("loop island partition validator should be registered")
        .requires_runtime_lane(Lane::SpatialPreparedProductValidation));
    assert!(plan
        .validator("ValidateLoopOperatorQueryProgression")
        .expect("operator progression validator should be registered")
        .requires_runtime_lane(Lane::TopologyDeclarationReview));
    assert!(plan
        .validator("ValidateLoopValidatorRuntimeRegistration")
        .expect("runtime registration validator should be registered")
        .requires_runtime_lane(Lane::QueryGraphInvariantPack));
    assert_eq!(registry.closeout().required_phase_2_operator_rows(), 60);
    assert_eq!(registry.closeout().required_phase_2_validator_rows(), 46);
    assert_eq!(
        registry.closeout().topology_grouped_declaration_operators(),
        1
    );
}

#[test]
fn phase_2_registry_preserves_query_fences_and_validator_obligations() {
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let plan = registry.validator_registration_plan();

    for operator_name in [
        "RejectUnindexedLoopFragment",
        "RejectSyntheticLoopLedgerConstruction",
    ] {
        let operator = matrix
            .operator(operator_name)
            .expect("public-contract fence operator should be registered");
        assert_eq!(
            operator.classification(),
            Class::QueryGraphCompositionProgram
        );
        assert_eq!(
            operator.required_query_surface(),
            Surface::QueryInvariantRegistration
        );
        assert!(operator
            .proof_obligations()
            .contains(&OperatorProof::TypedGraphCompositionDomainInvariantDenial));
    }

    let runtime_registration = plan
        .validator("ValidateLoopValidatorRuntimeRegistration")
        .expect("runtime registration validator should be registered");
    let topology_review = plan
        .validator("ValidateTopologyDeclarationFamilyCanonicalEntries")
        .expect("topology declaration review validator should be registered");

    assert!(runtime_registration
        .proof_obligations()
        .contains(&ValidatorProof::QueryInvariantRuntimeRegistration));
    assert!(topology_review
        .proof_obligations()
        .contains(&ValidatorProof::TopologyDeclarationReviewDenial));
}

#[test]
fn phase_2_registry_rejects_missing_required_rows_and_wrong_runtime_lanes() {
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let mut operators = registry.operators().to_vec();
    operators.retain(|operator| operator.operator_name() != "BuildReconstructedLoop");
    let missing_operator = PlanarBooleanLoopBlueprintRegistry::try_from_rows(
        operators,
        registry.validators().to_vec(),
    )
    .expect_err("missing required loop operators must fail closeout");
    assert_eq!(
        missing_operator,
        PlanarBooleanLoopBlueprintCloseoutDenial::MissingRequiredOperator
    );

    let replacement_validator = PlanarBooleanLoopValidatorRow::new(
        "ValidateLoopOperatorQueryProgression",
        Lane::SpatialPreparedProductValidation,
        true,
        &[ValidatorProof::RuntimeFacingDenialPathTypedAndInspectable],
    );
    let mut validators = registry.validators().to_vec();
    validators
        .retain(|validator| validator.validator_name() != "ValidateLoopOperatorQueryProgression");
    validators.push(replacement_validator);
    let wrong_validator_lane = PlanarBooleanLoopBlueprintRegistry::try_from_rows(
        registry.operators().to_vec(),
        validators,
    )
    .expect_err("topology legality validators need their declared runtime lanes");
    assert_eq!(
        wrong_validator_lane,
        PlanarBooleanLoopBlueprintCloseoutDenial::TopologyLegalityValidatorMissingRuntimeLane
    );
}

#[test]
fn phase_2_registry_rejects_duplicate_and_lane_dishonest_rows() {
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let mut operators = registry.operators().to_vec();
    operators.push(registry.operators()[0].clone());
    let duplicate = PlanarBooleanLoopBlueprintRegistry::try_from_rows(operators, vec![])
        .expect_err("duplicate operator names must fail closeout");
    assert_eq!(
        duplicate,
        PlanarBooleanLoopBlueprintCloseoutDenial::DuplicateOperatorName
    );

    let prepared_claims_topology = PlanarBooleanLoopBlueprintRegistry::try_from_rows(
        vec![PlanarBooleanLoopOperatorRow::new(
            "PreparedClaimsTopology",
            Class::PreparedSpatialOnly,
            Authority::WorthTopoQueryDeclaration,
            Surface::TopologyDeclarationEntry,
            None,
            &[OperatorProof::PreparedLoopProductOnly],
            None,
        )],
        vec![],
    )
    .expect_err("prepared operators must not claim topology authority");
    assert_eq!(
        prepared_claims_topology,
        PlanarBooleanLoopBlueprintCloseoutDenial::PreparedSpatialOperatorClaimsTopologyAuthority
    );
}

fn names<'a>(values: impl Iterator<Item = &'a str>) -> BTreeSet<&'a str> {
    values.collect()
}
