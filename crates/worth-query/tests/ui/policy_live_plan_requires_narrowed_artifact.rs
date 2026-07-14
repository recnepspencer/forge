use worth_query::facade::foundation::CanonicalQueryArtifact;
use worth_query::facade::policy::{admit_policy_aware_live_plan, PolicyAwareExecutionSeamError, PolicyAwareLivePlan, PolicyDriftDisposition, PolicyLiveDensityPosture};

fn expects_raw_query_lowerer(
    _: fn(
        &CanonicalQueryArtifact,
        &[String],
        PolicyDriftDisposition,
        PolicyLiveDensityPosture,
    ) -> Result<PolicyAwareLivePlan, PolicyAwareExecutionSeamError>,
) {
}

fn main() {
    expects_raw_query_lowerer(admit_policy_aware_live_plan);
}
