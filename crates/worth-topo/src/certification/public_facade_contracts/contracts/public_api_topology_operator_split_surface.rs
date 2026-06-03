fn _topology_operator_split_surface_contracts() {
    let _: fn(&str, forge_relational::facade::identity::EntityId) -> TopologyWireSplitHalfEdgeMember =
        |relation_create_key, half_edge_id| {
            TopologyWireSplitHalfEdgeMember::new(relation_create_key, half_edge_id)
        };
    let _: fn(
        &str,
        Vec<TopologyWireSplitHalfEdgeMember>,
    ) -> TopologySplitConnectedHalfEdgeSetToNewWireDeclaration =
        |wire_create_key, members| {
            TopologySplitConnectedHalfEdgeSetToNewWireDeclaration::new(
                wire_create_key,
                members,
            )
        };
    let _: fn(
        &str,
        &str,
        &str,
        forge_relational::facade::identity::EntityId,
        forge_relational::facade::identity::EntityId,
    ) -> TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration =
        |shell_create_key,
         region_relation_create_key,
         face_relation_create_key,
         region_id,
         face_id| {
            TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration::new(
                shell_create_key,
                region_relation_create_key,
                face_relation_create_key,
                region_id,
                face_id,
            )
        };
    let _: fn(
        TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    ) -> TopologyOperatorGroupedInput<TopologySplitConnectedHalfEdgeSetToNewWireDeclaration> =
        topology_grouped_operator_neighborhood::<TopologySplitConnectedHalfEdgeSetToNewWireDeclaration>;
    let _: fn(
        TopologyOperatorGroupedInput<TopologySplitConnectedHalfEdgeSetToNewWireDeclaration>,
        ForgeQuerySupportContributionAuthoring,
    ) -> TopologyOperatorGroupedContributionInput<TopologySplitConnectedHalfEdgeSetToNewWireDeclaration> =
        |input, contribution| input.with_shared_support_contribution(contribution);
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
        TopologyOperatorGroupedDeclaration<TopologySplitConnectedHalfEdgeSetToNewWireDeclaration>,
    ) -> TopologyOperatorGroupedOutcome<TopologySplitConnectedHalfEdgeSetToNewWireDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_grouped_operator_outcome::<
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
    let _: fn(
        TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
    ) -> TopologyOperatorGroupedInput<TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration> =
        topology_grouped_operator_neighborhood::<TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration>;
    let _: fn(
        TopologyOperatorGroupedInput<TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration>,
        ForgeQuerySupportContributionAuthoring,
    ) -> TopologyOperatorGroupedContributionInput<TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration> =
        |input, contribution| input.with_shared_support_contribution(contribution);
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorGroupedInput<TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration>,
    ) -> Result<
        TopologyOperatorGroupedDeclaration<TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration>,
        TopologyOperatorGroupedDeclarationStop,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::declare_topology_grouped_operator::<
        TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorGroupedDeclaration<TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration>,
    ) -> TopologyOperatorGroupedOutcome<TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_grouped_operator_outcome::<
            TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorGroupedContributionInput<TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration>,
    ) -> Result<
        TopologyOperatorGroupedContributionComposition<TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration>,
        TopologyOperatorGroupedContributionStop<TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration>,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::grouped_topology_operator_contributions_checked::<
        TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
    >;
}
