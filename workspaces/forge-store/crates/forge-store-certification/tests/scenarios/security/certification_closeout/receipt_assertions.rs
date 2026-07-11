use std::collections::BTreeSet;

use forge_store_certification::{S51CloseoutPerformanceReceipts, S51CloseoutPerformanceRows};

pub(crate) fn assert_exact_counter_backed_receipt_rows(receipts: &S51CloseoutPerformanceReceipts) {
    let expected_rows = expected_receipt_rows(receipts.rows());
    let receipt_rows = receipts.counter_backed_receipt().counter_rows();
    assert_eq!(receipt_rows.len(), expected_rows.len());

    let mut seen_names = BTreeSet::new();
    for row in receipt_rows {
        let name = row.name().as_str();
        assert!(seen_names.insert(name), "duplicate receipt counter {name}");
        let expected_count = expected_rows
            .iter()
            .find(|(expected_name, _)| *expected_name == name)
            .map(|(_, expected_count)| *expected_count)
            .unwrap_or_else(|| panic!("unexpected receipt counter {name}"));
        assert_eq!(
            row.observed_count(),
            expected_count,
            "receipt counter {name} carried an unexpected observed count"
        );
    }

    for (expected_name, _) in expected_rows {
        assert!(
            seen_names.contains(expected_name),
            "missing receipt counter {expected_name}"
        );
    }
}

fn expected_receipt_rows(rows: S51CloseoutPerformanceRows) -> [(&'static str, u64); 18] {
    [
        (
            "store.s5_1.closeout.scenario_evidence_rows",
            rows.scenario_evidence_rows(),
        ),
        (
            "store.s5_1.closeout.replay_transcripts",
            rows.replay_transcripts(),
        ),
        (
            "store.s5_1.closeout.lower_store_requests",
            rows.lower_store_requests(),
        ),
        (
            "store.s5_1.closeout.lower_store_current_authority_checks",
            rows.lower_store_current_authority_checks(),
        ),
        (
            "store.s5_1.closeout.lower_store_witness_sets_issued",
            rows.lower_store_witness_sets_issued(),
        ),
        (
            "store.s5_1.closeout.lower_store_denials",
            rows.lower_store_denials(),
        ),
        (
            "store.s5_1.closeout.physical_scope_drift",
            rows.physical_scope_drift(),
        ),
        (
            "store.s5_1.closeout.stale_key_posture",
            rows.stale_key_posture(),
        ),
        (
            "store.s5_1.closeout.wrong_tenant_scope",
            rows.wrong_tenant_scope(),
        ),
        (
            "store.s5_1.closeout.missing_authenticity_requirement",
            rows.missing_authenticity_requirement(),
        ),
        (
            "store.s5_1.closeout.replayed_custody_posture",
            rows.replayed_custody_posture(),
        ),
        (
            "store.s5_1.closeout.replay_wrong_tenant_scope",
            rows.replay_wrong_tenant_scope(),
        ),
        (
            "store.s5_1.closeout.replay_stale_key_posture",
            rows.replay_stale_key_posture(),
        ),
        (
            "store.s5_1.closeout.replay_missing_authenticity_requirement",
            rows.replay_missing_authenticity_requirement(),
        ),
        (
            "store.s5_1.closeout.replay_baseline_admissions",
            rows.replay_baseline_admissions(),
        ),
        (
            "store.s5_1.closeout.replay_attempts",
            rows.replay_attempts(),
        ),
        (
            "store.s5_1.closeout.replay_denials_before_decode",
            rows.replay_denials_before_logical_decode(),
        ),
        (
            "store.s5_1.closeout.handoff_admitted",
            rows.handoff_admitted(),
        ),
    ]
}
