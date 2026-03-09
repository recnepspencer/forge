//! Public API boundary for `forge-signal`.
//! External components should import through this module.
//!
//! Contract:
//! - `forge-signal` owns evaluation DAG scheduling.
//! - Host crates own external structural or state graphs, including cyclic ones.
//! - Compute closures may consume opaque host snapshots directly.

// Re-export Data constructs
pub use crate::data::aspect::{Aspect, AspectMask, AspectVersion, MAX_ASPECTS};
pub use crate::data::bitset::{BitsetFrontier, DenseBitset};
pub use crate::data::checkpoint::CheckpointBarrier;
pub use crate::data::checkpoint_policy::CheckpointPolicy;
pub use crate::data::comparator::{
    ComparatorPolicyResolver, DefaultComparatorPolicyResolver, DefaultComparatorResolver,
    TierPolicyResolver, VersionComparatorPolicy, VersionComparatorResolver,
};
pub use crate::data::core_profile::{
    AspectMaskBits, SignalCoreStorageProfile, StableHashValue, CORE_STORAGE_PROFILE,
    CORE_STORAGE_PROFILE_ID, HOT_VEC_INLINE_CAPACITY, STABLE_HASH_WIDTH_BITS,
};
pub use crate::data::dependency::DependencyEdge;
pub use crate::data::dirty_set::{BatchedDirtySet, DomainImpact};
pub use crate::data::effect_mapping::EffectMapping;
pub use crate::data::error::SignalError;
pub use crate::data::evaluator::CheckpointEvaluator;
pub use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
pub use crate::data::graph::{NodeBuilder, SignalGraph};
pub use crate::data::handle::NodeId;
pub use crate::data::node::{EvaluationCondition, NodeEntry, NodeEvaluationConfig, NodeState};
pub use crate::data::node_meta::NodeMetaStore;
pub use crate::data::output::{
    ChangedRegion, ComputationFamily, ComputationKey, KeyedComputation, MemoizedResultOrigin,
    NodeEvaluationResult, OutputChange, OutputIdentity, PartitionMatchMode, PartitionSubscription,
    PartitionToken, StructuralMemoKey,
};
pub use crate::data::subscriber_context::{SubscriberContext, SubscriberContextError};
pub use crate::data::telemetry::RuntimeTelemetry;
pub use crate::data::tier::{DependencyMode, DirtyPropagation, EvaluationTrigger, TierPolicy};
pub use crate::data::tier_policy_table::TierPolicyTable;
pub use crate::data::trace::{CausalityMetadata, TraceSummary};
pub use crate::diagnostics::{
    compare_execution_history, compare_execution_reports, compare_explanations, compare_failures,
    compare_flows, compare_graphs, compare_plans, diagnostics_for_graph, diagnostics_for_runtime,
    explanations_semantically_equivalent, graphs_semantically_equivalent, inspect_execution,
    inspect_flow, inspect_graph, inspect_plan, inspect_report, plans_semantically_equivalent,
    render_execution_history_summary, render_execution_report_summary, render_explanation_summary,
    render_failure_summary, render_flow_summary, render_graph_summary, render_plan_summary,
    repeat_run_summaries_equal, reports_semantically_equivalent,
    serial_parallel_reports_equivalent, ApplySummary, ArtifactMaterializationMode,
    ArtifactRetentionPolicy, ChangeInputSummary, DiagnosticMismatch, DiagnosticMismatchCategory,
    DiagnosticsPolicy, DiagnosticsProfile, EvaluationPlanSummary, ExecutionFailureContext,
    ExecutionFailurePhase, ExecutionHistoryNodeSummary, ExecutionHistorySummary,
    ExecutionInspector, ExecutionReportDiff, ExecutionReportSummary, ExplanationDiff,
    ExplanationSummary, FailureDiff, FailureSummary, FlowDiff, FlowInspector, FlowSummary,
    GraphDiagnostics, GraphDiff, GraphInspector, GraphSummary, HistoryDiff, InvalidationSummary,
    ParallelAdmissionPolicy, PlanDiff, PlanInspector, PlanningSummary, PrecomputeSummary,
    ReplayDetailPolicy, ReportInspector, RollbackDiagnostic, RollbackSummary, RuntimeDiagnostics,
    SemanticRetentionPolicy, SignalRuntimePolicy,
};

// Re-export Logic constructs
pub use crate::logic::checkpoint::CheckpointRuntime;
pub use crate::logic::context::EvaluationContext;
pub use crate::logic::evaluation::{
    apply_evaluation_result_with_policy_and_condition, ConditionEvaluationContext,
    ConditionResolver, DefaultConditionResolver, EvaluationExecutionMetadata,
    EvaluationRequestMode,
};
pub use crate::logic::events::{EventBus, EventFlushError, SubscriberRegistryError};
pub use crate::logic::explain::{
    ConditionDecision, MeaningfulChangeReason, NodeExplanation, UpstreamCause,
};
pub use crate::logic::invalidation::{mark_dirty, mark_dirty_with_regions};
#[cfg(feature = "parallel")]
pub use crate::logic::planner::ParallelExecutionPolicy;
pub use crate::logic::planner::{
    build_evaluation_plan, execute_prepared_plan, EvaluationPlan, EvaluationTask,
    ExecutionPruneReason, ExecutionRecordId, ExecutionReport, ExecutionStage, PlanSummary,
    SemanticSegmentId, SemanticTaskRange, StageBarrier, StageExecutionOutcome,
    StageExecutionRecord, StageExecutor, TaskExecutionOutcome, TaskExecutionRecord, TaskReason,
};
pub use crate::logic::prepared::{
    ExecutionReadView, ExecutionSnapshot, PreparedDependencyCapture, PreparedDependencyEdge,
    PreparedEvaluation, PreparedEvaluationOrigin, PreparedEvaluationOutcome, PreparedKeyedContext,
    PreparedMemoDecision, PreparedStage, PreparedTaskRecord, PreparedTraceData,
    SnapshotDependencyView, SnapshotNodeView, StageApplyResult, StageSnapshot,
};
pub use crate::logic::transaction::{
    emit_event_in_txn, flush_checkpoint_in_txn, SignalRuntime, SignalRuntimeBuilder,
    SignalRuntimeConfig, SignalTransaction, TransactionOutcome,
};
pub use crate::presentation::contracts::{
    DependencyGraphContract, RawPathComputeContract, StructuralStateBoundaryContract,
};
pub use crate::presentation::harness::{
    signal_bench, signal_parity_suite, SignalEvaluationDriver, SignalFixtureFactory,
    SignalHarnessAdapter, SignalHarnessAssert, SignalHarnessRuntime, SignalHarnessRuntimeBuilder,
    SignalHarnessSession, SignalMutationAction, SignalMutationBatch, SignalProfileCatalog,
    SignalScenario,
};
pub use crate::presentation::metrics::{GraphMetrics, RuntimeMetrics};
pub use crate::presentation::transaction_contract::TransactionRuntimeContract;
