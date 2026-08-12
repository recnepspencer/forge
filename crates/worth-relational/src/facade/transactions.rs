//! Transaction vocabulary exposed to Relational consumers.

pub use crate::transactions::data::{
    planned_aspect_field_locator, planned_single_field_locator, ApplyEntityAspectPatchIntent,
    ApplyRelationAspectPatchIntent, AspectEmissionTrace, AspectEvaluationTrace,
    AspectEvaluationTraceRow, AspectFieldPatch, AspectLifecycleTransitionClass,
    AspectTagAccuracyReport, AspectTraceEvidence, AuthoritativeApplyPlan, AuthorityMode,
    BulkEntityCreateIntent, BulkMutationLineagePlan, BulkMutationLocalityFootprint,
    BulkMutationNamingPlan, BulkMutationProvenancePlan, BulkMutationScope,
    BulkRelationCreateIntent, CommitAspectSummary, CommitAuthority, CommitChangeSummary,
    CommitConflict, CommitHistorySummary, CommitLog, CommitOutcome, CommitPatchBudgetSummary,
    CommitPhase, CommitPhaseTiming, CommitPublicationSummary, CommitResult, CommitSchemaSummary,
    CommitStructuralSummary, CommitSummary, CommitTopology, CommitTraceEvent, ConflictClass,
    CreateIntent, CreatedEntityRef, CrossContextEndpointClass, DeleteEntityIntent,
    DeleteRelationIntent, EntityAspectCreateIntent, EntityMutationIntent, EntityReference,
    EntitySpec, ExpectedBranchHead, LineageSafeBulkMutationBatch, MergeCommitMutationPlan,
    MergeExecutionOutcome, MergeExecutionStructuralSummary, MergeExecutionSummary,
    MergedCommitPlan, MutationIntent, NamingStableBulkMutationBatch, PatchVsTruthDeltaReport,
    PlannedBulkMutationBatch, PlannedLineageTransition, ProvenanceCompleteBulkMutationBatch,
    PublishedMergeExecutionAuthority, RecordRef, RelationAspectCreateIntent,
    RelationMutationIntent, RelationScope, RelationSpec, ReplaceEntityIntent, RollbackEffect,
    RollbackOutcome, RollbackSummary, SavepointId, TransactionCommitError, TransactionId,
    TransactionOptions, UndoRecord, UpdateEntityFieldsIntent, UpdateRelationEndpointsIntent,
    WorkerIntentBatch,
};
pub use crate::transactions::logic::{
    RelationalMutationInvariantEvidence, RelationalTransaction, ValidatedRelationalMutation,
};
pub use crate::transactions::{
    ValidatedMutationFootprint, ValidatedMutationFootprintNotRequested,
    ValidatedMutationFootprintProjection, ValidatedMutationFootprintWork,
};
pub use worth_foundational::facade::AspectFieldLocator;
