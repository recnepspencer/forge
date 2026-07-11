// Internal certification modules.

mod allocation_envelope_evidence;
#[cfg(test)]
mod allocation_envelope_evidence_tests;
#[cfg(test)]
mod aspect_native_authority_denial_tests;
#[cfg(test)]
mod aspect_native_diagnostic_evidence_tests;
#[cfg(test)]
mod aspect_native_harness_authoring_tests;
#[cfg(test)]
mod aspect_native_identity_tests;
#[cfg(test)]
mod aspect_native_performance_evidence_tests;
#[cfg(test)]
mod aspect_native_vocabulary_tests;
#[cfg(test)]
mod authority_projection_readmission_tests;
mod background_envelope_evidence;
#[cfg(test)]
mod background_envelope_evidence_tests;
mod binary_format_evidence;
mod bounded_memory_closeout;
#[cfg(test)]
mod bounded_memory_closeout_pressure_support;
#[cfg(test)]
mod bounded_memory_closeout_tests;
mod bounded_memory_harness_closeout;
mod bounded_memory_residency_suite;
mod buffer_pool_certification_bundle;
mod buffer_pool_scenario_definitions;
mod buffer_pool_scenario_plans;
mod buffer_pool_story_lanes;
mod buffer_pool_transcripts;
#[cfg(test)]
mod canonical_basis_entry_construction_tests;
#[cfg(test)]
mod canonical_basis_entry_denial_tests;
#[cfg(test)]
mod canonical_basis_entry_order_tests;
mod canonical_basis_source_inventory;
mod canonical_basis_source_registry;
mod canonical_basis_source_scan;
#[cfg(test)]
mod canonical_basis_source_tests;
mod certification_matrix;
#[cfg(test)]
mod checksum_declaration_tests;
#[cfg(test)]
mod chunk_integrity_without_blob_lifecycle_tests;
#[cfg(test)]
mod cross_family_wrong_scope_tests;
#[cfg(test)]
mod derived_index_damage_tests;
#[cfg(test)]
mod digest_authority_denial_tests;
#[cfg(test)]
mod digest_authority_equivalence_tests;
mod dirty_publication_evidence;
#[cfg(test)]
mod dirty_publication_evidence_tests;
mod drivers;
mod eviction_protection_evidence;
#[cfg(test)]
mod eviction_protection_evidence_tests;
mod extent_record_framing_evidence;
#[cfg(test)]
mod extent_record_framing_evidence_tests;
mod foundational_boundary_evidence;
#[cfg(test)]
mod foundational_boundary_evidence_tests;
mod foundational_boundary_performance;
#[cfg(test)]
mod foundational_integrity_evidence_tests;
mod harness;
#[cfg(test)]
mod harness_tests;
mod header_decode_evidence;
#[cfg(test)]
mod hostile_readmission_json_fixture_boundary_tests;
#[cfg(test)]
mod json_fixture_boundary_tests;
mod lanes;
mod large_store_pressure_evidence;
#[cfg(test)]
mod large_store_pressure_tests;
mod layout_observers;
mod manifest_discovery_evidence;
#[cfg(test)]
mod manifest_discovery_evidence_tests;
mod observed_trace;
mod observer_trace;
mod observers;
mod offline_verifier_evidence;
#[cfg(test)]
mod offline_verifier_evidence_tests;
mod oracles;
mod page_record_framing_evidence;
#[cfg(test)]
mod page_record_framing_evidence_tests;
mod physical_complexity_evidence;
#[cfg(test)]
mod physical_complexity_evidence_tests;
#[cfg(test)]
mod physical_container_integrity_hardening_tests;
#[cfg(test)]
mod physical_container_integrity_tests;
mod physical_foundation_evidence;
mod physical_identity_evidence;
mod physical_integrity_closeout_bundle;
mod physical_integrity_closeout_denial;

