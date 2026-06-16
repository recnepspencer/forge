use super::{
    EdgeSplitBlueprintCloseoutDenial, EdgeSplitOperatorBlueprint, EdgeSplitOperatorClassification,
    EdgeSplitOperatorProofObligation, EdgeSplitOperatorRow, EdgeSplitOperatorTruthAuthority,
    EdgeSplitRequiredQuerySurface, EdgeSplitValidatorProofObligation, EdgeSplitValidatorRow,
    EdgeSplitValidatorRuntimeLane,
};

mod prepared_product_rows;

#[test]
fn phase_1_blueprint_classifies_prepared_and_authoritative_split_lanes() {
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();

    assert_eq!(
        blueprint
            .operator("BuildOverlapEdgeChain")
            .expect("prepared overlap chain operator should be registered")
            .classification(),
        EdgeSplitOperatorClassification::PreparedSpatialOnly
    );
    assert_eq!(
        blueprint
            .operator("SplitIntersectedEdges")
            .expect("split intersected edges operator should be registered")
            .classification(),
        EdgeSplitOperatorClassification::SupportGatedFutureTopologyMutation
    );
    assert_eq!(
        blueprint
            .operator("MapSplitLedgerToTopologyOperatorDeclarations")
            .expect("graph mapping operator should be registered")
            .required_query_surface(),
        EdgeSplitRequiredQuerySurface::QueryGraphComposition
    );
    assert_eq!(
        blueprint
            .operator("ConvertOverlapToSharedTopology")
            .expect("future overlap conversion operator should be registered")
            .classification(),
        EdgeSplitOperatorClassification::SupportGatedFutureTopologyMutation
    );
    assert!(blueprint
        .operator("BuildOverlapEdgeChain")
        .expect("prepared overlap chain operator should be registered")
        .proof_obligations()
        .contains(&EdgeSplitOperatorProofObligation::NoTopologyTruthMutationInMilestone73));
}

#[test]
fn phase_1_closeout_proves_authority_lanes_and_validator_registration() {
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();
    let closeout = blueprint.closeout();

    assert!(closeout.certified_authoritative_topology_mutations_have_query_entries());
    assert!(closeout.certified_prepared_spatial_products_do_not_claim_topology_authority());
    assert!(closeout.certified_validators_use_runtime_visible_lanes());
    assert!(closeout.certified_phase_1_required_rows_present());
    assert_eq!(closeout.required_phase_1_operator_rows(), 8);
    assert_eq!(closeout.required_phase_1_validator_rows(), 2);
    assert!(closeout.prepared_spatial_operators() > 10);
    assert!(closeout.topology_grouped_declaration_operators() >= 2);
    assert!(closeout.query_graph_composition_programs() >= 3);
    assert!(closeout.runtime_facing_validator_count() >= 5);
}

#[test]
fn closeout_rejects_missing_required_phase_1_rows() {
    let missing_operator =
        EdgeSplitOperatorBlueprint::try_from_rows(vec![prepared("PreparedOnly")], vec![])
            .expect_err("missing required phase 1 operators must fail blueprint closeout");
    assert_eq!(
        missing_operator,
        EdgeSplitBlueprintCloseoutDenial::MissingRequiredOperator
    );

    let missing_validator = EdgeSplitOperatorBlueprint::try_from_rows(
        required_phase_1_operator_rows(),
        vec![EdgeSplitValidatorRow::new(
            "OnlyNonTopologyValidator",
            EdgeSplitValidatorRuntimeLane::SpatialPreparedProductValidation,
            false,
            &[EdgeSplitValidatorProofObligation::RuntimeFacingDenialPathTypedAndInspectable],
        )],
    )
    .expect_err("missing required phase 1 validators must fail blueprint closeout");
    assert_eq!(
        missing_validator,
        EdgeSplitBlueprintCloseoutDenial::MissingRequiredValidator
    );
}

