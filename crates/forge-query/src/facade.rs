//! Public API boundary for `forge-query`.
//! External crates should import through this module rather than reaching into
//! internal crate structure directly.

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
    FrontierBundleRoutePlanningError, FrontierComplexityContract, FrontierDisjointnessClass,
    FrontierPerformanceStatus, FrontierPlanningCounters, FrontierPlanningReport,
    FrontierPostureDigest, FrontierPredictionDriftOutcome, FrontierPreflightAdmissionError,
    FrontierRouteCounters, FrontierRoutePlanningError, FrontierRouteReport,
    FrontierSurfaceDigest, OrderedCollectionFrontierPreflight, ParallelAdmissionDecision,
    ParallelAdmissionRoute, SerialFallbackBundleRoutes, SerialFallbackReason,
    SerialFallbackRoute,
};
pub use crate::frontier_signal_adapter::{
    SignalAdmissionEvidenceError, SignalFrontierSurfaceEvidence,
};
pub use crate::identity::{
    BasisDigest, BindingFulfillmentDigest, CanonicalEquivalence, CanonicalQueryDigest,
    CanonicalResultShapeDigest, CollectionPlanDigest, PlanDigest, ResultDigest, SchemaBasisDigest,
    ValidatedQueryDigest, ValidatedResultShapeDigest,
};
pub use crate::live::{
    admit_region_scoped_live_plan, build_milestone_five_live_artifact, execute_live_change,
    execute_region_scoped_live_change, lower_region_scoped_execution_to_stream_contract,
    promote_preflight_bundle_to_live, replay_live_sequence, BoundedMaterializationLiveOutcome,
    BoundedMaterializationPatch, BoundedMaterializationPatchKind, BridgeChangeSummary,
    BridgeFieldDelta, BridgeLocalitySlice, BridgeRelationDelta, BridgeSliceCategory,
    ChangeRelevance, CoalescingDecision, CollectionMembershipChange, CollectionOrderingChange,
    DetailLiveOutcome, DetailPatch, IrrelevantChangeClass, LiveBoundedMaterializationPatchError,
    LiveCertificationLane, LiveCertificationRejectionLane, LiveChangeOrdinal, LiveChangeSequenceId,
    LiveCoalescingError, LiveCollectionPatchError, LiveDetailPatchError, LiveExecutionEnvelope,
    LiveExecutionError, LiveExecutionReport, LiveExpectedRejectionError, LivePatchDigest,
    LivePatchEnvelope, LivePatchPayload, LivePolicyCounters, LiveProgressBasis, LiveProgressError,
    LivePromotionDescriptor, LivePromotionError, LiveQueryFamily, LiveQueryPlan, LiveRefreshError,
    LiveReplayBundle, LiveReplayDigest, LiveReplayError, LiveReplayRun, LiveReplayStepInput,
    LiveStartBasis, LiveSubscriptionDigest, LocalityAdmissionClass, LocalityBreadthBudget,
    LocalityCostPosture, LocalityMatchKind, LocalityPredicateContract, LocalityScopeDigest,
    LocalityScopeKind, LocalityWideningBudget, MaterializationScopeChange,
    MaterializationScopeTransition, MembershipTransition, MilestoneFiveLiveAdapter,
    MilestoneFiveLiveArtifact, OrderedCollectionLiveOutcome, OrderedCollectionPatch,
    OrderedCollectionPatchKind, OrderingFieldDelta, PatchWidthAssessment, PatchWidthResolution,
    ProjectionFieldDelta, QueryFieldKey, QueryRelevanceContract, RefreshAdmissionClass,
    RefreshAdmissionMatrix, RefreshFallback, RegionScopedExecutionReport, RegionScopedLiveError,
    RegionScopedLiveExecutionEnvelope, RegionScopedLivePlan, RelevantChangeClass,
    StreamConsumerShape, StreamLoweredDeliveryContract, StreamLoweringCostPosture,
    StreamMemberWidthBudget, SuppressionDecision, SuppressionReason,
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
    admit_authoritative_preview_comparison_candidate,
    admit_preview_promotion_parity_comparison,
    admit_promotion_eligible_preview_session_plan_binding,
    admit_preview_workflow_foundation, admit_read_only_preview_session_plan_binding,
    bind_preflight_to_preview_session, derive_preview_comparison_eligibility,
    execute_preview_session_plan,
    execute_promotion_eligible_preview_session_plan, execute_read_only_preview_session_plan,
    AdmittedPreviewWorkflowFoundation, AuthoritativePreviewComparisonCandidate,
    PreviewBindingCounters, PreviewBindingError, PreviewBindingFailureClass, PreviewBindingIntent,
    PreviewBindingReport,
    PreviewComparisonCandidateArtifact, PreviewComparisonEligibilityArtifact,
    PreviewComparisonCounters, PreviewComparisonError, PreviewComparisonFailureClass,
    PreviewComplexityContract, PreviewEvaluationClass, PreviewExecutionComparisonAdmission,
    PreviewExecutionCounters, PreviewExecutionEnvelope, PreviewExecutionError,
    PreviewExecutionFailureClass, PreviewExecutionReport, PreviewLifecycleMetadata,
    PreviewPerformanceStatusMarker, PreviewSessionBasis, PreviewSessionBindingTuple,
    PreviewSessionPlanBinding, PreviewSessionQueryContext, PreviewWorkflowFoundationArtifact,
    PromotionEligiblePreviewEvaluation, PromotionEligiblePreviewExecutionEnvelope,
    PromotionEligiblePreviewSessionPlanBinding, PromotionParityPreviewComparisonAdmission,
    ReadOnlyPreviewEvaluation, ReadOnlyPreviewExecutionEnvelope,
    ReadOnlyPreviewSessionPlanBinding,
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
