use crate::facade::{MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileSuiteReport};

#[test]
fn closeout_exposes_validation_breadth_for_every_hostile_scenario() {
    let report = certify_closeout_report("m3.closeout.validation_breadth");

    assert_eq!(report.validation_breadth_rows.len(), 5);
    assert!(report.validation_breadth_rows.iter().all(|row| {
        row.validator_family_count() >= 3
            && row.validator_name_count() >= 3
            && row.mutation_family_count() > 0
            && row.changed_scope_count() > 0
            && row.naming_scope_count() > 0
            && row.derived_region_count() > 0
            && row.replay_checked()
            && row.row_digest().contains("validator_families=")
    }));
}

#[test]
fn validation_breadth_distinguishes_accepted_inspection_from_rejection_locality() {
    let report = certify_closeout_report("m3.closeout.validation_breadth.outcomes");

    let accepted_rows = report
        .validation_breadth_rows
        .iter()
        .filter(|row| row.outcome_class() == MilestoneThreeHostileOutcomeClass::Accepted)
        .collect::<Vec<_>>();
    assert_eq!(accepted_rows.len(), 3);
    assert!(accepted_rows
        .iter()
        .all(|row| row.derived_validation_row_count() > 0));

    let rejected_rows = report
        .validation_breadth_rows
        .iter()
        .filter(|row| row.outcome_class() == MilestoneThreeHostileOutcomeClass::Rejected)
        .collect::<Vec<_>>();
    assert_eq!(rejected_rows.len(), 2);
    assert!(rejected_rows
        .iter()
        .all(|row| row.localized_rejection_boundary_count() > 0));
}

fn certify_closeout_report(stem: &str) -> MilestoneThreeHostileSuiteReport {
    let _ = stem;
    crate::certification::test_support::cached_milestone_three_closeout_report()
}
