fn _topology_operator_surface_contracts() {
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        &'a mut forge_query::facade::ForgeQueryWorkspace,
    ) -> TopologyCurrentHeadReadSession<'a> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyCurrentHeadReadHandleExt>::topology_reads;
    let _: for<'a> fn(
        &'a TopologySnapshotReadOnlyConfiguredDomainHandle,
        &'a mut forge_query::facade::ForgeQueryWorkspace,
    ) -> TopologySnapshotReadOnlyReadSession<'a> =
        <TopologySnapshotReadOnlyConfiguredDomainHandle as TopologySnapshotReadOnlyReadHandleExt>::topology_reads;
    let _: Option<TopologyCurrentHeadReadSession<'_>> = None;
    let _: Option<TopologySnapshotReadOnlyReadSession<'_>> = None;
    let _: fn(
        &str,
        schema::facade::platform::entities::TopologyEntityKind,
    ) -> TopologyCreateTopologyEntityDeclaration =
        |create_key, kind| TopologyCreateTopologyEntityDeclaration::new(create_key, kind);
    let _: fn(
        &str,
        &str,
        schema::facade::platform::authority::EntityReference,
    ) -> TopologyCreateInnerLoopOnExistingFaceDeclaration =
        |loop_create_key, relation_create_key, face| {
            TopologyCreateInnerLoopOnExistingFaceDeclaration::new(
                loop_create_key,
                relation_create_key,
                face,
            )
        };
    let _: fn(
        &str,
        topology::facade::BoundaryMembershipKind,
        schema::facade::platform::authority::EntityReference,
        schema::facade::platform::authority::EntityReference,
    ) -> TopologyAttachBoundaryMembershipDeclaration =
        |create_key, kind, owner, member| {
            TopologyAttachBoundaryMembershipDeclaration::new(create_key, kind, owner, member)
        };
    let _: fn(
        &str,
        topology::facade::ShellOrWireMembershipKind,
        schema::facade::platform::authority::EntityReference,
        schema::facade::platform::authority::EntityReference,
    ) -> TopologyAttachShellOrWireMembershipDeclaration =
        |create_key, kind, owner, member| {
            TopologyAttachShellOrWireMembershipDeclaration::new(create_key, kind, owner, member)
        };
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyCreateTopologyEntityDeclaration,
    ) -> Result<
        forge_query::facade::ForgeQueryDeclarationEnvelope<
            TopologyQueryDomain,
            TopologyCreateTopologyEntityDeclaration,
        >,
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationTerminalError<
            TopologyQueryDomain,
            TopologyCreateTopologyEntityDeclaration,
        >,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry::<
        TopologyCreateTopologyEntityDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyCreateTopologyEntityDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationChecked<
        TopologyQueryDomain,
        TopologyCreateTopologyEntityDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_checked::<
        TopologyCreateTopologyEntityDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyCreateTopologyEntityDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationProof<
        TopologyQueryDomain,
        TopologyCreateTopologyEntityDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_proof::<
        TopologyCreateTopologyEntityDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyAttachBoundaryMembershipDeclaration,
    ) -> Result<
        forge_query::facade::ForgeQueryDeclarationEnvelope<
            TopologyQueryDomain,
            TopologyAttachBoundaryMembershipDeclaration,
        >,
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationTerminalError<
            TopologyQueryDomain,
            TopologyAttachBoundaryMembershipDeclaration,
        >,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry::<
        TopologyAttachBoundaryMembershipDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyAttachBoundaryMembershipDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationChecked<
        TopologyQueryDomain,
        TopologyAttachBoundaryMembershipDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_checked::<
        TopologyAttachBoundaryMembershipDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyAttachBoundaryMembershipDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationProof<
        TopologyQueryDomain,
        TopologyAttachBoundaryMembershipDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_proof::<
        TopologyAttachBoundaryMembershipDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyAttachShellOrWireMembershipDeclaration,
    ) -> Result<
        forge_query::facade::ForgeQueryDeclarationEnvelope<
            TopologyQueryDomain,
            TopologyAttachShellOrWireMembershipDeclaration,
        >,
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationTerminalError<
            TopologyQueryDomain,
            TopologyAttachShellOrWireMembershipDeclaration,
        >,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry::<
        TopologyAttachShellOrWireMembershipDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyAttachShellOrWireMembershipDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationChecked<
        TopologyQueryDomain,
        TopologyAttachShellOrWireMembershipDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_checked::<
        TopologyAttachShellOrWireMembershipDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyAttachShellOrWireMembershipDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationProof<
        TopologyQueryDomain,
        TopologyAttachShellOrWireMembershipDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_proof::<
        TopologyAttachShellOrWireMembershipDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyCreateInnerLoopOnExistingFaceDeclaration,
    ) -> Result<
        forge_query::facade::ForgeQueryDeclarationEnvelope<
            TopologyQueryDomain,
            TopologyCreateInnerLoopOnExistingFaceDeclaration,
        >,
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationTerminalError<
            TopologyQueryDomain,
            TopologyCreateInnerLoopOnExistingFaceDeclaration,
        >,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry::<
        TopologyCreateInnerLoopOnExistingFaceDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyCreateInnerLoopOnExistingFaceDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationChecked<
        TopologyQueryDomain,
        TopologyCreateInnerLoopOnExistingFaceDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_checked::<
        TopologyCreateInnerLoopOnExistingFaceDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyCreateInnerLoopOnExistingFaceDeclaration,
    ) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationProof<
        TopologyQueryDomain,
        TopologyCreateInnerLoopOnExistingFaceDeclaration,
    > = TopologyCurrentHeadConfiguredDomainHandle::orchestrate_declaration_entry_proof::<
        TopologyCreateInnerLoopOnExistingFaceDeclaration,
    >;
    let _: fn(&TopologyCurrentHeadReadSession<'_>) -> Vec<TopologyReadRequestFamily> =
        |session| session.supported_request_families();
    let _: fn(&TopologyCurrentHeadReadSession<'_>) -> TopologyReadFallbackPosture =
        |session| session.fallback_posture();
    let _: fn(&TopologyCurrentHeadReadSession<'_>) -> TopologyReadAggregateReport =
        |session| session.aggregate_report();
    let _: fn(&TopologyCurrentHeadReadSession<'_>) -> TopologyReadProofReport =
        |session| session.proof_report();
    let _: fn(&TopologyCurrentHeadReadSession<'_>) -> TopologyReadCloseoutReport =
        |session| session.closeout_report();
    let _: fn(TopologyReadExecutionEngine) -> &'static str =
        TopologyReadExecutionEngine::as_str;
    let _: fn(TopologyReadRequestFamily) -> &'static str =
        TopologyReadRequestFamily::as_str;
    let _: fn(&TopologyReadRequestReport) -> TopologyReadRequestFamily =
        TopologyReadRequestReport::request_family;
    let _: fn(&TopologyReadRequestReport) -> TopologyReadExecutionEngine =
        TopologyReadRequestReport::execution_engine;
    let _: fn(&TopologyHalfEdgeSharedVertexNeighborhoodView) -> &TopologyReadRequestReport =
        TopologyHalfEdgeSharedVertexNeighborhoodView::request_report;
    let _: fn(&TopologyHalfEdgeRadialNeighborhoodView) -> &TopologyReadRequestReport =
        TopologyHalfEdgeRadialNeighborhoodView::request_report;
    let _: fn(&TopologyLoopCycleView) -> &TopologyReadRequestReport =
        TopologyLoopCycleView::request_report;
    let _: fn(&TopologyLocalRewireNeighborhoodView) -> &TopologyReadRequestReport =
        TopologyLocalRewireNeighborhoodView::request_report;
    let _: fn(&TopologyReadProofReport) -> &TopologyReadAggregateReport =
        TopologyReadProofReport::request_aggregate;
    let _: fn(&TopologyReadProofReport) -> &TopologyReadParityAggregateReport =
        TopologyReadProofReport::parity_aggregate;
    let _: fn(TopologyReadCloseoutStatus) -> &'static str =
        TopologyReadCloseoutStatus::as_str;
    let _: fn(&TopologyReadCloseoutReport) -> &[TopologyReadCloseoutRow] =
        TopologyReadCloseoutReport::family_rows;
    let _: fn(
        &TopologyReadCloseoutReport,
        TopologyReadRequestFamily,
    ) -> TopologyReadCloseoutStatus = TopologyReadCloseoutReport::status;
    let _: fn(&TopologyReadCloseoutRow) -> &str = TopologyReadCloseoutRow::reason;
    let _: fn(&TopologyReadCloseoutRow) -> &str = TopologyReadCloseoutRow::row_digest;
    let _: fn(TopologyReadPhaseThreeBlocker) -> &'static str =
        TopologyReadPhaseThreeBlocker::as_str;
    let _: fn(TopologyReadPhaseThreeBlockerStatus) -> &'static str =
        TopologyReadPhaseThreeBlockerStatus::as_str;
    let _: fn(&TopologyReadCloseoutReport) -> &[TopologyReadPhaseThreeBlockerRow] =
        TopologyReadCloseoutReport::phase_three_blocker_rows;
    let _: fn(
        &TopologyReadCloseoutReport,
        TopologyReadPhaseThreeBlocker,
    ) -> TopologyReadPhaseThreeBlockerStatus =
        TopologyReadCloseoutReport::phase_three_blocker_status;
    let _: fn(TopologyNoNPlusOneContract) -> &'static str = TopologyNoNPlusOneContract::as_str;
    let _: fn(TopologyNoNPlusOneContractStatus) -> &'static str =
        TopologyNoNPlusOneContractStatus::as_str;
    let _: fn(&TopologyReadCloseoutReport) -> &[TopologyNoNPlusOneContractRow] =
        TopologyReadCloseoutReport::no_n_plus_one_contract_rows;
    let _: fn(
        &TopologyReadCloseoutReport,
        TopologyNoNPlusOneContract,
    ) -> TopologyNoNPlusOneContractStatus =
        TopologyReadCloseoutReport::no_n_plus_one_contract_status;
    let _: fn(&TopologyNoNPlusOneContractRow) -> TopologyNoNPlusOneContract =
        TopologyNoNPlusOneContractRow::contract;
    let _: fn(&TopologyNoNPlusOneContractRow) -> TopologyNoNPlusOneContractStatus =
        TopologyNoNPlusOneContractRow::status;
    let _: fn(&TopologyNoNPlusOneContractRow) -> &str = TopologyNoNPlusOneContractRow::reason;
    let _: fn(&TopologyNoNPlusOneContractRow) -> &str = TopologyNoNPlusOneContractRow::row_digest;
    let _: fn(
        TopologyRuntimeAdapters,
        String,
    ) -> Result<forge_query::facade::ForgeQueryWorkspace, TopologyRuntimeFailure> =
        topology_runtime;
    let _: fn(forge_relational::facade::runtime::RelationalRuntime) -> TopologyRuntimeAdapters =
        TopologyRuntimeAdapters::current_head;
    let _: fn(
        forge_relational::facade::runtime::RelationalReadView,
        SnapshotHandle,
    ) -> TopologyRuntimeAdapters = TopologyRuntimeAdapters::snapshot_read_only;
    let _: fn(&TopologyRuntimeAdapters) -> &TopologyRuntimeSupport =
        TopologyRuntimeAdapters::support;
    let _: fn(
        &TopologyRuntimeSupport,
        topology::facade::TopologyMutationFamily,
    ) -> TopologyQueryMutationFamilySupportStatus =
        TopologyRuntimeSupport::query_mutation_family_support_status;
    let _: fn(&TopologyRuntimeSupport) -> &[TopologyRuntimeMutationFamilySupportRow] =
        TopologyRuntimeSupport::query_mutation_family_support_rows;
    let _: fn(&TopologyRuntimeSupport) -> &[TopologyRuntimeMutationLaneSupportRow] =
        TopologyRuntimeSupport::query_mutation_lane_support_rows;
    let _: fn(
        &TopologyRuntimeSupport,
        TopologyQueryMutationLane,
    ) -> TopologyQueryMutationLaneSupportStatus =
        TopologyRuntimeSupport::query_mutation_lane_support_status;
    let _: fn(&TopologyRuntimeSupport) -> &[TopologyRuntimePostureRow] =
        TopologyRuntimeSupport::runtime_posture_rows;
    let _: fn(
        &TopologyRuntimeSupport,
        TopologyRuntimePostureCapability,
    ) -> TopologyRuntimePostureStatus = TopologyRuntimeSupport::runtime_posture_status;
    let _: fn(&TopologyRuntimeSupport) -> &[TopologyRuntimeReadFamilySupportRow] =
        TopologyRuntimeSupport::query_read_family_support_rows;
    let _: fn(
        &TopologyRuntimeSupport,
        TopologyReadRequestFamily,
    ) -> TopologyQueryReadFamilySupportStatus =
        TopologyRuntimeSupport::query_read_family_support_status;
    let _: fn(&TopologyRuntimeSupport) -> &TopologyRuntimeCloseout =
        TopologyRuntimeSupport::closeout;
    let _: fn(
        &TopologyRuntimeCloseout,
        TopologyRuntimeCloseoutFamily,
    ) -> TopologyRuntimeCloseoutStatus = TopologyRuntimeCloseout::status;
    let _: fn(TopologyQueryMutationLane) -> &'static str = TopologyQueryMutationLane::as_str;
    let _: Option<TopologyQueryMutationLaneExecutionShape> = None;
}

