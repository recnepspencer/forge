use worth_store_physical_isolation::{
    BlobOrphanReclaimBarrier, BlobOrphanReclaimProof, BlobPartialChunkOrphan,
};

use super::{
    BlobResumeCheckpoint, BlobResumeCheckpointStateKind, BlobResumeCounterSnapshot,
    BlobResumeDenial,
};

#[derive(Debug, PartialEq, Eq)]
pub struct BlobResumeSessionAbandoned {
    checkpoint: BlobResumeCheckpoint,
    reclaim_barrier: BlobOrphanReclaimBarrier,
    counters: BlobResumeCounterSnapshot,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobResumeSessionReclaimed {
    abandoned: BlobResumeSessionAbandoned,
    reclaim_proof: BlobOrphanReclaimProof,
    counters: BlobResumeCounterSnapshot,
}

impl BlobResumeSessionAbandoned {
    pub fn abandon(checkpoint: BlobResumeCheckpoint) -> Result<Self, BlobResumeDenial> {
        let leaf = checkpoint
            .latest_leaf()
            .ok_or(BlobResumeDenial::MissingS7ReclaimProof)?;
        let physical_reference = checkpoint
            .physical_reference()
            .ok_or(BlobResumeDenial::MissingS7ReclaimProof)?;
        let orphan = BlobPartialChunkOrphan::unreached(
            checkpoint.session_id().as_str(),
            leaf.ordinal().get(),
            leaf.identity().chunk_digest().as_str(),
            leaf.byte_range().len(),
            physical_reference,
        )
        .map_err(|_| BlobResumeDenial::MissingS7ReclaimProof)?;
        let reclaim_barrier = BlobOrphanReclaimBarrier::from_unreached_orphan(orphan, false)
            .map_err(|_| BlobResumeDenial::MissingS7ReclaimProof)?;
        Ok(Self {
            counters: checkpoint.counters().abandoned(),
            checkpoint: checkpoint.with_state(BlobResumeCheckpointStateKind::SessionAbandoned),
            reclaim_barrier,
        })
    }

    pub const fn reclaim_barrier(&self) -> &BlobOrphanReclaimBarrier {
        &self.reclaim_barrier
    }

    pub const fn counters(&self) -> BlobResumeCounterSnapshot {
        self.counters
    }

    pub fn into_checkpoint(self) -> BlobResumeCheckpoint {
        self.checkpoint
            .with_state(BlobResumeCheckpointStateKind::SessionAbandoned)
    }
}

impl BlobResumeSessionReclaimed {
    pub fn reclaim(
        abandoned: BlobResumeSessionAbandoned,
        reclaim_proof: BlobOrphanReclaimProof,
    ) -> Result<Self, BlobResumeDenial> {
        let expected = abandoned.reclaim_barrier().reclaim_identity();
        let actual = reclaim_proof.identity();
        if expected.session_digest() != actual.session_digest()
            || expected.chunk_ordinal() != actual.chunk_ordinal()
            || expected.chunk_digest() != actual.chunk_digest()
            || expected.durable_bytes() != actual.durable_bytes()
            || expected.physical_reference() != actual.physical_reference()
        {
            return Err(BlobResumeDenial::MissingS7ReclaimProof);
        }
        Ok(Self {
            counters: abandoned.counters().reclaimed(),
            abandoned,
            reclaim_proof,
        })
    }

    pub const fn reclaim_proof(&self) -> &BlobOrphanReclaimProof {
        &self.reclaim_proof
    }

    pub const fn counters(&self) -> BlobResumeCounterSnapshot {
        self.counters
    }

    pub fn into_checkpoint(self) -> BlobResumeCheckpoint {
        self.abandoned
            .checkpoint
            .with_state(BlobResumeCheckpointStateKind::SessionReclaimed)
    }
}
