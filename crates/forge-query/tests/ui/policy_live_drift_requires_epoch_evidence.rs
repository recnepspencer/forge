use forge_query::facade::{
    certify_policy_live_drift_evidence, PolicyAwareExecutionSeamError, PolicyAwareLivePlan,
    PolicyDriftDisposition, PolicyLiveDensityEvidence, PolicyLiveDriftEvidenceReport,
};

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
