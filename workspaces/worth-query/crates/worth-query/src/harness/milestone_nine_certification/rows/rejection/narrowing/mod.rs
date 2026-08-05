mod masked_influence;
mod relationship_proof;
mod reuse_budget;

use crate::harness::milestone_nine_certification::bundles::MilestoneNineRejectionRow;

pub(super) fn rejection_narrowing_rows() -> Vec<MilestoneNineRejectionRow> {
    let mut rows = Vec::new();
    rows.extend(masked_influence::masked_influence_rows());
    rows.extend(relationship_proof::relationship_proof_rows());
    rows.extend(reuse_budget::reuse_budget_rows());
    rows
}
