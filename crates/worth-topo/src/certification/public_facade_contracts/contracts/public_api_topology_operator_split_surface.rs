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
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    ) -> Result<
        forge_query::facade::ForgeQueryDeclarationEnvelope<
            TopologyQueryDomain,
            TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
        >,
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationTerminalError<
            TopologyQueryDomain,
            TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
        >,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry::<
        TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationChecked<
        TopologyQueryDomain,
        TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_checked::<
        TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationProof<
        TopologyQueryDomain,
        TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_proof::<
        TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
    ) -> Result<
        forge_query::facade::ForgeQueryDeclarationEnvelope<
            TopologyQueryDomain,
            TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
        >,
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationTerminalError<
            TopologyQueryDomain,
            TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
        >,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry::<
        TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationChecked<
        TopologyQueryDomain,
        TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_checked::<
        TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationProof<
        TopologyQueryDomain,
        TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_proof::<
        TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
    >;
}
