mod computation;
mod config;
mod execution;
mod state;
mod transaction;

pub use computation::{ComputationSpec, DefinedComputation, DefinedKeyedComputation};
pub use config::SignalRuntimeConfig;
pub use state::{
    BoundedJournalSegment, CheckpointBoundary, CheckpointRecord,
    DependencyIndexRebuildProof, JournalSegment, MergeSupportRebuildProof,
    ReconstructabilityProof, ReconstructabilityRecord, ReplaySuffixRebuildProof,
    RequiredDerivedRebuildSet, RuntimeMaterializer, RuntimeObserver, SignalRuntime,
    SignalRuntimeBuilder,
};
pub use state::{
    ArtifactMergeAction, ArtifactMergeComparable, BranchConflictResolutionPlan, BranchMergeBase,
    BranchMergeConflictEvidence, BranchMergeConflictKind, BranchMergeConflictRecord,
    BranchMergeConflictSummary, BranchMergeCounters, BranchMergeDivergence,
    BranchMergeExecutionSummary, BranchMergeKind, BranchMergePlan,
    BranchMergeReconciliationPolicy, BranchMergeRequest, BranchMergeResolutionRequirement,
    BranchMergeResult, ConflictResolutionRecord, ConflictResolutionStrategy,
    BranchMergeStrategy, BranchMergeFailureKind, BranchMutationJournalSlice,
    BranchMutationLedger, ConflictMergePolicy, ConservativeOverlapExpansion,
    DependencyFingerprint, DependencyRemapRecord, ExistingTargetMergePolicy,
    LoweredMergePlan, MergeBoundaryWitness, MergeBoundaryWitnessKind,
    MergeDecisionBasis, MergeNodeMap, MergeTouchedNodeSet, MergedArtifactRecord,
    NodeMergeInputState, NodeMergePlan, NodeReconciliationDecision,
    NodeReconciliationShape, PlannedMergeCandidateSet, ProofMinimalOverlapBasis,
    SourceNodeAdoptionPlanCore, SourceOnlyMergePolicy, StructuralMergeCandidateRecord,
    StructuralMergeJournalSlice,
};
pub use transaction::{
    AdvisoryRecord, DecisionDetail, DecisionLog, DecisionRecord, DecisionSummary,
    EvaluationSummary, IntegrityMarkers, SignalTransaction, TransactionOutcome,
    TransactionReplayEntry, TransactionResult, TransactionTiming,
};
