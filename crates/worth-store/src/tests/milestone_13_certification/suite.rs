mod evidence;
mod rows;

use self::evidence::collect_milestone_13_lane_evidence;
use self::rows::{
    artifact_digest_parity, coalesced_duplicate_suppression_exactness,
    continuation_move_interleaving_truth_parity, counter_snapshot_exactness,
    diagnostics_digest_divergence, foreground_read_move_interleaving_truth_parity,
    interleaving_counter_exactness, movement_read_interleaving_truth_parity,
    recalled_lane_truth_parity, restart_manifest_bounded_reconstruction, truth_digest_parity,
};
use super::super::harness::certification::{
    core::CertificationSuite, requirements::TIERING_AND_WORKING_SET_NON_AUTHORITY_TEST,
};

pub(super) fn milestone_13_suite() -> CertificationSuite<String, String> {
    let evidence = collect_milestone_13_lane_evidence();

    CertificationSuite::new(TIERING_AND_WORKING_SET_NON_AUTHORITY_TEST.suite_name)
        .with_canonical_row(truth_digest_parity(&evidence))
        .with_canonical_row(artifact_digest_parity(&evidence))
        .with_canonical_row(diagnostics_digest_divergence(&evidence))
        .with_canonical_row(counter_snapshot_exactness(&evidence))
        .with_canonical_row(recalled_lane_truth_parity(&evidence))
        .with_canonical_row(coalesced_duplicate_suppression_exactness(&evidence))
        .with_canonical_row(restart_manifest_bounded_reconstruction(&evidence))
        .with_canonical_row(movement_read_interleaving_truth_parity(&evidence))
        .with_canonical_row(foreground_read_move_interleaving_truth_parity(&evidence))
        .with_canonical_row(continuation_move_interleaving_truth_parity(&evidence))
        .with_canonical_row(interleaving_counter_exactness(&evidence))
}
