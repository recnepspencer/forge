use worth_query::facade::foundation::CanonicalQueryArtifact;
use worth_query::facade::policy::{lower_policy_aware_historical_plan, PolicyAwareExecutionSeamError, PolicyAwareHistoricalBasis, PolicyAwareHistoricalPlan};

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
