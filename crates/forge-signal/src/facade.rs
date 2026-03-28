//! Public API boundary for `forge-signal`.
//! External components should import through this module.
//!
//! Contract:
//! - `forge-signal` owns evaluation DAG scheduling.
//! - Host crates own external structural or state graphs, including cyclic ones.
//! - Compute closures may consume opaque host snapshots directly.
//!
//! Public shape:
//! - top-level `facade::*` is the curated daily-use path
//! - `facade::core` covers graph authoring primitives
//! - `facade::runtime` covers production runtime and transaction workflows
//! - `facade::diagnostics` covers explain / compare / health / history flows
//! - `facade::advanced` and `facade::integration` contain deeper specialist control

pub mod core {
    pub use crate::data::aspect::{Aspect, AspectMask, AspectVersion, MAX_ASPECTS};
    pub use crate::data::dependency::DependencyEdge;
    pub use crate::data::error::SignalError;
    pub use crate::data::graph::{NodeBuilder, SignalGraph};
    pub use crate::data::handle::NodeId;
    pub use crate::data::node::{EvaluationCondition, NodeState};
    pub use crate::data::output::{
        CanonicalChangedRegions, ChangedRegion, NodeEvaluationResult, OutputChange, OutputIdentity,
        PartitionMatchMode, PartitionSubscription, PartitionToken,
    };
    pub use crate::logic::invalidation::mark_dirty_batch;

    pub fn mark_dirty(
        graph: impl std::ops::DerefMut<Target = SignalGraph>,
        source: NodeId,
        changed_aspect: Aspect,
    ) -> Result<(), SignalError> {
        #[cfg(any(test, doctest))]
        {
            return crate::logic::invalidation::mark_dirty(graph, source, changed_aspect);
        }
        #[cfg(not(any(test, doctest)))]
        {
            let _ = crate::logic::invalidation::mark_dirty_batch(
                graph,
                &crate::data::proof::DirtyBatch::singleton(
                    source,
                    changed_aspect,
                    Vec::<ChangedRegion>::new(),
                ),
            )?;
            Ok(())
        }
    }

    pub fn mark_changed(
        graph: impl std::ops::DerefMut<Target = SignalGraph>,
        source: NodeId,
        changed_aspect: Aspect,
    ) -> Result<(), SignalError> {
        mark_dirty(graph, source, changed_aspect)
    }

    pub fn mark_dirty_with_regions(
        graph: impl std::ops::DerefMut<Target = SignalGraph>,
        source: NodeId,
        changed_aspect: Aspect,
        changed_regions: &[ChangedRegion],
    ) -> Result<(), SignalError> {
        #[cfg(any(test, doctest))]
        {
            return crate::logic::invalidation::mark_dirty_with_regions(
                graph,
                source,
                changed_aspect,
                changed_regions,
            );
        }
        #[cfg(not(any(test, doctest)))]
        {
            let _ = crate::logic::invalidation::mark_dirty_batch(
                graph,
                &crate::data::proof::DirtyBatch::singleton(
                    source,
                    changed_aspect,
                    changed_regions.to_vec(),
                ),
            )?;
            Ok(())
        }
    }

    pub fn mark_changed_with_regions(
        graph: impl std::ops::DerefMut<Target = SignalGraph>,
        source: NodeId,
        changed_aspect: Aspect,
        changed_regions: &[ChangedRegion],
    ) -> Result<(), SignalError> {
        mark_dirty_with_regions(graph, source, changed_aspect, changed_regions)
    }
}

pub mod runtime {
    pub use crate::data::checkpoint::CheckpointBarrier;
    pub use crate::data::checkpoint_policy::CheckpointPolicy;
    pub use crate::data::output::{ComputationFamily, ComputationKey, KeyedComputation};
    pub use crate::data::proof::{DirtyBatch, SemanticBatchCommit};
    pub use crate::data::tier::{DependencyMode, DirtyPropagation, EvaluationTrigger, TierPolicy};
    pub use crate::diagnostics::policy::SignalRuntimePolicy;
    pub use crate::logic::invalidation::mark_dirty_batch;
    pub use crate::logic::transaction::{
        BatchChangeSession, DefinedComputation, DefinedKeyedComputation, Recipe,
        EvaluationSummary, PlannedRuntimeMerge, RuntimeExecutionRequest, RuntimeHistory,
        RuntimeMerge, SignalRuntime, SignalRuntimeBuilder, SignalTransaction,
        TransactionExecutionRequest, TransactionOutcome, TransactionResult, TransactionTiming,
    };
    pub use crate::logic::transaction::SignalRuntimeConfig;
    pub type BatchChange = DirtyBatch;
    pub type BatchChangeResult = SemanticBatchCommit;
}

