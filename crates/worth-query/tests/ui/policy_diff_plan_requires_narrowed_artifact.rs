use worth_query::facade::{
    lower_policy_aware_diff_plan, CanonicalQueryArtifact, PolicyAwareDiffBasisPair,
    PolicyAwareDiffPlan, PolicyAwareExecutionSeamError,
};

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
