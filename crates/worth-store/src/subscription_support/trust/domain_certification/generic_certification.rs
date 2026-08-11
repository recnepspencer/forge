use super::digest::stable_digest;
use crate::subscription_support::trust::certification::{
    SupportCertificationCoverageWitness, SupportCertificationSummary,
};
use crate::subscription_support::trust::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use crate::subscription_support::trust::reports::CertifiedSupportTrustReport;
use crate::subscription_support::trust::taxonomy::SupportTrustStrength;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SupportGenericCertificationCounterSnapshot {
    certified_support_report_count: u64,
    generic_row_count: u64,
    index_probe_count: u64,
    receipt_reuse_count: u64,
    allocation_count: u64,
    physical_readiness_debt_count: u64,
}

impl SupportGenericCertificationCounterSnapshot {
    pub fn new(
        certified_support_report_count: u64,
        generic_row_count: u64,
        index_probe_count: u64,
        receipt_reuse_count: u64,
        allocation_count: u64,
        physical_readiness_debt_count: u64,
    ) -> Result<Self, SupportTrustFailure> {
        if certified_support_report_count == 0 || generic_row_count == 0 {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "generic support trust certification requires at least one certified support report and one generic row",
            ));
        }
        Ok(Self {
            certified_support_report_count,
            generic_row_count,
            index_probe_count,
            receipt_reuse_count,
            allocation_count,
            physical_readiness_debt_count,
        })
    }

    pub fn certified_support_report_count(&self) -> u64 {
        self.certified_support_report_count
    }

    pub fn generic_row_count(&self) -> u64 {
        self.generic_row_count
    }

    pub fn index_probe_count(&self) -> u64 {
        self.index_probe_count
    }

    pub fn receipt_reuse_count(&self) -> u64 {
        self.receipt_reuse_count
    }

    pub fn allocation_count(&self) -> u64 {
        self.allocation_count
    }

    pub fn physical_readiness_debt_count(&self) -> u64 {
        self.physical_readiness_debt_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportGenericCertificationReport {
    generic_row_id: String,
    certified_report: CertifiedSupportTrustReport,
    coverage_summary: SupportCertificationSummary,
    counter_snapshot: SupportGenericCertificationCounterSnapshot,
    generic_certification_digest: String,
}

impl SupportGenericCertificationReport {
    pub fn from_certified_support_trust(
        generic_row_id: impl Into<String>,
        certified_report: CertifiedSupportTrustReport,
        coverage_witness: &SupportCertificationCoverageWitness,
        counter_snapshot: SupportGenericCertificationCounterSnapshot,
    ) -> Result<Self, SupportTrustFailure> {
        let generic_row_id = require_non_empty("generic row id", generic_row_id)?;
        if certified_report.certification_stamp().trust_strength() == SupportTrustStrength::Rejected
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
                SupportTrustRecoveryPosture::RerunCertification,
                "generic support certification cannot advertise rejected support as certified semantic support",
            ));
        }
        let coverage_summary = coverage_witness.summary().clone();
        let mut report = Self {
            generic_row_id,
            certified_report,
            coverage_summary,
            counter_snapshot,
            generic_certification_digest: String::new(),
        };
        report.generic_certification_digest =
            stable_digest(&SupportGenericCertificationDigestBasis {
                generic_row_id: &report.generic_row_id,
                certified_report: &report.certified_report,
                coverage_summary: &report.coverage_summary,
                counter_snapshot: report.counter_snapshot,
            })?;
        Ok(report)
    }

    pub fn generic_row_id(&self) -> &str {
        &self.generic_row_id
    }

    pub fn certified_report(&self) -> &CertifiedSupportTrustReport {
        &self.certified_report
    }

    pub fn coverage_summary(&self) -> &SupportCertificationSummary {
        &self.coverage_summary
    }

    pub fn counter_snapshot(&self) -> SupportGenericCertificationCounterSnapshot {
        self.counter_snapshot
    }

    pub fn generic_certification_digest(&self) -> &str {
        &self.generic_certification_digest
    }
}

#[derive(Serialize)]
struct SupportGenericCertificationDigestBasis<'a> {
    generic_row_id: &'a str,
    certified_report: &'a CertifiedSupportTrustReport,
    coverage_summary: &'a SupportCertificationSummary,
    counter_snapshot: SupportGenericCertificationCounterSnapshot,
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
            format!("support trust domain certification {label} must be non-empty"),
        ));
    }
    Ok(value)
}
