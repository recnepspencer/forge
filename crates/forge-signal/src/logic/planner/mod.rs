#[cfg(feature = "parallel")]
mod apply;
#[cfg(feature = "parallel")]
mod apply_groups;
#[cfg(feature = "parallel")]
mod apply_trace;
mod execution;
#[cfg(feature = "parallel")]
mod executor_pool;
#[cfg(feature = "parallel")]
mod full_parallel;
mod plan_builder;
mod precompute;
mod reporting;
mod semantic;
mod test_execution;
#[cfg(test)]
mod test_helpers;
mod types;
mod validation;

pub use execution::{execute_prepared_plan, execute_prepared_plan_with_policy};
pub use plan_builder::{build_evaluation_plan, build_evaluation_plan_with_policy_resolver};
#[cfg(test)]
pub(crate) use test_execution::{
    execute_plan_with_policy_and_condition, execute_test_prepared_plan_with_resolvers,
};
pub use types::{
    EvaluationPlan, EvaluationTask, ExecutionPruneReason, ExecutionRecordId, ExecutionReport,
    ExecutionStage, PlanSummary, SemanticSegmentId, SemanticTaskRange, StageBarrier,
    StageExecutionOutcome, StageExecutionRecord, StageExecutor, TaskExecutionOutcome,
    TaskExecutionRecord, TaskReason,
};
#[cfg(feature = "parallel")]
#[allow(unused_imports)]
pub use types::{ParallelApplyMode, ParallelExecutionKind, ParallelExecutionPolicy};
