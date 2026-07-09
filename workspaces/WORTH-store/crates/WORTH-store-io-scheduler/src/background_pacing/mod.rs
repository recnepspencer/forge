#![doc = include_str!("compile_fail_proofs.md")]

mod admission;
mod basis;
mod budget;
mod capability;
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
mod streaming_pressure_link;
#[cfg(any(test, feature = "certification-test-authority"))]
mod test_authority;

pub use admission::admit_background_pacing;
pub use basis::BackgroundPacingAdmissionBasis;
pub use budget::{BackgroundResourceBudget, BackgroundResourceShortfall};
pub use capability::BackgroundPacingCapability;
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
pub use streaming_pressure_link::{
    admits_blob_ingest_pressure, admits_verification_pressure,
    ingest_pressure_foreground_lane_admits,
};
#[cfg(any(test, feature = "certification-test-authority"))]
pub use test_authority::{
    blob_ingest_background_capacity_for_certification_test,
    blob_ingest_deferred_background_capacity_for_certification_test,
    blob_ingest_denied_background_capacity_for_certification_test,
    blob_ingest_page_write_background_capacity_for_certification_test,
    blob_ingest_rebind_background_capacity_for_certification_test,
    blob_ingest_stale_background_capacity_for_certification_test,
    blob_ingest_throttled_background_capacity_for_certification_test,
    blob_ingest_wal_write_background_capacity_for_certification_test,
    checkpoint_flush_wal_background_capacity_for_certification_test,
    verification_deferred_background_capacity_for_certification_test,
    verification_denied_background_capacity_for_certification_test,
    verification_rebind_background_capacity_for_certification_test,
    verification_stale_background_capacity_for_certification_test,
    verification_throttled_background_capacity_for_certification_test,
    verification_zero_admitted_throttle_background_capacity_for_certification_test,
};

#[cfg(test)]
mod tests;
