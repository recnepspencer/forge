fn edge_split_operator_row<'a>(
    blueprint: &'a EdgeSplitOperatorBlueprint,
    operator_name: &str,
) -> &'a EdgeSplitOperatorRow {
    blueprint
        .operator(operator_name)
        .expect("phase 1 blueprint must expose named operator row")
}

fn edge_split_validator_row<'a>(
    blueprint: &'a EdgeSplitOperatorBlueprint,
    validator_name: &str,
) -> &'a EdgeSplitValidatorRow {
    blueprint
        .validator(validator_name)
        .expect("phase 1 blueprint must expose named validator row")
}

#[test]
fn edge_split_operator_declarations_expose_query_canonical_entries_and_family_markers() {
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();
    let declaration_registration =
        edge_split_operator_row(&blueprint, "RegisterEdgeSplitOperatorDeclarationFamily");
    let grouped = edge_split_operator_row(&blueprint, "SplitConnectedHalfEdgeSetToNewWire");

    let _: &EdgeSplitBlueprintCloseout = blueprint.closeout();
    let _: Option<EdgeSplitBlueprintCloseoutDenial> = None;
    let _: fn(
        &str,
        Vec<TopologyWireSplitHalfEdgeMember>,
    ) -> TopologySplitConnectedHalfEdgeSetToNewWireDeclaration =
        |wire_create_key, members| {
            TopologySplitConnectedHalfEdgeSetToNewWireDeclaration::new(wire_create_key, members)
        };

    assert_eq!(
        declaration_registration.classification(),
        EdgeSplitOperatorClassification::TopologyDeclarationFamily
    );
    assert_eq!(
        declaration_registration.required_query_surface(),
        EdgeSplitRequiredQuerySurface::TopologyDeclarationEntry
    );
    assert_eq!(
        declaration_registration.truth_authority(),
        EdgeSplitOperatorTruthAuthority::WorthTopoQueryDeclaration
    );
    assert_eq!(
        grouped.classification(),
        EdgeSplitOperatorClassification::TopologyGroupedDeclarationFamily
    );
    assert_eq!(
        grouped.topology_precedent(),
        Some("TopologySplitConnectedHalfEdgeSetToNewWireDeclaration")
    );
    assert!(blueprint
        .closeout()
        .certified_authoritative_topology_mutations_have_query_entries());
    assert!(blueprint
        .closeout()
        .certified_phase_1_required_rows_present());
    assert_eq!(blueprint.closeout().required_phase_1_operator_rows(), 8);
    assert_eq!(blueprint.closeout().required_phase_1_validator_rows(), 2);
}

#[test]
fn edge_split_blueprint_exposes_exact_required_phase_1_query_lanes() {
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();
    let required_operator_lanes = [
        (
            "RegisterEdgeSplitOperatorDeclarationFamily",
            EdgeSplitOperatorClassification::TopologyDeclarationFamily,
            EdgeSplitRequiredQuerySurface::TopologyDeclarationEntry,
        ),
        (
            "RegisterEdgeSplitGroupedOperatorFamily",
            EdgeSplitOperatorClassification::TopologyGroupedDeclarationFamily,
            EdgeSplitRequiredQuerySurface::TopologyGroupedDeclaration,
        ),
        (
            "RegisterEdgeSplitContributionWorkflow",
            EdgeSplitOperatorClassification::TopologyContributionWorkflow,
            EdgeSplitRequiredQuerySurface::TopologyContributionWorkflow,
        ),
        (
            "RegisterEdgeSplitGraphInvariantPack",
            EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
            EdgeSplitRequiredQuerySurface::QueryInvariantRegistration,
        ),
        (
            "MapSplitLedgerToTopologyOperatorDeclarations",
            EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
            EdgeSplitRequiredQuerySurface::QueryGraphComposition,
        ),
        (
            "ClassifyPreparedVsAuthoritativeSplitOperator",
            EdgeSplitOperatorClassification::PreparedSpatialOnly,
            EdgeSplitRequiredQuerySurface::None,
        ),
        (
            "ValidateSplitOperatorQueryProgression",
            EdgeSplitOperatorClassification::TopologyDeclarationFamily,
            EdgeSplitRequiredQuerySurface::TopologyDeclarationEntry,
        ),
        (
            "ValidateSplitValidatorRuntimeRegistration",
            EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
            EdgeSplitRequiredQuerySurface::QueryInvariantRegistration,
        ),
    ];

    for (operator_name, classification, query_surface) in required_operator_lanes {
        let row = edge_split_operator_row(&blueprint, operator_name);
        assert_eq!(row.classification(), classification);
        assert_eq!(row.required_query_surface(), query_surface);
    }

    let required_validator_lanes = [
        (
            "ValidateSplitOperatorQueryProgression",
            EdgeSplitValidatorRuntimeLane::TopologyDeclarationReview,
        ),
        (
            "ValidateSplitValidatorRuntimeRegistration",
            EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack,
        ),
    ];

    for (validator_name, runtime_lane) in required_validator_lanes {
        let row = edge_split_validator_row(&blueprint, validator_name);
        assert!(row.governs_topology_legality());
        assert!(row.requires_runtime_lane(runtime_lane));
    }
}

