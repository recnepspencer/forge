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

pub(crate) fn current_conflict_batch_admission_rows(
) -> Result<Vec<ConflictBatchAdmissionInventoryRow>, ConflictBatchAdmissionInventoryError> {
    let mut rows = Vec::new();
    rows.extend(workload_rows()?);
    rows.extend(lookup_consumed_rows()?);
    rows.extend(replay_undo_boundary_rows()?);
    rows.extend(coplanar_overlap_rows()?);
    rows.extend(evidence_lookup_family_rows()?);
    rows.extend(super::catalog_additions::additional_conflict_batch_admission_rows()?);
    rows.extend(super::query_support_rows::query_support_rows()?);
    rows.push(row(
        Surface::ConflictInventorySourceFirewall,
        "crates/worth-kernel/src/workload_composition/conflict_batch_admission_inventory/source_firewall.rs",
        "ConflictBatchAdmissionSourceFirewallReport",
        Owner::WorthKernel,
        "Milestone 13 Phase 1 closeout",
        AuthorityKind::SourceFirewallCloseout,
        Disposition::Migrate,
        ReplacementPhase::PhaseTwelveFirewallDeletion,
        "source firewall is closeout pressure, not replacement authority",
        "Phase 12 source firewalls replace inventory-only scanning with hard deletion",
        CertificationPosture::OrdinaryProductionReachable,
        CostPosture::SourceFirewallOnly,
        QuerySurface::NotQuery,
        RowScope::FirewallCloseout,
    )?);
    Ok(rows)
}

