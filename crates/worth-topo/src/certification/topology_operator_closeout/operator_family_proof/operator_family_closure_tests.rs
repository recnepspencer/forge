use super::operator_family_closure::ensure_operator_family_closure_rows;
use crate::certification::topology_operator_closeout::suite::certify_milestone_three_hostile_suite_impl;
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn operator_family_closure_rejects_decorative_rows_without_counts() {
    let mut report = certify_milestone_three_hostile_suite_impl(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        "m3.operator_family.decorative_row_tamper",
    )
    .expect("milestone three hostile suite should certify before tamper");

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




