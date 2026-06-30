#[path = "public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;
#[path = "public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
mod workload_evidence_support;

use workload_evidence_support::{
    assert_boolean_chain_accepts_only_completed_receipts_and_query_proof,
    assert_boolean_chain_query_proof_does_not_rewrite_ledger_identities,
    assert_boolean_chain_residue_manifest_is_capped_and_non_authority,
    assert_large_admitted_boolean_chain_scales_with_declared_breadth,
    assert_legacy_loop_closeout_cannot_claim_packet_backed_boundary,
    assert_loop_closeout_exposes_certified_runtime_registration_artifacts,
    assert_loop_ledger_rejects_manual_or_counterless_evidence,
    assert_loop_ledger_replay_branch_preserves_workload_requirement,
    assert_loop_ledger_satisfies_workload_requirement_and_runtime_registration,
    assert_loop_stage_requirement_maps_only_to_loop_ledger_receipts,
    assert_packet_backed_loop_closeout_matches_legacy_vertical_slice,
    assert_packet_backed_loop_closeout_rejects_foreign_scope_products,
    assert_public_closeout_rejects_mismatched_proof_products,
    assert_replay_undo_consumer_cutover_closes_from_ordinary_chain,
    assert_topology_undo_product_changes_packet_identity,
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
fn packet_backed_loop_closeout_matches_legacy_vertical_slice() {
    assert_packet_backed_loop_closeout_matches_legacy_vertical_slice();
}

#[test]
fn packet_backed_loop_closeout_rejects_foreign_scope_products() {
    assert_packet_backed_loop_closeout_rejects_foreign_scope_products();
}

#[test]
fn packet_identity_changes_when_topology_undo_product_changes() {
    assert_topology_undo_product_changes_packet_identity();
}

#[test]
fn replay_undo_consumer_cutover_closes_from_ordinary_chain() {
    reduced_pair_support::run_with_large_stack(|| {
        assert_replay_undo_consumer_cutover_closes_from_ordinary_chain();
    });
}

#[test]
fn public_closeout_rejects_mismatched_proof_products() {
    reduced_pair_support::run_with_large_stack(|| {
        assert_public_closeout_rejects_mismatched_proof_products();
    });
}

#[test]
fn legacy_loop_closeout_cannot_claim_packet_backed_boundary() {
    assert_legacy_loop_closeout_cannot_claim_packet_backed_boundary();
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
