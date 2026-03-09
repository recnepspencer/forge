use std::fmt;
#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;
#[cfg(feature = "parallel")]
use std::thread::available_parallelism;

use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;
use crate::logic::evaluation::EvaluationRequestMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationTask {
    pub node: NodeId,
    pub request_mode: EvaluationRequestMode,
    pub direct_request: bool,
    pub reason: TaskReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionStage {
    pub index: u32,
    pub tasks: Vec<EvaluationTask>,
    pub barrier: Option<StageBarrier>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlanSummary {
    pub requested_target_count: u32,
    pub stage_count: u32,
    pub task_count: u32,
    pub max_stage_width: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationPlan {
    pub request_mode: EvaluationRequestMode,
    pub targets: Vec<NodeId>,
    pub stages: Vec<ExecutionStage>,
    pub summary: PlanSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionPruneReason {
    CleanAtPlanTime,
    CleanAfterValidation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskExecutionOutcome {
    Recomputed,
    ValidatedClean,
    ConditionDeferred,
    ConditionRevertedClean,
    MemoizedReuse,
    PropagationSuppressed,
    Pruned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    SerialFallback,
    GroupedConcurrentApply,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskExecutionRecord {
    pub id: ExecutionRecordId,
    pub node: NodeId,
    pub scheduled_reason: TaskReason,
    pub direct_request: bool,
    pub outcome: TaskExecutionOutcome,
    pub prune_reason: Option<ExecutionPruneReason>,
    pub recomputed: bool,
    pub memoized_reuse: bool,
    pub condition_deferred: bool,
    pub condition_reverted_clean: bool,
    pub propagation_suppressed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageExecutionRecord {
    pub stage_index: u32,
    pub outcome: StageExecutionOutcome,
    #[cfg(feature = "parallel")]
    pub parallel_kind: Option<ParallelExecutionKind>,
    #[cfg(feature = "parallel")]
    pub apply_mode: Option<ParallelApplyMode>,
    #[cfg(feature = "parallel")]
    pub apply_group_count: u32,
    #[cfg(feature = "parallel")]
    pub serial_fallback_group_count: u32,
    #[cfg(feature = "parallel")]
    pub concurrent_apply_task_count: u32,
    #[cfg(feature = "parallel")]
    pub serial_apply_task_count: u32,
    pub snapshot_duration_nanos: u128,
    pub precompute_duration_nanos: u128,
    pub apply_duration_nanos: u128,
    pub duration_nanos: u128,
    pub task_records: Vec<TaskExecutionRecord>,
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
    pub(crate) fn uses_parallel_for_stage(&self, stage: &ExecutionStage) -> bool {
        matches!(
            self,
            Self::StagedParallelPrecompute { policy } | Self::FullParallel { policy }
                if stage.tasks.len() >= policy.min_stage_width.get()
        )
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

}

impl fmt::Display for PlanSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "targets={} stages={} tasks={} max_stage_width={}",
            self.requested_target_count, self.stage_count, self.task_count, self.max_stage_width
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
