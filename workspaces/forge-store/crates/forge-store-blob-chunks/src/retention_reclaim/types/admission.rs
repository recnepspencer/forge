use crate::retention_reclaim::candidate::BlobRetentionOrphanCandidate;
use crate::S6BlobReclaimNonClaimHandoff;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobRetentionReclaimAdmission {
    candidate: BlobRetentionOrphanCandidate,
    s6_posture: S6BlobReclaimNonClaimHandoff,
}

impl BlobRetentionReclaimAdmission {
    pub const fn candidate(&self) -> &BlobRetentionOrphanCandidate {
        &self.candidate
    }

    pub const fn s6_posture(&self) -> S6BlobReclaimNonClaimHandoff {
        self.s6_posture
    }

    pub(crate) fn into_parts(self) -> (BlobRetentionOrphanCandidate, S6BlobReclaimNonClaimHandoff) {
        (self.candidate, self.s6_posture)
    }

    pub(crate) fn construct(
        candidate: BlobRetentionOrphanCandidate,
        s6_posture: S6BlobReclaimNonClaimHandoff,
    ) -> Self {
        Self {
            candidate,
            s6_posture,
        }
    }
}
