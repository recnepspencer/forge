use super::*;
use crate::input::envelope::{TruthBranchIdentity, TruthCommitIdentity};

pub trait CommittedPatchSource: Send + Sync + 'static {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<crate::input::envelope::BridgeCommittedPatchEnvelope, RelationalBridgeSourceError>;
}

pub trait SnapshotReadSource: Send + Sync + 'static {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError>;
}

pub trait SnapshotReaderPool: Send + Sync + 'static {
    fn acquire(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError>;

    fn release(&self, reader: Box<dyn TruthSnapshotReader>);
}

pub trait TruthBranchHeadSource: Send + Sync + 'static {
    fn load_branch_head_patch(
        &self,
        branch_identity: &TruthBranchIdentity,
    ) -> Result<crate::input::envelope::BridgeCommittedPatchEnvelope, RelationalBridgeSourceError>;
}

pub trait RelationalBridgeSource:
    CommittedPatchSource + SnapshotReadSource + TruthBranchHeadSource
{
}

impl<T> RelationalBridgeSource for T where
    T: CommittedPatchSource + SnapshotReadSource + TruthBranchHeadSource
{
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalCommittedPatchRequest {
    commit_identity: TruthCommitIdentity,
}

impl RelationalCommittedPatchRequest {
    pub fn new(commit_identity: TruthCommitIdentity) -> Self {
        Self { commit_identity }
    }

    pub fn commit_identity(&self) -> &TruthCommitIdentity {
        &self.commit_identity
    }
}
