mod category_posture;
mod direct_acceptance_rows;
mod report_gate_shape;
mod scenario_locality;

use crate::facade::{certify_milestone_three_hostile_suite, MilestoneThreeHostileSuiteReport};
use crate::validation::reference_integrity::build_milestone_one_runtime;

fn certify_hostile_suite_report(stem: &str) -> MilestoneThreeHostileSuiteReport {
    certify_milestone_three_hostile_suite(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        stem,
    )
    .expect("milestone three hostile suite should certify")
}