pub(crate) const fn required_conflict_batch_admission_surfaces() -> &'static [Surface] {
    &[
        Surface::WorthWorkload,
        Surface::WorthWorkloadCompose,
        Surface::WorthWorkloadParts,
        Surface::RequireAdmittedStagePostures,
        Surface::RequireMatchingEvidenceLedger,
        Surface::LookupConsumedWorkloadComposition,
        Surface::LookupConsumedWorkloadCompositionAdmit,
        Surface::WorthWorkloadAdmitLookupConsumedWorkload,
        Surface::EvidenceLookupConsumedWorkloadHandoff,
        Surface::EvidenceLookupConsumedWorkloadHandoffWithTestBroadReceiptScanCount,
        Surface::BooleanSplitReplayUndoBoundaryAdmission,
        Surface::AdmittedBooleanSplitReplayUndoBoundary,
        Surface::BooleanSplitReplayUndoBoundaryRequest,
        Surface::CoplanarOverlapWorkloadOperator,
        Surface::CoplanarOverlapWorkloadOperatorExecute,
        Surface::CoplanarOverlapOperatorReceipt,
        Surface::EvidenceLookupFamilyDeclaration,
        Surface::EvidenceLookupFamilyQueryPosture,
        Surface::EvidenceLookupTopologyInputPosture,
        Surface::QueryForgeQueryWorkspace,
        Surface::QueryWorkspacePublicSupportMatrix,
        Surface::QueryWorkspacePublicApiContract,
        Surface::QueryWorkspacePublicHandleContract,
        Surface::QueryWorkspacePublicDownstreamDeliveryContract,
        Surface::QueryWorkspacePublicMutationSurfaceReport,
        Surface::QueryWorkspaceAdmitPublicApiFamily,
        Surface::QueryEvidenceReportDeclaration,
        Surface::QueryEvidenceReportScope,
        Surface::QueryHardProhibitionRegistry,
        Surface::QueryHardProhibitionDocumentationRows,
        Surface::QueryHardProhibitionBoundaryAudit,
        Surface::QueryBoundarySourceInventory,
        Surface::QueryBoundaryAuditSourceSet,
        Surface::QueryProjectSupportSnapshot,
        Surface::QueryProjectWorkspaceSupportSnapshot,
        Surface::QueryLoadSupportSnapshotDocument,
        Surface::QuerySupportPinningContract,
        Surface::QueryLoadSupportPinContractDocument,
        Surface::QueryInMemoryTestRuntime,
        Surface::QueryTestBackendSchema,
        Surface::QueryEvidenceReportAdoptionAudit,
        Surface::QueryConsumerResidueAudit,
        Surface::QueryConsumerResidueCertificationEvidence,
        Surface::QueryConsumerResidueClass,
        Surface::QueryConsumerResidueReport,
        Surface::QueryConsumerResidueSourceInventory,
        Surface::QueryConsumerResidueCertificationCaseEvidence,
        Surface::QueryTestBackendResidueAudit,
        Surface::QueryConsumeProjectionFacts,
        Surface::QueryDeclareProjectionFactConsumption,
        Surface::QueryProjectionConsumptionBindContract,
        Surface::QueryLowerRuntimeBoundaryEnvelopeSupport,
        Surface::QueryLowerRuntimeBoundarySourceSupport,
        Surface::QueryDeclarationScopedCapabilitySupport,
        Surface::QueryDeclarationScopedTraceabilitySupport,
        Surface::QueryDeclarationEnvelopeInput,
        Surface::QueryDeclarationEnvelope,
        Surface::QueryDeclarationEnvelopeChecked,
        Surface::CompletedBooleanSplitHandoff,
        Surface::CompletedBooleanSplitHandoffAdmitDownstreamSplitConsumption,
        Surface::CompletedBooleanSplitHandoffAdmitSplitSpatialTouchAuthority,
        Surface::CompletedBooleanLoopReconstructionHandoff,
        Surface::PlanarBooleanLoopRuntimeRegistrationProof,
        Surface::BooleanChainIntegrationHandoff,
        Surface::WorkloadCatalogPlanarBooleanCoplanarOverlapPair,
        Surface::WorkloadCatalogCoplanarOverlapStorm,
        Surface::OperatorOutcomeFromCoplanarOverlapReceipt,
        Surface::OperatorReceiptSetFromCoplanarOverlapReceipt,
        Surface::BooleanChainResidueRows,
        Surface::CertifiedProjectedOverlapBridgeAuthority,
        Surface::ProjectedOverlapFaceSet,
        Surface::ProjectedOverlapFaceGeometry,
        Surface::ProjectedOverlapCandidatePolicy,
        Surface::CertifiedProjectedOverlapFaceSet,
        Surface::CertifiedProjectedOverlapFace,
        Surface::CertifiedProjectedOverlapCandidatePair,
        Surface::CertifiedProjectedOverlapCandidatePairs,
        Surface::ProjectedOverlapFaceDenial,
        Surface::ProjectedOverlapExtractionContracts,
        Surface::CoplanarOverlapExtractionBundle,
        Surface::PlanarBooleanRawEdgeSplitScheduleSetAssembleFromAdmittedCandidates,
        Surface::PlanarBooleanOverlapEdgeChainsModule,
        Surface::PlanarBooleanBuildOverlapEdgeChains,
        Surface::PlanarBooleanOverlapEdgeChainSet,
        Surface::PlanarBooleanOverlapEdgeChainSetCertifiesPreparedOverlapChains,
        Surface::PlanarBooleanOverlapEdgeChain,
        Surface::PlanarBooleanOverlapEdgeChainMember,
        Surface::PlanarBooleanOverlapEdgeChainCounters,
        Surface::PlanarBooleanOverlapEdgeChainCountersPartialOverlapChains,
        Surface::OverlapChainIndexedInputs,
        Surface::FragmentOverlapsSubdivision,
        Surface::PlanarBooleanSplitChainValidationCountersOverlapChainsChecked,
        Surface::PlanarBooleanOverlapEdgeChainDenial,
        Surface::PlanarBooleanOverlapEdgeChainDenialKind,
        Surface::PlanarBooleanOverlapChainBoundaryRole,
        Surface::PlanarBooleanOverlapChainPosture,
        Surface::TopoEdgeSplitOverlapChainPublicContract,
        Surface::TraversalViewsOldAuthorityResidue,
        Surface::ReplayScopeProductBroadReceiptScanCounter,
        Surface::UndoScopeProductBroadReceiptScanCounter,
        Surface::EvidenceLookupDiagnosticsHiddenBroadReceiptScanCounter,
        Surface::EvidenceLookupInventoryBroadScanRowCounter,
        Surface::EvidenceLookupSourceFirewallMentionsBroadReceiptScan,
        Surface::EvidenceLookupSourceFirewallBroadReceiptScanRowCounter,
        Surface::EvidenceLookupWorkloadCutoverBroadReceiptScanCounter,
        Surface::EvidenceLookupStageCutoverBroadReceiptScanCounter,
        Surface::EvidenceLookupStageCutoverWithTestBroadReceiptScanCount,
        Surface::EvidenceLookupPlanSelectionBroadReceiptScanCounter,
        Surface::ReplayUndoSpatialBoundaryFixtureWithTestBroadReceiptScanCount,
        Surface::SpatialTouchBroadLedgerScanCounter,
        Surface::SpatialRollbackBooleanEventLedgerAdmission,
        Surface::SpatialRollbackProjectionReceiptAdmission,
        Surface::TopologyRollbackTraversalViewsAdmission,
        Surface::TopologyRollbackMaterializedGraphAdmission,
        Surface::HighValenceRebuildMotionCompatibilitySetter,
        Surface::HighValenceRequireRebuildMotionCompatibility,
        Surface::HighValenceRebuildMotionCompatibility,
        Surface::DuplicateSplitRejectContradictorySameParameterPoints,
        Surface::PointSplitCompatibilityBasis,
        Surface::ConflictInventorySourceFirewall,
    ]
}

