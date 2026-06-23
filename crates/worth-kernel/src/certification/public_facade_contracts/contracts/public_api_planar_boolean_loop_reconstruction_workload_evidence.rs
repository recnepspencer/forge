#[path = "public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
mod workload_evidence_support;

use workload_evidence_support::{
    assert_boolean_chain_accepts_only_completed_receipts_and_query_proof,
    assert_boolean_chain_query_proof_does_not_rewrite_ledger_identities,
    assert_boolean_chain_residue_manifest_is_capped_and_non_authority,
    assert_large_admitted_boolean_chain_scales_with_declared_breadth,
    assert_loop_closeout_exposes_certified_runtime_registration_artifacts,
    assert_loop_ledger_rejects_manual_or_counterless_evidence,
    assert_loop_ledger_replay_branch_preserves_workload_requirement,
    assert_loop_ledger_satisfies_workload_requirement_and_runtime_registration,
    assert_loop_stage_requirement_maps_only_to_loop_ledger_receipts,
};

#[test]
fn worth_workload_requires_boolean_loop_reconstruction_receipt_for_7_5_consumption() {
    assert_loop_ledger_satisfies_workload_requirement_and_runtime_registration();
}

#[test]
fn boolean_loop_reconstruction_workload_requirement_survives_replay_branch_closeout() {
    assert_loop_ledger_replay_branch_preserves_workload_requirement();
}

#[test]
fn boolean_loop_reconstruction_evidence_rejects_manual_or_counterless_rows() {
    assert_loop_ledger_rejects_manual_or_counterless_evidence();
}

#[test]
fn boolean_loop_reconstruction_stage_requirement_maps_only_to_loop_ledger_receipts() {
    assert_loop_stage_requirement_maps_only_to_loop_ledger_receipts();
}

#[test]
fn boolean_loop_reconstruction_closeout_exposes_certified_runtime_registration_artifacts() {
    assert_loop_closeout_exposes_certified_runtime_registration_artifacts();
}

#[test]
fn boolean_chain_7_5_handoff_accepts_ledger_receipts_plus_query_proof() {
    assert_boolean_chain_accepts_only_completed_receipts_and_query_proof();
}

#[test]
fn boolean_chain_query_proof_does_not_rewrite_split_or_loop_ledger_identity() {
    assert_boolean_chain_query_proof_does_not_rewrite_ledger_identities();
}

#[test]
fn boolean_chain_remaining_prep_ceremony_is_capped_residue() {
    assert_boolean_chain_residue_manifest_is_capped_and_non_authority();
}

#[test]
fn large_admitted_boolean_prep_workload_scales_with_declared_breadth() {
    assert_large_admitted_boolean_chain_scales_with_declared_breadth();
}
