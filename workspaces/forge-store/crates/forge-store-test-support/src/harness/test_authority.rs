//! Synthetic courtroom-only witnesses and shortcut attempts.
//!
//! These helpers exist to falsify production topology. They are not production
//! admission lanes and must not satisfy production capability APIs directly.

pub use super::milestone::s7_blob_harness_execution::{
    execute_s7_blob_harness_scenario, synthetic_s7_blob_harness_coverage_matrix,
    synthetic_s7_blob_harness_replay_bundle,
};
pub use super::physical_reference::{HarnessPhysicalReference, harness_physical_reference};
pub use super::physical_simulation::{
    ambiguous_locus_fault_attempt_fixture, arbitrary_byte_scribble_fault_attempt_fixture,
    crash_recovery_fault_locus, fake_in_memory_only_driver_attempt, io_pressure_fault_locus,
    observed_checksum_mismatch_boundary, observed_io_pressure_boundary,
    observed_torn_frame_boundary, page_generation_fault_locus,
    post_decode_corruption_fault_attempt_fixture, private_mutation_driver_attempt_fixture,
    private_mutation_fault_attempt_fixture, same_process_crash_fault_attempt_fixture,
    sleep_based_scheduling_driver_attempt, test_support_verdict_driver_attempt_fixture,
    wal_frame_payload_fault_locus,
};

pub mod s8_layout_access {
    pub use super::super::milestone::s8_layout_access::{
        S8LayoutAdversarialInputs, s8_layout_adversarial_inputs,
    };
}
