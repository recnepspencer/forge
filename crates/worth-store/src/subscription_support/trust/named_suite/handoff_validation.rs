use super::super::domain_certification::{
    SupportCertificationHandoffReport, SupportDomainCertificationBundle,
    SupportGenericCertificationReport, SupportRoadmapPhysicalReadinessPosture,
};
use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};

pub(super) fn validate_handoff(
    handoff_report: &SupportCertificationHandoffReport,
) -> Result<(), SupportTrustFailure> {
    if !handoff_report.semantic_support_trust_closed()
        || handoff_report.roadmap_physical_readiness_posture()
            != SupportRoadmapPhysicalReadinessPosture::PhysicalDatabaseReadinessDeferredToRoadmap2
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
            SupportTrustRecoveryPosture::WaitForMilestone14OrRoadmap2Evidence,
            "subscription-support accuracy suite requires semantic trust closure while keeping physical readiness debt explicit",
        ));
    }
    Ok(())
}

pub(super) fn validate_handoff_matches_phase_artifacts(
    generic_report: &SupportGenericCertificationReport,
    domain_bundle: &SupportDomainCertificationBundle,
    handoff_report: &SupportCertificationHandoffReport,
) -> Result<(), SupportTrustFailure> {
    if handoff_report.generic_certification_digest()
        != generic_report.generic_certification_digest()
        || handoff_report.domain_certification_digest()
            != domain_bundle.domain_certification_digest()
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "subscription-support accuracy handoff must be bound to the supplied generic and domain certification artifacts",
        ));
    }
    Ok(())
}
