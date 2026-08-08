mod aspect_field_patch;
mod aspect_reports;
mod aspect_traces;
mod bulk_plan_digest;
mod commit_log;
mod intents;
mod mutation_planning;
mod outcomes;
mod primitives;

pub(crate) use aspect_field_patch::validate_planned_aspect_field_locator;
pub use aspect_field_patch::{
    planned_aspect_field_locator, planned_single_field_locator, AspectFieldPatch,
};
pub use aspect_reports::{AspectTagAccuracyReport, PatchVsTruthDeltaReport};
pub use aspect_traces::{
    AspectEmissionTrace, AspectEvaluationTrace, AspectEvaluationTraceRow,
    AspectLifecycleTransitionClass, AspectTraceEvidence,
};
pub(crate) use bulk_plan_digest::{
    bulk_lineage_plan_digest, bulk_naming_plan_digest, bulk_provenance_plan_digest,
};
pub use commit_log::{
    CommitAspectSummary, CommitChangeSummary, CommitHistorySummary, CommitLog,
    CommitPatchBudgetSummary, CommitPhase, CommitPublicationSummary, CommitSummary,
    CommitTraceEvent,
};
pub use intents::{
    ApplyEntityAspectPatchIntent, ApplyRelationAspectPatchIntent, BulkEntityCreateIntent,
    BulkRelationCreateIntent, CreateIntent, DeleteEntityIntent, DeleteRelationIntent,
    EntityAspectCreateIntent, EntityMutationIntent, MutationIntent, RelationAspectCreateIntent,
    RelationMutationIntent, ReplaceEntityIntent, UpdateEntityFieldsIntent,
    UpdateRelationEndpointsIntent,
};
pub use mutation_planning::CommitTopology;
pub(crate) use outcomes::merge_commit_mutation_plan_token;
#[cfg(test)]
pub(crate) use outcomes::EntityUpdateMissingState;
pub use outcomes::{
    AspectDeltaFailureFields, AspectDeltaPatchConstructionDenial, AspectDeltaPatchValueDenial,
    AspectDeltaRecordClass, AspectFieldTargetRejectionReason, AuthoritativeApplyPlan,
    BulkImportRowDomain, BulkImportStage, BulkMutationAdmissionDenial, CommitConflict,
    CommitExecution, CommitOutcome, CommitPhaseTiming, CommitPublication, CommitResult,
    CommitSchemaSummary, CommitStructuralSummary, CommitValidation, CommitValidationSummary,
    ConflictClass, EntityAuthoritativeAspectStateDenial, EntityCascadeDeleteMissingState,
    EntityFieldUpdateMissingState, LoweredCommitPlan, MergeCommitMutationPlan,
    MergeExecutionOutcome, MergeExecutionStructuralSummary, MergeExecutionSummary,
    MergedCommitPlan, MutationStateInconsistencyEvidence, PublishedMergeExecutionAuthority,
    RecordAspectPatchDenial, RecordAspectPatchTarget, RelationEndpointUpdateMissingState,
    RollbackEffect, RollbackOutcome, RollbackSummary, TransactionCommitError, UndoRecord,
};
pub(crate) use primitives::{
    lineage_safe_bulk_mutation_batch, naming_stable_bulk_mutation_batch,
    provenance_complete_bulk_mutation_batch,
};
pub use primitives::{
    AuthorityMode, BulkMutationLineagePlan, BulkMutationLocalityFootprint, BulkMutationNamingPlan,
    BulkMutationProvenancePlan, BulkMutationScope, CommitAuthority, CreatedEntityRef,
    CrossContextEndpointClass, EntityReference, EntitySpec, ExistingRecordTarget,
    ExpectedBranchHead, LineageSafeBulkMutationBatch, NamingStableBulkMutationBatch,
    PlannedBulkMutationBatch, PlannedLineageTransition, ProvenanceCompleteBulkMutationBatch,
    RecordRef, RelationIdentity, RelationScope, RelationSpec, SavepointId, TransactionId,
    TransactionOptions, WorkerIntentBatch,
};
