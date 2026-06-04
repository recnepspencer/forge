mod admission_boundary_json_projection;
mod authority_denial_json_projection;
mod duplicate_authority_json_projection;
mod family_extension_json_projection;
mod feedback_loop_json_projection;
mod mapper_parity_json_projection;
mod replay_loop_isolation_json_projection;
mod replay_mismatch_json_projection;

pub(in crate::harness::adapter::adapter_impl) use admission_boundary_json_projection::{
    admission_boundary_certification_evidence_json, admission_boundary_matrix_json,
};
pub(in crate::harness::adapter::adapter_impl) use authority_denial_json_projection::{
    authority_denial_certification_evidence_json, authority_denial_matrix_json,
    authority_denial_zero_residue_proof_json,
};
pub(in crate::harness::adapter::adapter_impl) use duplicate_authority_json_projection::{
    duplicate_authority_boundary_matrix_json, duplicate_authority_matrix_json,
    duplicate_boundedness_proof_json, duplicate_idempotence_report_json,
    duplicate_loop_prevention_report_json,
};
pub(in crate::harness::adapter::adapter_impl) use family_extension_json_projection::{
    family_extension_certification_evidence_json, family_extension_matrix_json,
};
pub(in crate::harness::adapter::adapter_impl) use feedback_loop_json_projection::{
    feedback_certification_evidence_json, feedback_loop_matrix_json,
};
pub(in crate::harness::adapter::adapter_impl) use mapper_parity_json_projection::{
    mapper_parity_certification_evidence_json, mapper_parity_matrix_json,
};
pub(in crate::harness::adapter::adapter_impl) use replay_loop_isolation_json_projection::{
    replay_loop_certification_evidence_json, replay_loop_isolation_matrix_json,
};
pub(in crate::harness::adapter::adapter_impl) use replay_mismatch_json_projection::{
    replay_mismatch_matrix_json, replay_mismatch_restart_replay_json,
};
