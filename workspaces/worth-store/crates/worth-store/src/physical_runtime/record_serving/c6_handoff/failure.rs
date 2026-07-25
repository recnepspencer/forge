use crate::physical_runtime::{PhysicalSchedulerDenial, PhysicalWorkPreEffectDenial};

/// Failure to advance work through the sealed C.6 physical-work bridge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum C6PhysicalWorkHandoffFailure {
    RuntimeReleased,
    StaleOrForeignIdentity,
    CanonicalWritebackMismatch,
    SchedulerReservation(
        worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundAdmissionDenial,
    ),
    Residency(worth_store_buffer_pool::PhysicalResidencyDenial),
    Scheduler(PhysicalSchedulerDenial),
    SecureIo(worth_store_io_scheduler::SecureIoPreservationDenial),
    WritebackAdmission(super::super::PhysicalScheduledWritebackAdmissionDenial),
    PreEffect(PhysicalWorkPreEffectDenial),
}
