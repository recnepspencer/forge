mod admission;
mod execution;
mod narrowing;
mod phase_four;

use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationRow;

pub(in crate::harness::milestone_nine_certification) fn canonical_rows(
) -> Vec<MilestoneNineCertificationRow> {
    let mut rows = Vec::new();
    rows.extend(admission::canonical_admission_rows());
    rows.extend(narrowing::canonical_narrowing_rows());
    rows.extend(execution::canonical_execution_rows());
    rows.extend(phase_four::canonical_phase_four_rows());
    rows
}
