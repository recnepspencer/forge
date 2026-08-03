#[path = "schedule/decision.rs"]
mod decision;
#[path = "schedule/execution.rs"]
mod execution;

pub(crate) use decision::BoundedResidencySchedulePlan;
pub(in crate::bounded_residency) use decision::{
    EquivalentContenderIdentity, GateReleaseOrder, IndependentReadyWorkSelection, WorkerStartOrder,
};
pub(in crate::bounded_residency) use execution::{
    BoundedResidencyExecutedSchedule, ExecutedDuplicateFaultSchedule, ExecutedPrefetchSchedule,
};
