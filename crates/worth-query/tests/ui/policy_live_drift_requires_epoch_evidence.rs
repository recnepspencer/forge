use worth_query::facade::policy::{PolicyAwareExecutionSeamError, PolicyAwareLivePlan, PolicyDriftDisposition, PolicyLiveDensityEvidence, PolicyLiveDriftEvidenceReport};
use worth_query::facade::certification::certify_policy_live_drift_evidence;

fn expects_raw_drift_shortcut(
    _: fn(
        &PolicyAwareLivePlan,
        PolicyDriftDisposition,
        PolicyLiveDensityEvidence,
    ) -> Result<PolicyLiveDriftEvidenceReport, PolicyAwareExecutionSeamError>,
) {
}

fn main() {
    expects_raw_drift_shortcut(certify_policy_live_drift_evidence);
}
