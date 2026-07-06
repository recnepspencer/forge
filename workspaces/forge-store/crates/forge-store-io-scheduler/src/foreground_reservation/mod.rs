#![doc = include_str!("compile_fail_proofs.md")]

mod admission;
mod capacity;
mod capacity_admission;
mod counters;
mod denial;
mod envelope;
mod fairness;
mod lane;
mod proof;
mod receipt;
mod request;
mod resource_contract;
mod resource_units;
#[cfg(any(test, feature = "certification-test-authority"))]
mod test_authority;
mod violation;

pub use admission::admit_foreground_reservation;
pub use capacity_admission::{
    admit_foreground_reservation_capacity, ForegroundReservationCapacityAdmission,
    ForegroundReservationCapacityAdmissionDenial, ForegroundReservationCapacityAdmissionRequest,
    ForegroundReservationCapacityAuthority,
};
pub use counters::ForegroundReservationCounterSnapshot;
pub use denial::{ForegroundReservationAdmissionDenial, ForegroundReservationResourceShortfall};
pub use envelope::{ForegroundLatencyEnvelope, ForegroundLatencyEnvelopeKind};
pub use fairness::{
    ForegroundArbitrationDeclaration, ForegroundArbitrationPolicy, ForegroundFairnessClass,
    ForegroundFairnessDenial,
};
pub use lane::{ForegroundIoLaneKind, ForegroundLaneDeclaration};
pub use receipt::{
    ForegroundReservationAdmissionOutcome, ForegroundReservationDenied, ForegroundReservationHeld,
    ForegroundReservationReceipt, ForegroundReservationStaleRebindRequired,
    ForegroundReservationState,
};
pub use request::{
    reject_copied_s5_counters_as_foreground_reservation,
    reject_copied_security_scope_fields_as_foreground_reservation,
    reject_raw_lane_label_as_foreground_reservation,
    reject_semantic_priority_as_foreground_reservation,
    reject_terminal_projection_as_foreground_reservation, ForegroundReservationAdmissionRequest,
};
pub use resource_units::{
    BandwidthToken, CacheResidencyHint, DirtyPageBudget, FlushPermit, ForegroundResourceBudget,
    ForegroundResourceUnitDenial, ForegroundResourceUnitKind, QueueSlot, ReadAheadWindow,
    ReclaimPermit, SyncDebt, WorkerPermit, WriteBackWindow,
};
pub use violation::{ForegroundReservationViolationCause, ReservationViolatedWithCause};

#[cfg(any(test, feature = "certification-test-authority"))]
pub use test_authority::{
    admitted_page_write_reservation_for_certification_test,
    admitted_point_read_reservation_for_certification_test,
    admitted_point_read_reservation_for_security_scope_for_certification_test,
    admitted_range_read_reservation_for_certification_test,
    admitted_secure_frame_read_reservation_for_certification_test,
    admitted_wal_write_reservation_for_certification_test,
};

#[cfg(test)]
mod tests;
