#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobOrphanReclaimDenial {
    MissingSessionDigest,
    MissingChunkDigest,
    EmptyPartialChunk,
    AlreadyReachable,
    MissingS7ReclaimBarrier,
    OrphanReclaimIdentityMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobOrphanReclaimCounterSnapshot {
    barriers: u64,
    proofs: u64,
    denials: u64,
}

use crate::{CurrentGenerationPhysicalReference, ReclaimEligibilityProof};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPartialChunkOrphan {
    session_digest: String,
    chunk_ordinal: u64,
    chunk_digest: String,
    durable_bytes: u64,
    physical_reference: CurrentGenerationPhysicalReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobOrphanReclaimIdentity {
    session_digest: String,
    chunk_ordinal: u64,
    chunk_digest: String,
    durable_bytes: u64,
    physical_reference: CurrentGenerationPhysicalReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobOrphanReclaimBarrier {
    orphan: BlobPartialChunkOrphan,
    counters: BlobOrphanReclaimCounterSnapshot,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobOrphanReclaimCoverage {
    barrier: BlobOrphanReclaimBarrier,
    identity: BlobOrphanReclaimIdentity,
    reclaim_root_epoch: u64,
    reclaim_candidate_ranges: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobOrphanReclaimProof {
    barrier: BlobOrphanReclaimBarrier,
    identity: BlobOrphanReclaimIdentity,
    reclaim_root_epoch: u64,
    reclaim_candidate_ranges: u64,
    counters: BlobOrphanReclaimCounterSnapshot,
}

impl BlobPartialChunkOrphan {
    pub fn unreached(
        session_digest: impl Into<String>,
        chunk_ordinal: u64,
        chunk_digest: impl Into<String>,
        durable_bytes: u64,
        physical_reference: CurrentGenerationPhysicalReference,
    ) -> Result<Self, BlobOrphanReclaimDenial> {
        let session_digest = session_digest.into();
        let chunk_digest = chunk_digest.into();
        if session_digest.is_empty() {
            return Err(BlobOrphanReclaimDenial::MissingSessionDigest);
        }
        if chunk_digest.is_empty() {
            return Err(BlobOrphanReclaimDenial::MissingChunkDigest);
        }
        if durable_bytes == 0 {
            return Err(BlobOrphanReclaimDenial::EmptyPartialChunk);
        }
        Ok(Self {
            session_digest,
            chunk_ordinal,
            chunk_digest,
            durable_bytes,
            physical_reference,
        })
    }

    pub fn session_digest(&self) -> &str {
        &self.session_digest
    }

    pub const fn chunk_ordinal(&self) -> u64 {
        self.chunk_ordinal
    }

    pub fn chunk_digest(&self) -> &str {
        &self.chunk_digest
    }

    pub const fn durable_bytes(&self) -> u64 {
        self.durable_bytes
    }

    pub const fn physical_reference(&self) -> CurrentGenerationPhysicalReference {
        self.physical_reference
    }

    pub fn reclaim_identity(&self) -> BlobOrphanReclaimIdentity {
        BlobOrphanReclaimIdentity {
            session_digest: self.session_digest.clone(),
            chunk_ordinal: self.chunk_ordinal,
            chunk_digest: self.chunk_digest.clone(),
            durable_bytes: self.durable_bytes,
            physical_reference: self.physical_reference,
        }
    }
}

impl BlobOrphanReclaimBarrier {
    pub fn from_unreached_orphan(
        orphan: BlobPartialChunkOrphan,
        reachable: bool,
    ) -> Result<Self, BlobOrphanReclaimDenial> {
        if reachable {
            return Err(BlobOrphanReclaimDenial::AlreadyReachable);
        }
        Ok(Self {
            orphan,
            counters: BlobOrphanReclaimCounterSnapshot::start().with_barrier(),
        })
    }

    pub const fn orphan(&self) -> &BlobPartialChunkOrphan {
        &self.orphan
    }

    pub fn reclaim_identity(&self) -> BlobOrphanReclaimIdentity {
        self.orphan.reclaim_identity()
    }

    pub const fn counters(&self) -> BlobOrphanReclaimCounterSnapshot {
        self.counters
    }

    pub fn admit_reclaim_coverage(
        self,
        reclaim_eligibility: ReclaimEligibilityProof,
    ) -> Result<BlobOrphanReclaimCoverage, BlobOrphanReclaimDenial> {
        let identity = self.reclaim_identity();
        let receipt = reclaim_eligibility
            .admit_reachability_removal()
            .map_err(|_| BlobOrphanReclaimDenial::MissingS7ReclaimBarrier)?;
        if !receipt.covers_reclaimed_identity(identity.physical_reference()) {
            return Err(BlobOrphanReclaimDenial::MissingS7ReclaimBarrier);
        }
        Ok(BlobOrphanReclaimCoverage {
            reclaim_root_epoch: receipt.evidence().root_epoch().get(),
            reclaim_candidate_ranges: receipt.evidence().candidates().candidate_ranges().len()
                as u64,
            barrier: self,
            identity,
        })
    }
}

impl BlobOrphanReclaimProof {
    pub fn from_reclaim_coverage(coverage: BlobOrphanReclaimCoverage) -> Self {
        Self {
            reclaim_root_epoch: coverage.reclaim_root_epoch,
            reclaim_candidate_ranges: coverage.reclaim_candidate_ranges,
            counters: coverage.barrier.counters().with_proof(),
            barrier: coverage.barrier,
            identity: coverage.identity,
        }
    }

    pub const fn barrier(&self) -> &BlobOrphanReclaimBarrier {
        &self.barrier
    }

    pub const fn identity(&self) -> &BlobOrphanReclaimIdentity {
        &self.identity
    }

    pub const fn reclaim_root_epoch(&self) -> u64 {
        self.reclaim_root_epoch
    }

    pub const fn reclaim_candidate_ranges(&self) -> u64 {
        self.reclaim_candidate_ranges
    }

    pub const fn counters(&self) -> BlobOrphanReclaimCounterSnapshot {
        self.counters
    }
}

impl BlobOrphanReclaimIdentity {
    pub fn session_digest(&self) -> &str {
        &self.session_digest
    }

    pub const fn chunk_ordinal(&self) -> u64 {
        self.chunk_ordinal
    }

    pub fn chunk_digest(&self) -> &str {
        &self.chunk_digest
    }

    pub const fn durable_bytes(&self) -> u64 {
        self.durable_bytes
    }

    pub const fn physical_reference(&self) -> CurrentGenerationPhysicalReference {
        self.physical_reference
    }
}

impl BlobOrphanReclaimCounterSnapshot {
    pub const fn start() -> Self {
        Self {
            barriers: 0,
            proofs: 0,
            denials: 0,
        }
    }

    pub const fn with_barrier(self) -> Self {
        Self {
            barriers: self.barriers + 1,
            ..self
        }
    }

    pub const fn with_proof(self) -> Self {
        Self {
            proofs: self.proofs + 1,
            ..self
        }
    }

    pub const fn denied(self) -> Self {
        Self {
            denials: self.denials + 1,
            ..self
        }
    }

    pub const fn barriers(self) -> u64 {
        self.barriers
    }

    pub const fn proofs(self) -> u64 {
        self.proofs
    }

    pub const fn denials(self) -> u64 {
        self.denials
    }
}
