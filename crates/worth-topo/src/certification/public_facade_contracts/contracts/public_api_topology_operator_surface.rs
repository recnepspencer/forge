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
        TopologyOperatorCanonicalDeclaration<TopologyCreateTopologyEntityDeclaration>,
        TopologyOperatorDeclarationAdmissionError<TopologyCreateTopologyEntityDeclaration>,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::declare_topology_operator::<
        TopologyCreateTopologyEntityDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorCanonicalDeclaration<TopologyCreateTopologyEntityDeclaration>,
    ) -> Result<
        TopologyOperatorDeclarationLegalityEvidence<TopologyCreateTopologyEntityDeclaration>,
        TopologyOperatorDeclarationLegalityDenial<TopologyCreateTopologyEntityDeclaration>,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::review_topology_operator::<
        TopologyCreateTopologyEntityDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyCreateTopologyEntityDeclaration,
    ) -> TopologyOperatorDeclarationOutcome<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_outcome::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyCreateTopologyEntityDeclaration,
    ) -> Result<
        TopologyOperatorEnvelope<TopologyCreateTopologyEntityDeclaration>,
        TopologyOperatorEnvelopeTerminalError<TopologyCreateTopologyEntityDeclaration>,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_envelope::<
        TopologyCreateTopologyEntityDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyCreateTopologyEntityDeclaration,
    ) -> TopologyOperatorEnvelopeChecked<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_envelope_checked::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyCreateTopologyEntityDeclaration,
    ) -> TopologyOperatorEnvelopeProof<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_envelope_proof::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyCreateTopologyEntityDeclaration,
    ) -> Result<
        TopologyOperatorProgressedDeclaration<TopologyCreateTopologyEntityDeclaration>,
        TopologyOperatorProgressionError<TopologyCreateTopologyEntityDeclaration>,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::declare_review_and_progress_topology_operator::<
        TopologyCreateTopologyEntityDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorProgressedDeclaration<TopologyCreateTopologyEntityDeclaration>,
    ) -> Result<
        TopologyOperatorRoutePlan<TopologyCreateTopologyEntityDeclaration>,
        TopologyOperatorRoutePlanTerminalError<TopologyCreateTopologyEntityDeclaration>,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_route::<
        TopologyCreateTopologyEntityDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorProgressedDeclaration<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorRoutePlanChecked<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_route_checked::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorProgressedDeclaration<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorRoutePlanProof<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_route_proof::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorProgressedDeclaration<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorDeclarationReceiptChecked<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_receipt_checked::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorProgressedDeclaration<TopologyCreateTopologyEntityDeclaration>,
    ) -> Result<
        TopologyOperatorDeclarationReceipt<TopologyCreateTopologyEntityDeclaration>,
        TopologyOperatorDeclarationReceiptTerminalError<TopologyCreateTopologyEntityDeclaration>,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_receipt::<
        TopologyCreateTopologyEntityDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorProgressedDeclaration<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorDeclarationReceiptProof<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_receipt_proof::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorProgressedDeclaration<TopologyCreateTopologyEntityDeclaration>,
    ) -> Result<
        TopologyOperatorEnvelope<TopologyCreateTopologyEntityDeclaration>,
        TopologyOperatorEnvelopeFromProgressedTerminalError<
            TopologyCreateTopologyEntityDeclaration,
        >,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_envelope_from_progressed::<
        TopologyCreateTopologyEntityDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorProgressedDeclaration<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorEnvelopeFromProgressedChecked<
        TopologyCreateTopologyEntityDeclaration,
    > =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_envelope_from_progressed_checked::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorProgressedDeclaration<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorEnvelopeFromProgressedProof<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_envelope_from_progressed_proof::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: fn(
        TopologyOperatorEnvelope<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorSignalCompatibilityInput<TopologyCreateTopologyEntityDeclaration> =
        topology_operator_signal_workflow::<TopologyCreateTopologyEntityDeclaration>;
    let _: Option<
        TopologyOperatorSignalCompatibilitySubject<TopologyCreateTopologyEntityDeclaration>,
    > = None;
    let _: Option<
        TopologyOperatorSignalCompatibilityArtifact<TopologyCreateTopologyEntityDeclaration>,
    > = None;
    let _: Option<
        TopologyOperatorSignalCompatibilityChecked<TopologyCreateTopologyEntityDeclaration>,
    > = None;
    let _: Option<
        TopologyOperatorSignalCompatibilityOutcome<TopologyCreateTopologyEntityDeclaration>,
    > = None;
    let _: Option<
        TopologyOperatorSignalCompatibilityProof<TopologyCreateTopologyEntityDeclaration>,
    > = None;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorSignalCompatibilityInput<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorSignalCompatibilityOutcome<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_signal_compatibility::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorSignalCompatibilityInput<TopologyCreateTopologyEntityDeclaration>,
    ) -> forge_query::facade::ForgeQueryOrdinaryOutcome<
        TopologyOperatorSignalCompatibilityArtifact<TopologyCreateTopologyEntityDeclaration>,
    > = <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_signal_compatibility_outcome::<
        TopologyCreateTopologyEntityDeclaration,
    >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorSignalCompatibilityInput<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorSignalCompatibilityChecked<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_signal_compatibility_checked::<
            TopologyCreateTopologyEntityDeclaration,
        >;
    let _: for<'a> fn(
        &'a TopologyCurrentHeadConfiguredDomainHandle,
        TopologyOperatorSignalCompatibilityInput<TopologyCreateTopologyEntityDeclaration>,
    ) -> TopologyOperatorSignalCompatibilityProof<TopologyCreateTopologyEntityDeclaration> =
        <TopologyCurrentHeadConfiguredDomainHandle as TopologyOperatorWorkflowHandleExt>::orchestrate_topology_operator_signal_compatibility_proof::<
            TopologyCreateTopologyEntityDeclaration,
        >;
}

include!("topology_operator_surface/public_api_topology_operator_support_surface.rs");

