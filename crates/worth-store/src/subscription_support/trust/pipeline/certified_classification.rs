use super::super::certification::SupportCertificationCoverageWitness;
use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::super::reports::{CertifiedSupportTrustReport, SupportTrustCertificationStamp};
use super::coverage::SupportTrustCoverageChecked;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CertifiedSupportTrustClassified {
    report: CertifiedSupportTrustReport,
    coverage_witness: SupportCertificationCoverageWitness,
}

impl CertifiedSupportTrustClassified {
    pub fn report(&self) -> &CertifiedSupportTrustReport {
        &self.report
    }

    pub fn coverage_witness(&self) -> &SupportCertificationCoverageWitness {
        &self.coverage_witness
    }
}

pub fn classify_certified_support_trust(
    coverage_checked: SupportTrustCoverageChecked,
    certification_stamp: SupportTrustCertificationStamp,
) -> Result<CertifiedSupportTrustClassified, SupportTrustFailure> {
    if certification_stamp.row_id() != coverage_checked.covered_row_id() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "certification stamp row id must match the covered certification row",
        ));
    }
    if certification_stamp.evidence_bundle_digest() != coverage_checked.evidence_bundle_digest() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "certification stamp evidence digest must match the checked certification bundle",
        ));
    }
    let (operational, coverage_witness) = coverage_checked.into_certification_inputs();
    let report = CertifiedSupportTrustReport::from_operational_report(
        operational.into_certification_report(),
        certification_stamp,
    )?;
    Ok(CertifiedSupportTrustClassified {
        report,
        coverage_witness,
    })
}
