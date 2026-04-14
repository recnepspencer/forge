use crate::authority::CanonicalDigest;
use crate::evidence::CanonicalizationMetrics;
use forge_relational::facade::{
    history::{BranchHead, BranchId, CommitId},
    replay::CanonicalCommitEnvelope,
};

#[derive(Debug, Clone)]
pub struct RawRuntimeCommitEnvelope {
    envelope: CanonicalCommitEnvelope,
}

impl RawRuntimeCommitEnvelope {
    pub fn new(envelope: CanonicalCommitEnvelope) -> Self {
        Self { envelope }
    }

    pub fn envelope(&self) -> &CanonicalCommitEnvelope {
        &self.envelope
    }

    pub fn into_inner(self) -> CanonicalCommitEnvelope {
        self.envelope
    }
}

#[derive(Debug, Clone)]
pub struct CanonicalizedCommitEnvelope {
    envelope: CanonicalCommitEnvelope,
    digest: CanonicalDigest,
    canonicalization_version: u32,
    metrics: CanonicalizationMetrics,
}

impl CanonicalizedCommitEnvelope {
    pub(crate) fn new(
        envelope: CanonicalCommitEnvelope,
        digest: CanonicalDigest,
        canonicalization_version: u32,
        metrics: CanonicalizationMetrics,
    ) -> Self {
        Self {
            envelope,
            digest,
            canonicalization_version,
            metrics,
        }
    }

    pub fn envelope(&self) -> &CanonicalCommitEnvelope {
        &self.envelope
    }

    pub fn digest(&self) -> &CanonicalDigest {
        &self.digest
    }

    pub fn canonicalization_version(&self) -> u32 {
        self.canonicalization_version
    }

    pub fn metrics(&self) -> &CanonicalizationMetrics {
        &self.metrics
    }
}

#[derive(Debug, Clone)]
pub struct VerifiedAuthoritativeAppend {
    envelope: CanonicalCommitEnvelope,
    digest: CanonicalDigest,
    canonicalization_version: u32,
}

impl VerifiedAuthoritativeAppend {
    pub(crate) fn new(
        envelope: CanonicalCommitEnvelope,
        digest: CanonicalDigest,
        canonicalization_version: u32,
    ) -> Self {
        Self {
            envelope,
            digest,
            canonicalization_version,
        }
    }

    pub fn envelope(&self) -> &CanonicalCommitEnvelope {
        &self.envelope
    }

    pub fn digest(&self) -> &CanonicalDigest {
        &self.digest
    }

    pub fn canonicalization_version(&self) -> u32 {
        self.canonicalization_version
    }
}

#[derive(Debug, Clone)]
pub struct PersistedAuthoritativeCommit {
    envelope: CanonicalCommitEnvelope,
    digest: CanonicalDigest,
    canonicalization_version: u32,
    commit_sequence: u64,
}

impl PersistedAuthoritativeCommit {
    pub(crate) fn new(
        envelope: CanonicalCommitEnvelope,
        digest: CanonicalDigest,
        canonicalization_version: u32,
        commit_sequence: u64,
    ) -> Self {
        Self {
            envelope,
            digest,
            canonicalization_version,
            commit_sequence,
        }
    }

    pub fn envelope(&self) -> &CanonicalCommitEnvelope {
        &self.envelope
    }

    pub fn digest(&self) -> &CanonicalDigest {
        &self.digest
    }

    pub fn canonicalization_version(&self) -> u32 {
        self.canonicalization_version
    }

    pub fn commit_sequence(&self) -> u64 {
        self.commit_sequence
    }
}

#[derive(Debug, Clone)]
pub struct FetchedAuthoritativeCommit {
    envelope: CanonicalCommitEnvelope,
    digest: CanonicalDigest,
    canonicalization_version: u32,
    commit_sequence: u64,
}

impl FetchedAuthoritativeCommit {
    pub(crate) fn new(
        envelope: CanonicalCommitEnvelope,
        digest: CanonicalDigest,
        canonicalization_version: u32,
        commit_sequence: u64,
    ) -> Self {
        Self {
            envelope,
            digest,
            canonicalization_version,
            commit_sequence,
        }
    }

    pub fn envelope(&self) -> &CanonicalCommitEnvelope {
        &self.envelope
    }

    pub fn digest(&self) -> &CanonicalDigest {
        &self.digest
    }

    pub fn canonicalization_version(&self) -> u32 {
        self.canonicalization_version
    }

    pub fn commit_sequence(&self) -> u64 {
        self.commit_sequence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoritativeBranchHeadRecord {
    branch_id: BranchId,
    head: Option<forge_relational::facade::history::CommitReference>,
    head_update_sequence: u64,
}

impl AuthoritativeBranchHeadRecord {
    pub(crate) fn new(
        branch_id: BranchId,
        head: Option<forge_relational::facade::history::CommitReference>,
        head_update_sequence: u64,
    ) -> Self {
        Self {
            branch_id,
            head,
            head_update_sequence,
        }
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn head(&self) -> Option<&forge_relational::facade::history::CommitReference> {
        self.head.as_ref()
    }

    pub fn head_update_sequence(&self) -> u64 {
        self.head_update_sequence
    }

    pub fn branch_head(&self) -> BranchHead {
        BranchHead {
            branch_id: self.branch_id.clone(),
            head: self.head.clone(),
        }
    }

    pub fn head_commit_id(&self) -> Option<CommitId> {
        self.head.as_ref().map(|head| head.commit_id)
    }
}
