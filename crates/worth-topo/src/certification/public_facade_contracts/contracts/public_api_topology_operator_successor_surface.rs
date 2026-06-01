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
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyRewireLoopSuccessorProgramDeclaration,
    ) -> Result<
        forge_query::facade::ForgeQueryDeclarationEnvelope<
            TopologyQueryDomain,
            TopologyRewireLoopSuccessorProgramDeclaration,
        >,
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationTerminalError<
            TopologyQueryDomain,
            TopologyRewireLoopSuccessorProgramDeclaration,
        >,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry::<
        TopologyRewireLoopSuccessorProgramDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyRewireLoopSuccessorProgramDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationChecked<
        TopologyQueryDomain,
        TopologyRewireLoopSuccessorProgramDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_checked::<
        TopologyRewireLoopSuccessorProgramDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyRewireLoopSuccessorProgramDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationProof<
        TopologyQueryDomain,
        TopologyRewireLoopSuccessorProgramDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_proof::<
        TopologyRewireLoopSuccessorProgramDeclaration,
    >;
}
