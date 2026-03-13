//! Public API boundary for `forge-signal`.
//! External components should import through this module.
//!
//! Contract:
//! - `forge-signal` owns evaluation DAG scheduling.
//! - Host crates own external structural or state graphs, including cyclic ones.
//! - Compute closures may consume opaque host snapshots directly.

pub mod types {
    pub use crate::data::aspect::{Aspect, AspectMask, AspectVersion, MAX_ASPECTS};
    pub use crate::data::bitset::{BitsetFrontier, DenseBitset};
    pub use crate::data::checkpoint::CheckpointBarrier;
    pub use crate::data::checkpoint_policy::CheckpointPolicy;
    pub use crate::data::core_profile::{
        AspectMaskBits, SignalCoreStorageProfile, StableHashValue, CORE_STORAGE_PROFILE,
        CORE_STORAGE_PROFILE_ID, HOT_VEC_INLINE_CAPACITY, STABLE_HASH_WIDTH_BITS,
    };
    pub use crate::data::dependency::{
        CanonicalDependencies, DependencyEdge, SharedDependencySnapshot, SnapshotDeltaRecord,
    };
    pub use crate::data::dirty_set::{BatchedDirtySet, DomainImpact};
    pub use crate::data::error::SignalError;
    pub use crate::data::handle::NodeId;
    pub use crate::data::node::{
        ArtifactPolicyClass, AuthorityPolicy, CanonicalDependencyOrder, ComparatorBasis,
        CompileTimePerformanceContract, ContextRequirement, EquivalenceContract,
        EvaluationCondition, IdentityBasis, MaintenanceMode, NodeAuthorityContract, NodeContract,
        NodeEntry, NodeEvaluationConfig, NodeExecutionContract, NodeProjectionContract,
        NodeReuseContract, NodeSemanticContract, NodeState, PathClass, PerformanceCounterSurface,
        PerformanceEnforcementLayer, ResolvedPerformancePolicy, SuppressionBasis,
    };
    pub use crate::data::output::{
        CanonicalChangedRegions, ChangedRegion, ComputationFamily, ComputationKey,
        KeyedComputation, MemoizedResultOrigin, NodeEvaluationResult, OutputChange, OutputIdentity,
        PartitionMatchMode, PartitionSubscription, PartitionToken, StructuralMemoKey,
    };
    pub use crate::data::proof::{
        CanonicalForm, DedupedNodeBatch, DeltaForm, DependencyBatchEdit, DependencySetEdit,
        DesiredState, DirtyBatch, DirtyBatchEntry, DirtyDelta, FrontierWave, InvalidationFrontier,
        LocalityFootprint, LocallyOrderedShard, LoweredForm, MergeableOrderedStream,
        NarrowedPropagationSet, OrderedStreamItem, OrderedStreamMergeError, PartitionScopeSet,
        PatchPlan, PendingSnapshotBatch, ResolvedForm, SemanticBatchCommit, SingleConsumer,
        SnapshotBatchCommit, SortedSourceBatch, StructuralDelta, SubscriberRepair,
        SubscriberRepairBatch, SummaryForm, TouchedScopeSummary,
    };
    pub use crate::data::reuse::{
        ArtifactEquivalenceContract, ArtifactSemanticBoundary, ReuseBasis, ReuseBoundaryContext,
        ReuseBoundaryEvidence, ReuseBoundaryFailure, ReuseBoundaryProof, ReuseCertificationFailure,
        ReuseCertificationRecord, ReuseCrossing, ReuseSemanticRegionIdentity, ReuseSource,
    };
    pub use crate::data::tier::{DependencyMode, DirtyPropagation, EvaluationTrigger, TierPolicy};
    pub use crate::data::trace::{
        CausalityMetadata, HistoricalArtifactRecord, RetainedDiagnosticArtifact,
        RuntimeArtifactState, TraceSummary,
    };
    pub use crate::state::{
        SignalBranchHandle, SignalBranchId, SignalSnapshotDiagnostics, SignalSnapshotId,
        SignalSnapshotMeta, SignalSnapshotV1,
    };
}

pub mod graph {
    pub use crate::data::comparator::{
        ComparatorPolicyResolver, DefaultComparatorPolicyResolver, DefaultComparatorResolver,
        TierPolicyResolver, VersionComparatorPolicy, VersionComparatorResolver,
    };
    pub use crate::data::effect_mapping::EffectMapping;
    pub use crate::data::evaluator::CheckpointEvaluator;
    pub use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
    pub use crate::data::graph::{
        EvaluationStrategy, GcPressure, GraphObserver, NodeBuilder, ObservationLevel,
        ParallelismHint, SignalGraph,
    };
    pub use crate::data::node_meta::NodeMetaStore;
    pub use crate::data::subscriber_context::{SubscriberContext, SubscriberContextError};
    pub use crate::data::telemetry::RuntimeTelemetry;
    pub use crate::data::tier_policy_table::TierPolicyTable;
}

pub mod evaluation {
    pub use crate::logic::context::EvaluationContext;
    pub use crate::logic::evaluation::{
        AppliedEffectReport, ConditionEvaluationContext, ConditionResolver,
        DefaultConditionResolver, DeferralReason, DiagnosticEnvelope, EvaluationExecutionMetadata,
        EvaluationOutput, EvaluationRequestMode, EvaluationVerdict, IntoEvaluationOutput,
        OperationalEffect, SuppressionReason,
    };
    pub use crate::logic::explain::{
        CausalDisposition, CausalLink, ConditionDecision, MeaningfulChangeReason, NodeExplanation,
        RewiringDependency, RewiringSummary, ScopeProvenance, ScopeProvenanceKind, UpstreamCause,
    };
}