fn workload_rows(
) -> Result<Vec<ConflictBatchAdmissionInventoryRow>, ConflictBatchAdmissionInventoryError> {
    let path = "crates/worth-kernel/src/workload_composition/worth_workload.rs";
    Ok(vec![
        row(Surface::WorthWorkload, path, "WorthWorkload", Owner::WorthKernel, "workload composition consumers", AuthorityKind::WorkloadCompositionAdmission, Disposition::Migrate, ReplacementPhase::PhaseEightSelectedBatchPlan, "current composed workload is a prior-proof bundle before conflict routing exists", "batch-admission execution receipt consumes the same proof bundle", CertificationPosture::OrdinaryProductionReachable, CostPosture::LocalTypedAdmission, QuerySurface::NotQuery, RowScope::ConcreteSource)?,
        row(Surface::WorthWorkloadCompose, path, "WorthWorkload::compose", Owner::WorthKernel, "operator workload composition", AuthorityKind::WorkloadCompositionAdmission, Disposition::Migrate, ReplacementPhase::PhaseFourAdmittedConflictInput, "compose enforces stage posture and ledger matching but not aspect-routed conflict", "admitted conflict input consumes WorthWorkload as prior proof", CertificationPosture::OrdinaryProductionReachable, CostPosture::LocalTypedAdmission, QuerySurface::NotQuery, RowScope::ConcreteSource)?,
        row(Surface::WorthWorkloadParts, path, "WorthWorkloadParts", Owner::WorthKernel, "WorthWorkload::compose", AuthorityKind::WorkloadCompositionAdmission, Disposition::Migrate, ReplacementPhase::PhaseFourAdmittedConflictInput, "parts struct is the current workload boundary shape", "admitted conflict input names workload participant proof directly", CertificationPosture::OrdinaryProductionReachable, CostPosture::LocalTypedAdmission, QuerySurface::NotQuery, RowScope::ConcreteSource)?,
        row(Surface::RequireAdmittedStagePostures, path, "require_admitted_stage_postures", Owner::WorthKernel, "WorthWorkload::compose", AuthorityKind::WorkloadCompositionAdmission, Disposition::Migrate, ReplacementPhase::PhaseFourAdmittedConflictInput, "stage support posture is prior proof for conflict admission", "admitted conflict input consumes admitted stage posture as a proof prerequisite", CertificationPosture::OrdinaryProductionReachable, CostPosture::PriorProofBoundary, QuerySurface::NotQuery, RowScope::ConcreteSource)?,
        row(Surface::RequireMatchingEvidenceLedger, path, "require_matching_evidence_ledger", Owner::WorthKernel, "WorthWorkload::compose", AuthorityKind::WorkloadCompositionAdmission, Disposition::Migrate, ReplacementPhase::PhaseFourAdmittedConflictInput, "evidence ledger matching is current anti-fabrication proof", "selected conflict input consumes matched ledger identity instead of rescanning evidence", CertificationPosture::OrdinaryProductionReachable, CostPosture::ReceiptBackedTypedLookup, QuerySurface::NotQuery, RowScope::ConcreteSource)?,
    ])
}

