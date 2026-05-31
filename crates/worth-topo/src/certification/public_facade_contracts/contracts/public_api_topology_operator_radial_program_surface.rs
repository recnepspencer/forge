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
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologySpliceRadialAdjacencyProgramDeclaration,
    ) -> Result<
        forge_query::facade::ForgeQueryDeclarationEnvelope<
            TopologyQueryDomain,
            TopologySpliceRadialAdjacencyProgramDeclaration,
        >,
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationTerminalError<
            TopologyQueryDomain,
            TopologySpliceRadialAdjacencyProgramDeclaration,
        >,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry::<
        TopologySpliceRadialAdjacencyProgramDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologySpliceRadialAdjacencyProgramDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationChecked<
        TopologyQueryDomain,
        TopologySpliceRadialAdjacencyProgramDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_checked::<
        TopologySpliceRadialAdjacencyProgramDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologySpliceRadialAdjacencyProgramDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationProof<
        TopologyQueryDomain,
        TopologySpliceRadialAdjacencyProgramDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_proof::<
        TopologySpliceRadialAdjacencyProgramDeclaration,
    >;
}
