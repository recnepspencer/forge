use super::error::ConflictBatchAdmissionInventoryError;
use super::row::{
    ConflictBatchAdmissionAuthorityKind as AuthorityKind,
    ConflictBatchAdmissionCertificationPosture as CertificationPosture,
    ConflictBatchAdmissionCostPosture as CostPosture,
    ConflictBatchAdmissionDisposition as Disposition, ConflictBatchAdmissionInventoryRow,
    ConflictBatchAdmissionOwner as Owner, ConflictBatchAdmissionQuerySurface as QuerySurface,
    ConflictBatchAdmissionReplacementPhase as ReplacementPhase,
    ConflictBatchAdmissionRowScope as RowScope, ConflictBatchAdmissionSurfaceIdentity as Surface,
};

pub(crate) fn query_support_rows(
) -> Result<Vec<ConflictBatchAdmissionInventoryRow>, ConflictBatchAdmissionInventoryError> {
    Ok(vec![
        workspace_row(
            Surface::QueryForgeQueryWorkspace,
            "ForgeQueryWorkspace",
            "ordinary Query runtime front door",
        )?,
        workspace_row(
            Surface::QueryWorkspacePublicSupportMatrix,
            "ForgeQueryWorkspace::public_support_matrix",
            "support matrix inspection before downstream admission",
        )?,
        workspace_row(
            Surface::QueryWorkspacePublicApiContract,
            "ForgeQueryWorkspace::public_api_contract",
            "public facade-family API contract inspection",
        )?,
        workspace_row(
            Surface::QueryWorkspacePublicHandleContract,
            "ForgeQueryWorkspace::public_handle_contract",
            "public handle-family contract inspection",
        )?,
        workspace_row(
            Surface::QueryWorkspacePublicDownstreamDeliveryContract,
            "ForgeQueryWorkspace::public_downstream_delivery_contract",
            "stable downstream delivery and resume support contract",
        )?,
        workspace_row(
            Surface::QueryWorkspacePublicMutationSurfaceReport,
            "ForgeQueryWorkspace::public_mutation_surface_report",
            "preferred and lower-level mutation posture report",
        )?,
        workspace_row(
            Surface::QueryWorkspaceAdmitPublicApiFamily,
            "ForgeQueryWorkspace::admit_public_api_family",
            "typed admission for support-gated public families",
        )?,
        consumer_kit_row(
            Surface::QueryEvidenceReportDeclaration,
            "EvidenceReportDeclaration",
            "digest-bearing Consumer Kit evidence report declaration",
            QuerySurface::ConsumerKitProof,
        )?,
        consumer_kit_row(
            Surface::QueryEvidenceReportScope,
            "EvidenceReportScope",
            "Consumer Kit evidence report scope identity",
            QuerySurface::ConsumerKitProof,
        )?,
        consumer_kit_row(
            Surface::QueryHardProhibitionRegistry,
            "hard_prohibition_registry",
            "Query hard-prohibition registry",
            QuerySurface::ConsumerKitProof,
        )?,
        consumer_kit_row(
            Surface::QueryHardProhibitionDocumentationRows,
            "hard_prohibition_documentation_rows",
            "Query hard-prohibition documentation rows",
            QuerySurface::ConsumerKitProof,
        )?,
        consumer_kit_row(
            Surface::QueryHardProhibitionBoundaryAudit,
            "hard_prohibition_boundary_audit",
            "Query hard-prohibition boundary audit",
            QuerySurface::ConsumerKitProof,
        )?,
        consumer_kit_row(
            Surface::QueryBoundarySourceInventory,
            "query_boundary_source_inventory",
            "Query boundary source inventory for proof audits",
            QuerySurface::ConsumerKitProof,
        )?,
        consumer_kit_row(
            Surface::QueryBoundaryAuditSourceSet,
            "ForgeQueryBoundaryAuditSourceSet",
            "Consumer Kit boundary audit source set",
            QuerySurface::ConsumerKitProof,
        )?,
        consumer_kit_row(
            Surface::QueryProjectSupportSnapshot,
            "project_support_snapshot",
            "support snapshot projection",
            QuerySurface::SupportAdmission,
        )?,
        consumer_kit_row(
            Surface::QueryProjectWorkspaceSupportSnapshot,
            "project_workspace_support_snapshot",
            "workspace support snapshot projection",
            QuerySurface::SupportAdmission,
        )?,
        consumer_kit_row(
            Surface::QueryLoadSupportSnapshotDocument,
            "load_support_snapshot_document",
            "schema-versioned support snapshot document loader",
            QuerySurface::SupportAdmission,
        )?,
        consumer_kit_row(
            Surface::QuerySupportPinningContract,
            "support_pinning_contract",
            "support pin contract over live support rows",
            QuerySurface::SupportPinning,
        )?,
        consumer_kit_row(
            Surface::QueryLoadSupportPinContractDocument,
            "load_support_pin_contract_document",
            "support pin contract document loader",
            QuerySurface::SupportPinning,
        )?,
        consumer_kit_row(
            Surface::QueryInMemoryTestRuntime,
            "in_memory_test_runtime",
            "ordinary ForgeQueryWorkspace behavior for downstream test proof",
            QuerySurface::ConsumerKitProof,
        )?,
        consumer_kit_row(
            Surface::QueryTestBackendSchema,
            "ForgeQueryTestBackendSchema",
            "in-memory Query test backend schema",
            QuerySurface::ConsumerKitProof,
        )?,
        consumer_kit_row(
            Surface::QueryEvidenceReportAdoptionAudit,
            "evidence_report_adoption_audit",
            "Consumer Kit adoption audit",
            QuerySurface::ConsumerKitProof,
        )?,
        consumer_kit_row(
            Surface::QueryConsumerResidueAudit,
            "query_consumer_residue_audit",
            "Query consumer residue audit",
            QuerySurface::ConsumerKitProof,
        )?,
        consumer_kit_row(
            Surface::QueryConsumerResidueCertificationEvidence,
            "forge_query_consumer_residue_certification_evidence",
            "Consumer Kit residue certification evidence",
            QuerySurface::ConsumerKitProof,
        )?,
        consumer_kit_row(
            Surface::QueryConsumerResidueClass,
            "ForgeQueryConsumerResidueClass",
            "Consumer Kit residue class taxonomy",
            QuerySurface::ConsumerKitProof,
        )?,
        consumer_kit_row(
            Surface::QueryConsumerResidueReport,
            "ForgeQueryConsumerResidueReport",
            "Consumer Kit residue audit report",
            QuerySurface::ConsumerKitProof,
        )?,
        consumer_kit_row(
            Surface::QueryConsumerResidueSourceInventory,
            "ForgeQueryConsumerResidueSourceInventory",
            "Consumer Kit residue source inventory",
            QuerySurface::ConsumerKitProof,
        )?,
        consumer_kit_row(
            Surface::QueryConsumerResidueCertificationCaseEvidence,
            "ForgeQueryConsumerResidueCertificationCaseEvidence",
            "Consumer Kit residue certification case evidence",
            QuerySurface::ConsumerKitProof,
        )?,
        consumer_kit_row(
            Surface::QueryTestBackendResidueAudit,
            "query_test_backend_residue_audit",
            "Consumer Kit test-backend residue audit",
            QuerySurface::ConsumerKitProof,
        )?,
        query_row(
            Surface::QueryConsumeProjectionFacts,
            "crates/forge-query/docs/foundations/downstream-runtime-integration.md",
            "consume_projection_facts",
            "projection-consumption typed fact receipts",
            QuerySurface::ProjectionConsumption,
        )?,
        query_row(
            Surface::QueryDeclareProjectionFactConsumption,
            "crates/forge-query/docs/foundations/downstream-runtime-integration.md",
            "declare_projection_fact_consumption",
            "projection-consumption declaration receipts",
            QuerySurface::ProjectionConsumption,
        )?,
        query_row(
            Surface::QueryProjectionConsumptionBindContract,
            "crates/forge-query/docs/execution/intent-admission.md",
            "forge_query_projection_consumption_intent(...).admit()?.bind_contract()",
            "projection-consumption admitted contract binding",
            QuerySurface::ProjectionConsumption,
        )?,
        query_row(
            Surface::QueryLowerRuntimeBoundaryEnvelopeSupport,
            "crates/forge-query/docs/domain-capabilities/support/lower-runtime-support-and-boundary-traceability.md",
            "forge_query_domain(...).for_lower_runtime_boundary_envelope(...).supports_boundary_traceability(...).because(...).materialize()",
            "lower-runtime boundary-envelope support materialization",
            QuerySurface::LowerRuntimeBoundaryEnvelope,
        )?,
        query_row(
            Surface::QueryLowerRuntimeBoundarySourceSupport,
            "crates/forge-query/docs/domain-capabilities/support/lower-runtime-support-and-boundary-traceability.md",
            "forge_query_domain(...).for_lower_runtime_boundary_source(...).supports_boundary_traceability(...).because(...).materialize()",
            "lower-runtime boundary-source support materialization",
            QuerySurface::LowerRuntimeBoundaryEnvelope,
        )?,
        query_row(
            Surface::QueryDeclarationScopedCapabilitySupport,
            "crates/forge-query/docs/domain-capabilities/support/declaration-scoped-support-and-traceability.md",
            "forge_query_domain(...).for_intent(...).supports_capability(...).because(...).materialize()",
            "declaration-scoped capability support materialization",
            QuerySurface::DeclarationScopedSupport,
        )?,
        query_row(
            Surface::QueryDeclarationScopedTraceabilitySupport,
            "crates/forge-query/docs/domain-capabilities/support/declaration-scoped-support-and-traceability.md",
            "forge_query_domain(...).for_intent(...).supports_traceability(...).because(...).materialize()",
            "declaration-scoped traceability support materialization",
            QuerySurface::DeclarationScopedSupport,
        )?,
        query_row(
            Surface::QueryDeclarationEnvelopeInput,
            "crates/forge-query/docs/domain-capabilities/declaration-boundary-envelopes.md",
            "ForgeQueryDeclarationEnvelopeInput",
            "declaration boundary envelope input",
            QuerySurface::DeclarationBoundaryEnvelope,
        )?,
        query_row(
            Surface::QueryDeclarationEnvelope,
            "crates/forge-query/docs/domain-capabilities/declaration-boundary-envelopes.md",
            "ForgeQueryDeclarationEnvelope",
            "declaration boundary envelope artifact",
            QuerySurface::DeclarationBoundaryEnvelope,
        )?,
        query_row(
            Surface::QueryDeclarationEnvelopeChecked,
            "crates/forge-query/docs/domain-capabilities/declaration-boundary-envelopes.md",
            "ForgeQueryDeclarationEnvelopeChecked",
            "checked declaration boundary envelope outcome",
            QuerySurface::DeclarationBoundaryEnvelope,
        )?,
    ])
}

