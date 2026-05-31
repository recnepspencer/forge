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
    ) -> Result<
        forge_query::facade::ForgeQueryDeclarationEnvelope<
            TopologyQueryDomain,
            TopologyRetireTopologyEntityDeclaration,
        >,
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationTerminalError<
            TopologyQueryDomain,
            TopologyRetireTopologyEntityDeclaration,
        >,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry::<
        TopologyRetireTopologyEntityDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyDetachBoundaryMembershipDeclaration,
    ) -> Result<
        forge_query::facade::ForgeQueryDeclarationEnvelope<
            TopologyQueryDomain,
            TopologyDetachBoundaryMembershipDeclaration,
        >,
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationTerminalError<
            TopologyQueryDomain,
            TopologyDetachBoundaryMembershipDeclaration,
        >,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry::<
        TopologyDetachBoundaryMembershipDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyRewireLoopEndpointDeclaration,
    ) -> Result<
        forge_query::facade::ForgeQueryDeclarationEnvelope<
            TopologyQueryDomain,
            TopologyRewireLoopEndpointDeclaration,
        >,
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationTerminalError<
            TopologyQueryDomain,
            TopologyRewireLoopEndpointDeclaration,
        >,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry::<
        TopologyRewireLoopEndpointDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyDetachShellOrWireMembershipDeclaration,
    ) -> Result<
        forge_query::facade::ForgeQueryDeclarationEnvelope<
            TopologyQueryDomain,
            TopologyDetachShellOrWireMembershipDeclaration,
        >,
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationTerminalError<
            TopologyQueryDomain,
            TopologyDetachShellOrWireMembershipDeclaration,
        >,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry::<
        TopologyDetachShellOrWireMembershipDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologySpliceRadialAdjacencyDeclaration,
    ) -> Result<
        forge_query::facade::ForgeQueryDeclarationEnvelope<
            TopologyQueryDomain,
            TopologySpliceRadialAdjacencyDeclaration,
        >,
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationTerminalError<
            TopologyQueryDomain,
            TopologySpliceRadialAdjacencyDeclaration,
        >,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry::<
        TopologySpliceRadialAdjacencyDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyDetachRadialAdjacencyDeclaration,
    ) -> Result<
        forge_query::facade::ForgeQueryDeclarationEnvelope<
            TopologyQueryDomain,
            TopologyDetachRadialAdjacencyDeclaration,
        >,
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationTerminalError<
            TopologyQueryDomain,
            TopologyDetachRadialAdjacencyDeclaration,
        >,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry::<
        TopologyDetachRadialAdjacencyDeclaration,
    >;
}
