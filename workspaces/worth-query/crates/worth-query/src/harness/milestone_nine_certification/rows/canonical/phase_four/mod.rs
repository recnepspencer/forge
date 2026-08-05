mod composition;
mod employee_schema;
mod live_policy;

use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationRow;

pub(super) fn canonical_phase_four_rows() -> Vec<MilestoneNineCertificationRow> {
    let mut rows = Vec::new();
    rows.extend(employee_schema::employee_schema_rows());
    rows.extend(live_policy::live_policy_rows());
    rows.extend(composition::composition_rows());
    rows
}
