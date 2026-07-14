use worth_query::facade::foundation::CanonicalQueryArtifact;
use worth_query::facade::policy::{lower_policy_aware_branch_plan, PolicyAwareBranchPlan, PolicyAwareExecutionSeamError, PolicyAwareReadBasis};

fn expects_raw_query_lowerer(
    _: fn(
        &CanonicalQueryArtifact,
        PolicyAwareReadBasis,
    ) -> Result<PolicyAwareBranchPlan, PolicyAwareExecutionSeamError>,
) {
}

fn main() {
    expects_raw_query_lowerer(lower_policy_aware_branch_plan);
}
