#[cfg(feature = "parallel")]
mod apply;
#[cfg(feature = "parallel")]
mod apply_groups;
#[cfg(feature = "parallel")]
mod apply_trace;
mod stage_apply;
mod stage_recording;
mod execution;
mod execution_context;
mod execution_diagnostics;
mod execution_reporting;
#[cfg(feature = "parallel")]
mod executor_pool;
#[cfg(feature = "parallel")]
mod full_parallel;
mod plan_builder;
mod precompute;
mod precompute_dispatch;
mod precompute_reporting;
mod reporting;
mod rewiring;
mod semantic;
mod semantic_artifacts;
mod semantic_reporting;
mod stage_execution;
mod stage_precompute;
#[cfg(feature = "parallel")]
mod stage_admission;
mod test_execution;
#[cfg(test)]
mod test_helpers;
mod types;
mod validation;

pub use execution::{execute_prepared_plan, execute_prepared_plan_with_policy};
pub(crate) use execution::execute_evaluation_session_with_policy;
pub(crate) use plan_builder::build_evaluation_session_with_policy_resolver;
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
pub(crate) use types::EvaluationSession;
pub(crate) use types::StageCursor;
#[cfg(feature = "parallel")]
#[allow(unused_imports)]
pub use types::{ParallelApplyMode, ParallelExecutionKind, ParallelExecutionPolicy};
