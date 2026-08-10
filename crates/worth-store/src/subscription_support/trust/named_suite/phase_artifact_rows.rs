use super::super::certification::SupportCertificationEvidenceBundle;
use super::super::domain_certification::{
    SupportCertificationHandoffReport, SupportDomainCertificationBundle,
    SupportGenericCertificationReport,
};
use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::certification_row::SubscriptionSupportAccuracyCertificationRow;
use super::lane_evidence_set::SubscriptionSupportAccuracyLaneEvidenceSet;
use super::phase_artifact_evidence::expected_row_evidence_digests;
use super::row_kind::SubscriptionSupportAccuracyCertificationRowKind;

pub(super) fn build_required_rows_from_phase_artifacts(
    evidence_bundle: &SupportCertificationEvidenceBundle,
    generic_report: &SupportGenericCertificationReport,
    domain_bundle: &SupportDomainCertificationBundle,
    handoff_report: &SupportCertificationHandoffReport,
    lane_evidence: &SubscriptionSupportAccuracyLaneEvidenceSet,
) -> Result<Vec<SubscriptionSupportAccuracyCertificationRow>, SupportTrustFailure> {
    expected_row_evidence_digests(
        evidence_bundle,
        generic_report,
        domain_bundle,
        handoff_report,
        lane_evidence,
    )?
    .into_iter()
    .map(build_required_row)
    .collect()
}

fn build_required_row(
    (row_kind, evidence_digest): (SubscriptionSupportAccuracyCertificationRowKind, String),
) -> Result<SubscriptionSupportAccuracyCertificationRow, SupportTrustFailure> {
    SubscriptionSupportAccuracyCertificationRow::new(row_kind, evidence_digest, 0, 0)
}

pub(super) fn validate_rows_match_phase_artifacts(
    rows: &[SubscriptionSupportAccuracyCertificationRow],
    evidence_bundle: &SupportCertificationEvidenceBundle,
    generic_report: &SupportGenericCertificationReport,
    domain_bundle: &SupportDomainCertificationBundle,
    handoff_report: &SupportCertificationHandoffReport,
    lane_evidence: &SubscriptionSupportAccuracyLaneEvidenceSet,
) -> Result<(), SupportTrustFailure> {
    let expected = expected_row_evidence_digests(
        evidence_bundle,
        generic_report,
        domain_bundle,
        handoff_report,
        lane_evidence,
    )?;
    for row in rows {
        match expected.get(&row.row_kind()) {
            Some(expected_digest) if expected_digest == row.evidence_digest() => {}
            _ => {
                return Err(SupportTrustFailure::new(
                    SupportTrustFailureKind::SupportTrustCoverageMissing,
                    SupportTrustRecoveryPosture::RerunCertification,
                    "subscription-support accuracy suite row evidence must match the supplied phase artifacts",
                ));
            }
        }
    }
    Ok(())
}