fn workspace_row(
    surface: Surface,
    surface_name: &'static str,
    current_caller: &'static str,
) -> Result<ConflictBatchAdmissionInventoryRow, ConflictBatchAdmissionInventoryError> {
    query_row(
        surface,
        "crates/forge-query/docs/foundations/support-matrix-and-admission.md",
        surface_name,
        current_caller,
        QuerySurface::SupportAdmission,
    )
}

fn consumer_kit_row(
    surface: Surface,
    surface_name: &'static str,
    current_caller: &'static str,
    query_surface: QuerySurface,
) -> Result<ConflictBatchAdmissionInventoryRow, ConflictBatchAdmissionInventoryError> {
    query_row(
        surface,
        "crates/forge-query/docs/foundations/consumer-kit.md",
        surface_name,
        current_caller,
        query_surface,
    )
}

fn query_row(
    surface: Surface,
    source_path: &'static str,
    surface_name: &'static str,
    current_caller: &'static str,
    query_surface: QuerySurface,
) -> Result<ConflictBatchAdmissionInventoryRow, ConflictBatchAdmissionInventoryError> {
    ConflictBatchAdmissionInventoryRow::builder()
        .surface_identity(surface)
        .source_path(source_path)
        .surface_name(surface_name)
        .owner(Owner::ForgeQuery)
        .current_caller(current_caller)
        .authority_kind(AuthorityKind::QuerySupportProofSurface)
        .disposition(Disposition::QueryGap)
        .replacement_phase(ReplacementPhase::BlockedOnQueryCapability)
        .blocker(
            "Query support proof is required but cannot mint conflict or batch-admission authority locally",
        )
        .removal_trigger(
            "later phases consume real Query proof surfaces through admitted inputs or leave a named Query gap",
        )
        .certification_posture(CertificationPosture::QuerySupportOnlyCannotMintConflictAuthority)
        .cost_posture(CostPosture::QueryOwnedSupportProjection)
        .query_surface(query_surface)
        .row_scope(RowScope::QuerySupportSummary)
        .build()
}
