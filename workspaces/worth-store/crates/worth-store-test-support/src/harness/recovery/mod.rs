pub mod closeout;
#[cfg(feature = "certification-world")]
pub mod counter_evidence;
#[cfg(feature = "certification-world")]
pub mod coverage;
pub mod memory_budget;
#[cfg(feature = "certification-world")]
mod physical_source;
#[path = "drivers/fault_scheduler.rs"]
mod s4_fault_scheduler;
#[path = "fixtures/recovery_handoff.rs"]
mod s4_recovery_handoff_fixture;
#[path = "fixtures/recovery_integrity.rs"]
mod s4_recovery_integrity_fixture;
#[path = "fixtures/recovery_integrity_input.rs"]
mod s4_recovery_integrity_input_fixture;
#[path = "fixtures/recovery_physical.rs"]
mod s4_recovery_physical_fixture;
#[path = "fixtures/recovery_readiness.rs"]
mod s4_recovery_readiness_fixture;
#[path = "drivers/storage_interposer.rs"]
mod s4_storage_interposer;
#[cfg(feature = "certification-world")]
pub mod wal_durability;
pub mod wal_tail;

#[cfg(feature = "certification-world")]
pub use physical_source::{
    deterministic_checkpoint_plus_tail_source, deterministic_selected_compaction_product,
};
pub use s4_fault_scheduler::{FaultSchedulerDriver, ScheduledFault};
pub use s4_recovery_integrity_input_fixture::with_admitted_recovery_integrity_input;
pub use s4_storage_interposer::{StorageBoundaryEvent, StorageBoundaryInterposerDriver};
