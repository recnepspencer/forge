mod admission;
mod backend_completion_execution;
mod backpressure;
mod completion;
mod counters;
mod denial;
mod execution;
mod execution_accessors;
mod group_admission;
mod grouping;
#[cfg(test)]
mod grouping_basis_tests;
mod observation;
mod plan;
mod proof;
mod read_ahead;
mod replay;
#[cfg(test)]
mod test_execution;
mod work;
mod write_back;

pub use admission::{admit_queue_execution_plan, QueueExecutionAdmissionRequest};
pub use backend_completion_execution::{
    execute_grouped_ready_queue_plans, execute_ready_queue_plan,
};
pub use backpressure::QueueBackpressureCause;
pub use counters::QueueExecutionCounterSnapshot;
pub use denial::{QueueExecutionAdmissionDenial, QueueGroupingDenial};
pub use execution::{
    ExecutedQueueEvidence, QueueExecutionBackpressured, QueueExecutionDenied,
    QueueExecutionOutcome, QueueExecutionViolation, QueueExecutionViolationCause,
};
pub use group_admission::{
    group_ready_queue_pair, QueueGroupedReadyPlans, QueueGroupingOutcome, QueueGroupingRejected,
};
pub use grouping::{QueueGroupingBasis, QueueRecoveryOrdering, QueueWritebackPolicy};
pub(crate) use observation::QueueExecutionObservation;
pub use plan::{AdmittedQueueExecutionPlan, QueueExecutedPlan, QueueExecutionReadyPlan};
pub use proof::{
    queue_execution_lowering_authority, QueueExecutionLoweringAuthority, QueueExecutionProgression,
};
pub use read_ahead::QueueReadAheadBasis;
pub use replay::{QueueExecutionPlanBinding, QueueExecutionReplayIdentity};
pub use work::{
    lower_background_queue_lease, lower_buffer_pool_queue_declaration, lower_wal_queue_declaration,
    QueueWorkClass, QueueWorkDeclaration, S6QueueDurabilityClass,
};
pub use write_back::QueueWriteBackBasis;

#[cfg(test)]
pub(crate) use test_execution::execute_admitted_queue_plan;
#[cfg(test)]
pub(crate) mod test_support;
#[cfg(test)]
mod tests;
