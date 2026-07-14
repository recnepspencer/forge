use crate::retention_reclaim::candidate::BlobRetentionOrphanCandidate;
use crate::BlobReclaimPolicyEvidence;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobRetentionReclaimAdmission {
    candidate: BlobRetentionOrphanCandidate,
    reclaim_policy_evidence: BlobReclaimPolicyEvidence,
}

impl BlobRetentionReclaimAdmission {
    pub const fn candidate(&self) -> &BlobRetentionOrphanCandidate {
        &self.candidate
    }

    pub const fn reclaim_policy_evidence(&self) -> &BlobReclaimPolicyEvidence {
        &self.reclaim_policy_evidence
    }

    pub(crate) fn into_parts(self) -> (BlobRetentionOrphanCandidate, BlobReclaimPolicyEvidence) {
        (self.candidate, self.reclaim_policy_evidence)
    }

    pub(crate) fn construct(
        candidate: BlobRetentionOrphanCandidate,
        reclaim_policy_evidence: BlobReclaimPolicyEvidence,
    ) -> Self {
        Self {
            candidate,
            reclaim_policy_evidence,
        }
    }
}