fn lookup_consumed_rows(
) -> Result<Vec<ConflictBatchAdmissionInventoryRow>, ConflictBatchAdmissionInventoryError> {
    let path =
        "crates/worth-kernel/src/workload_composition/worth_workload/lookup_consumed_workload/mod.rs";
    Ok(vec![
        row(Surface::LookupConsumedWorkloadComposition, path, "LookupConsumedWorkloadComposition", Owner::WorthKernel, "WorthWorkload::admit_lookup_consumed_workload", AuthorityKind::LookupConsumedWorkloadAdmission, Disposition::Migrate, ReplacementPhase::PhaseFourAdmittedConflictInput, "lookup-consumed workload handoff is current scan-denial seed", "conflict admission consumes this as no-broad-scan prior proof", CertificationPosture::OrdinaryProductionReachable, CostPosture::ReceiptBackedTypedLookup, QuerySurface::NotQuery, RowScope::ConcreteSource)?,
        row(Surface::LookupConsumedWorkloadCompositionAdmit, path, "LookupConsumedWorkloadComposition::admit", Owner::WorthKernel, "WorthWorkload::admit_lookup_consumed_workload", AuthorityKind::LookupConsumedWorkloadAdmission, Disposition::Migrate, ReplacementPhase::PhaseFourAdmittedConflictInput, "admit rejects stage-index mismatch, raw scans, broad receipt scans, and caller-owned scans", "admitted conflict input preserves those denials before selection", CertificationPosture::OrdinaryProductionReachable, CostPosture::ReceiptBackedTypedLookup, QuerySurface::NotQuery, RowScope::ConcreteSource)?,
        row(Surface::WorthWorkloadAdmitLookupConsumedWorkload, path, "WorthWorkload::admit_lookup_consumed_workload", Owner::WorthKernel, "lookup-consumed workload callers", AuthorityKind::LookupConsumedWorkloadAdmission, Disposition::Migrate, ReplacementPhase::PhaseElevenConsumerSweep, "current convenience entry must not become a hidden batch-admission lane", "consumer sweep routes grouped admission through selected plans", CertificationPosture::OrdinaryProductionReachable, CostPosture::ReceiptBackedTypedLookup, QuerySurface::NotQuery, RowScope::ConcreteSource)?,
        row(Surface::EvidenceLookupConsumedWorkloadHandoff, path, "EvidenceLookupConsumedWorkloadHandoff", Owner::WorthSpatial, "LookupConsumedWorkloadComposition::admit", AuthorityKind::LookupConsumedWorkloadAdmission, Disposition::Migrate, ReplacementPhase::PhaseFourAdmittedConflictInput, "handoff carries scan counters conflict admission must preserve", "admitted conflict input consumes handoff identity and counters", CertificationPosture::OrdinaryProductionReachable, CostPosture::ReceiptBackedTypedLookup, QuerySurface::NotQuery, RowScope::ConcreteSource)?,
    ])
}

