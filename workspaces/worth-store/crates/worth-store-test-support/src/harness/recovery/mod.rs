pub mod checkpoint_basis;
#[cfg(feature = "certification-world")]
pub mod checkpoint_durability;
#[cfg(feature = "certification-world")]
pub mod checkpoint_publication;
pub mod closeout;
#[cfg(feature = "certification-world")]
pub mod compaction_mutation;
#[cfg(feature = "certification-world")]
pub mod compaction_observation;
#[cfg(feature = "certification-world")]
pub mod counter_evidence;
#[cfg(feature = "certification-world")]
pub mod coverage;
#[cfg(feature = "certification-world")]
pub mod dirty_publication;
pub mod memory_budget;
pub mod redo_replay;
mod reopened_artifact;
#[cfg(feature = "certification-world")]
#[path = "drivers/crash_harness.rs"]
mod s4_crash_harness;
#[path = "drivers/fault_scheduler.rs"]
mod s4_fault_scheduler;
#[path = "drivers/fresh_runtime.rs"]
mod s4_fresh_runtime;
#[path = "fixtures/persisted_recovery.rs"]
mod s4_persisted_recovery;
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
    feature = "physical-compaction-fixtures"
))]
pub mod source_precedence;
#[cfg(feature = "certification-world")]
pub mod wal_durability;
pub mod wal_tail;

pub use reopened_artifact::reopened_recovery_artifact_fixture;
#[cfg(feature = "certification-world")]
pub use s4_crash_harness::{ExecutedS4CrashHarnessDenial, ExecutedS4CrashHarnessTranscript};
pub use s4_fault_scheduler::{FaultSchedulerDriver, ScheduledFault};
pub use s4_fresh_runtime::{
    deterministic_recovery_fresh_runtime_driver, FreshRuntimeRecoveryDriver, RecoveryRuntimePosture,
};
pub use s4_persisted_recovery::{
    deterministic_recovery_artifacts, duplicate_role_recovery_artifacts,
    incomplete_recovery_artifacts, malformed_recovery_record,
    recovery_artifacts_with_operation_digest, reordered_recovery_artifacts,
    runtime_disagreement_recovery_artifacts, runtime_state_mismatch_recovery_artifacts,
};
pub use s4_recovery_entry_fixture::{
    admitted_recovery_entry, admitted_recovery_partial_publication_recovery_entry,
};
pub use s4_storage_interposer::{StorageBoundaryEvent, StorageBoundaryInterposerDriver};
