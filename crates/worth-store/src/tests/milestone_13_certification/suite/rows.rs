use super::super::super::harness::certification::core::{AssertionClass, CanonicalRow, LaneResult};
use super::evidence::Milestone13LaneEvidence;

pub(super) fn truth_digest_parity(evidence: &Milestone13LaneEvidence) -> CanonicalRow<String> {
    CanonicalRow::new(
        "truth_digest_parity",
        vec![
            LaneResult::new(
                "control",
                evidence.baseline.control_bundle.truth_digest.clone(),
            ),
            LaneResult::new("moved", evidence.baseline.moved_bundle.truth_digest.clone()),
            LaneResult::new(
                "local_reopened",
                evidence
                    .restart
                    .local_file
                    .reopened_bundle
                    .truth_digest
                    .clone(),
            ),
            LaneResult::new(
                "sqlite_reopened",
                evidence.restart.sqlite.reopened_bundle.truth_digest.clone(),
            ),
        ],
        &[AssertionClass::Equality],
    )
}

pub(super) fn artifact_digest_parity(evidence: &Milestone13LaneEvidence) -> CanonicalRow<String> {
    CanonicalRow::new(
        "artifact_digest_parity",
        vec![
            LaneResult::new(
                "control",
                evidence.baseline.control_bundle.artifact_digest.clone(),
            ),
            LaneResult::new(
                "moved",
                evidence.baseline.moved_bundle.artifact_digest.clone(),
            ),
            LaneResult::new(
                "local_reopened",
                evidence
                    .restart
                    .local_file
                    .reopened_bundle
                    .artifact_digest
                    .clone(),
            ),
            LaneResult::new(
                "sqlite_reopened",
                evidence
                    .restart
                    .sqlite
                    .reopened_bundle
                    .artifact_digest
                    .clone(),
            ),
        ],
        &[AssertionClass::Equality],
    )
}

pub(super) fn diagnostics_digest_divergence(
    evidence: &Milestone13LaneEvidence,
) -> CanonicalRow<String> {
    CanonicalRow::new(
        "diagnostics_digest_divergence",
        vec![
            LaneResult::new(
                "control",
                evidence.baseline.control_bundle.diagnostics_digest.clone(),
            ),
            LaneResult::new(
                "moved",
                evidence.baseline.moved_bundle.diagnostics_digest.clone(),
            ),
            LaneResult::new(
                "local_reopened",
                evidence
                    .restart
                    .local_file
                    .reopened_bundle
                    .diagnostics_digest
                    .clone(),
            ),
            LaneResult::new(
                "sqlite_reopened",
                evidence
                    .restart
                    .sqlite
                    .reopened_bundle
                    .diagnostics_digest
                    .clone(),
            ),
        ],
        &[AssertionClass::Inequality],
    )
}

pub(super) fn counter_snapshot_exactness(
    evidence: &Milestone13LaneEvidence,
) -> CanonicalRow<String> {
    CanonicalRow::new(
        "counter_snapshot_exactness",
        vec![
            LaneResult::new(
                "moved",
                serde_json::to_string(&evidence.baseline.moved_bundle.counter_contract).unwrap(),
            ),
            LaneResult::new(
                "local_moved",
                serde_json::to_string(&evidence.restart.local_file.moved_bundle.counter_contract)
                    .unwrap(),
            ),
            LaneResult::new(
                "sqlite_moved",
                serde_json::to_string(&evidence.restart.sqlite.moved_bundle.counter_contract)
                    .unwrap(),
            ),
            LaneResult::new(
                "expected",
                evidence.baseline.expected_counter_contract.clone(),
            ),
        ],
        &[AssertionClass::Equality, AssertionClass::ExactCounter],
    )
}

pub(super) fn recalled_lane_truth_parity(
    evidence: &Milestone13LaneEvidence,
) -> CanonicalRow<String> {
    CanonicalRow::new(
        "recalled_lane_truth_parity",
        vec![
            LaneResult::new(
                "control",
                evidence.baseline.control_bundle.truth_digest.clone(),
            ),
            LaneResult::new(
                "recalled",
                evidence.interleaving.recalled_bundle.truth_digest.clone(),
            ),
            LaneResult::new(
                "sqlite_reopened",
                evidence.restart.sqlite.reopened_bundle.truth_digest.clone(),
            ),
        ],
        &[AssertionClass::Equality],
    )
}

