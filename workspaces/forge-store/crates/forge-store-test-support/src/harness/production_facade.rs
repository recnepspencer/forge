//! Harness helpers that must exercise production-owned capabilities through
//! public lifecycle facades.

pub use super::fixtures::{
    AllocationSentinel, AspectDerivedSegmentReference, LargeRecordStreamPressure,
    LargeStorePressureClass, LargeStorePressureFixture, MemoryPressureDriverInput,
    NativeAspectPhysicalReferenceDenial, NativeStoreAspectFixture,
    StoreHostileReadmissionJsonFixture, StoreHostileReadmissionJsonFixtureBoundaryOutcome,
    StoreHostileReadmissionJsonFixtureBoundaryWitness, StoreJsonFixtureBoundaryDenial,
    StoreTerminalProjectionJsonFixture, StoreTerminalProjectionJsonFixtureBoundaryOutcome,
    StoreTerminalProjectionJsonFixtureBoundaryWitness, require_native_store_aspect_fixture,
};
pub use super::milestone::{
    ExecutedS4CrashHarnessDenial, ExecutedS4CrashHarnessTranscript, FaultSchedulerDriver,
    FreshRuntimeRecoveryDriver, RecoveryRuntimePosture, S6InterferenceTestProfile,
    S6IoPressureTestProfile, S51SecurityScopeHarnessExecution,
    S51SecurityScopeHarnessReplayExecution, ScheduledFault, StorageBoundaryEvent,
    StorageBoundaryInterposerDriver, admitted_s4_partial_publication_recovery_entry,
    admitted_s4_recovery_entry, ci_memory_envelope_s7_blob_harness_seed,
    deterministic_s4_fresh_runtime_driver, deterministic_s4_recovery_artifacts,
    deterministic_s6_interference_profile, deterministic_s6_io_pressure_profile,
    duplicate_role_s4_recovery_artifacts,
    execute_s5_1_security_scope_harness_replay_with_physical_replay,
    execute_s5_1_security_scope_harness_scenario,
    execute_s7_blob_harness_real_multi_gb_temp_file_fixture,
    execute_s7_blob_harness_temp_file_fixture_smoke, heavy_multi_gb_s7_blob_harness_seed,
    incomplete_s4_recovery_artifacts, large_s6_io_pressure_profile, local_s7_blob_harness_seed,
    malformed_s4_recovery_record, reordered_s4_recovery_artifacts,
    runtime_disagreement_s4_recovery_artifacts, runtime_state_mismatch_s4_recovery_artifacts,
    s4_recovery_artifacts_with_operation_digest, s5_1_security_scope_drift_scenario,
    s5_1_security_scope_metadata_preservation_scenarios,
    s5_1_security_scope_metadata_preserved_scenario,
    s5_1_security_scope_missing_authenticity_scenario,
    s5_1_security_scope_replayed_custody_scenario, s5_1_security_scope_stale_key_scenario,
    physical_isolation_boundary_fact, physical_isolation_boundary_yieldpoint,
    s5_1_security_scope_wrong_tenant_scenario,
};
pub use super::physical_simulation::{
    admitted_ci_certification_driver_contracts, admitted_developer_smoke_driver_contracts,
    ci_certification_replay_seed, ci_certification_state_space_budget,
    deterministic_ci_certification_schedule, deterministic_developer_smoke_schedule,
    developer_smoke_replay_seed, developer_smoke_state_space_budget,
    production_backed_physical_fixture_materialization, unbound_production_driver,
};
