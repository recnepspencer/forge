use forge_store_contracts::StableDigest;

use crate::{BlobChunkIdentity, BlobReachabilityReclaimRelease, S6BlobReclaimNonClaimHandoff};

use super::{
    candidate::BlobRetentionOrphanCandidate, counters::BlobRetentionReclaimCounterSnapshot,
    residue::BlobLocalizedReclaimResidue,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobRetentionReclaimPermit {
    identity: StableDigest,
    candidate: BlobRetentionOrphanCandidate,
    s6_posture: S6BlobReclaimNonClaimHandoff,
    residue: BlobLocalizedReclaimResidue,
    counters: BlobRetentionReclaimCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobRetentionReclaimReceipt {
    chunk_identity: BlobChunkIdentity,
    permit_identity: StableDigest,
    counters: BlobRetentionReclaimCounterSnapshot,
}

impl BlobRetentionReclaimPermit {
    pub(crate) fn from_candidate(
        candidate: BlobRetentionOrphanCandidate,
        s6_posture: S6BlobReclaimNonClaimHandoff,
        residue: BlobLocalizedReclaimResidue,
        counters: BlobRetentionReclaimCounterSnapshot,
    ) -> Self {
        let identity = StableDigest::new(format!(
            "s7.retention.reclaim.permit:{}:{:?}",
            candidate.identity().as_str(),
            s6_posture.security_scope()
        ))
        .expect("retention reclaim permit identity is nonempty");
        Self {
            identity,
            candidate,
            s6_posture,
            residue,
            counters,
        }
    }

    pub const fn identity(&self) -> &StableDigest {
        &self.identity
    }

    pub const fn candidate(&self) -> &BlobRetentionOrphanCandidate {
        &self.candidate
    }

    pub const fn chunk_identity(&self) -> &BlobChunkIdentity {
        self.candidate.chunk_identity()
    }

    pub const fn reclaim_release(&self) -> &BlobReachabilityReclaimRelease {
        self.candidate.release()
    }

    pub const fn s6_posture(&self) -> &S6BlobReclaimNonClaimHandoff {
        &self.s6_posture
    }

    pub const fn residue_report(&self) -> &BlobLocalizedReclaimResidue {
        &self.residue
    }

    pub const fn counters(&self) -> BlobRetentionReclaimCounterSnapshot {
        self.counters
    }

    pub fn retention_receipt(&self) -> BlobRetentionReclaimReceipt {
        BlobRetentionReclaimReceipt {
            chunk_identity: self.chunk_identity().clone(),
            permit_identity: self.identity.clone(),
            counters: self.counters,
        }
    }
}

impl BlobRetentionReclaimReceipt {
    pub const fn chunk_identity(&self) -> &BlobChunkIdentity {
        &self.chunk_identity
    }

    pub const fn permit_identity(&self) -> &StableDigest {
        &self.permit_identity
    }

    pub const fn counters(&self) -> BlobRetentionReclaimCounterSnapshot {
        self.counters
    }
}