mod physical_integrity_closeout_handoff;
mod physical_integrity_closeout_harness;
mod physical_integrity_closeout_harness_execution;
mod physical_integrity_closeout_harness_runner;
mod physical_integrity_closeout_line_cap;
#[cfg(test)]
mod physical_integrity_closeout_line_cap_tests;
mod physical_integrity_closeout_owned_file;
mod physical_integrity_closeout_proof;
mod physical_integrity_closeout_report;
mod physical_integrity_closeout_suite;
mod physical_integrity_closeout_suite_kind;
#[cfg(test)]
mod physical_integrity_closeout_tests;
#[cfg(test)]
mod physical_integrity_entry_authority_tests;
#[cfg(test)]
mod physical_scope_admission_tests;
mod physical_substrate_certification_authority;
mod physical_substrate_certification_denial;
mod physical_substrate_certification_reports;
mod physical_substrate_certification_scan;
mod physical_substrate_closeout;
mod physical_substrate_closeout_story;
#[cfg(test)]
mod physical_substrate_closeout_tests;
mod physical_substrate_complexity_suite;
mod physical_substrate_foundation_suite;
mod physical_substrate_manifest_suite;
mod physical_substrate_story_suite;
mod pin_lifecycle_evidence;
#[cfg(test)]
mod pin_lifecycle_evidence_tests;
mod platform_facade_evidence;
#[cfg(test)]
mod platform_facade_evidence_tests;
#[cfg(test)]
mod pre_decode_physical_admission_tests;
mod protected_integrity_view_evidence;
#[cfg(test)]
mod public_facade_dependency_tests;
#[cfg(test)]
mod quarantine_sealing_tests;
mod record_view_evidence;
#[cfg(test)]
mod record_view_evidence_admission_tests;
#[cfg(test)]
mod record_view_evidence_conflict_tests;
mod resident_frame_authority_evidence;
#[cfg(test)]
mod resident_frame_authority_evidence_tests;
mod runtime_verifier_comparison;
#[cfg(test)]
mod runtime_verifier_comparison_tests;
mod runtime_verifier_diagnostics;
mod runtime_verifier_support;
#[cfg(test)]
mod s0_handoff_contract_tests;
mod s0_handoff_gate_evidence;
mod s2_acceptance_suite_transcript;
mod s2_entry_boundary_evidence;
mod s3_readiness_handoff;
#[cfg(test)]
mod s4_integrity_damage_map_tests;
#[cfg(test)]
mod s4_integrity_handoff_tests;
#[cfg(test)]
mod s4_quarantine_receipt_binding_tests;
#[cfg(test)]
mod s4_recovery_entry_admission_tests;
mod recovery_harness;
#[cfg(test)]
#[path = "s5_1_authenticity_integrity_separation_tests/mod.rs"]
mod s5_1_authenticity_integrity_separation_tests;
#[cfg(test)]
mod s5_1_blob_chunk_scope_dedupe_tests;
mod s5_1_closeout;
#[cfg(test)]
mod s5_1_recovery_scope_propagation_tests;
mod s5_evidence_materialization;
mod s5_physical_isolation_closeout;
mod s5_physical_isolation_harness;
#[path = "s6.rs"]
mod s6;
mod s6_access_policy;
#[path = "s6_evidence_materialization/mod.rs"]
mod s6_evidence_materialization;
mod s6_flush_durability;
mod s6_io_pressure_harness_closeout;
mod s6_latency_interference;
mod s6_queue_execution;
pub use s6_access_policy::{S6AccessPolicyEvidenceOutcomeKind, S6AccessPolicyEvidenceRow};
pub use s6_flush_durability::S6FlushDurabilityEvidenceRow;
pub use s6_queue_execution::{
    S6CertifiedQueueExecutionEvidence, S6QueueExecutionCertificationDenial,
};

mod scale_fixture;
mod scale_property;
mod scenario_definition;
mod scenario_execution;
mod scenario_plan;
mod scenario_plan_rules;
mod scenario_planned_work_evidence;
#[cfg(test)]
mod scrub_execution_tests;
mod speculative_work_evidence;
#[cfg(test)]
mod speculative_work_evidence_tests;
mod store_certification_program;
mod store_json_residue_certification;
mod store_json_residue_denial;
mod store_json_residue_entry;
mod store_json_residue_exports;
mod store_json_residue_inventory;
mod store_json_residue_prelude_scan;
mod store_json_residue_scan;
#[cfg(test)]
mod store_json_residue_tests;
mod story_transcript;
mod synthetic_closeout_exports;
mod synthetic_closeout_rejection;
#[cfg(test)]
mod terminal_projection_json_fixture_boundary_tests;
#[cfg(test)]
mod wal_frame_integrity_tests;