pub(super) fn coalesced_duplicate_suppression_exactness(
    evidence: &Milestone13LaneEvidence,
) -> CanonicalRow<String> {
    CanonicalRow::new(
        "coalesced_duplicate_suppression_exactness",
        vec![
            LaneResult::new(
                "moved",
                format!(
                    "{}:{}",
                    evidence
                        .baseline
                        .moved_bundle
                        .counter_contract
                        .recall_coalesced_request_count,
                    evidence
                        .baseline
                        .moved_bundle
                        .counter_contract
                        .recall_duplicate_suppression_count
                ),
            ),
            LaneResult::new(
                "sqlite_moved",
                format!(
                    "{}:{}",
                    evidence
                        .restart
                        .sqlite
                        .moved_bundle
                        .counter_contract
                        .recall_coalesced_request_count,
                    evidence
                        .restart
                        .sqlite
                        .moved_bundle
                        .counter_contract
                        .recall_duplicate_suppression_count
                ),
            ),
            LaneResult::new("expected", "1:1".to_string()),
        ],
        &[AssertionClass::Equality, AssertionClass::ExactCounter],
    )
}

pub(super) fn restart_manifest_bounded_reconstruction(
    evidence: &Milestone13LaneEvidence,
) -> CanonicalRow<String> {
    CanonicalRow::new(
        "restart_manifest_bounded_reconstruction",
        vec![
            LaneResult::new(
                "sqlite_before_reopen",
                evidence.restart.sqlite.before_reopen_manifest.clone(),
            ),
            LaneResult::new(
                "sqlite_after_reopen",
                evidence.restart.sqlite.after_reopen_manifest.clone(),
            ),
            LaneResult::new(
                "local_before_reopen",
                evidence.restart.local_file.before_reopen_manifest.clone(),
            ),
            LaneResult::new(
                "local_after_reopen",
                evidence.restart.local_file.after_reopen_manifest.clone(),
            ),
        ],
        &[AssertionClass::Equality],
    )
}

pub(super) fn movement_read_interleaving_truth_parity(
    evidence: &Milestone13LaneEvidence,
) -> CanonicalRow<String> {
    CanonicalRow::new(
        "movement_read_interleaving_truth_parity",
        vec![
            LaneResult::new(
                "control",
                evidence.baseline.control_bundle.truth_digest.clone(),
            ),
            LaneResult::new(
                "interleaved",
                evidence
                    .interleaving
                    .interleaved_bundle
                    .truth_digest
                    .clone(),
            ),
        ],
        &[AssertionClass::Equality],
    )
}

pub(super) fn foreground_read_move_interleaving_truth_parity(
    evidence: &Milestone13LaneEvidence,
) -> CanonicalRow<String> {
    CanonicalRow::new(
        "foreground_read_move_interleaving_truth_parity",
        vec![
            LaneResult::new(
                "control",
                evidence.baseline.control_bundle.truth_digest.clone(),
            ),
            LaneResult::new(
                "foreground_interleaved",
                evidence
                    .interleaving
                    .foreground_interleaved_bundle
                    .truth_digest
                    .clone(),
            ),
        ],
        &[AssertionClass::Equality],
    )
}

pub(super) fn continuation_move_interleaving_truth_parity(
    evidence: &Milestone13LaneEvidence,
) -> CanonicalRow<String> {
    CanonicalRow::new(
        "continuation_move_interleaving_truth_parity",
        vec![
            LaneResult::new(
                "control",
                evidence.baseline.control_bundle.truth_digest.clone(),
            ),
            LaneResult::new(
                "continuation_interleaved",
                evidence
                    .interleaving
                    .continuation_interleaved_bundle
                    .truth_digest
                    .clone(),
            ),
        ],
        &[AssertionClass::Equality],
    )
}

pub(super) fn interleaving_counter_exactness(
    evidence: &Milestone13LaneEvidence,
) -> CanonicalRow<String> {
    CanonicalRow::new(
        "interleaving_counter_exactness",
        vec![
            LaneResult::new(
                "observed",
                format!(
                    "{}:{}:{}:{}",
                    evidence
                        .interleaving
                        .interleaving_counter_contract
                        .tier_interleaved_read_count,
                    evidence
                        .interleaving
                        .interleaving_counter_contract
                        .tier_interleaved_continuation_count,
                    evidence
                        .interleaving
                        .interleaving_counter_contract
                        .tier_interleaving_recall_count,
                    evidence
                        .interleaving
                        .interleaving_counter_contract
                        .tier_interleaving_parity_failure_count
                ),
            ),
            LaneResult::new("expected", "2:1:1:0".to_string()),
        ],
        &[AssertionClass::Equality, AssertionClass::ExactCounter],
    )
}
