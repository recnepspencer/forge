use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::row_evidence::SupportCertificationRowEvidence;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCertificationRow {
    evidence: SupportCertificationRowEvidence,
}

impl SupportCertificationRow {
    pub fn new(evidence: SupportCertificationRowEvidence) -> Result<Self, SupportTrustFailure> {
        if evidence.declared_row_digest() != evidence.recomputed_row_digest()? {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "support trust certification row digest does not match structured evidence",
            ));
        }
        if evidence.forbidden_exact_overclaim_count != 0 || evidence.global_scan_debt_count != 0 {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
                SupportTrustRecoveryPosture::RerunCertification,
                "support trust certification rows require zero exact-overclaim and global-scan debt counters",
            ));
        }
        Ok(Self { evidence })
    }

    pub fn evidence(&self) -> &SupportCertificationRowEvidence {
        &self.evidence
    }
}
