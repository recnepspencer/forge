#[path = "public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
mod loop_workload_evidence_support;
#[path = "public_api_spatial_touch_receipt_admission_support.rs"]
mod spatial_touch_receipt_admission_support;

use super::edge_splitting_public_contract_support;
use super::edge_splitting_replay_parity_support;
use super::metaboss_support;
use super::reduced_pair_support;

#[test]
fn phase4_spatial_touch_segment_pair_handoff_admits_real_completed_workload_ledger() {
    reduced_pair_support::run_with_large_stack(|| {
        spatial_touch_receipt_admission_support::
            assert_segment_pair_receipt_admits_from_completed_workload_handoff();
    });
}

#[test]
fn phase4_spatial_touch_event_ledger_handoff_admits_real_completed_workload_ledger() {
    reduced_pair_support::run_with_large_stack(|| {
        spatial_touch_receipt_admission_support::
            assert_event_ledger_receipt_admits_from_completed_workload_handoff();
    });
}

#[test]
fn phase4_spatial_touch_split_handoff_admits_real_completed_workload_ledger() {
    reduced_pair_support::run_with_large_stack(|| {
        spatial_touch_receipt_admission_support::
            assert_split_receipt_admits_from_completed_workload_handoff();
    });
}

#[test]
fn phase4_spatial_touch_loop_handoff_admits_real_completed_workload_ledger() {
    reduced_pair_support::run_with_large_stack(|| {
        spatial_touch_receipt_admission_support::
            assert_loop_receipt_admits_from_completed_workload_handoff();
    });
}

#[test]
fn phase4_spatial_touch_split_replay_equivalence_uses_completed_workload_ledgers() {
    reduced_pair_support::run_with_large_stack(|| {
        spatial_touch_receipt_admission_support::
            assert_split_replay_preserves_completed_workload_spatial_touch_authority();
    });
}

#[test]
fn phase4_spatial_touch_loop_replay_equivalence_uses_completed_workload_ledgers() {
    reduced_pair_support::run_with_large_stack(|| {
        spatial_touch_receipt_admission_support::
            assert_loop_replay_preserves_completed_workload_spatial_touch_authority();
    });
}

#[test]
fn phase6_spatial_touch_migrated_split_consumer_uses_facade_lookup_authority() {
    reduced_pair_support::run_with_large_stack(|| {
        edge_splitting_public_contract_support::
            assert_split_downstream_migration_uses_spatial_facade_proof_product();
    });
}

#[test]
fn phase10_cross_crate_replay_preserves_kernel_spatial_query_handoff() {
    reduced_pair_support::run_with_large_stack(|| {
        spatial_touch_receipt_admission_support::
            assert_split_replay_preserves_cross_crate_spatial_query_handoff();
    });
}
