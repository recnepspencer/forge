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
    AggregateFunctionFamily, AggregateGroupingShape, AggregateInputBreadth,
    AggregateShapeArtifact, CollectionOrderingBasis, CollectionOrderingDirection,
    CollectionPlanBundle, CollectionPlanningContext, CollectionResultFamily,
    CollectionWindowPolicy, CursorAdvanceContract, CursorBoundaryDigest,
    DerivedFieldComputationClass, DerivedFieldPlanArtifact, MaterializationBreadthClass,
    OpaquePageCursor, OrderingKeyPath, OrderingTieBreakContract, PostReadShapingPlan,
    RollupEdgeClass, RollupShapeArtifact, StableOrderingContract, TraversalBoundContract,
    TraversalDepthLimit, TraversalEdgeClass,
};
pub use crate::diagnostics::{
    CanonicalizationCounters, CanonicalizationReport, CanonicalizationWarning,
    CompatibilityEvidence, IdentityFreezeEvidence, NormalizationEvent,
};
pub use crate::execution::{
    execute_preflight_bundle, ExecutionCounters, ExecutionError, ExecutionFailureClass,
    ExecutionReport, ExecutionResultEnvelope,
};
pub use crate::identity::{
    BasisDigest, BindingFulfillmentDigest, CanonicalEquivalence, CanonicalQueryDigest,
    CanonicalResultShapeDigest, CollectionPlanDigest, PlanDigest, ResultDigest,
    SchemaBasisDigest, ValidatedQueryDigest, ValidatedResultShapeDigest,
};
pub use crate::planning::{
    ExecutionCostMarker, ExecutionMechanics, ExecutionPlanBundle, FallbackDisposition,
    PlannedExecutionRoute, PlannedQueryArtifact, PlannedResultShapeArtifact, PlanningAmbientContext,
    PlanningCounters, PlanningError, PlanningFailureClass, PlanningReport,
    PlanningRequestContext, PlanningSemanticInputs, plan_validated_bundle,
    plan_validated_bundle_for_collection_family, planning_request_context_for_bound,
    planning_request_context_for_direct, seed_execution_plan,
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
