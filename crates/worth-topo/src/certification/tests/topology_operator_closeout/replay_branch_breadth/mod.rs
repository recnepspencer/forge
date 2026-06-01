use crate::facade::{certify_milestone_three_closeout, MilestoneThreeHostileSuiteReport};
use crate::validation::reference_integrity::milestone_one_runtime_builder;

#[test]
fn closeout_exposes_replay_and_branch_breadth_as_direct_evidence() {
    let report = certify_closeout_report("m3.closeout.replay_branch_breadth");

    assert_eq!(report.replay_branch_breadth_rows.len(), 1);
    let row = report
        .replay_branch_breadth_rows
        .first()
        .expect("closeout should include replay/branch breadth row");

    assert_eq!(
        row.replay_checked_scenario_count(),
        row.required_scenario_count()
    );
    assert_eq!(row.replay_step_count(), row.replay_comparison_step_count());
    assert_eq!(row.replay_mismatch_count(), 0);
    assert_eq!(
        row.accepted_branch_local_row_count(),
        row.required_accepted_branch_local_count()
    );
    assert_eq!(
        row.rejected_branch_local_row_count(),
        row.required_rejected_branch_local_count()
    );
    assert_eq!(
        row.branch_truth_digest_count(),
        row.accepted_branch_local_row_count()
    );
    assert_eq!(
        row.unchanged_rejected_branch_count(),
        row.required_rejected_branch_local_count()
    );
    assert!(row.row_digest().contains("replay_checked="));
    assert!(row.row_digest().contains("branch_rows="));
}

fn certify_closeout_report(stem: &str) -> MilestoneThreeHostileSuiteReport {
    certify_milestone_three_closeout(
        || {
            milestone_one_runtime_builder()
                .expect("milestone one runtime builder")
                .build()
        },
        stem,
    )
    .expect("milestone three closeout should certify")
}
