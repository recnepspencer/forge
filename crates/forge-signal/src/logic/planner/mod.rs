pub(crate) mod apply;
mod execution;
pub(crate) mod model;
pub(crate) mod planning;
pub(crate) mod precompute;
pub(crate) mod semantic;
#[cfg(test)]
mod tests;

pub(crate) use self::apply::stage as stage_apply;
#[cfg(feature = "parallel")]
pub(crate) use self::apply::full_parallel;
pub(crate) use self::execution::task_reporting as reporting;
pub(crate) use self::model as types;
pub(crate) use self::planning as plan_builder;
pub(crate) use self::planning::validation;
pub(crate) use self::precompute::{reporting as precompute_reporting, stage as stage_precompute};
#[cfg(feature = "parallel")]
pub(crate) use self::precompute::{admission as stage_admission, executor_pool};
pub(crate) use self::semantic::stage_recording;

pub use execution::{execute_prepared_plan, execute_prepared_plan_with_policy};
pub(crate) use execution::execute_evaluation_session_with_policy;
pub(crate) use plan_builder::build_evaluation_session_with_policy_resolver;
pub use plan_builder::{build_evaluation_plan, build_evaluation_plan_with_policy_resolver};
#[cfg(test)]
pub(crate) use tests::{
    execute_plan_with_policy_and_condition, execute_test_prepared_plan_with_resolvers,
};
pub use model::{
    EvaluationPlan, EvaluationTask, ExecutionPruneReason, ExecutionRecordId, ExecutionReport,
    ExecutionStage, PlanSummary, SemanticSegmentId, SemanticTaskRange, StageBarrier,
    StageExecutionOutcome, StageExecutionRecord, StageExecutor, TaskExecutionOutcome,
    TaskExecutionRecord, TaskReason,
};
pub(crate) use model::EvaluationSession;
pub(crate) use model::StageCursor;
#[cfg(feature = "parallel")]
#[allow(unused_imports)]
pub use model::{ParallelApplyMode, ParallelExecutionKind, ParallelExecutionPolicy};
