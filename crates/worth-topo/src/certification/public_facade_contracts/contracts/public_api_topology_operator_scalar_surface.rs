fn _topology_operator_scalar_surface_contracts() {
    let _: fn(
        forge_relational::facade::identity::EntityId,
        schema::facade::platform::entities::TopologyEntityKind,
    ) -> TopologyRetireTopologyEntityDeclaration =
        TopologyRetireTopologyEntityDeclaration::new;
    let _: fn(
        forge_relational::facade::identity::RelationId,
        topology::facade::BoundaryMembershipKind,
    ) -> TopologyDetachBoundaryMembershipDeclaration =
        TopologyDetachBoundaryMembershipDeclaration::new;
    let _: fn(
        forge_relational::facade::identity::RelationId,
        topology::facade::LoopEndpointKind,
        forge_relational::facade::identity::EntityId,
        forge_relational::facade::identity::EntityId,
    ) -> TopologyRewireLoopEndpointDeclaration = TopologyRewireLoopEndpointDeclaration::new;
    let _: fn(
        forge_relational::facade::identity::RelationId,
        topology::facade::ShellOrWireMembershipKind,
    ) -> TopologyDetachShellOrWireMembershipDeclaration =
        TopologyDetachShellOrWireMembershipDeclaration::new;
    let _: fn(
        forge_relational::facade::identity::RelationId,
        forge_relational::facade::identity::EntityId,
        forge_relational::facade::identity::EntityId,
    ) -> TopologySpliceRadialAdjacencyDeclaration =
        TopologySpliceRadialAdjacencyDeclaration::new;
    let _: fn(forge_relational::facade::identity::RelationId)
        -> TopologyDetachRadialAdjacencyDeclaration =
        TopologyDetachRadialAdjacencyDeclaration::new;

    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyRetireTopologyEntityDeclaration,
    ) -> TopologyOperatorDeclarationOutcome<TopologyRetireTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_outcome::<
            TopologyRetireTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyDetachBoundaryMembershipDeclaration,
    ) -> TopologyOperatorDeclarationOutcome<TopologyDetachBoundaryMembershipDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_outcome::<
            TopologyDetachBoundaryMembershipDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyRewireLoopEndpointDeclaration,
    ) -> TopologyOperatorDeclarationOutcome<TopologyRewireLoopEndpointDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_outcome::<
            TopologyRewireLoopEndpointDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyDetachShellOrWireMembershipDeclaration,
    ) -> TopologyOperatorDeclarationOutcome<TopologyDetachShellOrWireMembershipDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_outcome::<
            TopologyDetachShellOrWireMembershipDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologySpliceRadialAdjacencyDeclaration,
    ) -> TopologyOperatorDeclarationOutcome<TopologySpliceRadialAdjacencyDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_outcome::<
            TopologySpliceRadialAdjacencyDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyDetachRadialAdjacencyDeclaration,
    ) -> TopologyOperatorDeclarationOutcome<TopologyDetachRadialAdjacencyDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_outcome::<
            TopologyDetachRadialAdjacencyDeclaration,
        >;
}