fn replay_undo_boundary_rows(
) -> Result<Vec<ConflictBatchAdmissionInventoryRow>, ConflictBatchAdmissionInventoryError> {
    let path =
        "crates/worth-kernel/src/workload_composition/worth_workload/replay_undo_boundary/mod.rs";
    Ok(vec![
        row(
            Surface::BooleanSplitReplayUndoBoundaryAdmission,
            path,
            "admit_boolean_split_replay_undo_boundary",
            Owner::WorthKernel,
            "boolean split replay/undo boundary callers",
            AuthorityKind::ReplayUndoBoundaryAdmission,
            Disposition::Migrate,
            ReplacementPhase::PhaseElevenConsumerSweep,
            "replay/undo boundary admission must require the explicit lookup-consumed batch-execution cluster before packet assembly",
            "consumer sweep routes replay/undo grouped admission through selected plans and workload-attached batch execution",
            CertificationPosture::OrdinaryProductionReachable,
            CostPosture::PriorProofBoundary,
            QuerySurface::NotQuery,
            RowScope::ConcreteSource,
        )?,
        row(
            Surface::AdmittedBooleanSplitReplayUndoBoundary,
            path,
            "AdmittedBooleanSplitReplayUndoBoundary",
            Owner::WorthKernel,
            "admit_boolean_split_replay_undo_boundary",
            AuthorityKind::ReplayUndoBoundaryAdmission,
            Disposition::Migrate,
            ReplacementPhase::PhaseFourAdmittedConflictInput,
            "admitted boundary is current replay/undo proof product seed",
            "selected conflict input binds admitted replay/undo proof identity",
            CertificationPosture::OrdinaryProductionReachable,
            CostPosture::PriorProofBoundary,
            QuerySurface::NotQuery,
            RowScope::ConcreteSource,
        )?,
        row(
            Surface::BooleanSplitReplayUndoBoundaryRequest,
            path,
            "BooleanSplitReplayUndoBoundaryRequest",
            Owner::WorthKernel,
            "admit_boolean_split_replay_undo_boundary",
            AuthorityKind::ReplayUndoBoundaryAdmission,
            Disposition::Migrate,
            ReplacementPhase::PhaseFourAdmittedConflictInput,
            "request shape is a prior-proof boundary, not conflict authority",
            "admitted conflict input consumes admitted boundary output only",
            CertificationPosture::OrdinaryProductionReachable,
            CostPosture::PriorProofBoundary,
            QuerySurface::NotQuery,
            RowScope::ConcreteSource,
        )?,
    ])
}

