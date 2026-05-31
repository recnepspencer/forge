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
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    ) -> Result<
        forge_query::facade::ForgeQueryDeclarationEnvelope<
            TopologyQueryDomain,
            TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
        >,
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationTerminalError<
            TopologyQueryDomain,
            TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
        >,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry::<
        TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationChecked<
        TopologyQueryDomain,
        TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_checked::<
        TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationProof<
        TopologyQueryDomain,
        TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_proof::<
        TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    ) -> Result<
        forge_query::facade::ForgeQueryDeclarationEnvelope<
            TopologyQueryDomain,
            TopologyRehomeAllOwnedFacesToNewShellDeclaration,
        >,
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationTerminalError<
            TopologyQueryDomain,
            TopologyRehomeAllOwnedFacesToNewShellDeclaration,
        >,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry::<
        TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationChecked<
        TopologyQueryDomain,
        TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_checked::<
        TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationProof<
        TopologyQueryDomain,
        TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_proof::<
        TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    >;
}
