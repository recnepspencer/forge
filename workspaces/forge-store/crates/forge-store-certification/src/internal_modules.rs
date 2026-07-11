// Internal certification modules.

mod recovery_harness;
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
