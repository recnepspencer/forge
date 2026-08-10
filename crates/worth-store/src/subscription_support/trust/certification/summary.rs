use super::super::failure::SupportTrustFailure;
use super::certification_row::SupportCertificationRow;
use super::certification_validation::stable_digest;
use super::coverage_plan::SubscriptionSupportCertificationCoveragePlan;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCertificationGapReport {
    missing_row_ids: Vec<String>,
}

impl SupportCertificationGapReport {
    pub fn from_plan_and_rows(
        plan: &SubscriptionSupportCertificationCoveragePlan,
        rows: &[SupportCertificationRow],
    ) -> Self {
        let missing_row_ids = plan
            .required_rows()
            .iter()
            .filter(|required| {
                !rows
                    .iter()
                    .any(|row| row.evidence().row_id() == required.row_id())
            })
            .map(|required| required.row_id().to_string())
            .collect();
        Self { missing_row_ids }
    }

    pub fn is_empty(&self) -> bool {
        self.missing_row_ids.is_empty()
    }

    pub fn missing_row_ids(&self) -> &[String] {
        &self.missing_row_ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCertificationSummary {
    row_count: u64,
    artifact_digest: String,
    subscription_support_digest: String,
    diagnostics_digest: String,
    counter_digest: String,
    certification_summary_digest: String,
}

impl SupportCertificationSummary {
    pub fn row_count(&self) -> u64 {
        self.row_count
    }

    pub fn certification_summary_digest(&self) -> &str {
        &self.certification_summary_digest
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

    pub fn counter_digest(&self) -> &str {
        &self.counter_digest
    }
}

pub(super) fn summarize_rows(
    rows: &[SupportCertificationRow],
) -> Result<SupportCertificationSummary, SupportTrustFailure> {
    let artifact_digests = rows
        .iter()
        .map(|row| row.evidence().artifact_digest.as_str())
        .collect::<Vec<_>>();
    let subscription_support_digests = rows
        .iter()
        .map(|row| row.evidence().subscription_support_digest.as_str())
        .collect::<Vec<_>>();
    let diagnostics_digests = rows
        .iter()
        .map(|row| row.evidence().diagnostics_digest.as_str())
        .collect::<Vec<_>>();
    let counter_digests = rows
        .iter()
        .map(|row| row.evidence().counter_digest.as_str())
        .collect::<Vec<_>>();
    let row_digests = rows
        .iter()
        .map(|row| row.evidence().declared_row_digest())
        .collect::<Vec<_>>();
    let mut summary = SupportCertificationSummary {
        row_count: rows.len() as u64,
        artifact_digest: stable_digest(&artifact_digests)?,
        subscription_support_digest: stable_digest(&subscription_support_digests)?,
        diagnostics_digest: stable_digest(&diagnostics_digests)?,
        counter_digest: stable_digest(&counter_digests)?,
        certification_summary_digest: String::new(),
    };
    summary.certification_summary_digest = stable_digest(&(
        summary.row_count,
        &summary.artifact_digest,
        &summary.subscription_support_digest,
        &summary.diagnostics_digest,
        &summary.counter_digest,
        row_digests,
    ))?;
    Ok(summary)
}
