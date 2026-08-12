pub mod checkpoint_basis;
#[cfg(feature = "certification-world")]
pub mod checkpoint_durability;
#[cfg(feature = "certification-world")]
pub mod checkpoint_publication;
pub mod closeout;
#[cfg(feature = "certification-world")]
pub mod counter_evidence;
#[cfg(feature = "certification-world")]
pub mod coverage;
pub mod memory_budget;
pub mod redo_replay;
#[cfg(feature = "certification-world")]
#[path = "drivers/crash_harness.rs"]
mod s4_crash_harness;
#[path = "drivers/fault_scheduler.rs"]
mod s4_fault_scheduler;
#[path = "fixtures/recovery_entry.rs"]
mod s4_recovery_entry_fixture;
#[path = "fixtures/recovery_handoff.rs"]
mod s4_recovery_handoff_fixture;
#[path = "fixtures/recovery_integrity.rs"]
mod s4_recovery_integrity_fixture;
#[path = "fixtures/recovery_physical.rs"]
mod s4_recovery_physical_fixture;
#[path = "fixtures/recovery_readiness.rs"]
mod s4_recovery_readiness_fixture;
#[path = "drivers/storage_interposer.rs"]
mod s4_storage_interposer;
#[cfg(any(
    feature = "certification-world",
    feature = "physical-compaction-fixtures",
    feature = "physical-isolation-fixtures"
))]
pub mod source_precedence;
#[cfg(feature = "certification-world")]
pub mod wal_durability;
pub mod wal_tail;

#[cfg(feature = "certification-world")]
pub use s4_crash_harness::{ExecutedS4CrashHarnessDenial, ExecutedS4CrashHarnessTranscript};
pub use s4_fault_scheduler::{FaultSchedulerDriver, ScheduledFault};
pub use s4_recovery_entry_fixture::{
    with_admitted_recovery_entry, with_admitted_recovery_partial_publication_entry,
};
pub use s4_storage_interposer::{StorageBoundaryEvent, StorageBoundaryInterposerDriver};
