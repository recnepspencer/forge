use super::admission_test_support::{
    complete_ledger_from_rows, synthetic_authority_rows_with_label_only_transform,
    synthetic_authority_rows_with_missing_receipt_backed_counters,
    synthetic_authority_rows_with_synthetic_replay, with_receipt_row,
};
use super::*;
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, BooleanEvidenceRowAuthority, WorkloadEvidenceRow,
};
use crate::workload_platform::planar_boolean_edge_splitting::{
    source_carriers_for_tests, split_pair_receipt_for_tests,
};

#[test]
fn guard_contract_denies_each_current_guard_failure_before_authority() {
    let carriers = source_carriers_for_tests();
    let receipt = split_pair_receipt_for_tests(&carriers);

    assert_guard_failure_detail(
        &receipt,
        synthetic_authority_rows_with_label_only_transform(),
        "coordinate-changing transform evidence",
    );
    assert_guard_failure_detail(
        &receipt,
        synthetic_authority_rows_with_synthetic_replay(),
        "retained artifacts and replay checkpoints",
    );
    assert_guard_failure_detail(
        &receipt,
        synthetic_authority_rows_with_missing_receipt_backed_counters(),
        "receipt-backed counters for every stage",
    );
}

fn assert_guard_failure_detail<T>(
    receipt: &T,
    authority_rows: Vec<WorkloadEvidenceRow>,
    expected_detail: &str,
) where
    T: BooleanEvidenceReceipt + BooleanEvidenceRowAuthority + 'static,
{
    let complete = complete_ledger_from_rows(with_receipt_row(
        authority_rows,
        WorkloadEvidenceRow::from_boolean_evidence_receipt(receipt),
    ));
    let denial = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(receipt)
        .with_complete_ledger(&complete)
        .admit()
        .expect_err("guard failure must deny before authority construction");

    assert_eq!(
        denial.kind(),
        SpatialGeometryEvidenceTouchDenialKind::GuardFailure
    );
    assert!(denial.detail().contains(expected_detail));
}
