//! Public API boundary for `forge-query`.
//! External crates should import through this module rather than reaching into
//! internal crate structure directly.

pub use crate::application::{
    CapabilityAdmissionDecision, CapabilityAdmissionError, CapabilityAdmissionFailureClass,
    ConfigurationAdmissionError, ConfigurationAdmissionFailureClass, ForgeQueryApplicationFacade,
    ForgeQueryCapabilityDescriptor, ForgeQueryCapabilityFamily, ForgeQueryCapabilityRegistry,
    ForgeQueryCapabilityResolution, ForgeQueryCapabilityStatus, ForgeQueryCapabilitySupportStatus,
    ForgeQueryConfig, ForgeQueryConfigCounters, ForgeQueryConfigSectionFamily,
    ForgeQueryConfigSectionResolution, ForgeQueryFacadeCounters, ForgeQueryFacadeError,
    ForgeQueryFacadeFailureClass, ForgeQueryQueryConfig, ForgeQueryRelationalConfig,
    ForgeQueryRuntimeBridgeConfig, ForgeQuerySignalConfig, ForgeQueryStoreConfig,
    ForgeQueryQueryContextSupportProfile, ForgeQuerySubsystemOwner, ForgeQuerySupportMatrix,
    ForgeQuerySupportReport, ForgeQuerySupportReportCounters, ForgeQuerySupportSectionPosture,
    HistoricalEvaluationCapability, LiveQueryCapability, PreviewSessionCapability,
    QueryContextCapability, QueryContextDeferredScopeMarker, QueryReadCapability,
    ValidatedForgeQueryConfig, WorkflowOrchestrationCapability,
};
pub use crate::authoring::{
    AspectFieldSelector, AuthoredBundleError, AuthoredBundleFailureClass, AuthoredResultShapeField,
    AuthoringError, AuthoringFailureClass, CollectionAuthoredQuery, CollectionAuthoredResultShape,
    CollectionQueryBuilder, CollectionResultShapeBuilder, DetailAuthoredQuery,
    DetailAuthoredResultShape, DetailQueryBuilder, DetailResultShapeBuilder, EqualityPredicate,
    GuidedAuthoringPath, IntegerComparisonOperator, IntegerComparisonPredicate, OrderingDirection,
    OrderingSelector, PredicateSelector, QueryFamily, ResultShapeFamily, RootEntityKey,
    ScalarPredicateValue, TraversalSelector,
};
pub use crate::basis::{
    preflight_execution_basis, resolve_snapshot_basis, snapshot_resolution_report,
    BasisAuthorityFamily, BasisPreflightError, BasisResolutionError, BasisResolutionMode,
    ExecutionBasisIntent, ExecutionPreflightBundle, ResolvedBasisProof, ResolvedSnapshotBasis,
    ResolvedSnapshotIdentity, SnapshotLineageClass, SnapshotResolutionReport,
};
pub use crate::binding::{
    derive_binding_requirements, resolve_bindings, BindingError, BindingFailureClass,
    BindingRequirement, BindingRequirements, BindingResolution, BindingResolutionError,
    BoundBinding, BoundBindings, IdentityBindingDescriptor, NonIdentityBindingMetadata,
    NonIdentityBindingMetadataKey, QueryBindingDescriptor, QueryBindingSlot, QueryBindingSubject,
};
pub use crate::canonicalization::{
    canonicalize_request, CanonicalOrderingEntry, CanonicalPredicateEntry,
    CanonicalPredicateFamily, CanonicalProjectionEntry, CanonicalQueryArtifact,
    CanonicalQueryBundle, CanonicalResultField, CanonicalResultShapeArtifact,
    CanonicalTraversalEntry, CanonicalizationFailureClass, QueryCanonicalizationError,
};
pub use crate::collection::{
    AggregateFunctionFamily, AggregateGroupingShape, AggregateInputBreadth, AggregateShapeArtifact,
    CollectionOrderingBasis, CollectionOrderingDirection, CollectionPlanBundle,
    CollectionPlanningContext, CollectionResultFamily, CollectionWindowPolicy,
    CursorAdvanceContract, CursorBoundaryDigest, DerivedFieldComputationClass,
    DerivedFieldPlanArtifact, MaterializationBreadthClass, OpaquePageCursor, OrderingKeyPath,
    OrderingTieBreakContract, PostReadShapingPlan, RollupEdgeClass, RollupShapeArtifact,
    StableOrderingContract, TraversalBoundContract, TraversalDepthLimit, TraversalEdgeClass,
};
pub use crate::correspondence::{
    resolve_correspondence_evidence, AdvisoryStructuralAmbiguous, AdvisoryStructuralUnique,
    CorrespondenceAmbiguityEnvelope, CorrespondenceCandidateSet, CorrespondenceComplexityContract,
    CorrespondenceCostPosture, CorrespondenceCounterSnapshot, CorrespondenceDenied,
    CorrespondenceDisagreementEnvelope, CorrespondenceEvaluationError,
    CorrespondenceEvaluationFailureClass, CorrespondenceEvaluationRequest,
    CorrespondenceEvidenceResolved, CorrespondenceOutcome, CorrespondencePerformanceStatusMarker,
    CorrespondenceVocabularyReport, LineageContinuity, LineageStructuralDisagreement,
    StructuralCandidateBudget, StructuralCandidateDiscoveryPlan,
    StructuralCandidateOrderingContract, UniqueStructuralCorrespondenceWitness,
};
pub use crate::correspondence_history::{
    compose_correspondence_historical_envelope, compose_historical_admission_denied_envelope,
    compose_historical_path_denied_envelope, CorrespondenceHistoricalAmbiguityEnvelope,
    CorrespondenceHistoricalDeniedEnvelope, CorrespondenceHistoricalDisagreementEnvelope,
    CorrespondenceHistoricalEnvelope, CorrespondenceHistoricalSuccessEnvelope,
    HistoricalPathAdmissionDeniedEnvelope, HistoricalPathDeniedEnvelope,
    MetadataPreservingHistoricalResultView,
};
pub use crate::correspondence_history_parity::{
    build_correspondence_historical_parity_bundle, CorrespondenceHistoricalParityBundle,
    CorrespondenceHistoricalParityBundleError, CorrespondenceHistoricalParityVariant,
};
pub use crate::diagnostics::{
    CanonicalizationCounters, CanonicalizationReport, CanonicalizationWarning,
    CompatibilityEvidence, IdentityFreezeEvidence, NormalizationEvent,
};
pub use crate::execution::{
    execute_parallel_admission_route, execute_preflight_bundle, execute_serial_fallback_route,
    ExecutionCounters, ExecutionError, ExecutionFailureClass, ExecutionReport,
    ExecutionResultEnvelope,
};
pub use crate::frontier_planning::{
    BoundedMaterializationFrontierPreflight, FrontierAwarePlan, FrontierBreadthPrediction,
    FrontierBundleRoutePlanningError, FrontierComplexityContract, FrontierCounterSnapshot,
    FrontierDisjointnessClass, FrontierParityBundle, FrontierParityBundleError,
    FrontierPerformanceStatus, FrontierPlanningCounters, FrontierPlanningReport,
    FrontierPostureDigest, FrontierPredictionDriftOutcome, FrontierPreflightAdmissionError,
    FrontierRouteCounters, FrontierRoutePlanningError, FrontierRouteReport, FrontierSurfaceDigest,
    OrderedCollectionFrontierPreflight, ParallelAdmissionDecision, ParallelAdmissionRoute,
    ParallelAdmissionRouteSet, PlannedRouteFamily, SerialFallbackBundleRoutes,
    SerialFallbackReason, SerialFallbackRoute,
};
pub use crate::frontier_signal_adapter::{
    SignalAdmissionEvidenceError, SignalFrontierSurfaceEvidence,
};
pub use crate::historical::{
    admit_historical_evaluation_path, materialization_metadata_from_resolved,
    resolve_historical_materialization_path, AdmittedHistoricalPathClass,
    HistoricalCapabilityDescriptor, HistoricalCounterSnapshot, HistoricalEvaluationAdmission,
    HistoricalEvaluationError, HistoricalEvaluationFailureClass, HistoricalEvaluationRequest,
    HistoricalMaterializationDescriptor, HistoricalMaterializationPathMetadata,
    HistoricalPathAdmitted, HistoricalPathCompatibilityOutcome, HistoricalPathComplexityContract,
    HistoricalPathCostPosture, HistoricalPathRequested, HistoricalPathResolved,
    HistoricalPathReuseDescriptor, HistoricalPathSubstitutionDenied,
    HistoricalPathVocabularyReport, HistoricalPerformanceStatusMarker,
    HistoricalReconstructionBudget, HistoricalReplaySpanBudget, PerformancePredictionDriftOutcome,
    ReplayTailReuseEligibility, RequestedHistoricalPathClass, ResolvedHistoricalPathClass,
    RetainedStateReuseEligibility,
};
pub use crate::identity::{
    BasisDigest, BindingFulfillmentDigest, CanonicalEquivalence, CanonicalQueryDigest,
    CanonicalResultShapeDigest, CollectionPlanDigest, CorrespondenceCostPostureDigest,
    CorrespondenceOutcomeDigest, CounterSnapshotDigest, FailureDigest, HistoricalCostPostureDigest,
    HistoricalPathClassDigest, LineageDigest, PlanDigest, ResultDigest, SchemaBasisDigest,
    ValidatedQueryDigest, ValidatedResultShapeDigest,
};
pub use crate::live::{
    admit_region_scoped_live_plan, build_milestone_five_live_artifact, execute_live_change,
    execute_region_scoped_live_change, lower_region_scoped_execution_to_stream_contract,
    promote_preflight_bundle_to_live, replay_live_sequence, AdmittedStreamConsumerContract,
    BoundedMaterializationLiveOutcome, BoundedMaterializationPatch,
    BoundedMaterializationPatchKind, BridgeChangeSummary, BridgeFieldDelta, BridgeLocalitySlice,
    BridgeRelationDelta, BridgeSliceCategory, ChangeRelevance, CoalescingDecision,
    CollectionMembershipChange, CollectionOrderingChange, DeliveryContractLowering,
    DeliveryContractReplayRecord, DeliveryLocalityOutcome, DetailLiveOutcome, DetailPatch,
    IrrelevantChangeClass, LiveBoundedMaterializationPatchError, LiveCertificationLane,
    LiveCertificationRejectionLane, LiveChangeOrdinal, LiveChangeSequenceId, LiveCoalescingError,
    LiveCollectionPatchError, LiveDetailPatchError, LiveExecutionEnvelope, LiveExecutionError,
    LiveExecutionReport, LiveExpectedRejectionError, LivePatchDigest, LivePatchEnvelope,
    LivePatchPayload, LivePolicyCounters, LiveProgressBasis, LiveProgressError,
    LivePromotionDescriptor, LivePromotionError, LiveQueryFamily, LiveQueryPlan, LiveRefreshError,
    LiveReplayBundle, LiveReplayDigest, LiveReplayError, LiveReplayRun, LiveReplayStepInput,
    LiveStartBasis, LiveSubscriptionDigest, LocalityAdmissionClass, LocalityAwareRelevanceContract,
    LocalityBreadthBudget, LocalityCostPosture, LocalityMaintenanceClass, LocalityMatchClass,
    LocalityMatchKind, LocalityPerformanceStatus, LocalityPredicateContract,
    LocalityScopeAdmission, LocalityScopeDigest, LocalityScopeKind, LocalitySemanticBasis,
    LocalityWideningBudget, LocalityWideningDecision, LocalityWideningPolicy,
    MaterializationScopeChange, MaterializationScopeTransition, MembershipTransition,
    MilestoneFiveLiveAdapter, MilestoneFiveLiveArtifact, OrderedCollectionLiveOutcome,
    OrderedCollectionPatch, OrderedCollectionPatchKind, OrderingFieldDelta, PartitionSliceMatch,
    PatchWidthAssessment, PatchWidthResolution, ProjectionFieldDelta, QueryDeliveryContract,
    QueryFieldKey, QueryRelevanceContract, RefreshAdmissionClass, RefreshAdmissionMatrix,
    RefreshFallback, RegionScopedExecutionReport, RegionScopedLiveCounters, RegionScopedLiveError,
    RegionScopedLiveExecutionEnvelope, RegionScopedLivePlan, RegionScopedPlanningReport,
    RegionScopedReplayBundle, RegionScopedSubscriptionIdentity, RegionSliceMatch,
    RelevantChangeClass, StreamConsumerShape, StreamContractDigest, StreamContractRequest,
    StreamLoweredDeliveryContract, StreamLoweringAdmissionClass, StreamLoweringCostPosture,
    StreamMemberProjection, StreamMemberWidthBudget, StreamWindowCompatibility,
    StreamWindowWidthBudget, SuppressionDecision, SuppressionReason,
};
pub use crate::live_performance::{
    CoalescingAdmissionClass, DebtPerformance, ForbiddenPerformance, IncrementalMaintenanceClass,
    IncrementalPatchEligibility, LiveMaintenanceComplexityContract, LiveMaintenanceCostClass,
    LiveMaintenanceWorkUnit, LivePerformanceReport, PatchWidthBudget, PatchWidthPolicy,
    PatchWidthUnit, PerformanceStatus, PerformanceStatusMarker, RefreshAdmissionStatus,
    RefreshCostClass, VerifiedPerformance,
};
pub use crate::planning::{
    plan_validated_bundle, plan_validated_bundle_for_collection_family,
    planning_request_context_for_bound, planning_request_context_for_direct, seed_execution_plan,
    ExecutionCostMarker, ExecutionMechanics, ExecutionPlanBundle, FallbackDisposition,
    PlannedExecutionRoute, PlannedQueryArtifact, PlannedResultShapeArtifact,
    PlanningAmbientContext, PlanningCounters, PlanningError, PlanningFailureClass, PlanningReport,
    PlanningRequestContext, PlanningSemanticInputs,
};
pub use crate::preview::{
    admit_authoritative_preview_comparison_candidate, admit_preview_live_session_plan,
    admit_preview_promotion_parity_comparison, admit_preview_workflow_foundation,
    admit_preview_workflow_foundation_request,
    admit_promotion_eligible_preview_session_plan_binding,
    admit_read_only_preview_session_plan_binding, assess_preview_live_drift,
    bind_preflight_to_preview_session, execute_preview_live_session_plan,
    execute_promotion_eligible_preview_session_plan, execute_read_only_preview_session_plan,
    AdmittedPreviewWorkflowFoundation, AuthoritativePreviewComparisonCandidate,
    PreviewBindingCounters, PreviewBindingError, PreviewBindingFailureClass, PreviewBindingReport,
    PreviewComparisonCounters, PreviewComparisonEligibilityArtifact, PreviewComparisonError,
    PreviewComparisonFailureClass, PreviewComplexityContract, PreviewEvaluationClass,
    PreviewExecutionCounters, PreviewExecutionError, PreviewExecutionFailureClass,
    PreviewExecutionReport, PreviewLifecycleMetadata, PreviewLiveAdmissionReport,
    PreviewLiveCounters, PreviewLiveDriftDenied, PreviewLiveDriftOutcome, PreviewLiveError,
    PreviewLiveExecutionEnvelope, PreviewLiveFailureClass, PreviewLiveMaintained,
    PreviewLiveRebindArtifact, PreviewLiveSessionPlanBinding, PreviewPerformanceStatusMarker,
    PreviewSessionBasis, PreviewSessionBindingTuple, PreviewSessionPlanBinding,
    PreviewSessionQueryContext, PreviewWorkflowFoundationArtifact, PreviewWorkflowFoundationError,
    PreviewWorkflowFoundationFailureClass, PreviewWorkflowFoundationRequest,
    PromotionEligiblePreviewEvaluation, PromotionEligiblePreviewExecutionEnvelope,
    PromotionEligiblePreviewSessionPlanBinding, PromotionParityPreviewComparisonAdmission,
    ReadOnlyPreviewEvaluation, ReadOnlyPreviewExecutionEnvelope, ReadOnlyPreviewSessionPlanBinding,
};
pub use crate::query_context::{
    admit_query_basis_context, attach_diff_query_metadata, attach_query_basis_metadata,
    bind_diff_query_context, bind_query_basis_context, build_query_basis_result_bundle,
    build_query_diff_result_bundle, execute_query_basis_context, shape_query_diff_change_set,
    AdmittedDiffQueryContext, AdmittedQueryBasisContext, ComparisonBasisFamily,
    DiffQueryMetadata, HistoricalAdmissionClass, HistoricalMaterializationCostClass,
    QueryBasisContextBinding, QueryBasisContextRequest, QueryBasisMetadata,
    QueryBasisResultBundle, QueryContextAdmissionError, QueryContextAdmissionFailureClass,
    QueryContextBindingSource, QueryContextBudgetClass, QueryContextCostClass,
    QueryContextCounters, QueryContextDriftOutcome, QueryContextExecutionArtifact,
    QueryContextExecutionCounters, QueryContextExecutionFamily, QueryContextFamily,
    QueryContextPredictionDriftOutcome, QueryContextPredictionReport, QueryDiffChangeFamily,
    QueryDiffChangeRow, QueryDiffChangeSetArtifact, QueryDiffResultBundle,
};
pub use crate::schema_view::{
    QuerySchemaView, SchemaFieldKind, SchemaFieldView, SchemaRelationView,
};
pub use crate::typed::{
    TypedCollectionQuery, TypedCollectionQueryBuilder, TypedCollectionResultShape,
    TypedCollectionResultShapeBuilder, TypedDetailQuery, TypedDetailQueryBuilder,
    TypedDetailResultShape, TypedDetailResultShapeBuilder, TypedEqualityField,
    TypedGuidedAuthoringPath, TypedIntegerComparableField, TypedMembershipField,
    TypedOrderableField, TypedPresenceField, TypedProjectableField, TypedSchemaField,
    TypedSchemaRoot, TypedStringContainsField, TypedTraversalRelation,
};
pub use crate::validation::{
    validate_canonical_bundle, QueryValidationCounters, QueryValidationError,
    QueryValidationReport, ValidatedOrderingEntry, ValidatedOrderingSet, ValidatedPredicateEntry,
    ValidatedPredicateSet, ValidatedProjectionEntry, ValidatedQueryArtifact, ValidatedQueryBundle,
    ValidatedResultShapeArtifact, ValidatedResultShapeBinding, ValidatedTraversalEntry,
    ValidationEvent, ValidationFailureClass, ValidationRejectionMatrix, ValidationWarning,
};
pub use crate::workflow::{
    admit_query_workflow_declaration, bind_workflow_context, build_workflow_replay_bundle,
    inspect_merge_conflicts, inspect_post_merge_outcome, lower_merge_workflow_declaration,
    lower_mutation_intent_declaration, lower_query_writeback_declaration,
    shape_merge_authority_outcome, shape_mutation_authority_outcome,
    shape_writeback_authority_outcome, ConflictInspectionFamily, ConflictInspectionRow,
    LoweredMergeWorkflowDeclaration, LoweredMutationIntentDeclaration, MergeAuthorityTarget,
    MergeClassAdmission, MergeLoweringInput, MergeWorkflowIntent, MutationIntentFamily,
    MutationLoweringInput, PostMergeInspectionFamily, PostMergeInspectionRow,
    QueryConflictInspectionArtifact, QueryPostMergeInspectionArtifact, QueryWorkflowDeclaration,
    QueryWritebackDeclaration, RelationalStrategyTarget, WorkflowAdmissionError,
    WorkflowAdmissionFailureClass, WorkflowAdmissionReport, WorkflowAuthorityOutcomeArtifact,
    WorkflowAuthorityOutcomeFamily, WorkflowAuthorityTargetFamily, WorkflowBasisFamily,
    WorkflowBindingSource, WorkflowBudgetClass, WorkflowBudgetOutcome, WorkflowContextBinding,
    WorkflowCostClass, WorkflowCounters, WorkflowDeclarationFamily, WorkflowDeclarationRequest,
    WorkflowExplicitRebindArtifact, WorkflowFreshnessBinding, WorkflowFreshnessPolicy,
    WorkflowInspectionBudget, WorkflowInspectionCounters, WorkflowInspectionError,
    WorkflowInspectionFailureClass, WorkflowLoweringCounters, WorkflowLoweringError,
    WorkflowLoweringFailureClass, WorkflowPredictionDriftOutcome, WorkflowPredictionReport,
    WorkflowPreviewEvaluationClass, WorkflowReplayBundle, WorkflowStalenessClass,
    WorkflowStalenessOutcome, WritebackCausalityBinding, WritebackDeclarationFamily,
    WritebackLoweringInput,
};
