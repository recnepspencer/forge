//! Milestone-scoped harness fixtures grouped by roadmap phase.

pub mod s4_recovery_physics;
pub mod s5_1_security_scope_harness;
pub mod s5_physical_isolation;
pub mod s6_interference_profiles;
pub mod s6_io_pressure_profiles;
pub mod s7_blob_harness_execution;
pub mod s7_blob_harness_heavy_fixture;
pub mod s7_blob_harness_profiles;
pub mod s8_layout_access;

pub use s4_recovery_physics::{
    ExecutedS4CrashHarnessDenial, ExecutedS4CrashHarnessTranscript, FaultSchedulerDriver,
    FreshRuntimeRecoveryDriver, RecoveryRuntimePosture, ScheduledFault, StorageBoundaryEvent,
    StorageBoundaryInterposerDriver, admitted_s4_partial_publication_recovery_entry,
    admitted_s4_recovery_entry, deterministic_s4_fresh_runtime_driver,
    deterministic_s4_recovery_artifacts, duplicate_role_s4_recovery_artifacts,
    incomplete_s4_recovery_artifacts, malformed_s4_recovery_record,
    reordered_s4_recovery_artifacts, runtime_disagreement_s4_recovery_artifacts,
    runtime_state_mismatch_s4_recovery_artifacts, s4_recovery_artifacts_with_operation_digest,
};
pub use s5_1_security_scope_harness::{
    S51SecurityScopeHarnessExecution, S51SecurityScopeHarnessReplayExecution,
    execute_s5_1_security_scope_harness_replay_with_physical_replay,
    execute_s5_1_security_scope_harness_scenario, s5_1_security_scope_drift_scenario,
    s5_1_security_scope_metadata_preservation_scenarios,
    s5_1_security_scope_metadata_preserved_scenario,
    s5_1_security_scope_missing_authenticity_scenario,
    s5_1_security_scope_replayed_custody_scenario, s5_1_security_scope_stale_key_scenario,
    s5_1_security_scope_wrong_tenant_scenario,
};
pub use s5_physical_isolation::{s5_boundary_fact, s5_boundary_yieldpoint};
pub use s6_interference_profiles::{
    S6InterferenceTestProfile, deterministic_s6_interference_profile,
};
pub use s6_io_pressure_profiles::{
    S6IoPressureTestProfile, deterministic_s6_io_pressure_profile, large_s6_io_pressure_profile,
};
pub use s7_blob_harness_execution::execute_s7_blob_harness_scenario;
pub use s7_blob_harness_heavy_fixture::{
    execute_s7_blob_harness_real_multi_gb_temp_file_fixture,
    execute_s7_blob_harness_temp_file_fixture_smoke,
};
pub use s7_blob_harness_profiles::{
    ci_memory_envelope_s7_blob_harness_seed, heavy_multi_gb_s7_blob_harness_seed,
    local_s7_blob_harness_seed,
};
