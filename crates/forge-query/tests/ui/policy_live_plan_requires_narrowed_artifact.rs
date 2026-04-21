use forge_query::facade::{
    admit_policy_aware_live_plan, CanonicalQueryArtifact, PolicyAwareExecutionSeamError,
    PolicyAwareLivePlan, PolicyDriftDisposition, PolicyLiveDensityPosture,
};

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
