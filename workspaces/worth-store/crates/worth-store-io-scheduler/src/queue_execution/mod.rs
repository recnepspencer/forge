mod admission;
mod execution;
#[cfg(test)]
mod grouping_basis_tests;
mod observation;
mod policy;
#[cfg(test)]
mod test_execution;

pub use admission::{
    admit_queue_execution_plan, admit_queue_policy_receipt, QueueExecutionAdmissionRequest,
    QueuePolicyAdmissionReceipt,
};
pub use admission::{
    group_ready_queue_pair, QueueExecutionAdmissionDenial, QueueGroupedReadyPlans,
    QueueGroupingDenial, QueueGroupingOutcome, QueueGroupingRejected,
};
pub use execution::{
    execute_grouped_ready_queue_plans, execute_ready_queue_plan,
    queue_execution_lowering_authority, AdmittedQueueExecutionPlan, ExecutedQueueEvidence,
    QueueExecutedPlan, QueueExecutionBackpressured, QueueExecutionDenied,
    QueueExecutionLoweringAuthority, QueueExecutionOutcome, QueueExecutionProgression,
    QueueExecutionReadyPlan, QueueExecutionViolation, QueueExecutionViolationCause,
};
pub(crate) use observation::{
    QueueExecutionCounterBasis, QueueExecutionObservation, QueueExecutionUnitCounts,
};
pub use observation::{
    QueueExecutionCounterSnapshot, QueueExecutionPlanBinding, QueueExecutionReplayIdentity,
};
pub use policy::{
    lower_background_queue_lease, lower_buffer_pool_read_queue_declaration,
    lower_buffer_pool_writeback_queue_declaration, lower_physical_foreground_work,
    lower_wal_queue_declaration, QueueBackpressureCause, QueueDurabilityClass, QueueGroupingBasis,
    QueueLocalityIdentity, QueueLocalityRange, QueueLocalityRelation, QueueReadAheadBasis,
    QueueRecoveryOrdering, QueueWorkClass, QueueWorkDeclaration, QueueWriteBackBasis,
    QueueWritebackPolicy,
};
#[cfg(test)]
pub(crate) use test_execution::execute_admitted_queue_plan;
#[cfg(test)]
pub(crate) mod test_support;
#[cfg(test)]
mod tests;
