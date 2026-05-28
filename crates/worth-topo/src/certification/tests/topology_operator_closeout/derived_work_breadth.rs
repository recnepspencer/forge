use crate::facade::{
    certify_milestone_three_closeout, milestone_three_closeout_requirements,
};
use crate::validation::reference_integrity::milestone_one_runtime_builder;

#[test]
fn milestone_three_closeout_requires_derived_work_breadth_rows() {
    let requirements = milestone_three_closeout_requirements();
    let report = certify_milestone_three_closeout(
        || {
            milestone_one_runtime_builder()
                .expect(" milestone one runtime builder")
                .build()
        },
        "milestone-three-derived-work-breadth",
    )
    .expect("milestone three closeout should certify");

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




