use super::row_builder::ConflictBatchAdmissionInventoryRowBuilder;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConflictBatchAdmissionSurfaceIdentity {
    WorthWorkload,
    WorthWorkloadCompose,
    WorthWorkloadParts,
    RequireAdmittedStagePostures,
    RequireMatchingEvidenceLedger,
    LookupConsumedWorkloadComposition,
    LookupConsumedWorkloadCompositionAdmit,
    WorthWorkloadAdmitLookupConsumedWorkload,
    EvidenceLookupConsumedWorkloadHandoff,
    EvidenceLookupConsumedWorkloadHandoffWithTestBroadReceiptScanCount,
    BooleanSplitReplayUndoBoundaryAdmission,
    AdmittedBooleanSplitReplayUndoBoundary,
    BooleanSplitReplayUndoBoundaryRequest,
    CoplanarOverlapWorkloadOperator,
    CoplanarOverlapWorkloadOperatorFromStageLinks,
    CoplanarOverlapWorkloadOperatorWithCertificationContext,
    CoplanarOverlapWorkloadOperatorWithExtractionBundle,
    CoplanarOverlapWorkloadOperatorExecute,
    CoplanarOverlapOperatorReceipt,
    EvidenceLookupFamilyDeclaration,
    EvidenceLookupDiagnosticWitnessShape,
    EvidenceLookupEvidenceClassSet,
    EvidenceLookupFamilyIndexPosture,
    EvidenceLookupFamilyQueryPosture,
    EvidenceLookupTopologyInputPosture,
    QueryConsumerKitSupportSurfaces,
    QueryProjectionConsumptionSupportSurface,
    QueryLowerRuntimeBoundarySupportSurface,
    QueryDeclarationScopedSupportSurface,
    QueryDeclarationBoundaryEnvelopeSurface,
    QueryForgeQueryWorkspace,
    QueryWorkspacePublicSupportMatrix,
    QueryWorkspacePublicApiContract,
    QueryWorkspacePublicHandleContract,
    QueryWorkspacePublicDownstreamDeliveryContract,
    QueryWorkspacePublicMutationSurfaceReport,
    QueryWorkspaceAdmitPublicApiFamily,
    QueryEvidenceReportDeclaration,
    QueryEvidenceReportScope,
    QueryHardProhibitionRegistry,
    QueryHardProhibitionDocumentationRows,
    QueryHardProhibitionBoundaryAudit,
    QueryBoundarySourceInventory,
    QueryBoundaryAuditSourceSet,
    QueryProjectSupportSnapshot,
    QueryProjectWorkspaceSupportSnapshot,
    QueryLoadSupportSnapshotDocument,
    QuerySupportPinningContract,
    QueryLoadSupportPinContractDocument,
    QueryInMemoryTestRuntime,
    QueryTestBackendSchema,
    QueryEvidenceReportAdoptionAudit,
    QueryConsumerResidueAudit,
    QueryConsumerResidueCertificationEvidence,
    QueryConsumerResidueClass,
    QueryConsumerResidueReport,
    QueryConsumerResidueSourceInventory,
    QueryConsumerResidueCertificationCaseEvidence,
    QueryTestBackendResidueAudit,
    QueryConsumeProjectionFacts,
    QueryDeclareProjectionFactConsumption,
    QueryProjectionConsumptionBindContract,
    QueryLowerRuntimeBoundaryEnvelopeSupport,
    QueryLowerRuntimeBoundarySourceSupport,
    QueryDeclarationScopedCapabilitySupport,
    QueryDeclarationScopedTraceabilitySupport,
    QueryDeclarationEnvelopeInput,
    QueryDeclarationEnvelope,
    QueryDeclarationEnvelopeChecked,
    CompletedBooleanSplitHandoff,
    CompletedBooleanSplitHandoffAdmitDownstreamSplitConsumption,
    CompletedBooleanSplitHandoffAdmitSplitSpatialTouchAuthority,
    CompletedBooleanLoopReconstructionHandoff,
    PlanarBooleanLoopRuntimeRegistrationProof,
    BooleanChainIntegrationHandoff,
    WorkloadCatalogPlanarBooleanCoplanarOverlapPair,
    WorkloadCatalogCoplanarOverlapStorm,
    OperatorOutcomeFromCoplanarOverlapReceipt,
    OperatorReceiptSetFromCoplanarOverlapReceipt,
    BooleanChainResidueRows,
    CertifiedProjectedOverlapBridgeAuthority,
    ProjectedOverlapFaceSet,
    ProjectedOverlapFaceGeometry,
    ProjectedOverlapCandidatePolicy,
    CertifiedProjectedOverlapFaceSet,
    CertifiedProjectedOverlapFace,
    CertifiedProjectedOverlapCandidatePair,
    CertifiedProjectedOverlapCandidatePairs,
    ProjectedOverlapFaceDenial,
    ProjectedOverlapExtractionContracts,
    CoplanarOverlapExtractionBundle,
    PlanarBooleanRawEdgeSplitScheduleSetAssembleFromAdmittedCandidates,
    PlanarBooleanOverlapEdgeChainsModule,
    PlanarBooleanBuildOverlapEdgeChains,
    PlanarBooleanOverlapEdgeChainSet,
    PlanarBooleanOverlapEdgeChainSetCertifiesPreparedOverlapChains,
    PlanarBooleanOverlapEdgeChain,
    PlanarBooleanOverlapEdgeChainMember,
    PlanarBooleanOverlapEdgeChainCounters,
    PlanarBooleanOverlapEdgeChainCountersPartialOverlapChains,
    OverlapChainIndexedInputs,
    FragmentOverlapsSubdivision,
    PlanarBooleanSplitChainValidationCountersOverlapChainsChecked,
    PlanarBooleanOverlapEdgeChainDenial,
    PlanarBooleanOverlapEdgeChainDenialKind,
    PlanarBooleanOverlapChainBoundaryRole,
    PlanarBooleanOverlapChainPosture,
    TopoEdgeSplitOverlapChainPublicContract,
    TraversalViewsOldAuthorityResidue,
    ReplayUndoBroadReceiptScanCounters,
    ReplayScopeProductBroadReceiptScanCounter,
    UndoScopeProductBroadReceiptScanCounter,
    EvidenceLookupBroadScanCounters,
    EvidenceLookupDiagnosticsHiddenBroadReceiptScanCounter,
    EvidenceLookupInventoryBroadScanRowCounter,
    EvidenceLookupSourceFirewallMentionsBroadReceiptScan,
    EvidenceLookupSourceFirewallBroadReceiptScanRowCounter,
    EvidenceLookupWorkloadCutoverBroadReceiptScanCounter,
    EvidenceLookupStageCutoverBroadReceiptScanCounter,
    EvidenceLookupStageCutoverWithTestBroadReceiptScanCount,
    EvidenceLookupPlanSelectionBroadReceiptScanCounter,
    EvidenceLookupReuseDecisionBroadReceiptScanCounter,
    EvidenceLookupReuseExecutionInputBroadReceiptScanCounter,
    ConflictInputLookupRouteDenial,
    LookupConsumedWorkloadReuseProductSerialization,
    LookupConsumedWorkloadReuseProductCompatibility,
    LookupConsumedWorkloadRequireResolutionProductSerialization,
    LookupConsumedWorkloadRequireResolutionProductCompatibility,
    LookupConsumedWorkloadMismatchLocusNameSerialization,
    LookupConsumedWorkloadMismatchLocusNameCompatibility,
    ReplayUndoSpatialBoundaryFixtureWithTestBroadReceiptScanCount,
    SpatialTouchBroadLedgerScanCounter,
    SpatialRollbackBooleanEventLedgerAdmission,
    SpatialRollbackProjectionReceiptAdmission,
    TopologyRollbackTraversalViewsAdmission,
    TopologyRollbackMaterializedGraphAdmission,
    HighValenceRebuildMotionCompatibilitySetter,
    HighValenceRequireRebuildMotionCompatibility,
    HighValenceRebuildMotionCompatibility,
    DuplicateSplitRejectContradictorySameParameterPoints,
    PointSplitCompatibilityBasis,
    LookupConsumedReuseResolutionSelectedCompatibilityBasisIdentityDigest,
    TopologyDerivedReuseDecisionSelectedCompatibilityBasisIdentityDigest,
    TopologyDerivedRebuildDenialSelectedCompatibilityBasisIdentityDigest,
    TopologyDerivedReuseExecutionInputSelectedCompatibilityBasisIdentityDigest,
    DerivedEquivalenceContractReportSelectedCompatibilityBasisIdentityDigest,
    TopologySelectedCompatibilityBasisIdentityStruct,
    TopologySelectedCompatibilityBasisIdentityIdentityDigest,
    SelectedTopologyEquivalenceFamilyCompatibilityBasisIdentity,
    SelectedTopologyEquivalenceFamilyCompatibilityPosture,
    TopologySelectedEquivalenceFamilyDeclarationCompatibilityPosture,
    TopologySelectedEquivalenceComparatorContractCompatibilityPosture,
    TopologyCompatibilityPostureEnum,
    TopologyQueryBackedConsumerFamilyRowSelectedCompatibilityBasisIdentityDigest,
    TopologyQueryBackedReadFamilyAdmissionAuthoritySelectedCompatibilityBasisDigestForAdmission,
    TopologyQueryBackedReadFamilySelectedRouteAuthorityCompatibilityBasisTraitMethod,
    TopologyQueryBackedReadFamilyAdmissionAuthorityCompatibilityBasisTraitMethod,
    TopologyDerivedReadDiagnosticSelectedRouteAuthorityCompatibilityBasisTraitMethodTopo,
    WorthTouchedGraphConflictSelectedRoutePacketSelectedCompatibilityBasisIdentityDigest,
    WorthTouchedGraphConflictSelectedRoutePacketTopologyQuerySelectedCompatibilityBasisIdentityDigest,
    TopologyQuerySelectedCompatibilityBasisIdentityDigestTraitMethod,
    TopologyDerivedReadDiagnosticSelectedRouteAuthorityCompatibilityBasisTraitMethod,
    WorthTouchedGraphConflictProofChainTopologyQuerySelectedCompatibilityBasisIdentityDigest,
    WorthTouchedGraphConflictMilestoneFourteenSeedTopologyQuerySelectedCompatibilityBasisIdentityDigest,
    TopologyQueryBackedConsumerCutoverWithTestLoopCycleSelectedCompatibilityBasisIdentityOverride,
    EvidenceLookupIndexProductSelectedCompatibilityBasisIdentityDigest,
    EvidenceLookupIndexProductSelectedCompatibilityPosture,
    EvidenceLookupRoutePacketSelectedCompatibilityBasisIdentityDigest,
    EvidenceLookupIndexReuseDecisionSelectedCompatibilityBasisIdentityDigest,
    EvidenceLookupIndexRebuildDenialSelectedCompatibilityBasisIdentityDigest,
    SelectedSpatialEquivalenceFamilyCompatibilityBasisIdentity,
    SelectedSpatialEquivalenceFamilyCompatibilityPosture,
    SpatialSelectedCompatibilityBasisIdentityStruct,
    SpatialSelectedCompatibilityBasisIdentityIdentityDigest,
    SpatialSelectedEquivalenceFamilyDeclarationCompatibilityPosture,
    SpatialCompatibilityPostureEnum,
    WorthWorkloadOrdinaryConsumerSweepCloseout,
    WorthWorkloadOrdinaryConsumerSweepRequireAllCoveredConsumersOnCompiledProductLane,
    WorthWorkloadOrdinaryConsumerSweepRequireZeroBroadScanFallbackOnOrdinaryPath,
    ConflictInventorySourceFirewall,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictBatchAdmissionAuthorityKind {
    WorkloadCompositionAdmission,
    LookupConsumedWorkloadAdmission,
    ReplayUndoBoundaryAdmission,
    OperationalOverlapExecution,
    OperationalOverlapReceipt,
    EvidenceLookupFamilyDeclaration,
    EvidenceLookupPostureProof,
    SpatialTouchAuthorityAdmission,
    CompatibilityPostureAdmission,
    CutLineResidue,
    QuerySupportProofSurface,
    SourceFirewallCloseout,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictBatchAdmissionDisposition {
    Migrate,
    Delete,
    Cap,
    CertificationOnly,
    QueryGap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictBatchAdmissionOwner {
    WorthKernel,
    WorthTopo,
    WorthSpatial,
    ForgeQuery,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictBatchAdmissionReplacementPhase {
    PhaseTwoSharedVocabulary,
    PhaseThreeConflictCatalog,
    PhaseFourAdmittedConflictInput,
    PhaseFiveSelectedConflictPlan,
    PhaseSixIndependenceProof,
    PhaseSevenBatchCatalog,
    PhaseEightSelectedBatchPlan,
    PhaseNineExecutionReceipt,
    PhaseElevenConsumerSweep,
    PhaseTwelveFirewallDeletion,
    PhaseThirteenPublicReadCloseoutCutover,
    NotReplacedCertificationOnly,
    BlockedOnQueryCapability,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictBatchAdmissionCertificationPosture {
    OrdinaryProductionReachable,
    CertificationOnlyDeniedAsOrdinaryProof,
    NonOrdinaryResidueDeniedAsOrdinaryProof,
    QuerySupportOnlyCannotMintConflictAuthority,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictBatchAdmissionCostPosture {
    LocalTypedAdmission,
    ReceiptBackedTypedLookup,
    PriorProofBoundary,
    OperationalOverlapExecution,
    QueryOwnedSupportProjection,
    CappedResidue,
    SourceFirewallOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictBatchAdmissionQuerySurface {
    NotQuery,
    ConsumerKitProof,
    SupportAdmission,
    SupportPinning,
    ProjectionConsumption,
    LowerRuntimeBoundaryEnvelope,
    DeclarationScopedSupport,
    DeclarationBoundaryEnvelope,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictBatchAdmissionRowScope {
    ConcreteSource,
    QuerySupportSummary,
    FirewallCloseout,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictBatchAdmissionInventoryRow {
    pub(super) surface_identity: ConflictBatchAdmissionSurfaceIdentity,
    pub(super) source_path: String,
    pub(super) surface_name: String,
    pub(super) owner: ConflictBatchAdmissionOwner,
    pub(super) current_caller: String,
    pub(super) authority_kind: ConflictBatchAdmissionAuthorityKind,
    pub(super) disposition: ConflictBatchAdmissionDisposition,
    pub(super) replacement_phase: ConflictBatchAdmissionReplacementPhase,
    pub(super) blocker: String,
    pub(super) removal_trigger: String,
    pub(super) certification_posture: ConflictBatchAdmissionCertificationPosture,
    pub(super) cost_posture: ConflictBatchAdmissionCostPosture,
    pub(super) query_surface: ConflictBatchAdmissionQuerySurface,
    pub(super) row_scope: ConflictBatchAdmissionRowScope,
}

impl ConflictBatchAdmissionInventoryRow {
    pub(crate) fn builder() -> ConflictBatchAdmissionInventoryRowBuilder {
        ConflictBatchAdmissionInventoryRowBuilder::default()
    }

    pub const fn surface_identity(&self) -> ConflictBatchAdmissionSurfaceIdentity {
        self.surface_identity
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn surface_name(&self) -> &str {
        &self.surface_name
    }

    pub const fn owner(&self) -> ConflictBatchAdmissionOwner {
        self.owner
    }

    pub fn current_caller(&self) -> &str {
        &self.current_caller
    }

    pub const fn authority_kind(&self) -> ConflictBatchAdmissionAuthorityKind {
        self.authority_kind
    }

    pub const fn disposition(&self) -> ConflictBatchAdmissionDisposition {
        self.disposition
    }

    pub const fn replacement_phase(&self) -> ConflictBatchAdmissionReplacementPhase {
        self.replacement_phase
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub const fn certification_posture(&self) -> ConflictBatchAdmissionCertificationPosture {
        self.certification_posture
    }

    pub const fn cost_posture(&self) -> ConflictBatchAdmissionCostPosture {
        self.cost_posture
    }

    pub const fn query_surface(&self) -> ConflictBatchAdmissionQuerySurface {
        self.query_surface
    }

    pub const fn row_scope(&self) -> ConflictBatchAdmissionRowScope {
        self.row_scope
    }
}