#[test]
fn edge_split_grouped_operator_workflow_preserves_grouped_support_and_contribution_evidence() {
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();
    let grouped_registration =
        edge_split_operator_row(&blueprint, "RegisterEdgeSplitGroupedOperatorFamily");
    let contribution_registration =
        edge_split_operator_row(&blueprint, "RegisterEdgeSplitContributionWorkflow");

    let _: fn(
        TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    ) -> TopologyOperatorGroupedInput<TopologySplitConnectedHalfEdgeSetToNewWireDeclaration> =
        topology_grouped_operator_neighborhood::<TopologySplitConnectedHalfEdgeSetToNewWireDeclaration>;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorGroupedInput<TopologySplitConnectedHalfEdgeSetToNewWireDeclaration>,
    ) -> Result<
        TopologyOperatorGroupedDeclaration<TopologySplitConnectedHalfEdgeSetToNewWireDeclaration>,
        TopologyOperatorGroupedDeclarationStop,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::declare_topology_grouped_operator::<
        TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorGroupedContributionInput<TopologySplitConnectedHalfEdgeSetToNewWireDeclaration>,
    ) -> Result<
        TopologyOperatorGroupedContributionComposition<TopologySplitConnectedHalfEdgeSetToNewWireDeclaration>,
        TopologyOperatorGroupedContributionStop<TopologySplitConnectedHalfEdgeSetToNewWireDeclaration>,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::grouped_topology_operator_contributions_checked::<
        TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    >;

    assert_eq!(
        grouped_registration.required_query_surface(),
        EdgeSplitRequiredQuerySurface::TopologyGroupedDeclaration
    );
    assert_eq!(
        contribution_registration.required_query_surface(),
        EdgeSplitRequiredQuerySurface::TopologyContributionWorkflow
    );
    assert!(grouped_registration
        .proof_obligations()
        .contains(&EdgeSplitOperatorProofObligation::GroupedSupportAndContributionEvidence));
    assert!(contribution_registration
        .proof_obligations()
        .contains(&EdgeSplitOperatorProofObligation::RetainedContributionSemanticProjection));
}

#[test]
fn edge_split_validators_register_through_invariant_or_declaration_review_lanes() {
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();
    let progression = edge_split_validator_row(&blueprint, "ValidateSplitOperatorQueryProgression");
    let runtime_registration =
        edge_split_validator_row(&blueprint, "ValidateSplitValidatorRuntimeRegistration");
    let prepared = edge_split_validator_row(&blueprint, "ValidateSplitFragmentDomainCoverage");

    assert!(progression.governs_topology_legality());
    assert!(progression.requires_runtime_lane(EdgeSplitValidatorRuntimeLane::TopologyDeclarationReview));
    assert!(runtime_registration.governs_topology_legality());
    assert!(
        runtime_registration.requires_runtime_lane(EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack)
    );
    assert!(!prepared.governs_topology_legality());
    assert!(
        prepared.requires_runtime_lane(EdgeSplitValidatorRuntimeLane::SpatialPreparedProductValidation)
    );
    assert!(blueprint
        .closeout()
        .certified_validators_use_runtime_visible_lanes());
}

