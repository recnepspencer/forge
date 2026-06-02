fn _topology_operator_grouped_rehome_surface_contracts() {
    let _: fn(&str, forge_relational::facade::identity::EntityId) -> TopologyWireRehomeHalfEdgeMember =
        |relation_create_key, half_edge_id| {
            TopologyWireRehomeHalfEdgeMember::new(relation_create_key, half_edge_id)
        };
    let _: fn(&str, forge_relational::facade::identity::EntityId) -> TopologyShellRehomeFaceMember =
        |relation_create_key, face_id| {
            TopologyShellRehomeFaceMember::new(relation_create_key, face_id)
        };
    let _: fn(
        &str,
        forge_relational::facade::identity::EntityId,
        Vec<TopologyWireRehomeHalfEdgeMember>,
    ) -> TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration =
        |wire_create_key, retired_wire_id, members| {
            TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration::new(
                wire_create_key,
                retired_wire_id,
                members,
            )
        };
    let _: fn(
        &str,
        &str,
        forge_relational::facade::identity::EntityId,
        forge_relational::facade::identity::EntityId,
        Vec<TopologyShellRehomeFaceMember>,
    ) -> TopologyRehomeAllOwnedFacesToNewShellDeclaration =
        |shell_create_key, region_relation_create_key, region_id, retired_shell_id, members| {
            TopologyRehomeAllOwnedFacesToNewShellDeclaration::new(
                shell_create_key,
                region_relation_create_key,
                region_id,
                retired_shell_id,
                members,
            )
        };
    let _: fn(
        TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    ) -> TopologyOperatorGroupedInput<TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration> =
        topology_grouped_operator_neighborhood::<TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration>;
    let _: fn(
        TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    ) -> TopologyOperatorGroupedInput<TopologyRehomeAllOwnedFacesToNewShellDeclaration> =
        topology_grouped_operator_neighborhood::<TopologyRehomeAllOwnedFacesToNewShellDeclaration>;
    let _: fn(
        TopologyOperatorGroupedInput<TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration>,
        ForgeQuerySupportContributionAuthoring,
    ) -> TopologyOperatorGroupedContributionInput<TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration> =
        |input, contribution| input.with_shared_support_contribution(contribution);
    let _: fn(
        TopologyOperatorGroupedInput<TopologyRehomeAllOwnedFacesToNewShellDeclaration>,
        ForgeQuerySupportContributionAuthoring,
    ) -> TopologyOperatorGroupedContributionInput<TopologyRehomeAllOwnedFacesToNewShellDeclaration> =
        |input, contribution| input.with_shared_support_contribution(contribution);
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorGroupedInput<TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration>,
    ) -> Result<
        TopologyOperatorGroupedDeclaration<TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration>,
        TopologyOperatorGroupedDeclarationStop,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::declare_topology_grouped_operator::<
        TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorGroupedDeclaration<TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration>,
    ) -> TopologyOperatorGroupedOutcome<TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_grouped_operator_outcome::<
            TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        &TopologyOperatorGroupedDeclaration<TopologyRehomeAllOwnedFacesToNewShellDeclaration>,
    ) -> forge_query::facade::ForgeQueryGroupedSupportReport =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::topology_grouped_operator_support::<
            TopologyRehomeAllOwnedFacesToNewShellDeclaration,
        >;
    let _: Option<
        TopologyOperatorGroupedContributionComposition<TopologyRehomeAllOwnedFacesToNewShellDeclaration>,
    > = None;
    let _: Option<
        TopologyOperatorGroupedContributionStop<TopologyRehomeAllOwnedFacesToNewShellDeclaration>,
    > = None;
    let _: Option<TopologyOperatorGroupedContributionMemberContext> = None;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorGroupedContributionInput<TopologyRehomeAllOwnedFacesToNewShellDeclaration>,
    ) -> Result<
        TopologyOperatorGroupedContributionComposition<TopologyRehomeAllOwnedFacesToNewShellDeclaration>,
        TopologyOperatorGroupedContributionStop<TopologyRehomeAllOwnedFacesToNewShellDeclaration>,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::grouped_topology_operator_contributions_checked::<
        TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    ) -> TopologyOperatorDeclarationOutcome<TopologyRehomeAllOwnedFacesToNewShellDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_outcome::<
            TopologyRehomeAllOwnedFacesToNewShellDeclaration,
        >;
}