#[test]
fn closeout_rejects_required_rows_with_wrong_lanes() {
    let mut wrong_operator_lane = required_phase_1_operator_rows();
    wrong_operator_lane
        .retain(|operator| operator.operator_name() != "RegisterEdgeSplitContributionWorkflow");
    wrong_operator_lane.push(query_graph("RegisterEdgeSplitContributionWorkflow"));

    let operator_denial = EdgeSplitOperatorBlueprint::try_from_rows(
        wrong_operator_lane,
        required_phase_1_validator_rows(),
    )
    .expect_err("required phase 1 operator rows must use their exact Query lanes");
    assert_eq!(
        operator_denial,
        EdgeSplitBlueprintCloseoutDenial::RequiredOperatorLaneMismatch
    );

    let mut wrong_validator_lane = required_phase_1_validator_rows();
    wrong_validator_lane.retain(|validator| {
        validator.validator_name() != "ValidateSplitValidatorRuntimeRegistration"
    });
    wrong_validator_lane.push(EdgeSplitValidatorRow::new(
        "ValidateSplitValidatorRuntimeRegistration",
        EdgeSplitValidatorRuntimeLane::SpatialPreparedProductValidation,
        false,
        &[EdgeSplitValidatorProofObligation::RuntimeFacingDenialPathTypedAndInspectable],
    ));

    let validator_denial = EdgeSplitOperatorBlueprint::try_from_rows(
        required_phase_1_operator_rows(),
        wrong_validator_lane,
    )
    .expect_err("required phase 1 validator rows must use their exact runtime lanes");
    assert_eq!(
        validator_denial,
        EdgeSplitBlueprintCloseoutDenial::RequiredValidatorLaneMismatch
    );
}

#[test]
fn closeout_rejects_duplicate_and_misclassified_operator_rows() {
    let duplicate = EdgeSplitOperatorBlueprint::try_from_rows(
        vec![prepared("DuplicateSplitOp"), prepared("DuplicateSplitOp")],
        vec![],
    )
    .expect_err("duplicate operator names must fail blueprint closeout");
    assert_eq!(
        duplicate,
        EdgeSplitBlueprintCloseoutDenial::DuplicateOperatorName
    );

    let prepared_claims_topology = EdgeSplitOperatorBlueprint::try_from_rows(
        vec![EdgeSplitOperatorRow::new(
            "PreparedClaimsTopology",
            EdgeSplitOperatorClassification::PreparedSpatialOnly,
            EdgeSplitOperatorTruthAuthority::WorthTopoQueryDeclaration,
            EdgeSplitRequiredQuerySurface::TopologyDeclarationEntry,
            None,
            &[EdgeSplitOperatorProofObligation::PreparedSplitProductOnly],
            None,
        )],
        vec![],
    )
    .expect_err("prepared operator must not claim topology authority");
    assert_eq!(
        prepared_claims_topology,
        EdgeSplitBlueprintCloseoutDenial::PreparedSpatialOperatorClaimsTopologyAuthority
    );

    let graph_without_graph_surface = EdgeSplitOperatorBlueprint::try_from_rows(
        vec![EdgeSplitOperatorRow::new(
            "GraphWithoutGraphSurface",
            EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
            EdgeSplitOperatorTruthAuthority::ForgeQueryGraphComposition,
            EdgeSplitRequiredQuerySurface::TopologyDeclarationEntry,
            None,
            &[EdgeSplitOperatorProofObligation::QueryGraphCompositionProgram],
            None,
        )],
        vec![],
    )
    .expect_err("graph composition operator must require graph composition surface");
    assert_eq!(
        graph_without_graph_surface,
        EdgeSplitBlueprintCloseoutDenial::GraphCompositionOperatorMissingGraphSurface
    );
}

#[test]
fn closeout_rejects_topology_legality_validator_without_runtime_lane() {
    let denial = EdgeSplitOperatorBlueprint::try_from_rows(
        vec![prepared("PreparedOnly")],
        vec![EdgeSplitValidatorRow::new(
            "ValidateTopologyButOnlySpatial",
            EdgeSplitValidatorRuntimeLane::SpatialPreparedProductValidation,
            true,
            &[EdgeSplitValidatorProofObligation::RuntimeFacingDenialPathTypedAndInspectable],
        )],
    )
    .expect_err("topology legality validators need a runtime-facing lane");

    assert_eq!(
        denial,
        EdgeSplitBlueprintCloseoutDenial::TopologyLegalityValidatorMissingRuntimeLane
    );
}

fn prepared(operator_name: &'static str) -> EdgeSplitOperatorRow {
    EdgeSplitOperatorRow::new(
        operator_name,
        EdgeSplitOperatorClassification::PreparedSpatialOnly,
        EdgeSplitOperatorTruthAuthority::WorthSpatialPrepared,
        EdgeSplitRequiredQuerySurface::None,
        None,
        &[EdgeSplitOperatorProofObligation::PreparedSplitProductOnly],
        None,
    )
}

