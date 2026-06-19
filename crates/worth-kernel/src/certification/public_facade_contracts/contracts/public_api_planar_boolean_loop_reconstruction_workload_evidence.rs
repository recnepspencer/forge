#[path = "public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
mod workload_evidence_support;

use workload_evidence_support::{
    assert_loop_closeout_rejects_malformed_runtime_registration_artifacts,
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
fn boolean_loop_reconstruction_closeout_rejects_malformed_runtime_registration_artifacts() {
    assert_loop_closeout_rejects_malformed_runtime_registration_artifacts();
}
