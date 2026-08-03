use worth_relational::facade::history::BranchId;
use worth_runtime_bridge::facade::TruthBranchIdentity;

const PRIMARY_APPLICATION_BRANCH: &str = "main";

pub(super) fn primary_relational_branch_id() -> BranchId {
    BranchId(PRIMARY_APPLICATION_BRANCH.to_owned())
}

pub(super) fn primary_truth_branch_identity() -> TruthBranchIdentity {
    TruthBranchIdentity::from_relational_branch_id(PRIMARY_APPLICATION_BRANCH)
}
