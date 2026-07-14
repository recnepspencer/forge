use worth_query::facade::foundation::CanonicalQueryArtifact;
use worth_query::facade::policy::{lower_policy_aware_diff_plan, PolicyAwareDiffBasisPair, PolicyAwareDiffPlan, PolicyAwareExecutionSeamError};

fn expects_raw_query_lowerer(
    _: fn(
        &CanonicalQueryArtifact,
        PolicyAwareDiffBasisPair,
    ) -> Result<PolicyAwareDiffPlan, PolicyAwareExecutionSeamError>,
) {
}

fn main() {
    expects_raw_query_lowerer(lower_policy_aware_diff_plan);
}
