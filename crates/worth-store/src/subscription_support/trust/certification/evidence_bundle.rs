use super::super::failure::SupportTrustFailure;
use super::super::reports::OperationalSupportTrustReport;
use super::batch_scope::SupportCertificationBatchScope;
use super::certification_row::SupportCertificationRow;
use super::certification_validation::{
    require_non_empty, stable_digest, validate_certification_counters,
};
use super::counter_snapshot::SupportCertificationCounterSnapshot;
use super::coverage_matrix::{
    validate_first_ship_family_coverage, SupportCertificationCoverageMatrix,
    SupportCertificationCoverageWitness,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCertificationEvidenceBundle {
    run_id: String,
    coverage_matrix: SupportCertificationCoverageMatrix,
    batch_scope: SupportCertificationBatchScope,
    counter_snapshot: SupportCertificationCounterSnapshot,
    artifact_digest: String,
    subscription_support_digest: String,
    diagnostics_digest: String,
    counter_snapshot_digest: String,
    certification_summary_digest: String,
    evidence_bundle_digest: String,
}

impl SupportCertificationEvidenceBundle {
    pub fn new(
        run_id: impl Into<String>,
        coverage_matrix: SupportCertificationCoverageMatrix,
        batch_scope: SupportCertificationBatchScope,
        counter_snapshot: SupportCertificationCounterSnapshot,
    ) -> Result<Self, SupportTrustFailure> {
        validate_first_ship_family_coverage(&coverage_matrix)?;
        validate_certification_counters(&coverage_matrix, batch_scope, counter_snapshot)?;
        let summary = coverage_matrix.summary();
        let counter_snapshot_digest = stable_digest(&counter_snapshot)?;
        let mut bundle = Self {
            run_id: require_non_empty("run id", run_id)?,
            artifact_digest: summary.artifact_digest().to_string(),
            subscription_support_digest: summary.subscription_support_digest().to_string(),
            diagnostics_digest: summary.diagnostics_digest().to_string(),
            counter_snapshot_digest,
            certification_summary_digest: summary.certification_summary_digest().to_string(),
            coverage_matrix,
            batch_scope,
            counter_snapshot,
            evidence_bundle_digest: String::new(),
        };
        bundle.evidence_bundle_digest =
            stable_digest(&SupportCertificationEvidenceBundleDigestBasis {
                run_id: &bundle.run_id,
                artifact_digest: &bundle.artifact_digest,
                subscription_support_digest: &bundle.subscription_support_digest,
                diagnostics_digest: &bundle.diagnostics_digest,
                counter_snapshot_digest: &bundle.counter_snapshot_digest,
                certification_summary_digest: &bundle.certification_summary_digest,
                batch_scope: &bundle.batch_scope,
                counter_snapshot: &bundle.counter_snapshot,
            })?;
        Ok(bundle)
    }

    pub fn evidence_bundle_digest(&self) -> &str {
        &self.evidence_bundle_digest
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

    pub fn counter_snapshot(&self) -> SupportCertificationCounterSnapshot {
        self.counter_snapshot
    }

    pub fn batch_scope(&self) -> SupportCertificationBatchScope {
        self.batch_scope
    }

    pub(crate) fn coverage_rows(&self) -> &[SupportCertificationRow] {
        self.coverage_matrix.rows()
    }

    pub(crate) fn covered_row_id_for_operational_report(
        &self,
        report: &OperationalSupportTrustReport,
    ) -> Option<&str> {
        self.coverage_matrix
            .covered_row_id_for_operational_report(report)
    }

    pub(crate) fn into_witness(self) -> SupportCertificationCoverageWitness {
        self.coverage_matrix.into_witness()
    }
}

#[derive(Serialize)]
struct SupportCertificationEvidenceBundleDigestBasis<'a> {
    run_id: &'a str,
    artifact_digest: &'a str,
    subscription_support_digest: &'a str,
    diagnostics_digest: &'a str,
    counter_snapshot_digest: &'a str,
    certification_summary_digest: &'a str,
    batch_scope: &'a SupportCertificationBatchScope,
    counter_snapshot: &'a SupportCertificationCounterSnapshot,
}
