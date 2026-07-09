use worth_query::facade::{
    lower_policy_aware_historical_plan, CanonicalQueryArtifact, PolicyAwareExecutionSeamError,
    PolicyAwareHistoricalBasis, PolicyAwareHistoricalPlan,
};

fn expects_raw_query_lowerer(
    _: fn(
        &CanonicalQueryArtifact,
        PolicyAwareHistoricalBasis,
    ) -> Result<PolicyAwareHistoricalPlan, PolicyAwareExecutionSeamError>,
) {
}

fn main() {
    expects_raw_query_lowerer(lower_policy_aware_historical_plan);
}
