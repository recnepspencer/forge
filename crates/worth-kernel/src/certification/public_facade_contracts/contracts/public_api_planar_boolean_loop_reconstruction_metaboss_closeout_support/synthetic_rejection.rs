use super::guard_coverage::assert_loop_reconstruction_guard_coverage_contract;
use super::public_contract_support::support::{
    assert_loop_public_contract_fences_reject_foreign_authority,
    assert_loop_public_contract_surfaces_preserve_real_workload_backed_identities,
};
use super::workload_evidence_support::{
    assert_loop_ledger_rejects_manual_or_counterless_evidence,
    assert_loop_replay_closeout_rejects_foreign_loop_authority,
    assert_loop_replay_closeout_rejects_foreign_retained_replay_authority,
};

pub(crate) fn assert_loop_reconstruction_metaboss_rejects_synthetic_loop_ledgers_raw_fragments_and_hand_filled_evidence(
) {
    assert_loop_reconstruction_guard_coverage_contract();
    assert_loop_ledger_rejects_manual_or_counterless_evidence();
    assert_loop_public_contract_surfaces_preserve_real_workload_backed_identities();
    assert_loop_public_contract_fences_reject_foreign_authority();
    assert_loop_replay_closeout_rejects_foreign_loop_authority();
    assert_loop_replay_closeout_rejects_foreign_retained_replay_authority();
}
