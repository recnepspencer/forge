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

pub use s4_crash_harness::{ExecutedS4CrashHarnessDenial, ExecutedS4CrashHarnessTranscript};
pub use s4_fault_scheduler::{FaultSchedulerDriver, ScheduledFault};
pub use s4_fresh_runtime::{
    deterministic_recovery_fresh_runtime_driver, FreshRuntimeRecoveryDriver, RecoveryRuntimePosture,
};
pub use s4_persisted_recovery::{
    deterministic_recovery_artifacts, duplicate_role_recovery_artifacts,
    incomplete_recovery_artifacts, malformed_recovery_record,
    reordered_recovery_artifacts, runtime_disagreement_recovery_artifacts,
    runtime_state_mismatch_recovery_artifacts, recovery_artifacts_with_operation_digest,
};
pub use s4_recovery_entry_fixture::{
    admitted_recovery_partial_publication_recovery_entry, admitted_recovery_entry,
};
pub use s4_storage_interposer::{StorageBoundaryEvent, StorageBoundaryInterposerDriver};
