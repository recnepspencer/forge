use super::super::certification::SupportCertificationEvidenceBundle;
use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyCertificationOutputs {
    artifact_digest: String,
    subscription_support_digest: String,
    diagnostics_digest: String,
    counter_snapshot_digest: String,
    certification_summary_digest: String,
}

impl SubscriptionSupportAccuracyCertificationOutputs {
    pub fn from_evidence_bundle(
        evidence_bundle: &SupportCertificationEvidenceBundle,
    ) -> Result<Self, SupportTrustFailure> {
        Ok(Self {
            artifact_digest: require_non_empty(
                "artifact digest",
                evidence_bundle.artifact_digest(),
            )?,
            subscription_support_digest: require_non_empty(
                "subscription-support digest",
                evidence_bundle.subscription_support_digest(),
            )?,
            diagnostics_digest: require_non_empty(
                "diagnostics digest",
                evidence_bundle.diagnostics_digest(),
            )?,
            counter_snapshot_digest: require_non_empty(
                "counter snapshot digest",
                evidence_bundle.counter_snapshot_digest(),
            )?,
            certification_summary_digest: require_non_empty(
                "certification summary digest",
                evidence_bundle.certification_summary_digest(),
            )?,
        })
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }

    pub fn subscription_support_digest(&self) -> &str {
        &self.subscription_support_digest
    }

    pub fn diagnostics_digest(&self) -> &str {
        &self.diagnostics_digest
    }

    pub fn counter_snapshot_digest(&self) -> &str {
        &self.counter_snapshot_digest
    }

    pub fn certification_summary_digest(&self) -> &str {
        &self.certification_summary_digest
    }
}

fn require_non_empty(
    label: &'static str,
    value: impl Into<String>,
) -> Result<String, SupportTrustFailure> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            format!("subscription-support accuracy suite {label} must be non-empty"),
        ));
    }
    Ok(value)
}