pub mod advanced {
    pub use crate::data::comparator::{
        ComparatorPolicyResolver, DefaultComparatorPolicyResolver, DefaultComparatorResolver,
        TierPolicyResolver, VersionComparatorPolicy, VersionComparatorResolver,
    };
    pub use crate::data::graph::{
        EvaluationStrategy, GcPressure, GraphMaterializer, GraphObserver, ObservationLevel,
        ParallelismHint,
    };
    pub use crate::logic::checkpoint::CheckpointRuntime;
    pub use crate::logic::context::EvaluationContext;
    pub use crate::logic::events::{EventBus, EventFlushError, SubscriberRegistryError};
    pub use crate::logic::evaluation::{
        AppliedEffectReport, ConditionEvaluationContext, ConditionResolver,
        DefaultConditionResolver, DeferralReason, DiagnosticEnvelope, EvaluationExecutionMetadata,
        EvaluationOutput, EvaluationRequestMode, EvaluationVerdict, IntoEvaluationOutput,
        OperationalEffect, SuppressionReason,
    };
    pub use crate::logic::planner::{
        build_evaluation_plan, execute_prepared_plan, CandidateTask, EligibleTask, EvaluationPlan,
        ExecutedTask, ExecutionPruneReason, ExecutionRecordId, ExecutionReport, ExecutionStage,
        PlanSummary, ResolvedExecutionStrategy, ResolvedMaintenanceStrategy, SemanticSegmentId,
        SemanticTaskRange, StageBarrier, StageExecutionOutcome, StageExecutionRecord,
        StageExecutor, TaskExecutionOutcome, TaskExecutionRecord, TaskReason,
    };
    pub use crate::logic::prepared::{ExecutionReadView, PreparedEvaluation};
    #[cfg(feature = "parallel")]
    pub use crate::logic::planner::ParallelExecutionPolicy;
    pub use crate::state::{
        SignalBranchHandle, SignalBranchId, SignalSnapshotDiagnostics, SignalSnapshotId,
        SignalSnapshotMeta, SignalSnapshotV1, SnapshotArtifactRestoreMode,
        SnapshotArtifactRetentionPolicy, SnapshotDependencyRestoreMode,
        SnapshotRestoreCoarseReason, SnapshotRestoreIntent, SnapshotRestorePlan,
        SnapshotStateRestoreMode,
    };
}

