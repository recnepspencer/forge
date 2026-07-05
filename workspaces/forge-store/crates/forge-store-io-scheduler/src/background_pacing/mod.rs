#![doc = include_str!("compile_fail_proofs.md")]

mod admission;
mod basis;
mod budget;
mod capacity;
mod class;
mod counters;
mod debt;
mod denial;
mod lease;
mod outcome;
mod proof;
mod request;
mod shape;

pub use admission::admit_background_pacing;
pub use basis::BackgroundPacingAdmissionBasis;
pub use budget::{BackgroundResourceBudget, BackgroundResourceShortfall};
pub use capacity::{
    admit_background_capacity, BackgroundCapacityAdmission, BackgroundCapacityAdmissionRequest,
};
pub use class::BackgroundIoPressureClass;
pub use counters::BackgroundPacingCounterSnapshot;
pub use debt::{BackgroundDebtKind, BackgroundIoDebt};
pub use denial::BackgroundPacingDenial;
pub use lease::{BackgroundIdleCapacityLease, BackgroundLeaseRevocation};
pub use outcome::{
    BackgroundPacingAdmittedWithDebt, BackgroundPacingDeferred, BackgroundPacingDenied,
    BackgroundPacingOutcome, BackgroundPacingStaleRebindKind, BackgroundPacingStaleRebindRequired,
    BackgroundPacingThrottle, BackgroundPacingViolation, BackgroundPacingYield,
};
pub use proof::{
    BackgroundPacingFreshness, BackgroundPacingProgressionDrift,
    BackgroundPacingProgressionEvidence,
};
pub use request::{
    reject_elapsed_time_as_background_pacing_authority,
    reject_log_line_as_background_pacing_authority,
    reject_raw_background_label_as_background_pacing_authority,
    reject_semantic_lifecycle_receipt_as_background_pacing_authority,
    reject_worker_local_queue_as_background_pacing_authority, BackgroundIdleCapacityLeaseRequest,
};
pub use shape::BackgroundIoPressureShape;

#[cfg(test)]
mod tests;
