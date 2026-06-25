use std::sync::OnceLock;

use crate::certification::certify_milestone_three_closeout;
use crate::facade::MilestoneThreeHostileSuiteReport;
use crate::validation::reference_integrity::build_milestone_one_runtime;

pub(crate) fn cached_milestone_three_hostile_suite_report() -> MilestoneThreeHostileSuiteReport {
    cached_milestone_three_closeout_report()
}

pub(crate) fn cached_milestone_three_closeout_report() -> MilestoneThreeHostileSuiteReport {
    static REPORT: OnceLock<MilestoneThreeHostileSuiteReport> = OnceLock::new();

    REPORT
        .get_or_init(|| {
            certify_milestone_three_closeout(
                || build_milestone_one_runtime().expect("milestone one runtime builder"),
                "m3.cached_closeout",
            )
            .expect("milestone three closeout should certify")
        })
        .clone()
}