fn coplanar_overlap_rows(
) -> Result<Vec<ConflictBatchAdmissionInventoryRow>, ConflictBatchAdmissionInventoryError> {
    let operator_path =
        "crates/worth-spatial/src/workload_platform/workload_operators/coplanar_overlap.rs";
    let receipt_path =
        "crates/worth-spatial/src/workload_platform/workload_operators/coplanar_overlap_receipt.rs";
    Ok(vec![
        row(Surface::CoplanarOverlapWorkloadOperator, operator_path, "CoplanarOverlapWorkloadOperator", Owner::WorthSpatial, "workload overlap operator callers", AuthorityKind::OperationalOverlapExecution, Disposition::Migrate, ReplacementPhase::PhaseThreeConflictCatalog, "operator is current operational overlap surface before shared conflict families exist", "spatial conflict family catalog names equivalent overlap authority", CertificationPosture::OrdinaryProductionReachable, CostPosture::OperationalOverlapExecution, QuerySurface::NotQuery, RowScope::ConcreteSource)?,
        row(Surface::CoplanarOverlapWorkloadOperatorFromStageLinks, operator_path, "CoplanarOverlapWorkloadOperator::from_stage_links", Owner::WorthSpatial, "coplanar overlap workload setup", AuthorityKind::OperationalOverlapExecution, Disposition::Migrate, ReplacementPhase::PhaseFourAdmittedConflictInput, "stage-link constructor is current overlap input boundary", "admitted conflict input accepts only sealed prior proof links", CertificationPosture::OrdinaryProductionReachable, CostPosture::PriorProofBoundary, QuerySurface::NotQuery, RowScope::ConcreteSource)?,
        row(Surface::CoplanarOverlapWorkloadOperatorWithCertificationContext, operator_path, "CoplanarOverlapWorkloadOperator::with_certification_context", Owner::WorthSpatial, "coplanar overlap workload setup", AuthorityKind::OperationalOverlapExecution, Disposition::Migrate, ReplacementPhase::PhaseFourAdmittedConflictInput, "certification context binding prevents loose overlap execution", "spatial conflict admission consumes typed context identity", CertificationPosture::OrdinaryProductionReachable, CostPosture::PriorProofBoundary, QuerySurface::NotQuery, RowScope::ConcreteSource)?,
        row(Surface::CoplanarOverlapWorkloadOperatorWithExtractionBundle, operator_path, "CoplanarOverlapWorkloadOperator::with_extraction_bundle", Owner::WorthSpatial, "coplanar overlap workload setup", AuthorityKind::OperationalOverlapExecution, Disposition::Migrate, ReplacementPhase::PhaseFourAdmittedConflictInput, "extraction bundle carries retained overlap receipt identities", "spatial conflict input consumes extraction proof without broad scan", CertificationPosture::OrdinaryProductionReachable, CostPosture::ReceiptBackedTypedLookup, QuerySurface::NotQuery, RowScope::ConcreteSource)?,
        row(Surface::CoplanarOverlapWorkloadOperatorExecute, operator_path, "CoplanarOverlapWorkloadOperator::execute", Owner::WorthSpatial, "coplanar overlap workload operator", AuthorityKind::OperationalOverlapExecution, Disposition::Migrate, ReplacementPhase::PhaseNineExecutionReceipt, "execute currently decides operational overlap receipt from stage links and extraction receipts", "batch-admission execution receipt replaces execution-local conflict meaning", CertificationPosture::OrdinaryProductionReachable, CostPosture::OperationalOverlapExecution, QuerySurface::NotQuery, RowScope::ConcreteSource)?,
        row(Surface::CoplanarOverlapOperatorReceipt, receipt_path, "CoplanarOverlapOperatorReceipt", Owner::WorthSpatial, "CoplanarOverlapWorkloadOperator::execute", AuthorityKind::OperationalOverlapReceipt, Disposition::Migrate, ReplacementPhase::PhaseNineExecutionReceipt, "receipt counters are current overlap evidence and later execution receipt seed", "conflict execution receipt binds shared overlap identity and semantic counters", CertificationPosture::OrdinaryProductionReachable, CostPosture::OperationalOverlapExecution, QuerySurface::NotQuery, RowScope::ConcreteSource)?,
    ])
}

