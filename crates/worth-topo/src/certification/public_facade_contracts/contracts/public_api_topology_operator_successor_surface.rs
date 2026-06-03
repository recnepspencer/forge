fn _topology_operator_successor_surface_contracts() {
    let _: fn(
        forge_relational::facade::identity::RelationId,
        topology::facade::LoopSuccessorKind,
        forge_relational::facade::identity::EntityId,
        forge_relational::facade::identity::EntityId,
    ) -> TopologyLoopSuccessorRewireMember =
        |relation_id, kind, half_edge_id, successor_half_edge_id| {
            TopologyLoopSuccessorRewireMember::new(
                relation_id,
                kind,
                half_edge_id,
                successor_half_edge_id,
            )
        };
    let _: fn(Vec<TopologyLoopSuccessorRewireMember>) -> TopologyRewireLoopSuccessorProgramDeclaration =
        TopologyRewireLoopSuccessorProgramDeclaration::new;
    let _: fn(
        TopologyRewireLoopSuccessorProgramDeclaration,
    ) -> TopologyOperatorGroupedInput<TopologyRewireLoopSuccessorProgramDeclaration> =
        topology_grouped_operator_neighborhood::<TopologyRewireLoopSuccessorProgramDeclaration>;
    let _: fn(
        TopologyOperatorGroupedInput<TopologyRewireLoopSuccessorProgramDeclaration>,
        ForgeQuerySupportContributionAuthoring,
    ) -> TopologyOperatorGroupedContributionInput<TopologyRewireLoopSuccessorProgramDeclaration> =
        |input, contribution| input.with_shared_support_contribution(contribution);
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorGroupedInput<TopologyRewireLoopSuccessorProgramDeclaration>,
    ) -> Result<
        TopologyOperatorGroupedDeclaration<TopologyRewireLoopSuccessorProgramDeclaration>,
        TopologyOperatorGroupedDeclarationStop,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::declare_topology_grouped_operator::<
        TopologyRewireLoopSuccessorProgramDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorGroupedDeclaration<TopologyRewireLoopSuccessorProgramDeclaration>,
    ) -> TopologyOperatorGroupedOutcome<TopologyRewireLoopSuccessorProgramDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_grouped_operator_outcome::<
            TopologyRewireLoopSuccessorProgramDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorGroupedContributionInput<TopologyRewireLoopSuccessorProgramDeclaration>,
    ) -> Result<
        TopologyOperatorGroupedContributionComposition<TopologyRewireLoopSuccessorProgramDeclaration>,
        TopologyOperatorGroupedContributionStop<TopologyRewireLoopSuccessorProgramDeclaration>,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::grouped_topology_operator_contributions_checked::<
        TopologyRewireLoopSuccessorProgramDeclaration,
    >;
}
