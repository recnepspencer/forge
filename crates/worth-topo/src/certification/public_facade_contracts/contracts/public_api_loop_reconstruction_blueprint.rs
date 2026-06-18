use topology::facade::{
    PlanarBooleanLoopBlueprintCloseout, PlanarBooleanLoopBlueprintCloseoutDenial,
    PlanarBooleanLoopBlueprintRegistry, PlanarBooleanLoopBlueprintRegistryIdentity,
    PlanarBooleanLoopOperatorClassification as Class,
    PlanarBooleanLoopOperatorClassificationMatrix, PlanarBooleanLoopOperatorProofObligation,
    PlanarBooleanLoopOperatorRow, PlanarBooleanLoopOperatorTruthAuthority as Authority,
    PlanarBooleanLoopRequiredQuerySurface as Surface,
    PlanarBooleanLoopValidatorProofObligation, PlanarBooleanLoopValidatorRegistrationPlan,
    PlanarBooleanLoopValidatorRow, PlanarBooleanLoopValidatorRuntimeLane as Lane,
};

fn loop_operator_row<'a>(
    matrix: &'a PlanarBooleanLoopOperatorClassificationMatrix,
    operator_name: &str,
) -> &'a PlanarBooleanLoopOperatorRow {
    matrix
        .operator(operator_name)
        .expect("phase 2 loop matrix must expose named operator row")
}

fn loop_validator_row<'a>(
    plan: &'a PlanarBooleanLoopValidatorRegistrationPlan,
    validator_name: &str,
) -> &'a PlanarBooleanLoopValidatorRow {
    plan.validator(validator_name)
        .expect("phase 2 loop validator plan must expose named validator row")
}

#[test]
fn loop_reconstruction_public_blueprint_exposes_phase_2_artifacts() {
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let plan = registry.validator_registration_plan();
    let closeout = registry.closeout();

    let registration =
        loop_operator_row(&matrix, "RegisterLoopReconstructionOperatorDeclarationFamily");
    let ledger =
        loop_operator_row(&matrix, "AssemblePlanarBooleanLoopReconstructionLedger");
    let validator = loop_validator_row(&plan, "ValidateLoopValidatorRuntimeRegistration");

    let _: &PlanarBooleanLoopBlueprintCloseout = closeout;
    let _: Option<PlanarBooleanLoopBlueprintCloseoutDenial> = None;
    let _: &PlanarBooleanLoopBlueprintRegistryIdentity = registry.identity();

    assert_eq!(registration.classification(), Class::TopologyDeclarationFamily);
    assert_eq!(registration.required_query_surface(), Surface::TopologyDeclarationEntry);
    assert_eq!(ledger.classification(), Class::QueryGraphCompositionProgram);
    assert_eq!(ledger.required_query_surface(), Surface::QueryGraphComposition);
    assert!(validator.requires_runtime_lane(Lane::QueryGraphInvariantPack));
    assert!(validator.governs_topology_legality());
    assert_eq!(matrix.registry_identity(), registry.identity());
    assert_eq!(plan.registry_identity(), registry.identity());
    assert!(closeout.certified_phase_2_required_rows_present());
    assert_eq!(closeout.support_gated_future_operators(), 0);
}

#[test]
fn loop_reconstruction_public_blueprint_preserves_prepared_and_query_proof_obligations() {
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let plan = registry.validator_registration_plan();
    let prepared = loop_operator_row(&matrix, "RecoverBooleanLoopSourceCarriers");
    let invariant = loop_operator_row(&matrix, "RegisterLoopReconstructionGraphInvariantPack");
    let validator = loop_validator_row(&plan, "ValidateLoopOperatorQueryProgression");

    assert_eq!(prepared.truth_authority(), Authority::WorthSpatialPrepared);
    assert_eq!(prepared.required_query_surface(), Surface::None);
    assert!(prepared
        .proof_obligations()
        .contains(&PlanarBooleanLoopOperatorProofObligation::PreparedLoopProductOnly));
    assert!(prepared.proof_obligations().contains(
        &PlanarBooleanLoopOperatorProofObligation::NoTopologyTruthMutationInMilestone74
    ));

    assert_eq!(invariant.required_query_surface(), Surface::QueryInvariantRegistration);
    assert!(invariant.proof_obligations().contains(
        &PlanarBooleanLoopOperatorProofObligation::TypedGraphCompositionDomainInvariantDenial
    ));
    assert!(validator
        .proof_obligations()
        .contains(&PlanarBooleanLoopValidatorProofObligation::RuntimeFacingDenialPathTypedAndInspectable));
    assert!(loop_validator_row(&plan, "ValidateLoopValidatorRuntimeRegistration")
        .proof_obligations()
        .contains(&PlanarBooleanLoopValidatorProofObligation::QueryInvariantRuntimeRegistration));
    assert!(loop_validator_row(&plan, "ValidateTopologyDeclarationFamilyCanonicalEntries")
        .proof_obligations()
        .contains(&PlanarBooleanLoopValidatorProofObligation::TopologyDeclarationReviewDenial));
}

#[test]
fn loop_reconstruction_public_blueprint_registers_anti_theatre_fence_rows() {
    let matrix = PlanarBooleanLoopBlueprintRegistry::phase_2().operator_classification_matrix();

    for operator_name in [
        "RejectUnindexedLoopFragment",
        "RejectSyntheticLoopLedgerConstruction",
    ] {
        let operator = loop_operator_row(&matrix, operator_name);
        assert_eq!(operator.classification(), Class::QueryGraphCompositionProgram);
        assert_eq!(operator.required_query_surface(), Surface::QueryInvariantRegistration);
        assert!(operator.proof_obligations().contains(
            &PlanarBooleanLoopOperatorProofObligation::TypedGraphCompositionDomainInvariantDenial
        ));
    }
}