#[test]
fn edge_split_endpoint_boundary_normalization_rows_are_publicly_registered() {
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();

    for operator_name in [
        "CollapseEndpointNoOpSplits",
        "RecordEndpointContactDecision",
        "ValidateEndpointNoOpSplitPolicy",
        "RejectEndpointSplitThatWouldCreateZeroLengthFragment",
    ] {
        let row = edge_split_operator_row(&blueprint, operator_name);
        assert_eq!(
            row.classification(),
            EdgeSplitOperatorClassification::PreparedSpatialOnly
        );
        assert_eq!(
            row.truth_authority(),
            EdgeSplitOperatorTruthAuthority::WorthSpatialPrepared
        );
        assert_eq!(row.required_query_surface(), EdgeSplitRequiredQuerySurface::None);
        assert!(!row.may_commit_topology_in_7_3());
    }

    for validator_name in [
        "ValidateEndpointNoOpSplitPolicy",
        "RejectEndpointSplitThatWouldCreateZeroLengthFragment",
    ] {
        let row = edge_split_validator_row(&blueprint, validator_name);
        assert!(!row.governs_topology_legality());
        assert!(row.requires_runtime_lane(
            EdgeSplitValidatorRuntimeLane::SpatialPreparedProductValidation
        ));
    }
}

#[test]
fn edge_split_overlap_chain_rows_are_publicly_registered_as_prepared_products() {
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();

    for operator_name in [
        "BuildOverlapEdgeChain",
        "ResolveEdgeEdgePartialOverlap",
        "ResolveCoincidentButOppositeSenseEdges",
        "ResolveCoincidentEdgesDifferentParameterization",
        "ClassifyOverlapChainBoundaryRole",
        "ValidateCoincidentOppositeSensePreservation",
    ] {
        let row = edge_split_operator_row(&blueprint, operator_name);
        assert_eq!(
            row.classification(),
            EdgeSplitOperatorClassification::PreparedSpatialOnly
        );
        assert_eq!(
            row.truth_authority(),
            EdgeSplitOperatorTruthAuthority::WorthSpatialPrepared
        );
        assert_eq!(row.required_query_surface(), EdgeSplitRequiredQuerySurface::None);
        assert!(!row.may_commit_topology_in_7_3());
        assert!(row
            .proof_obligations()
            .contains(&EdgeSplitOperatorProofObligation::PreparedSplitProductOnly));
        assert!(row
            .proof_obligations()
            .contains(&EdgeSplitOperatorProofObligation::NoTopologyTruthMutationInMilestone73));
    }

    let validator = edge_split_validator_row(&blueprint, "ValidateCoincidentOppositeSensePreservation");
    assert!(!validator.governs_topology_legality());
    assert!(validator.requires_runtime_lane(
        EdgeSplitValidatorRuntimeLane::SpatialPreparedProductValidation
    ));

    let conversion = edge_split_operator_row(&blueprint, "ConvertOverlapToSharedTopology");
    assert_eq!(
        conversion.classification(),
        EdgeSplitOperatorClassification::SupportGatedFutureTopologyMutation
    );
    assert!(!conversion.may_commit_topology_in_7_3());
}

