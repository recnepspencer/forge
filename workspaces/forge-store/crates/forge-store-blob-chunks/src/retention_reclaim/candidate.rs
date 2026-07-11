use forge_store_contracts::StableDigest;
use forge_store_physical_format::PhysicalGenerationOwner;
use forge_store_physical_isolation::BlobOrphanReclaimBarrier;

use crate::{BlobChunkIdentity, BlobReachabilityReclaimRelease, S6BlobReclaimNonClaimHandoff};

use super::denial::BlobRetentionReclaimDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobRetentionOrphanSource {
    ReachabilityRelease,
    AbandonedResumeSession,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobRetentionPhysicalOrphanIdentity {
    owner: PhysicalGenerationOwner,
    durable_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BlobRetentionPhysicalOrphanClaim {
    chunk_identity: BlobChunkIdentity,
    physical_identity: BlobRetentionPhysicalOrphanIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobRetentionOrphanCandidate {
    chunk_identity: BlobChunkIdentity,
    release: BlobReachabilityReclaimRelease,
    physical_identity: BlobRetentionPhysicalOrphanIdentity,
    source: BlobRetentionOrphanSource,
    identity: StableDigest,
}

impl BlobRetentionPhysicalOrphanIdentity {
    pub(crate) fn from_resume_barrier(barrier: &BlobOrphanReclaimBarrier) -> Self {
        let identity = barrier.reclaim_identity();
        Self {
            owner: identity.physical_reference().owner(),
            durable_bytes: identity.durable_bytes(),
        }
    }

    pub const fn owner(self) -> PhysicalGenerationOwner {
        self.owner
    }

    pub const fn durable_bytes(self) -> u64 {
        self.durable_bytes
    }
}

impl BlobRetentionPhysicalOrphanClaim {
    pub(crate) fn from_admitted_s6_posture(
        release: &BlobReachabilityReclaimRelease,
        s6_posture: &S6BlobReclaimNonClaimHandoff,
    ) -> Result<Self, BlobRetentionReclaimDenial> {
        let durable_bytes = u64::from(s6_posture.region().byte_len());
        if durable_bytes == 0 {
            return Err(identity_mismatch_denial());
        }
        Ok(Self {
            chunk_identity: release.chunk_identity().clone(),
            physical_identity: BlobRetentionPhysicalOrphanIdentity {
                owner: s6_posture.region().reference().generation_owner(),
                durable_bytes,
            },
        })
    }

    pub const fn chunk_identity(&self) -> &BlobChunkIdentity {
        &self.chunk_identity
    }

    pub const fn physical_identity(&self) -> BlobRetentionPhysicalOrphanIdentity {
        self.physical_identity
    }

    pub(crate) fn matches_resume_barrier(&self, barrier: &BlobOrphanReclaimBarrier) -> bool {
        let identity = barrier.reclaim_identity();
        identity.chunk_digest() == self.chunk_identity.chunk_digest().as_str()
            && identity.durable_bytes() == self.physical_identity.durable_bytes()
            && identity.physical_reference().owner() == self.physical_identity.owner()
    }
}

impl BlobRetentionOrphanCandidate {
    pub(crate) fn from_reachability_release(
        release: BlobReachabilityReclaimRelease,
        physical_claim: BlobRetentionPhysicalOrphanClaim,
    ) -> Result<Self, BlobRetentionReclaimDenial> {
        if release.chunk_identity() != physical_claim.chunk_identity() {
            return Err(identity_mismatch_denial());
        }
        Self::from_parts(
            release,
            physical_claim.physical_identity(),
            BlobRetentionOrphanSource::ReachabilityRelease,
        )
    }

    pub(crate) fn from_abandoned_resume_barrier(
        release: BlobReachabilityReclaimRelease,
        barrier: &BlobOrphanReclaimBarrier,
    ) -> Result<Self, BlobRetentionReclaimDenial> {
        Self::from_parts(
            release,
            BlobRetentionPhysicalOrphanIdentity::from_resume_barrier(barrier),
            BlobRetentionOrphanSource::AbandonedResumeSession,
        )
    }

    fn from_parts(
        release: BlobReachabilityReclaimRelease,
        physical_identity: BlobRetentionPhysicalOrphanIdentity,
        source: BlobRetentionOrphanSource,
    ) -> Result<Self, BlobRetentionReclaimDenial> {
        if release.released_edges().is_empty() {
            return Err(identity_mismatch_denial());
        }
        let chunk_identity = release.chunk_identity().clone();
        let identity = candidate_identity(&chunk_identity, physical_identity, source);
        Ok(Self {
            chunk_identity,
            release,
            physical_identity,
            source,
            identity,
        })
    }

    pub const fn identity(&self) -> &StableDigest {
        &self.identity
    }

    pub const fn chunk_identity(&self) -> &BlobChunkIdentity {
        &self.chunk_identity
    }

    pub const fn release(&self) -> &BlobReachabilityReclaimRelease {
        &self.release
    }

    pub const fn physical_identity(&self) -> BlobRetentionPhysicalOrphanIdentity {
        self.physical_identity
    }

    pub const fn source(&self) -> BlobRetentionOrphanSource {
        self.source
    }
}

fn candidate_identity(
    chunk_identity: &BlobChunkIdentity,
    physical_identity: BlobRetentionPhysicalOrphanIdentity,
    source: BlobRetentionOrphanSource,
) -> StableDigest {
    let source = match source {
        BlobRetentionOrphanSource::ReachabilityRelease => "reachability-release",
        BlobRetentionOrphanSource::AbandonedResumeSession => "resume-abandoned",
    };
    StableDigest::new(format!(
        "s7.retention.reclaim.candidate:{}:{:?}:{}:{}",
        chunk_identity.chunk_digest().as_str(),
        physical_identity.owner(),
        physical_identity.durable_bytes(),
        source
    ))
    .expect("retention reclaim candidate identity is nonempty")
}

fn identity_mismatch_denial() -> BlobRetentionReclaimDenial {
    BlobRetentionReclaimDenial::ReclaimCandidateIdentityMismatch {
        counters: super::counters::BlobRetentionReclaimCounterSnapshot::start()
            .record_identity_mismatch_denial(),
    }
}
