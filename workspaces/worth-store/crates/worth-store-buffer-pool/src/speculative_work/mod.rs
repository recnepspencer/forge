mod speculative_work_admission;
mod speculative_work_budget;
mod speculative_work_counters;
mod speculative_work_denials;
mod speculative_work_plan;
mod speculative_work_request;

#[cfg(test)]
mod speculative_work_honesty_tests;
#[cfg(test)]
mod speculative_work_tests;

pub use speculative_work_admission::SpeculativePhysicalWorkAdmission;
pub use speculative_work_budget::SpeculativeWorkBudgetSnapshot;
pub use speculative_work_counters::SpeculativeWorkCounterSnapshot;
pub use speculative_work_denials::{
    SpeculativePhysicalWorkDenial, SpeculativePhysicalWorkDenialKind, SpeculativeResidencyDenial,
};
pub use speculative_work_plan::{
    PrefetchAdmission, PrefetchPlan, ReadAheadAdmission, ReadAheadPlan,
    SpeculativeWorkReplayIdentity, WriteBehindAdmission, WriteBehindPlan,
};
pub use speculative_work_request::{
    PrefetchRequest, PrefetchWindow, ReadAheadRequest, SpeculativeWorkRequestDenial,
    WriteBehindRequest,
};