pub mod integration {
    pub use crate::data::effect_mapping::EffectMapping;
    pub use crate::data::evaluator::CheckpointEvaluator;
    pub use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
    pub use crate::data::node::{
        ArtifactPolicyClass, AuthorityPolicy, CanonicalDependencyOrder, ComparatorBasis,
        CompileTimePerformanceContract, ContextRequirement, EquivalenceContract, IdentityBasis,
        MaintenanceMode, NodeAuthorityContract, NodeContract, NodeEvaluationConfig,
        NodeExecutionContract, NodeProjectionContract, NodeReuseContract, NodeSemanticContract,
        PathClass, PerformanceCounterSurface, PerformanceEnforcementLayer,
        ResolvedPerformancePolicy, SuppressionBasis,
    };
    pub use crate::data::proof::{
        CanonicalForm, DedupedNodeBatch, DeltaForm, DependencyBatchEdit, DependencySetEdit,
        DesiredState, DirtyBatch, DirtyBatchEntry, DirtyDelta, FrontierEntryClassification,
        FrontierExecutionCounters, FrontierExecutionSummary, FrontierInclusionBasis, FrontierPlan,
        FrontierPredictedCounters, FrontierSeedCause, FrontierValidationDecision, FrontierWave,
        FrontierWaveEntryPlan, FrontierWaveEntrySummary, FrontierWavePlan, FrontierWaveSummary,
        InvalidationFrontier, InvalidationSeed, InvalidationSeedBatch, InvalidationTraceRecord,
        LocalityFootprint, LocallyOrderedShard, LoweredForm, MergeableOrderedStream,
        MixedSnapshotBatchCommit, NarrowedPropagationSet, OrderedStreamItem,
        OrderedStreamMergeError, PartitionScopeSet, PatchPlan, PendingSnapshotBatch, ResolvedForm,
        SemanticBatchCommit, SingleConsumer, SnapshotBatchCommit, SortedSourceBatch,
        StableShapeSnapshotBatchCommit, StructuralDelta, SubscriberRepair, SubscriberRepairBatch,
        SummaryForm, TouchedScopeSummary, TransitiveFrontierRoot,
    };
    pub use crate::data::reuse::{
        ArtifactEquivalenceContract, ArtifactSemanticBoundary, PersistentCorrespondenceEvidence,
        ReuseBasis, ReuseBoundaryAuthority, ReuseBoundaryContext, ReuseBoundaryEvidence,
        ReuseBoundaryFailure, ReuseBoundaryProof, ReuseCertificationFailure,
        ReuseCertificationRecord, ReuseCrossing, ReuseOrigin, ReuseSemanticRegionIdentity,
        ReuseSource, ReuseStrategy, ReuseStrategyBoundaryAuthority,
    };
    pub use crate::data::subscriber_context::{SubscriberContext, SubscriberContextError};
    pub use crate::data::telemetry::RuntimeTelemetry;
    pub use crate::data::tier_policy_table::TierPolicyTable;
    pub use crate::logic::transaction::{
        ArtifactMergeAction, ArtifactMergeComparable, BranchMergeBase,
        BranchMergeConflictEvidence, BranchMergeConflictKind, BranchMergeConflictRecord,
        BranchMergeConflictSummary, BranchMergeCounters, BranchMergeDivergence,
        BranchMergeExecutionSummary, BranchMergeFailureKind, BranchMergeKind, BranchMergePlan,
        BranchMergeReconciliationPolicy, BranchMergeRequest, BranchMergeResult,
        BranchMergeStrategy, BranchMutationJournalSlice, BranchMutationLedger,
        ConflictMergePolicy, ConservativeOverlapExpansion, DependencyFingerprint,
        DependencyRemapRecord, ExistingTargetMergePolicy, LoweredMergePlan,
        MergeBoundaryWitness, MergeBoundaryWitnessKind, MergeDecisionBasis, MergeNodeMap,
        MergeTouchedNodeSet, MergedArtifactRecord, NodeMergeInputState, NodeMergePlan,
        NodeReconciliationDecision, NodeReconciliationShape, PlannedMergeCandidateSet,
        ProofMinimalOverlapBasis, RuntimeMaterializer, SourceNodeAdoptionPlanCore,
        SourceOnlyMergePolicy, StructuralMergeCandidateRecord, StructuralMergeJournalSlice,
    };
}

pub mod diagnostics {
    pub use crate::diagnostics::{
        compare_execution_history, compare_execution_reports, compare_explanations,
        compare_failures, compare_flows, compare_graphs, compare_lineage_records, compare_plans,
        compare_replay_slices, diagnostics_for_graph, diagnostics_for_runtime,
        explanations_semantically_equivalent, graphs_semantically_equivalent, inspect_execution,
        inspect_flow, inspect_graph, inspect_plan, inspect_report, lineage_records_equivalent,
        plans_semantically_equivalent, render_execution_history_summary,
        render_execution_report_summary, render_explanation_summary, render_failure_summary,
        render_flow_summary, render_graph_summary, render_plan_summary, repeat_run_summaries_equal,
        replay_slices_equivalent, reports_semantically_equivalent,
        serial_parallel_reports_equivalent, ApplySummary, ArtifactRetentionPolicy,
        ArtifactTransitionKind, ChangeInputSummary, DiagnosticMismatch, DiagnosticMismatchCategory,
        DiagnosticsAvailability, DiagnosticsTier, EvaluationPlanSummary, EventEpochOutcome,
        EventEpochSummary, EventSubscriberOutcome, EventSubscriberOutcomeKind,
        ExecutionFailureContext, ExecutionFailurePhase, ExecutionHistoryNodeSummary,
        ExecutionHistorySummary, ExecutionInspector, ExecutionReportDiff, ExecutionReportSummary,
        ExplanationDiff, ExplanationSummary, FailureDiff, FailureSummary, FlowCauseSample,
        FlowDiff, FlowInspector, FlowSummary, FrontierCyclePolicy, FrontierPropagationPolicy,
        FrontierTracingPolicy, GraphComparisonDiagnostics, GraphDiagnostics, GraphDiff,
        GraphInspector, GraphSummary,
        HistoryDiff, InvalidationCause, InvalidationSummary, LineageArtifactId, LineageDiff,
        LineageRecord, LineageRecordKind, ParallelAdmissionPolicy, PlanDiff, PlanInspector,
        PlanningSummary, PrecomputeSummary, ReconstructionBudget, ReplayCursor, ReplayDetailPolicy,
        ReplayDiff, ReplayEventKind, ReplayFrame, ReplaySlice, ReportInspector, RetentionBudget,
        RollbackDiagnostic, RollbackSummary, RuntimeDiagnostics, SemanticRetentionPolicy,
        SignalRuntimePolicy, SnapshotRestoreKind, SnapshotRestoreLineageMode,
    };
}