pub mod planning {
    #[cfg(feature = "parallel")]
    pub use crate::logic::planner::ParallelExecutionPolicy;
    pub use crate::logic::planner::{
        build_evaluation_plan, execute_prepared_plan, CandidateTask, EligibleTask, EvaluationPlan,
        ExecutedTask, ExecutionPruneReason, ExecutionRecordId, ExecutionReport, ExecutionStage,
        PlanSummary, ResolvedExecutionStrategy, ResolvedMaintenanceStrategy, SemanticSegmentId,
        SemanticTaskRange, StageBarrier, StageExecutionOutcome, StageExecutionRecord, StageExecutor,
        TaskExecutionOutcome, TaskExecutionRecord, TaskReason,
    };
}

pub mod performance {
    pub use crate::data::performance::{
        ArtifactPolicyClass, AuthorityPolicy, CanonicalDependencyOrder, ComparatorBasis,
        CompileTimePerformanceContract, EquivalenceContract, IdentityBasis, MaintenanceMode,
        PathClass, PerformanceCounterSurface, PerformanceEnforcementLayer,
        ResolvedExecutionStrategy, ResolvedMaintenanceStrategy, ResolvedPerformancePolicy,
        SuppressionBasis,
    };
}

pub mod proof {
    pub use crate::data::proof::{
        CanonicalForm, DedupedNodeBatch, DeltaForm, DependencyBatchEdit, DependencySetEdit,
        DesiredState, DirtyBatch, DirtyBatchEntry, DirtyDelta, FrontierWave, InvalidationFrontier,
        LocalityFootprint, LocallyOrderedShard, LoweredForm, MergeableOrderedStream,
        NarrowedPropagationSet, OrderedStreamItem, OrderedStreamMergeError, PartitionScopeSet,
        PatchPlan, PendingSnapshotBatch, ResolvedForm, SemanticBatchCommit, SingleConsumer,
        SnapshotBatchCommit, SortedSourceBatch, StructuralDelta, SubscriberRepair,
        SubscriberRepairBatch, SummaryForm, TouchedScopeSummary,
    };
}

pub mod transaction {
    pub use crate::logic::checkpoint::CheckpointRuntime;
    pub use crate::logic::events::{EventBus, EventFlushError, SubscriberRegistryError};
    pub use crate::logic::invalidation::mark_dirty_batch;
    pub use crate::logic::transaction::RuntimeObserver;
    pub use crate::logic::transaction::{
        emit_event_in_txn, flush_checkpoint_in_txn, AdvisoryRecord, ComputationSpec,
        DecisionDetail, DecisionLog, DecisionRecord, DecisionSummary, DefinedComputation,
        DefinedKeyedComputation, EvaluationSummary, IntegrityMarkers, SignalRuntime,
        SignalRuntimeBuilder, SignalRuntimeConfig, SignalTransaction, TransactionOutcome,
        TransactionReplayEntry, TransactionResult, TransactionTiming,
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
        serial_parallel_reports_equivalent, ApplySummary, ArtifactMaterializationMode,
        ArtifactRetentionPolicy, ArtifactTransitionKind, ChangeInputSummary, DiagnosticMismatch,
        DiagnosticMismatchCategory, DiagnosticsPolicy, DiagnosticsProfile, EvaluationPlanSummary,
        EventEpochOutcome, EventEpochSummary, EventSubscriberOutcome, EventSubscriberOutcomeKind,
        ExecutionFailureContext, ExecutionFailurePhase, ExecutionHistoryNodeSummary,
        ExecutionHistorySummary, ExecutionInspector, ExecutionReportDiff, ExecutionReportSummary,
        ExplanationDiff, ExplanationSummary, FailureDiff, FailureSummary, FlowCauseSample,
        FlowDiff, FlowInspector, FlowSummary, GraphDiagnostics, GraphDiff, GraphInspector,
        GraphSummary, HistoryDiff, InvalidationCause, InvalidationSummary, LineageArtifactId,
        LineageDiff, LineageRecord, LineageRecordKind, ParallelAdmissionPolicy, PlanDiff,
        PlanInspector, PlanningSummary, PrecomputeSummary, ReplayCursor, ReplayDetailPolicy,
        ReplayDiff, ReplayEventKind, ReplayFrame, ReplaySlice, ReportInspector, RollbackDiagnostic,
        RollbackSummary, RuntimeDiagnostics, SemanticRetentionPolicy, SignalRuntimePolicy,
        SnapshotRestoreKind, SnapshotRestoreLineageMode,
    };
}

pub mod harness {
    pub use crate::presentation::contracts::{
        DependencyGraphContract, RawPathComputeContract, StructuralStateBoundaryContract,
    };
    pub use crate::presentation::deployment::{SignalDeploymentPlan, SignalDeploymentPreset};
    pub use crate::presentation::harness::{
        signal_bench, signal_parity_suite, SignalEvaluationDriver, SignalFixtureFactory,
        SignalHarnessAssert, SignalHarnessBridge, SignalHarnessRuntime,
        SignalHarnessRuntimeBuilder, SignalHarnessSession, SignalMutationAction,
        SignalMutationBatch, SignalProfileCatalog, SignalScenario,
    };
    pub use crate::presentation::metrics::{GraphMetrics, RuntimeMetrics};
    pub use crate::presentation::transaction_contract::TransactionRuntimeContract;
}

#[cfg(test)]
pub(crate) use self::harness::*;
pub(crate) use self::{
    diagnostics::*, evaluation::*, graph::*, planning::*, transaction::*, types::*,
};
#[cfg(any(test, doctest))]
#[allow(unused_imports)]
pub(crate) use crate::logic::invalidation::{mark_dirty, mark_dirty_with_regions};
#[cfg(test)]
pub use crate::tests::support::GraphDependencyBatchExt;
