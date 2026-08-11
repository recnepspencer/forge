use super::super::certification::{
    SupportCertificationCoverageWitness, SupportCertificationEvidenceBundle,
};
use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::operational_classification::OperationalSupportTrustClassified;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustCoverageChecked {
    operational: OperationalSupportTrustClassified,
    coverage_witness: SupportCertificationCoverageWitness,
    covered_row_id: String,
    evidence_bundle_digest: String,
}

impl SupportTrustCoverageChecked {
    pub fn operational(&self) -> &OperationalSupportTrustClassified {
        &self.operational
    }

    pub fn coverage_witness(&self) -> &SupportCertificationCoverageWitness {
        &self.coverage_witness
    }

    pub fn covered_row_id(&self) -> &str {
        &self.covered_row_id
    }

    pub fn evidence_bundle_digest(&self) -> &str {
        &self.evidence_bundle_digest
    }

    pub(super) fn into_certification_inputs(
        self,
    ) -> (
        OperationalSupportTrustClassified,
        SupportCertificationCoverageWitness,
    ) {
        (self.operational, self.coverage_witness)
    }
}

pub fn check_support_trust_coverage(
    operational: OperationalSupportTrustClassified,
    evidence_bundle: SupportCertificationEvidenceBundle,
) -> Result<SupportTrustCoverageChecked, SupportTrustFailure> {
    let covered_row_id = evidence_bundle
        .covered_row_id_for_operational_report(operational.report())
        .ok_or_else(|| {
            SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "support trust certification evidence bundle does not cover the operational trust report",
            )
        })?
        .to_string();
    let evidence_bundle_digest = evidence_bundle.evidence_bundle_digest().to_string();
    let coverage_witness = evidence_bundle.into_witness();
    Ok(SupportTrustCoverageChecked {
        operational,
        coverage_witness,
        covered_row_id,
        evidence_bundle_digest,
    })
}
