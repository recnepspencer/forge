mod admission;
mod enforcement;
mod execution;
mod narrowing;

use crate::harness::milestone_nine_certification::bundles::MilestoneNineRejectionRow;

pub(in crate::harness::milestone_nine_certification) fn rejection_rows(
) -> Vec<MilestoneNineRejectionRow> {
    let mut rows = Vec::new();
    rows.extend(admission::rejection_admission_rows());
    rows.extend(narrowing::rejection_narrowing_rows());
    rows.extend(execution::rejection_execution_rows());
    rows.extend(enforcement::rejection_enforcement_rows());
    rows
}
