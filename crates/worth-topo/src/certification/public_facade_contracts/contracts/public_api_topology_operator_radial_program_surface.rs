fn _topology_operator_radial_program_surface_contracts() {
    let _: fn(
        forge_relational::facade::identity::RelationId,
        forge_relational::facade::identity::EntityId,
        forge_relational::facade::identity::EntityId,
    ) -> TopologyRadialSpliceMember = |relation_id, half_edge_id, radial_next_half_edge_id| {
        TopologyRadialSpliceMember::new(relation_id, half_edge_id, radial_next_half_edge_id)
    };
    let _: fn(Vec<TopologyRadialSpliceMember>) -> TopologySpliceRadialAdjacencyProgramDeclaration =
        TopologySpliceRadialAdjacencyProgramDeclaration::new;
    let _: fn(
        TopologySpliceRadialAdjacencyProgramDeclaration,
    ) -> TopologyOperatorGroupedInput<TopologySpliceRadialAdjacencyProgramDeclaration> =
        topology_grouped_operator_neighborhood::<TopologySpliceRadialAdjacencyProgramDeclaration>;
    let _: fn(
        TopologyOperatorGroupedInput<TopologySpliceRadialAdjacencyProgramDeclaration>,
        ForgeQuerySupportContributionAuthoring,
    ) -> TopologyOperatorGroupedContributionInput<TopologySpliceRadialAdjacencyProgramDeclaration> =
        |input, contribution| input.with_shared_support_contribution(contribution);
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorGroupedInput<TopologySpliceRadialAdjacencyProgramDeclaration>,
    ) -> Result<
        TopologyOperatorGroupedDeclaration<TopologySpliceRadialAdjacencyProgramDeclaration>,
        TopologyOperatorGroupedDeclarationStop,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::declare_topology_grouped_operator::<
        TopologySpliceRadialAdjacencyProgramDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorGroupedDeclaration<TopologySpliceRadialAdjacencyProgramDeclaration>,
    ) -> TopologyOperatorGroupedOutcome<TopologySpliceRadialAdjacencyProgramDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_grouped_operator_outcome::<
            TopologySpliceRadialAdjacencyProgramDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorGroupedContributionInput<TopologySpliceRadialAdjacencyProgramDeclaration>,
    ) -> Result<
        TopologyOperatorGroupedContributionComposition<TopologySpliceRadialAdjacencyProgramDeclaration>,
        TopologyOperatorGroupedContributionStop<TopologySpliceRadialAdjacencyProgramDeclaration>,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::grouped_topology_operator_contributions_checked::<
        TopologySpliceRadialAdjacencyProgramDeclaration,
    >;
}
