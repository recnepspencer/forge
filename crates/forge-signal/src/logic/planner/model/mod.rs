use std::fmt;
#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;
#[cfg(feature = "parallel")]
use std::thread::available_parallelism;

use serde::{Deserialize, Serialize};

#[cfg(feature = "parallel")]
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::dependency::CanonicalDependencies;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::aspect::AspectMask;
use crate::data::node::{AuthorityPolicy, PathClass};
use crate::data::output::MemoizedResultOrigin;
use crate::data::performance::{ResolvedExecutionStrategy, ResolvedMaintenanceStrategy};
use crate::data::proof::{
    DedupedNodeBatch, LoweredForm, OrderedStreamItem, PartitionScopeSet, SortedSourceBatch,
    StructuralDelta,
};
use crate::data::reuse::{ReuseBasis, ReuseOrigin};
use crate::data::trace::RuntimeArtifactState;
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::evaluation::{DeferralReason, EvaluationVerdict, SuppressionReason};
use crate::logic::explain::RewiringSummary;
use crate::logic::prepared::PreparedEvaluation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskReason {
    Dirty,
    MaybeStaleValidation,
    ConditionForced,
    RequestedTarget,
    DependencyRequired,
    MemoValidation,
    PartitionScopedDependency,
    OutputDiffDependent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StageBarrier {
    StageBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionRecordId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SemanticSegmentId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticTaskRange {
    pub start: ExecutionRecordId,
    pub end: ExecutionRecordId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidateTask {
    pub node: NodeId,
    pub request_mode: EvaluationRequestMode,
    pub direct_request: bool,
    pub trigger_reason: TaskReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EligibleTask {
    pub node: NodeId,
    pub request_mode: EvaluationRequestMode,
    pub direct_request: bool,
    pub reason: TaskReason,
    pub admission: EligibleTaskAdmission,
}

impl OrderedStreamItem for EligibleTask {
    type OrderKey = (u32, u32);

    fn order_key(&self) -> Self::OrderKey {
        (self.node.index(), self.node.generation())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EligibleTaskAdmission {
    pub node_state_at_admission: Option<NodeState>,
    pub dirty_partition_scopes_present: bool,
    pub maybe_stale: Option<MaybeStaleAdmission>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaybeStaleAdmission {
    pub unchanged_at_admission: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplyFootprint {
    pub partitions: PartitionScopeSet,
    pub touched_nodes: DedupedNodeBatch,
    pub touched_sources: SortedSourceBatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisjointApplyGroup {
    pub task_indices: Vec<usize>,
    pub footprint: ApplyFootprint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharedSurfacePolicy {
    ReductionOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MutationDomain {
    LoweredStage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisjointApplyProof {
    pub stage_index: u32,
    pub mutation_domain: MutationDomain,
    pub group_footprints: Vec<ApplyFootprint>,
    pub shared_surface_policy: SharedSurfacePolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReductionOrderingContract {
    StageTaskIndexOrder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReductionWorkClass {
    DeterministicPublicationOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConcurrentApplyReductionPlan {
    pub ordering_contract: ReductionOrderingContract,
    pub allowed_work: ReductionWorkClass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ApplyPlanSerialFallbackReason {
    FullParallelUnsupportedByMutableEngine,
}

impl ApplyPlanSerialFallbackReason {
    #[cfg(feature = "parallel")]
    pub fn code(self) -> &'static str {
        match self {
            Self::FullParallelUnsupportedByMutableEngine => {
                "full-parallel-unsupported-by-mutable-engine"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParallelAdmissionReason {
    SerialExecutor,
    BelowMinStageWidth,
    BelowPolicyWorkThreshold,
    ValidationHeavyStage,
    BelowFullParallelThreshold,
    FullParallelUnsupportedByMutableEngine,
    AdmittedOperational,
    AdmittedDevelopment,
    AdmittedForensic,
    AdmittedProofSafeGroupedConcurrent,
}

impl ParallelAdmissionReason {
    pub fn code(self) -> &'static str {
        match self {
            Self::SerialExecutor => "serial-executor",
            Self::BelowMinStageWidth => "below-min-stage-width",
            Self::BelowPolicyWorkThreshold => "below-policy-work-threshold",
            Self::ValidationHeavyStage => "validation-heavy-stage",
            Self::BelowFullParallelThreshold => "below-full-parallel-threshold",
            Self::FullParallelUnsupportedByMutableEngine => {
                "full-parallel-unsupported-by-mutable-engine"
            }
            Self::AdmittedOperational => "admitted-operational",
            Self::AdmittedDevelopment => "admitted-development",
            Self::AdmittedForensic => "admitted-forensic",
            Self::AdmittedProofSafeGroupedConcurrent => {
                "admitted-proof-safe-grouped-concurrent"
            }
        }
    }

    pub fn message(self) -> &'static str {
        match self {
            Self::SerialExecutor => "parallelism was not requested for this stage",
            Self::BelowMinStageWidth => {
                "stage stayed serial because it did not meet the executor's minimum stage width"
            }
            Self::BelowPolicyWorkThreshold => {
                "stage stayed serial because the active runtime policy estimated the work was too small to amortize parallel overhead"
            }
            Self::ValidationHeavyStage => {
                "stage stayed serial because it was validation-heavy and unlikely to benefit from parallel overhead"
            }
            Self::BelowFullParallelThreshold => {
                "stage stayed out of full parallel mode because the active policy requires a larger stage for grouped concurrent apply"
            }
            Self::FullParallelUnsupportedByMutableEngine => {
                "stage stayed out of full parallel mode because the current mutable graph engine does not support concurrent apply yet"
            }
            Self::AdmittedOperational => {
                "stage ran in parallel under the low-overhead operational policy"
            }
            Self::AdmittedDevelopment => {
                "stage ran in parallel under the development policy"
            }
            Self::AdmittedForensic => {
                "stage ran in parallel under the forensic policy"
            }
            Self::AdmittedProofSafeGroupedConcurrent => {
                "stage ran through proof-safe grouped concurrent apply with deterministic reduction-only publication"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SerialApplyPlan {
    pub groups: Vec<DisjointApplyGroup>,
    pub rejection_reason: Option<ApplyPlanSerialFallbackReason>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConcurrentApplyPlan {
    pub groups: Vec<DisjointApplyGroup>,
    pub proof: DisjointApplyProof,
    pub reduction: ConcurrentApplyReductionPlan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoweredApplyPlan {
    Serial(SerialApplyPlan),
    GroupedConcurrent(ConcurrentApplyPlan),
}

#[derive(Debug, Clone)]
pub(crate) struct LoweredTaskExecution {
    pub prepared: PreparedEvaluation,
    pub before_state: NodeState,
    pub before_artifact_state: Option<RuntimeArtifactState>,
    pub dependency_updates: u32,
    pub recomputed: bool,
    pub partition_aware: bool,
    pub rewiring: Option<RewiringSummary>,
}

#[derive(Debug, Clone)]
pub struct LoweredTask {
    pub task_index: usize,
    pub node: NodeId,
    pub produced_aspects: AspectMask,
    pub dependency_inputs: CanonicalDependencies,
    #[cfg(feature = "parallel")]
    pub comparator_policy: VersionComparatorPolicy,
    pub path_class: PathClass,
    pub authority_policy: AuthorityPolicy,
    pub footprint: ApplyFootprint,
    pub(crate) execution: LoweredTaskExecution,
}

#[derive(Debug, Clone)]
pub struct LoweredStagePlan {
    pub stage_index: u32,
    pub tasks: Vec<LoweredTask>,
    pub lowered_apply_plan: LoweredApplyPlan,
    pub dirty_delta: StructuralDelta,
    pub execution_strategy: ResolvedExecutionStrategy,
    pub maintenance_strategy: ResolvedMaintenanceStrategy,
    pub authority_policy: AuthorityPolicy,
}

impl LoweredStagePlan {
    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn apply_groups(&self) -> &[DisjointApplyGroup] {
        match &self.lowered_apply_plan {
            LoweredApplyPlan::Serial(plan) => plan.groups.as_slice(),
            LoweredApplyPlan::GroupedConcurrent(plan) => plan.groups.as_slice(),
        }
    }
}

impl LoweredForm for ApplyFootprint {}
impl LoweredForm for DisjointApplyGroup {}
impl LoweredForm for LoweredTask {}
impl LoweredForm for LoweredStagePlan {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionStage {
    pub index: u32,
    pub tasks: Vec<EligibleTask>,
    pub barrier: Option<StageBarrier>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct StageCursor {
    pub index: u32,
    pub start: usize,
    pub end: usize,
    pub barrier: Option<StageBarrier>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlanSummary {
    pub requested_target_count: u32,
    pub stage_count: u32,
    pub task_count: u32,
    pub max_stage_width: u32,
    pub contract_pruned_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationPlan {
    pub request_mode: EvaluationRequestMode,
    pub targets: Vec<NodeId>,
    pub stages: Vec<ExecutionStage>,
    pub summary: PlanSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct EvaluationCursor {
    pub request_mode: EvaluationRequestMode,
    pub targets: Vec<NodeId>,
    pub tasks: Vec<EligibleTask>,
    pub stages: Vec<StageCursor>,
    pub summary: PlanSummary,
}

#[derive(Debug, Clone)]
pub(crate) struct SessionScratch<'a> {
    pub targets: &'a [NodeId],
    pub tasks: &'a [EligibleTask],
    pub stages: &'a [StageCursor],
    pub summary: PlanSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionPruneReason {
    CleanAtPlanTime,
    CleanAfterValidation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskExecutionOutcome {
    Recomputed,
    ValidatedClean,
    ConditionDeferred,
    ConditionRevertedClean,
    MemoizedReuse,
    SnapshotRestoreReuse,
    ReconciliationAdoption,
    CrossIdentityPersistentReuse,
    PartialArtifactSplice,
    PropagationSuppressed,
    Pruned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StageExecutionOutcome {
    CompletedSerial,
    #[cfg(feature = "parallel")]
    CompletedParallel,
}

#[cfg(feature = "parallel")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParallelExecutionKind {
    StagedParallelPrecompute,
    FullParallel,
}

#[cfg(feature = "parallel")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParallelApplyMode {
    SerialApply,
    GroupedConcurrentApply,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskExecutionRecord {
    pub id: ExecutionRecordId,
    pub semantic_segment_id: SemanticSegmentId,
    pub node: NodeId,
    pub scheduled_reason: TaskReason,
    pub direct_request: bool,
    pub outcome: TaskExecutionOutcome,
    pub verdict: Option<EvaluationVerdict>,
    pub suppression_reason: Option<SuppressionReason>,
    pub deferral_reason: Option<DeferralReason>,
    pub prune_reason: Option<ExecutionPruneReason>,
    pub recomputed: bool,
    pub memoized_origin: MemoizedResultOrigin,
    pub reuse_basis: ReuseBasis,
    pub reuse_origin: ReuseOrigin,
    pub propagation_suppressed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutedTask {
    pub task: EligibleTask,
    pub record: TaskExecutionRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageExecutionRecord {
    pub stage_index: u32,
    pub outcome: StageExecutionOutcome,
    pub authority_policy: Option<AuthorityPolicy>,
    pub parallel_admission_reason: Option<ParallelAdmissionReason>,
    #[cfg(feature = "parallel")]
    pub parallel_kind: Option<ParallelExecutionKind>,
    #[cfg(feature = "parallel")]
    pub apply_mode: Option<ParallelApplyMode>,
    #[cfg(feature = "parallel")]
    pub apply_group_count: u32,
    #[cfg(feature = "parallel")]
    pub serial_apply_rejection_reason: Option<ApplyPlanSerialFallbackReason>,
    #[cfg(feature = "parallel")]
    pub serial_fallback_group_count: u32,
    #[cfg(feature = "parallel")]
    pub concurrent_apply_task_count: u32,
    #[cfg(feature = "parallel")]
    pub serial_apply_task_count: u32,
    pub snapshot_duration_nanos: u128,
    pub precompute_duration_nanos: u128,
    pub apply_duration_nanos: u128,
    pub semantic_finalize_duration_nanos: u128,
    pub duration_nanos: u128,
    pub semantic_task_range: Option<SemanticTaskRange>,
    pub semantic_segment_count: u32,
    pub task_records: Vec<TaskExecutionRecord>,
}

impl StageExecutionRecord {
    pub fn parallel_admission_message(&self) -> &'static str {
        self.parallel_admission_reason
            .map(ParallelAdmissionReason::message)
            .unwrap_or("parallel admission reason was not recorded")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionReport {
    pub plan_summary: PlanSummary,
    pub stage_count: u32,
    pub task_count: u32,
    pub tasks_executed: u32,
    pub tasks_pruned: u32,
    pub tasks_validated_clean: u32,
    pub tasks_deferred_by_condition: u32,
    pub tasks_reverted_clean_by_condition: u32,
    pub tasks_satisfied_by_memoization: u32,
    pub tasks_with_suppressed_propagation: u32,
    pub execution_snapshots_built: u32,
    pub prepared_evaluations_produced: u32,
    pub prepared_evaluations_applied: u32,
    pub dependency_capture_updates: u32,
    pub execution_snapshot_nanos: u128,
    pub stage_precompute_nanos: u128,
    pub stage_apply_nanos: u128,
    pub semantic_finalize_nanos: u128,
    pub semantic_segment_count: u32,
    pub stages: Vec<StageExecutionRecord>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum StageExecutor {
    #[default]
    Serial,
    #[cfg(feature = "parallel")]
    StagedParallelPrecompute { policy: ParallelExecutionPolicy },
    #[cfg(feature = "parallel")]
    FullParallel { policy: ParallelExecutionPolicy },
}

impl StageExecutor {
    /// Conservative full-parallel preset for request-driven or observability-heavy workloads.
    ///
    /// Prefer this when you want a sane default without tuning low-level
    /// worker/chunk/apply-group parameters yourself.
    #[cfg(feature = "parallel")]
    pub fn conservative_parallel() -> Self {
        Self::full_parallel(16).with_parallel_policy(
            ParallelExecutionPolicy::new(
                NonZeroUsize::new(16).expect("constant min stage width is non-zero"),
            )
            .with_worker_count(2)
            .with_chunk_size(2)
            .with_apply_group_min_width(2)
            .with_max_concurrent_apply_groups(2),
        )
    }

    /// Balanced full-parallel preset for general production workloads.
    ///
    /// This is the default "I want parallelism, but not a science project"
    /// choice for most nontrivial deployments.
    #[cfg(feature = "parallel")]
    pub fn balanced_parallel() -> Self {
        Self::full_parallel(12).with_parallel_policy(
            ParallelExecutionPolicy::new(
                NonZeroUsize::new(12).expect("constant min stage width is non-zero"),
            )
            .with_worker_count(available_parallelism().map_or(4, |count| count.get().min(4)))
            .with_chunk_size(2)
            .with_apply_group_min_width(1)
            .with_max_concurrent_apply_groups(2),
        )
    }

    /// Aggressive full-parallel preset for heavier compute kernels and hostile certification.
    ///
    /// Prefer this for benchmark pressure, heavy compute, or when you want to
    /// intentionally push more work into concurrent apply.
    #[cfg(feature = "parallel")]
    pub fn aggressive_parallel() -> Self {
        Self::full_parallel(8).with_parallel_policy(
            ParallelExecutionPolicy::new(
                NonZeroUsize::new(8).expect("constant min stage width is non-zero"),
            )
            .with_worker_count(available_parallelism().map_or(4, |count| count.get().min(8)))
            .with_chunk_size(1)
            .with_apply_group_min_width(1)
            .with_max_concurrent_apply_groups(4),
        )
    }

    #[cfg(feature = "parallel")]
    pub fn parallel(min_stage_width: usize) -> Self {
        Self::staged_parallel_precompute(min_stage_width)
    }

    #[cfg(feature = "parallel")]
    pub fn staged_parallel_precompute(min_stage_width: usize) -> Self {
        let min_stage_width = match NonZeroUsize::new(min_stage_width.max(1)) {
            Some(width) => width,
            None => unreachable!("parallel min stage width is clamped to at least one"),
        };
        Self::StagedParallelPrecompute {
            policy: ParallelExecutionPolicy::new(min_stage_width),
        }
    }

    #[cfg(feature = "parallel")]
    pub fn full_parallel(min_stage_width: usize) -> Self {
        let min_stage_width = match NonZeroUsize::new(min_stage_width.max(1)) {
            Some(width) => width,
            None => unreachable!("parallel min stage width is clamped to at least one"),
        };
        Self::FullParallel {
            policy: ParallelExecutionPolicy::new(min_stage_width),
        }
    }

    #[cfg(feature = "parallel")]
    pub fn with_parallel_policy(self, policy: ParallelExecutionPolicy) -> Self {
        match self {
            Self::Serial => Self::Serial,
            Self::StagedParallelPrecompute { .. } => Self::StagedParallelPrecompute { policy },
            Self::FullParallel { .. } => Self::FullParallel { policy },
        }
    }

    #[cfg(feature = "parallel")]
    pub(crate) fn parallel_kind(&self) -> Option<ParallelExecutionKind> {
        match self {
            Self::Serial => None,
            Self::StagedParallelPrecompute { .. } => {
                Some(ParallelExecutionKind::StagedParallelPrecompute)
            }
            Self::FullParallel { .. } => Some(ParallelExecutionKind::FullParallel),
        }
    }

    #[cfg(feature = "parallel")]
    pub(crate) fn parallel_policy(&self) -> Option<ParallelExecutionPolicy> {
        match self {
            Self::Serial => None,
            Self::StagedParallelPrecompute { policy } | Self::FullParallel { policy } => {
                Some(*policy)
            }
        }
    }

    #[cfg(feature = "parallel")]
    pub(crate) fn is_full_parallel(&self) -> bool {
        matches!(self, Self::FullParallel { .. })
    }
}

#[cfg(feature = "parallel")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParallelExecutionPolicy {
    pub min_stage_width: NonZeroUsize,
    pub worker_count: Option<NonZeroUsize>,
    pub chunk_size: Option<NonZeroUsize>,
    pub apply_group_min_width: NonZeroUsize,
    pub max_concurrent_apply_groups: Option<NonZeroUsize>,
}

#[cfg(feature = "parallel")]
impl ParallelExecutionPolicy {
    pub fn new(min_stage_width: NonZeroUsize) -> Self {
        Self {
            min_stage_width,
            worker_count: None,
            chunk_size: None,
            apply_group_min_width: min_stage_width,
            max_concurrent_apply_groups: None,
        }
    }

    pub fn with_worker_count(mut self, worker_count: usize) -> Self {
        self.worker_count = NonZeroUsize::new(worker_count.max(1));
        self
    }

    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = NonZeroUsize::new(chunk_size.max(1));
        self
    }

    pub fn with_apply_group_min_width(mut self, min_width: usize) -> Self {
        self.apply_group_min_width = NonZeroUsize::new(min_width.max(1))
            .expect("apply group min width is clamped to at least one");
        self
    }

    pub fn with_max_concurrent_apply_groups(mut self, max_groups: usize) -> Self {
        self.max_concurrent_apply_groups = NonZeroUsize::new(max_groups.max(1));
        self
    }

    pub(crate) fn chunk_size_for(self, task_count: usize) -> usize {
        if let Some(chunk_size) = self.chunk_size {
            return chunk_size.get().min(task_count.max(1));
        }
        let workers = self
            .worker_count
            .map(|count| count.get())
            .or_else(|| available_parallelism().ok().map(|count| count.get()))
            .unwrap_or(1)
            .max(1);
        task_count.div_ceil(workers).max(1)
    }

    pub(crate) fn worker_count_for(self, task_count: usize) -> usize {
        self.worker_count
            .map(|count| count.get())
            .or_else(|| available_parallelism().ok().map(|count| count.get()))
            .unwrap_or(1)
            .min(task_count.max(1))
            .max(1)
    }

    pub(crate) fn max_apply_group_count_for(self, task_count: usize) -> usize {
        self.max_concurrent_apply_groups
            .map(|count| count.get())
            .unwrap_or_else(|| self.worker_count_for(task_count))
            .min(task_count.max(1))
            .max(1)
    }
}

impl fmt::Display for PlanSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "targets={} stages={} tasks={} max_stage_width={} contract_pruned={}",
            self.requested_target_count,
            self.stage_count,
            self.task_count,
            self.max_stage_width,
            self.contract_pruned_count
        )
    }
}

impl fmt::Display for EvaluationPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "EvaluationPlan {}", self.summary)?;
        for stage in &self.stages {
            writeln!(f, "  stage {} tasks={}", stage.index, stage.tasks.len())?;
            for task in &stage.tasks {
                writeln!(
                    f,
                    "    {} direct={} reason={:?} mode={:?}",
                    task.node, task.direct_request, task.reason, task.request_mode
                )?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for ExecutionReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "ExecutionReport stages={} tasks={} executed={} pruned={} validated_clean={} deferred={} memoized={} suppressed={}",
            self.stage_count,
            self.task_count,
            self.tasks_executed,
            self.tasks_pruned,
            self.tasks_validated_clean,
            self.tasks_deferred_by_condition,
            self.tasks_satisfied_by_memoization,
            self.tasks_with_suppressed_propagation
        )?;
        for stage in &self.stages {
            writeln!(
                f,
                "  stage {} outcome={:?} tasks={}",
                stage.stage_index,
                stage.outcome,
                stage.task_records.len()
            )?;
        }
        Ok(())
    }
}