#[test]
fn edge_split_chain_validation_rows_are_publicly_registered_as_prepared_products() {
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();

    for operator_name in [
        "ValidateSplitEdgeChainClosure",
        "ValidateSplitFragmentDomainCoverage",
        "ValidateNoDanglingSplitChainReferences",
        "ValidateOverlapChainFragmentReferences",
        "RejectSplitChainGapOrOverlap",
    ] {
        let row = edge_split_operator_row(&blueprint, operator_name);
        assert_eq!(
            row.classification(),
            EdgeSplitOperatorClassification::PreparedSpatialOnly
        );
        assert_eq!(
            row.truth_authority(),
            EdgeSplitOperatorTruthAuthority::WorthSpatialPrepared
        );
        assert_eq!(row.required_query_surface(), EdgeSplitRequiredQuerySurface::None);
        assert!(!row.may_commit_topology_in_7_3());
    }

    for validator_name in [
        "ValidateSplitEdgeChainClosure",
        "ValidateSplitFragmentDomainCoverage",
        "ValidateNoDanglingSplitChainReferences",
        "ValidateOverlapChainFragmentReferences",
        "RejectSplitChainGapOrOverlap",
    ] {
        let row = edge_split_validator_row(&blueprint, validator_name);
        assert!(!row.governs_topology_legality());
        assert!(row.requires_runtime_lane(
            EdgeSplitValidatorRuntimeLane::SpatialPreparedProductValidation
        ));
    }
}

#[test]
fn edge_split_graph_composition_rejects_domain_invalid_topology_with_typed_invariant_denial() {
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();
    let invariant_pack = edge_split_operator_row(&blueprint, "RegisterEdgeSplitGraphInvariantPack");
    let graph_program =
        edge_split_operator_row(&blueprint, "MapSplitLedgerToTopologyOperatorDeclarations");
    let runtime_registration =
        edge_split_validator_row(&blueprint, "ValidateSplitValidatorRuntimeRegistration");

    assert_eq!(
        invariant_pack.required_query_surface(),
        EdgeSplitRequiredQuerySurface::QueryInvariantRegistration
    );
    assert_eq!(
        graph_program.classification(),
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram
    );
    assert_eq!(
        graph_program.truth_authority(),
        EdgeSplitOperatorTruthAuthority::ForgeQueryGraphComposition
    );
    assert!(invariant_pack
        .proof_obligations()
        .contains(&EdgeSplitOperatorProofObligation::TypedGraphCompositionDomainInvariantDenial));
    assert!(runtime_registration
        .proof_obligations()
        .contains(&EdgeSplitValidatorProofObligation::RuntimeFacingDenialPathTypedAndInspectable));
}

#[test]
fn prepared_spatial_split_artifacts_cannot_be_called_as_authoritative_topology_mutations() {
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();
    let prepared = edge_split_operator_row(&blueprint, "BuildSplitEdgeFragments");
    let authoritative = edge_split_operator_row(&blueprint, "SplitConnectedHalfEdgeSetToNewWire");

    assert_eq!(
        prepared.classification(),
        EdgeSplitOperatorClassification::PreparedSpatialOnly
    );
    assert_eq!(
        prepared.required_query_surface(),
        EdgeSplitRequiredQuerySurface::None
    );
    assert!(!prepared.may_commit_topology_in_7_3());
    assert!(authoritative.may_commit_topology_in_7_3());

    for future_operator_name in [
        "SplitEdge",
        "SplitIntersectedEdges",
        "InsertVertexOnEdgeForTJunction",
        "SplitEdgeAtOverlapInterval",
        "SplitEdgeAndCurves",
        "ConvertOverlapToSharedTopology",
        "ExtractCoplanarOverlapLoops",
    ] {
        let future = edge_split_operator_row(&blueprint, future_operator_name);
        assert_eq!(
            future.classification(),
            EdgeSplitOperatorClassification::SupportGatedFutureTopologyMutation
        );
        assert!(!future.may_commit_topology_in_7_3());
        assert!(future.support_warning().is_some());
        assert!(future
            .proof_obligations()
            .contains(&EdgeSplitOperatorProofObligation::ExplicitFutureSupportPosture));
    }

    assert!(blueprint
        .closeout()
        .certified_prepared_spatial_products_do_not_claim_topology_authority());
}
