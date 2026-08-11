use super::super::certification::SupportCertificationEvidenceBundle;
use super::super::domain_certification::{
    SupportDomainCertificationBundle, SupportGenericCertificationReport,
};
use super::super::failure::SupportTrustFailure;
use super::performance_validation::{
    validate_certification_performance, validate_domain_performance, validate_generic_performance,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyPerformanceCloseout {
    certification_row_count: u64,
    certification_index_probe_count: u64,
    certification_receipt_reuse_count: u64,
    certification_allocation_count: u64,
    generic_row_count: u64,
    generic_index_probe_count: u64,
    generic_receipt_reuse_count: u64,
    generic_allocation_count: u64,
    domain_scenario_row_count: u64,
    domain_index_probe_count: u64,
    domain_receipt_reuse_count: u64,
    domain_allocation_count: u64,
    forbidden_exact_overclaim_count: u64,
    global_scan_debt_count: u64,
}

impl SubscriptionSupportAccuracyPerformanceCloseout {
    pub(super) fn from_phase_artifacts(
        evidence_bundle: &SupportCertificationEvidenceBundle,
        generic_report: &SupportGenericCertificationReport,
        domain_bundle: &SupportDomainCertificationBundle,
    ) -> Result<Self, SupportTrustFailure> {
        validate_certification_performance(evidence_bundle)?;
        validate_generic_performance(generic_report)?;
        validate_domain_performance(domain_bundle)?;
        let certification_counters = evidence_bundle.counter_snapshot();
        let generic_counters = generic_report.counter_snapshot();
        let domain_counters = domain_bundle.counter_snapshot();
        Ok(Self {
            certification_row_count: certification_counters.coverage_row_count(),
            certification_index_probe_count: certification_counters.index_probe_count(),
            certification_receipt_reuse_count: certification_counters.receipt_reuse_count(),
            certification_allocation_count: certification_counters.allocation_count(),
            generic_row_count: generic_counters.generic_row_count(),
            generic_index_probe_count: generic_counters.index_probe_count(),
            generic_receipt_reuse_count: generic_counters.receipt_reuse_count(),
            generic_allocation_count: generic_counters.allocation_count(),
            domain_scenario_row_count: domain_counters.scenario_row_count(),
            domain_index_probe_count: domain_counters.index_probe_count(),
            domain_receipt_reuse_count: domain_counters.receipt_reuse_count(),
            domain_allocation_count: domain_counters.allocation_count(),
            forbidden_exact_overclaim_count: certification_counters
                .forbidden_exact_overclaim_count(),
            global_scan_debt_count: certification_counters.global_scan_debt_count(),
        })
    }

    pub fn certification_row_count(&self) -> u64 {
        self.certification_row_count
    }

    pub fn certification_index_probe_count(&self) -> u64 {
        self.certification_index_probe_count
    }

    pub fn certification_receipt_reuse_count(&self) -> u64 {
        self.certification_receipt_reuse_count
    }

    pub fn certification_allocation_count(&self) -> u64 {
        self.certification_allocation_count
    }

    pub fn generic_row_count(&self) -> u64 {
        self.generic_row_count
    }

    pub fn generic_index_probe_count(&self) -> u64 {
        self.generic_index_probe_count
    }

    pub fn generic_receipt_reuse_count(&self) -> u64 {
        self.generic_receipt_reuse_count
    }

    pub fn generic_allocation_count(&self) -> u64 {
        self.generic_allocation_count
    }

    pub fn domain_scenario_row_count(&self) -> u64 {
        self.domain_scenario_row_count
    }

    pub fn domain_index_probe_count(&self) -> u64 {
        self.domain_index_probe_count
    }

    pub fn domain_receipt_reuse_count(&self) -> u64 {
        self.domain_receipt_reuse_count
    }

    pub fn domain_allocation_count(&self) -> u64 {
        self.domain_allocation_count
    }

    pub fn forbidden_exact_overclaim_count(&self) -> u64 {
        self.forbidden_exact_overclaim_count
    }

    pub fn global_scan_debt_count(&self) -> u64 {
        self.global_scan_debt_count
    }
}