fn evidence_lookup_family_rows(
) -> Result<Vec<ConflictBatchAdmissionInventoryRow>, ConflictBatchAdmissionInventoryError> {
    let declaration_path =
        "crates/worth-spatial/src/workload_platform/evidence_lookup_family_catalog/declaration.rs";
    let posture_path =
        "crates/worth-spatial/src/workload_platform/evidence_lookup_family_catalog/posture/mod.rs";
    Ok(vec![
        row(
            Surface::EvidenceLookupFamilyDeclaration,
            declaration_path,
            "EvidenceLookupFamilyDeclaration",
            Owner::WorthSpatial,
            "evidence lookup family catalog",
            AuthorityKind::EvidenceLookupFamilyDeclaration,
            Disposition::Migrate,
            ReplacementPhase::PhaseThreeConflictCatalog,
            "conflict family declarations should preserve this explicit declaration shape",
            "spatial conflict catalog follows declaration-owned family identity",
            CertificationPosture::OrdinaryProductionReachable,
            CostPosture::PriorProofBoundary,
            QuerySurface::NotQuery,
            RowScope::ConcreteSource,
        )?,
        row(
            Surface::EvidenceLookupDiagnosticWitnessShape,
            posture_path,
            "EvidenceLookupDiagnosticWitnessShape",
            Owner::WorthSpatial,
            "EvidenceLookupFamilyDeclaration",
            AuthorityKind::EvidenceLookupPostureProof,
            Disposition::Migrate,
            ReplacementPhase::PhaseThreeConflictCatalog,
            "diagnostic witness posture is proof/support metadata, not conflict authority",
            "conflict family catalog carries its own witness without local helpers",
            CertificationPosture::OrdinaryProductionReachable,
            CostPosture::PriorProofBoundary,
            QuerySurface::NotQuery,
            RowScope::ConcreteSource,
        )?,
        row(
            Surface::EvidenceLookupEvidenceClassSet,
            posture_path,
            "EvidenceLookupEvidenceClassSet",
            Owner::WorthSpatial,
            "EvidenceLookupFamilyDeclaration",
            AuthorityKind::EvidenceLookupPostureProof,
            Disposition::Migrate,
            ReplacementPhase::PhaseThreeConflictCatalog,
            "evidence class set is prior proof for evidence-overlap classification",
            "shared conflict vocabulary names evidence overlap from proof products",
            CertificationPosture::OrdinaryProductionReachable,
            CostPosture::PriorProofBoundary,
            QuerySurface::NotQuery,
            RowScope::ConcreteSource,
        )?,
        row(
            Surface::EvidenceLookupFamilyIndexPosture,
            posture_path,
            "EvidenceLookupFamilyIndexPosture",
            Owner::WorthSpatial,
            "EvidenceLookupFamilyDeclaration",
            AuthorityKind::EvidenceLookupPostureProof,
            Disposition::Migrate,
            ReplacementPhase::PhaseThreeConflictCatalog,
            "index posture carries lookup cost meaning conflict must not rediscover",
            "conflict catalog consumes posture as declared cost proof",
            CertificationPosture::OrdinaryProductionReachable,
            CostPosture::PriorProofBoundary,
            QuerySurface::NotQuery,
            RowScope::ConcreteSource,
        )?,
        row(
            Surface::EvidenceLookupFamilyQueryPosture,
            posture_path,
            "EvidenceLookupFamilyQueryPosture",
            Owner::WorthSpatial,
            "EvidenceLookupFamilyDeclaration",
            AuthorityKind::EvidenceLookupPostureProof,
            Disposition::Migrate,
            ReplacementPhase::PhaseFourAdmittedConflictInput,
            "Query posture is a prior proof input for evidence lookup, not conflict authority",
            "admitted conflict input consumes Query posture only through sealed products",
            CertificationPosture::OrdinaryProductionReachable,
            CostPosture::PriorProofBoundary,
            QuerySurface::NotQuery,
            RowScope::ConcreteSource,
        )?,
        row(
            Surface::EvidenceLookupTopologyInputPosture,
            posture_path,
            "EvidenceLookupTopologyInputPosture",
            Owner::WorthSpatial,
            "EvidenceLookupFamilyDeclaration",
            AuthorityKind::EvidenceLookupPostureProof,
            Disposition::Migrate,
            ReplacementPhase::PhaseFourAdmittedConflictInput,
            "topology input posture identifies topology proof dependency",
            "admitted conflict input binds topology posture through prior proof",
            CertificationPosture::OrdinaryProductionReachable,
            CostPosture::PriorProofBoundary,
            QuerySurface::NotQuery,
            RowScope::ConcreteSource,
        )?,
    ])
}

fn row(
    surface_identity: Surface,
    source_path: &'static str,
    surface_name: &'static str,
    owner: Owner,
    current_caller: &'static str,
    authority_kind: AuthorityKind,
    disposition: Disposition,
    replacement_phase: ReplacementPhase,
    blocker: &'static str,
    removal_trigger: &'static str,
    certification_posture: CertificationPosture,
    cost_posture: CostPosture,
    query_surface: QuerySurface,
    row_scope: RowScope,
) -> Result<ConflictBatchAdmissionInventoryRow, ConflictBatchAdmissionInventoryError> {
    ConflictBatchAdmissionInventoryRow::builder()
        .surface_identity(surface_identity)
        .source_path(source_path)
        .surface_name(surface_name)
        .owner(owner)
        .current_caller(current_caller)
        .authority_kind(authority_kind)
        .disposition(disposition)
        .replacement_phase(replacement_phase)
        .blocker(blocker)
        .removal_trigger(removal_trigger)
        .certification_posture(certification_posture)
        .cost_posture(cost_posture)
        .query_surface(query_surface)
        .row_scope(row_scope)
        .build()
}
