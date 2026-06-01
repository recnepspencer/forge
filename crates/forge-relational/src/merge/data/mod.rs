mod ancestry;
mod artifacts;
mod causal;
mod conflicts;
mod decisions;
mod execution;
mod execution_artifacts;
mod identity;
mod inspection_digest;
mod plans;
mod policy;
mod requests;
mod resolved_value_strategy;

pub use ancestry::{
    BranchDeltaSummary, MergeAncestrySummary, MergeBaseSelectionRule, ResolvedMergeBase,
};
pub use artifacts::{
    MergeArtifactDigestBasis, MergeBaseDigestBasis, MergeCausalDigestBasis,
    MergeConflictDigestBasis, MergeExecutionAuthorityContract, MergeExecutionAuthorizationRule,
    MergeExecutionConsumptionRule, MergeExecutionDecisionSurface, MergeIdentityDigestBasis,
    MergeLoweredAspectDigestRow, MergeLoweredPlanDigestBasis, MergePlanningArtifactCore,
    MergePlanningSummary, MergePolicyAspectDigestRow, MergePolicyDigestBasis,
    MergeRequestDigestBasis, MergeSchemaKindClass, MergeSchemaKindSemanticSnapshot,
    MergeSchemaSnapshotDigestBasis, RelationalMergeInspectionAdmission,
    RelationalMergeInspectionArtifact, RelationalMergeInspectionInput,
    RelationalMergeInspectionRow,
};
pub use causal::{
    BranchCausalDot, CausalAnnotationSummary, CausalFrontier, CommitCausalMetadata,
    CommitCausalRelation, MergeCausalEvidenceModel, MergeRecordCausalAnnotation,
    MergeRecordCausalDisposition,
};
pub use conflicts::{
    AspectConflictEvidence, ConflictClassificationSummary, DeletionMergeClass,
    EndpointContinuityClass, MergeConflictClass, MergeConflictClassification,
    MergeVisibilityEvidence, MergeVisibilityEvidenceKind, MergeVisibilityState,
    RelationConflictEvidence, RelationConflictPropagation, RelationContinuityClass,
    StrategyConflictClass, StrategyConflictEvidence, TopologyRegionConflictReason,
};
pub use decisions::{
    MergePlanningDecisionKind, MergePlanningDecisionLog, MergePlanningDecisionLogDigestBasis,
    MergePlanningDecisionRecord,
};
pub(crate) use execution::{
    aspect_reference, bound_parent_order, compiled_executable_plan_digest, equality_witness_digest,
    materialized_value_aspect_key, merge_execution_diagnostics_digest, schema_snapshot_digest,
    visible_record_snapshot, ExecutionReadyLoweredMergePlan,
};
pub use execution::{
    AdoptSourceRecordPlan, BoundExecutableMergePlan, BoundExecutableMergeRecordPlan,
    ConvergeDeletedOnBothSidesRecordPlan, DeletedOnBothSidesSemantics, ExecutableAspectPlan,
    MaterializedAspectValue, MaterializedAspectValueEvidence, MergeExecutableRecordProvenance,
    MergeExecutionAuthorityBinding, MergeExecutionCompilationError, MergeExecutionDeniedRecord,
    MergeExecutionError, MergeExecutionFreshnessPolicy, MergeExecutionMutationPlanError,
    MergeExecutionPreparationError, MergeExecutionReadinessReport, MergeExecutionRequest,
    MergeLineageContinuityVerdict, MergeValueMaterialization, MergeValueSourceSide,
    PreparedMergeExecution, PreserveSharedRecordPlan, ReconcileRecordPlan, ReconciledIdentityBasis,
    RuntimeInstanceId, SharedTruthWitness, VisibleMergeRecordSnapshot,
};
pub(crate) use execution_artifacts::diagnostics_plan_from_record_plans;
pub use execution_artifacts::{
    ExecutedMergeAspectClass, ExecutedMergeAspectDiagnosticRow, ExecutedMergeRecordClass,
    ExecutedMergeRecordDiagnosticRow, MergeExecutionDiagnosticsPlan,
};
pub use identity::{
    CustomIdentityBasisIdentity, IdentityBasisDeclaration, IdentityBasisKind, IdentityBasisScope,
    IdentityDiscoverySummary, IdentityMatchCandidate, IdentityMatchClass, IdentityResolutionReason,
    MergeRecordIdentity, SchemaDeclaredCorrespondenceValidationSummary,
};
pub(crate) use inspection_digest::{
    merge_inspection_artifact_digest, merge_inspection_lowered_plan_digest,
    merge_inspection_row_digest,
};
pub use plans::MergePlanningError;
pub(crate) use plans::{
    BranchCommitDelta, BranchTouchedRecordDelta, CausallyAnnotatedMergePlan,
    ConflictClassifiedMergePlan, HistoryScopedMergePlan, IdentityScopedMergePlan, LoweredMergePlan,
    PolicyResolvedMergePlan, ValidatedSchemaDeclaredCorrespondence, VisibleMergeRecord,
    VisibleMergeRecordKind,
};
pub use policy::{
    AspectComparisonState, AspectMergePolicyDeclaration, AspectMergePolicyKind,
    AspectPolicyResolutionRecord, AuthorizedAspectValueSurface, AuthorizedAspectValueUsage,
    CustomMergePolicyIdentity, DeletionExecutionClass, LoweredAspectAction,
    LoweredAspectDenialIntent, LoweredAspectExecutionIntent, LoweredAspectOutcome,
    LoweredMergeAction, LoweredMergeBlockedReason, LoweredMergePlanRecord, LoweredMergePlanSummary,
    LoweredMergeRejectedReason, LoweredRecordDecision, LoweredRecordDecisionKind,
    LoweredRecordDenialAspectIntent, LoweredRecordDenialBundle, LoweredRecordDenialKind,
    LoweredRecordExecutionAspectIntent, LoweredRecordExecutionBundle,
    LoweredRecordExecutionIntentKind, MergeExecutableClass, MergeExecutionReadiness,
    MergeManualResolutionClass, MergePolicyDecisionBoundary, MergePolicyOwnershipClass,
    MergePolicyOwnershipSurface, MergePolicyProofBoundary, MergePolicyRejectClass,
    MergePolicyResolution, MergePolicyResolutionRecord, MergePolicyResolutionSummary,
    MergeResolutionClass, ResolvedAspectMergePolicy, TopologyExecutionClass,
    TopologyRewireAdmissionPolicy,
};
pub use requests::{MergeIntent, MergePlanningRequest};
pub use resolved_value_strategy::MergeResolvedAspectValueStrategy;
