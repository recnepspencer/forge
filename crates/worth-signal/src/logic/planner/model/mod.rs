mod admission;
mod apply;
mod frontier_route_receipt;
mod plan;
mod report;
mod strategy;
mod task;

pub use self::admission::ParallelAdmissionReason;
#[cfg(feature = "parallel")]
pub use self::apply::ApplyPlanSerialFallbackReason;
pub(crate) use self::apply::LoweredTaskExecution;
pub use self::apply::{
    ApplyFootprint, ConcurrentApplyPlan, ConcurrentApplyReductionPlan, DisjointApplyGroup,
    DisjointApplyProof, LoweredApplyPlan, LoweredStagePlan, LoweredTask, MutationDomain,
    ReductionOrderingContract, ReductionWorkClass, SerialApplyPlan, SharedSurfacePolicy,
};
pub use self::frontier_route_receipt::{
    FrontierRouteEvidenceReason, FrontierRouteEvidenceReceipt, FrontierRouteEvidenceReceiptError,
    FrontierRouteSerialFallbackReason,
};
pub(crate) use self::plan::{EvaluationCursor, SessionScratch, StageCursor};
pub use self::plan::{EvaluationPlan, ExecutionStage, PlanSummary};
pub use self::report::{
    ExecutedTask, ExecutionPruneReason, ExecutionReport, StageExecutionOutcome,
    StageExecutionRecord, TaskExecutionOutcome, TaskExecutionRecord,
};
#[cfg(feature = "parallel")]
pub use self::report::{ParallelApplyMode, ParallelExecutionKind};
#[cfg(feature = "parallel")]
pub use self::strategy::ParallelExecutionPolicy;
pub use self::strategy::StageExecutor;
pub use self::task::{
    CandidateTask, EligibleTask, EligibleTaskAdmission, ExecutionRecordId, MaybeStaleAdmission,
    SemanticSegmentId, SemanticTaskRange, StageBarrier, TaskReason,
};
