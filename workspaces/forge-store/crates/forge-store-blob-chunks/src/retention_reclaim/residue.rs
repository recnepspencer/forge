use forge_store_contracts::StableDigest;

use super::{
    candidate::BlobRetentionOrphanCandidate, counters::BlobRetentionReclaimCounterSnapshot,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlobReclaimResidueKind {
    AbandonedResumeSessionBytes,
    FailedReclaimBytes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobLocalizedReclaimResidue {
    identity: StableDigest,
    kind: BlobReclaimResidueKind,
    durable_bytes: u64,
    counters: BlobRetentionReclaimCounterSnapshot,
}

impl BlobLocalizedReclaimResidue {
    pub(crate) fn from_candidate(
        candidate: &BlobRetentionOrphanCandidate,
        kind: BlobReclaimResidueKind,
        counters: BlobRetentionReclaimCounterSnapshot,
    ) -> Self {
        let identity = StableDigest::new(format!(
            "s7.retention.residue:{}:{:?}:{}",
            candidate.identity().as_str(),
            kind,
            candidate.physical_identity().durable_bytes()
        ))
        .expect("retention residue identity is nonempty");
        Self {
            identity,
            kind,
            durable_bytes: candidate.physical_identity().durable_bytes(),
            counters,
        }
    }

    pub const fn identity(&self) -> &StableDigest {
        &self.identity
    }

    pub const fn kind(&self) -> BlobReclaimResidueKind {
        self.kind
    }

    pub const fn durable_bytes(&self) -> u64 {
        self.durable_bytes
    }

    pub const fn counters(&self) -> BlobRetentionReclaimCounterSnapshot {
        self.counters
    }

    pub const fn can_satisfy_blob_content(&self) -> bool {
        false
    }

    pub const fn can_satisfy_reachability(&self) -> bool {
        false
    }
}