fn required_phase_1_operator_rows() -> Vec<EdgeSplitOperatorRow> {
    vec![
        query_declaration("RegisterEdgeSplitOperatorDeclarationFamily"),
        query_grouped("RegisterEdgeSplitGroupedOperatorFamily"),
        query_contribution("RegisterEdgeSplitContributionWorkflow"),
        query_invariant("RegisterEdgeSplitGraphInvariantPack"),
        query_graph("MapSplitLedgerToTopologyOperatorDeclarations"),
        prepared("ClassifyPreparedVsAuthoritativeSplitOperator"),
        query_declaration("ValidateSplitOperatorQueryProgression"),
        query_invariant("ValidateSplitValidatorRuntimeRegistration"),
    ]
}

fn required_phase_1_validator_rows() -> Vec<EdgeSplitValidatorRow> {
    vec![
        EdgeSplitValidatorRow::new(
            "ValidateSplitOperatorQueryProgression",
            EdgeSplitValidatorRuntimeLane::TopologyDeclarationReview,
            true,
            &[EdgeSplitValidatorProofObligation::RuntimeFacingDenialPathTypedAndInspectable],
        ),
        EdgeSplitValidatorRow::new(
            "ValidateSplitValidatorRuntimeRegistration",
            EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack,
            true,
            &[EdgeSplitValidatorProofObligation::RuntimeFacingDenialPathTypedAndInspectable],
        ),
    ]
}

fn query_declaration(operator_name: &'static str) -> EdgeSplitOperatorRow {
    EdgeSplitOperatorRow::new(
        operator_name,
        EdgeSplitOperatorClassification::TopologyDeclarationFamily,
        EdgeSplitOperatorTruthAuthority::WorthTopoQueryDeclaration,
        EdgeSplitRequiredQuerySurface::TopologyDeclarationEntry,
        Some("TopologyOperatorWorkflowHandleExt"),
        &[EdgeSplitOperatorProofObligation::TopologyOperatorDeclarationReview],
        None,
    )
}

fn query_grouped(operator_name: &'static str) -> EdgeSplitOperatorRow {
    EdgeSplitOperatorRow::new(
        operator_name,
        EdgeSplitOperatorClassification::TopologyGroupedDeclarationFamily,
        EdgeSplitOperatorTruthAuthority::WorthTopoQueryDeclaration,
        EdgeSplitRequiredQuerySurface::TopologyGroupedDeclaration,
        Some("topology_grouped_operator_neighborhood"),
        &[EdgeSplitOperatorProofObligation::GroupedSupportAndContributionEvidence],
        None,
    )
}

fn query_contribution(operator_name: &'static str) -> EdgeSplitOperatorRow {
    EdgeSplitOperatorRow::new(
        operator_name,
        EdgeSplitOperatorClassification::TopologyContributionWorkflow,
        EdgeSplitOperatorTruthAuthority::WorthTopoQueryDeclaration,
        EdgeSplitRequiredQuerySurface::TopologyContributionWorkflow,
        Some("topology_operator_contribution_workflow"),
        &[EdgeSplitOperatorProofObligation::RetainedContributionSemanticProjection],
        None,
    )
}

fn query_graph(operator_name: &'static str) -> EdgeSplitOperatorRow {
    EdgeSplitOperatorRow::new(
        operator_name,
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitOperatorTruthAuthority::ForgeQueryGraphComposition,
        EdgeSplitRequiredQuerySurface::QueryGraphComposition,
        Some("workspace.compose_graph_with_invariant_pack"),
        &[EdgeSplitOperatorProofObligation::QueryGraphCompositionProgram],
        None,
    )
}

fn query_invariant(operator_name: &'static str) -> EdgeSplitOperatorRow {
    EdgeSplitOperatorRow::new(
        operator_name,
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitOperatorTruthAuthority::ForgeQueryGraphComposition,
        EdgeSplitRequiredQuerySurface::QueryInvariantRegistration,
        Some("ForgeQueryRuntime::builder().invariant_registration_artifact"),
        &[EdgeSplitOperatorProofObligation::TypedGraphCompositionDomainInvariantDenial],
        None,
    )
}
