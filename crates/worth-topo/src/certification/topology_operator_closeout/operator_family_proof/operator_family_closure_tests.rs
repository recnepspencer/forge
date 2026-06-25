use super::operator_family_closure::ensure_operator_family_closure_rows;

#[test]
fn operator_family_closure_rejects_decorative_rows_without_counts() {
    let mut report =
        crate::certification::test_support::cached_milestone_three_hostile_suite_report();

    let row = report
        .operator_family_closure_rows
        .iter_mut()
        .next()
        .expect("operator family closure should emit rows");
    row.legal_execution_count = 0;
    row.hostile_workload_count = 0;
    row.derived_breadth_evidence_count = 0;

    assert!(
        ensure_operator_family_closure_rows(&report).is_err(),
        "operator-family closeout must reject rows whose labels are not backed by proof counts"
    );
}
