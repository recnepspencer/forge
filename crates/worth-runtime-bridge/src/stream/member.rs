use std::sync::Arc;

use crate::identity::{BridgeIdentity, StreamMemberIdentityTag};
use crate::input::envelope::{
    BridgeCommittedPatchEnvelope, TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity,
};
use crate::routing::canonicalization::digest_string;
use crate::snapshot::TruthSnapshotIdentity;

type StreamMemberIdentity = BridgeIdentity<StreamMemberIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalStreamMember {
    stream_member_identity: StreamMemberIdentity,
    source_branch: TruthBranchIdentity,
    source_commit: TruthCommitIdentity,
    source_patch: TruthPatchIdentity,
    source_snapshot: TruthSnapshotIdentity,
    committed_envelope: BridgeCommittedPatchEnvelope,
    digest: Arc<str>,
}

impl CanonicalStreamMember {
    pub(crate) fn from_envelope(envelope: BridgeCommittedPatchEnvelope) -> Self {
        let basis = format!(
            "canonical-stream-member|branch={}|commit={}|patch={}|snapshot={}|envelope-digest={}",
            envelope.branch_identity().as_str(),
            envelope.commit_identity().as_str(),
            envelope.patch_identity().as_str(),
            envelope.snapshot_identity().as_str(),
            envelope.digest().as_str(),
        );
        let digest = digest_string("canonical-stream-member", &basis);
        Self {
            stream_member_identity: StreamMemberIdentity::admit_bridge_owned(digest.clone()),
            source_branch: envelope.branch_identity().clone(),
            source_commit: envelope.commit_identity().clone(),
            source_patch: envelope.patch_identity().clone(),
            source_snapshot: envelope.snapshot_identity().clone(),
            committed_envelope: envelope,
            digest,
        }
    }

    pub fn stream_member_identity(&self) -> &str {
        self.stream_member_identity.as_str()
    }

    pub fn source_branch(&self) -> &TruthBranchIdentity {
        &self.source_branch
    }

    pub fn source_commit(&self) -> &TruthCommitIdentity {
        &self.source_commit
    }

    pub fn source_patch(&self) -> &TruthPatchIdentity {
        &self.source_patch
    }

    pub fn source_snapshot(&self) -> &TruthSnapshotIdentity {
        &self.source_snapshot
    }

    pub fn committed_envelope(&self) -> &BridgeCommittedPatchEnvelope {
        &self.committed_envelope
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
