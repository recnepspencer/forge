mod category_posture;
mod direct_acceptance_rows;
mod report_gate_shape;
mod scenario_locality;

use crate::facade::MilestoneThreeHostileSuiteReport;

fn certify_hostile_suite_report(stem: &str) -> MilestoneThreeHostileSuiteReport {
    let _ = stem;
    crate::certification::test_support::cached_milestone_three_hostile_suite_report()
}