pub mod history {
    pub use crate::diagnostics::{LineageRecord, ReplaySlice, SnapshotRestoreKind};
    pub use crate::state::{
        SignalBranchHandle, SignalBranchId, SignalSnapshotDiagnostics, SignalSnapshotId,
        SignalSnapshotMeta, SignalSnapshotV1, SnapshotArtifactRestoreMode,
        SnapshotArtifactRetentionPolicy, SnapshotDependencyRestoreMode,
        SnapshotRestoreCoarseReason, SnapshotRestoreIntent, SnapshotRestorePlan,
        SnapshotStateRestoreMode,
    };
}

#[cfg(not(test))]
pub use self::core::{
    mark_changed, mark_changed_with_regions, mark_dirty, mark_dirty_with_regions, Aspect,
    AspectMask, AspectVersion,
    CanonicalChangedRegions, ChangedRegion, DependencyEdge, EvaluationCondition, NodeBuilder,
    NodeEvaluationResult, NodeId, NodeState, OutputChange, OutputIdentity, PartitionMatchMode,
    PartitionSubscription, PartitionToken, SignalError, SignalGraph, MAX_ASPECTS,
};
#[cfg(test)]
pub use self::core::*;
#[cfg(not(test))]
pub use self::diagnostics::{diagnostics_for_graph, diagnostics_for_runtime};
#[cfg(test)]
pub use self::diagnostics::*;
#[cfg(not(test))]
pub use self::advanced::{
    EvaluationContext, EvaluationRequestMode,
};
#[cfg(all(feature = "parallel", not(test)))]
pub use self::advanced::ParallelExecutionPolicy;
#[cfg(test)]
pub use self::advanced::*;
#[cfg(test)]
pub use self::integration::*;
#[cfg(not(test))]
pub use self::runtime::{
    mark_dirty_batch, BatchChange, BatchChangeResult, BatchChangeSession, EvaluationSummary,
    RuntimeHistory, RuntimeMerge, SignalRuntime, SignalRuntimePolicy, SignalTransaction,
    TransactionOutcome, TransactionResult, TransactionTiming,
};
#[cfg(test)]
pub use self::runtime::*;
#[cfg(test)]
pub use crate::presentation::boundaries::contracts::*;
#[cfg(test)]
pub use crate::presentation::boundaries::transaction_contract::*;
#[cfg(test)]
pub use crate::data::core_profile::CORE_STORAGE_PROFILE_ID;
#[cfg(test)]
pub use crate::data::dependency::CanonicalDependencies;
#[cfg(test)]
pub use crate::data::output::MemoizedResultOrigin;
#[cfg(test)]
pub use crate::data::trace::{
    ArtifactAuthorityClass, ArtifactMergeAuthority, CausalityMetadata, MergeAdoptability,
    RetainedDiagnosticArtifact,
};
#[cfg(test)]
pub use crate::logic::explain::{
    CausalDisposition, ConditionDecision, NodeExplanation, ScopeProvenanceKind, UpstreamCause,
};
#[cfg(test)]
pub use crate::logic::transaction::{DecisionDetail, DecisionRecord};
#[cfg(test)]
pub use crate::presentation::outputs::deployment::*;
#[cfg(test)]
pub use crate::presentation::harness::*;
#[cfg(test)]
pub use crate::presentation::metrics::{GraphMetrics, RuntimeMetrics};
#[cfg(test)]
pub use crate::tests::support::GraphDependencyBatchExt;
