use crate::facade::milestone_three_closeout_requirements;

#[test]
fn milestone_three_closeout_requires_derived_work_breadth_rows() {
    let requirements = milestone_three_closeout_requirements();
    let report = crate::certification::test_support::cached_milestone_three_closeout_report();

    assert_eq!(
        report.derived_work_breadth_rows.len(),
        requirements.required_family_rows.len()
    );
    assert!(report.derived_work_breadth_rows.iter().any(|row| {
        row.locality_claim_mismatch()
            && row.fallback_count() > 0
            && row.actual_derived_validation_row_count() > 0
    }));
    assert!(report
        .derived_work_breadth_rows
        .iter()
        .all(|row| row.declared_changed_scope_count() > 0));
}
