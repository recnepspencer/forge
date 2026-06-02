fn _topology_operator_support_surface_contracts() {
    let _: fn(
        TopologyOperatorEnvelope<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorContinuationTarget<TopologyCreateTopologyEntityDeclaration> =
        topology_operator_continuation_target::<TopologyCreateTopologyEntityDeclaration>;
    let _: Option<TopologyOperatorContinuationTarget<TopologyCreateTopologyEntityDeclaration>> =
        None;
    let _: Option<TopologyOperatorPreparedContinuation<TopologyCreateTopologyEntityDeclaration>> =
        None;
    let _: Option<
        TopologyOperatorPreparedContinuationChecked<TopologyCreateTopologyEntityDeclaration>,
    > = None;
    let _: Option<
        TopologyOperatorPreparedContinuationOutcome<TopologyCreateTopologyEntityDeclaration>,
    > = None;
    let _: Option<
        TopologyOperatorPreparedContinuationProof<TopologyCreateTopologyEntityDeclaration>,
    > = None;
    let _: Option<TopologyOperatorContinuationExecution<TopologyCreateTopologyEntityDeclaration>> =
        None;
    let _: Option<
        TopologyOperatorContinuationExecutionChecked<TopologyCreateTopologyEntityDeclaration>,
    > = None;
    let _: Option<
        TopologyOperatorContinuationExecutionOutcome<TopologyCreateTopologyEntityDeclaration>,
    > = None;
    let _: Option<TopologyOperatorContinuationExecutionProof<TopologyCreateTopologyEntityDeclaration>> =
        None;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorContinuationTarget<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorPreparedContinuationOutcome<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::prepare_topology_operator_continuation::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorContinuationTarget<TopologyCreateTopologyEntityDeclaration>,
    ) -> forge_query::facade::ForgeQueryOrdinaryOutcome<
        TopologyOperatorPreparedContinuation<TopologyCreateTopologyEntityDeclaration>,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::prepare_topology_operator_continuation_outcome::<
        TopologyCreateTopologyEntityDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorContinuationTarget<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorPreparedContinuationChecked<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::prepare_topology_operator_continuation_checked::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorContinuationTarget<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorPreparedContinuationProof<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::prepare_topology_operator_continuation_proof::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorPreparedContinuation<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorContinuationExecutionOutcome<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::execute_topology_operator_prepared_continuation::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorPreparedContinuation<TopologyCreateTopologyEntityDeclaration>,
    ) -> forge_query::facade::ForgeQueryOrdinaryOutcome<
        TopologyOperatorContinuationExecution<TopologyCreateTopologyEntityDeclaration>,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::execute_topology_operator_prepared_continuation_outcome::<
        TopologyCreateTopologyEntityDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorPreparedContinuation<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorContinuationExecutionChecked<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::execute_topology_operator_prepared_continuation_checked::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorPreparedContinuation<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorContinuationExecutionProof<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::execute_topology_operator_prepared_continuation_proof::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: Option<TopologyOperatorRoutePlan<TopologyCreateTopologyEntityDeclaration>> = None;
    let _: Option<TopologyOperatorDeclarationReceipt<TopologyCreateTopologyEntityDeclaration>> =
        None;
    let _: Option<
        TopologyOperatorDeclarationReceiptProof<TopologyCreateTopologyEntityDeclaration>,
    > = None;
    let _: Option<
        TopologyOperatorEnvelopeFromProgressedProof<TopologyCreateTopologyEntityDeclaration>,
    > = None;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyAttachBoundaryMembershipDeclaration,
    ) -> TopologyOperatorDeclarationOutcome<TopologyAttachBoundaryMembershipDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_outcome::<
            TopologyAttachBoundaryMembershipDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyAttachShellOrWireMembershipDeclaration,
    ) -> TopologyOperatorDeclarationOutcome<TopologyAttachShellOrWireMembershipDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_outcome::<
            TopologyAttachShellOrWireMembershipDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyCreateInnerLoopOnExistingFaceDeclaration,
    ) -> TopologyOperatorDeclarationOutcome<TopologyCreateInnerLoopOnExistingFaceDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_outcome::<
            TopologyCreateInnerLoopOnExistingFaceDeclaration,
        >;
    let _: fn(
        TopologyCreateTopologyEntityDeclaration,
    ) -> TopologyOperatorContributionInput<TopologyCreateTopologyEntityDeclaration> =
        topology_operator_contribution_workflow::<TopologyCreateTopologyEntityDeclaration>;
    let _: fn(
        ForgeQuerySupportContributionAuthoring,
    ) -> TopologyOperatorContributionIntent = TopologyOperatorContributionIntent::support;
    let _: fn(
        forge_query::facade::ForgeQueryContinuityContributionAuthoring,
    ) -> TopologyOperatorContributionIntent = TopologyOperatorContributionIntent::continuity;
    let _: Option<TopologyOperatorContributionArtifact<TopologyCreateTopologyEntityDeclaration>> =
        None;
    let _: Option<TopologyOperatorContributionChecked<TopologyCreateTopologyEntityDeclaration>> =
        None;
    let _: Option<TopologyOperatorContributionProof<TopologyCreateTopologyEntityDeclaration>> =
        None;
    let _: Option<
        TopologyOperatorContributionCheckedOutcome<TopologyCreateTopologyEntityDeclaration>,
    > = None;
    let _: Option<TopologyOperatorContributionOutcome<TopologyCreateTopologyEntityDeclaration>> =
        None;
    let _: fn(
        ForgeQueryContributionComposedClassification,
    ) -> ForgeQueryContributionComposedClassification = |classification| classification;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorContributionInput<TopologyCreateTopologyEntityDeclaration>,
    ) -> Result<
        TopologyOperatorContributionArtifact<TopologyCreateTopologyEntityDeclaration>,
        TopologyOperatorContributionCheckedOutcome<TopologyCreateTopologyEntityDeclaration>,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_with_contributions::<
        TopologyCreateTopologyEntityDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorContributionInput<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorContributionOutcome<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_with_contributions_outcome::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorContributionInput<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorContributionChecked<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_with_contributions_checked::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorContributionInput<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorContributionProof<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_with_contributions_proof::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorRoutePlanChecked<TopologyCreateTopologyEntityDeclaration>,
    ) -> Option<forge_query::facade::ForgeQueryRecoveryBrief> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::recover_topology_operator_route_checked::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorEnvelopeChecked<TopologyCreateTopologyEntityDeclaration>,
    ) -> Option<forge_query::facade::ForgeQueryRecoveryBrief> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::recover_topology_operator_envelope_checked::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        &'a TopologyOperatorEnvelopeProof<TopologyCreateTopologyEntityDeclaration>,
    ) -> Option<forge_query::facade::ForgeQueryRecoveryBrief> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::recover_topology_operator_envelope_proof::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorDeclarationReceiptChecked<TopologyCreateTopologyEntityDeclaration>,
    ) -> Option<forge_query::facade::ForgeQueryRecoveryBrief> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::recover_topology_operator_receipt_checked::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorSignalCompatibilityChecked<TopologyCreateTopologyEntityDeclaration>,
    ) -> Option<forge_query::facade::ForgeQueryRecoveryBrief> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::recover_topology_operator_signal_compatibility_checked::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorSignalCompatibilityProof<TopologyCreateTopologyEntityDeclaration>,
    ) -> Option<forge_query::facade::ForgeQueryRecoveryBrief> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::recover_topology_operator_signal_compatibility_proof::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorPreparedContinuationChecked<TopologyCreateTopologyEntityDeclaration>,
    ) -> Option<forge_query::facade::ForgeQueryRecoveryBrief> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::recover_topology_operator_prepared_continuation_checked::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorPreparedContinuationProof<TopologyCreateTopologyEntityDeclaration>,
    ) -> Option<forge_query::facade::ForgeQueryRecoveryBrief> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::recover_topology_operator_prepared_continuation_proof::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorContinuationExecutionChecked<TopologyCreateTopologyEntityDeclaration>,
    ) -> Option<forge_query::facade::ForgeQueryRecoveryBrief> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::recover_topology_operator_continuation_execution_checked::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorContinuationExecutionProof<TopologyCreateTopologyEntityDeclaration>,
    ) -> Option<forge_query::facade::ForgeQueryRecoveryBrief> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::recover_topology_operator_continuation_execution_proof::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorContributionChecked<TopologyCreateTopologyEntityDeclaration>,
    ) -> Option<forge_query::facade::ForgeQueryRecoveryBrief> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::recover_topology_operator_contribution_checked::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorContributionProof<TopologyCreateTopologyEntityDeclaration>,
    ) -> Option<forge_query::facade::ForgeQueryRecoveryBrief> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::recover_topology_operator_contribution_proof::<
            TopologyCreateTopologyEntityDeclaration,
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
    let _: fn(TopologyReadExecutionEngine) -> &'static str = TopologyReadExecutionEngine::as_str;
    let _: fn(TopologyReadRequestFamily) -> &'static str = TopologyReadRequestFamily::as_str;
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
    let _: fn(TopologyReadCloseoutStatus) -> &'static str = TopologyReadCloseoutStatus::as_str;
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
