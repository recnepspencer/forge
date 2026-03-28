mod computation;
mod config;
mod execution;
mod state;
mod transaction;

pub use computation::{DefinedComputation, DefinedKeyedComputation, Recipe};
pub use config::SignalRuntimeConfig;
pub use execution::{RuntimeExecutionRequest, TransactionExecutionRequest};
pub use state::{
    ArtifactMergeAction, ArtifactMergeComparable, BranchConflictResolutionPlan, BranchMergeBase,
    BranchMergeConflictEvidence, BranchMergeConflictKind, BranchMergeConflictRecord,
    BranchMergeConflictSummary, BranchMergeCounters, BranchMergeDivergence,
    BranchMergeExecutionSummary, BranchMergeFailureKind, BranchMergeKind, BranchMergePlan,
    BranchMergeReconciliationPolicy, BranchMergeRequest, BranchMergeResolutionRequirement,
    BranchMergeResult, BranchMergeStrategy, BranchMutationJournalSlice, BranchMutationLedger,
    ConflictMergePolicy, ConflictResolutionRecord, ConflictResolutionStrategy,
    ConservativeOverlapExpansion, DependencyFingerprint, DependencyRemapRecord,
    ExistingTargetMergePolicy, LoweredMergePlan, MergeBoundaryWitness, MergeBoundaryWitnessKind,
    MergeDecisionBasis, MergeNodeMap, MergeTouchedNodeSet, MergedArtifactRecord,
    NodeMergeInputState, NodeMergePlan, NodeReconciliationDecision, NodeReconciliationShape,
    PlannedMergeCandidateSet, ProofMinimalOverlapBasis, SourceNodeAdoptionPlanCore,
    SourceOnlyMergePolicy, StructuralMergeCandidateRecord, StructuralMergeJournalSlice,
};
pub use state::{
    BoundedJournalSegment, CheckpointBoundary, CheckpointRecord, DependencyIndexRebuildProof,
    JournalSegment, MergeSupportRebuildProof, ReconstructabilityProof, ReconstructabilityRecord,
    ReplaySuffixRebuildProof, RequiredDerivedRebuildSet, RuntimeHistory, RuntimeMaterializer,
    RuntimeMerge, RuntimeObserver, PlannedRuntimeMerge, SignalRuntime, SignalRuntimeBuilder,
};
pub use transaction::{
    AdvisoryRecord, BatchChangeSession, DecisionDetail, DecisionLog, DecisionRecord,
    DecisionSummary, EvaluationSummary, IntegrityMarkers, SignalTransaction,
    TransactionOutcome, TransactionReplayEntry, TransactionResult, TransactionTiming,
};
