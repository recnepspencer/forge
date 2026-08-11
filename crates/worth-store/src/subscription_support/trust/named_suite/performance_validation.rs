use super::super::certification::SupportCertificationBatchScopeKind;
use super::super::certification::SupportCertificationEvidenceBundle;
use super::super::domain_certification::{
    SupportDomainCertificationBundle, SupportGenericCertificationReport,
};
use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::super::performance::{
    SupportTrustAllocationScope, SupportTrustDensityClass, SupportTrustPathClass,
};

pub(super) fn validate_certification_performance(
    evidence_bundle: &SupportCertificationEvidenceBundle,
) -> Result<(), SupportTrustFailure> {
    let batch_scope = evidence_bundle.batch_scope();
    let counters = evidence_bundle.counter_snapshot();
    let valid_scope = batch_scope.scope_kind()
        == SupportCertificationBatchScopeKind::CertificationScopeLocal
        && batch_scope.density_class() == SupportTrustDensityClass::CertificationScopeLocal
        && batch_scope.path_class() == SupportTrustPathClass::BatchCertificationPath
        && batch_scope.allocation_scope() == SupportTrustAllocationScope::BatchCertification;
    if !valid_scope
        || batch_scope.row_count() != 4
        || batch_scope.expected_index_probes() != 4
        || batch_scope.expected_receipt_reuse_count() != 3
        || batch_scope.expected_allocation_count() != 1
        || counters.coverage_row_count() != batch_scope.row_count()
        || counters.index_probe_count() != batch_scope.expected_index_probes()
        || counters.receipt_reuse_count() != batch_scope.expected_receipt_reuse_count()
        || counters.allocation_count() != batch_scope.expected_allocation_count()
        || counters.forbidden_exact_overclaim_count() != 0
        || counters.global_scan_debt_count() != 0
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "subscription-support accuracy closeout requires exact certification performance counters and bounded batch access",
        ));
    }
    Ok(())
}

pub(super) fn validate_generic_performance(
    generic_report: &SupportGenericCertificationReport,
) -> Result<(), SupportTrustFailure> {
    let counters = generic_report.counter_snapshot();
    if counters.certified_support_report_count() != 1
        || counters.generic_row_count() != 1
        || counters.index_probe_count() != 1
        || counters.receipt_reuse_count() != 1
        || counters.allocation_count() != 1
        || counters.physical_readiness_debt_count() != 1
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "subscription-support accuracy closeout requires exact generic certification counters and explicit physical-readiness debt",
        ));
    }
    Ok(())
}

pub(super) fn validate_domain_performance(
    domain_bundle: &SupportDomainCertificationBundle,
) -> Result<(), SupportTrustFailure> {
    let counters = domain_bundle.counter_snapshot();
    if counters.scenario_row_count() != 5
        || counters.certified_semantic_row_count() != 3
        || counters.explicit_debt_row_count() != 2
        || counters.index_probe_count() != 5
        || counters.receipt_reuse_count() != 4
        || counters.allocation_count() != 1
        || counters.physical_readiness_debt_count() != counters.explicit_debt_row_count()
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "subscription-support accuracy closeout requires exact domain scenario counters and future-owned debt rows",
        ));
    }
    Ok(())
}
