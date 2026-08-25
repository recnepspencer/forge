use crate::history::data::CommitId;
use crate::storage::overlay::PartitionState;
use std::sync::Arc;

/// Recovery-lane image source for one exact immutable branch root.
///
/// The durability owner performs wire conversion. This owner-side carrier
/// keeps branch truth distinct from the runtime's main-derived storage mirror.
#[derive(Debug, Clone)]
pub(crate) struct RelationalBranchRootCheckpoint {
    commit_id: CommitId,
    partitions: Vec<PartitionState>,
    schema_authority: Arc<super::RelationalBranchRootSchemaAuthority>,
}

impl RelationalBranchRootCheckpoint {
    pub(crate) fn new(
        commit_id: CommitId,
        partitions: Vec<PartitionState>,
        schema_authority: Arc<super::RelationalBranchRootSchemaAuthority>,
    ) -> Self {
        Self {
            commit_id,
            partitions,
            schema_authority,
        }
    }

    pub(crate) const fn commit_id(&self) -> CommitId {
        self.commit_id
    }

    pub(crate) fn partitions(&self) -> &[PartitionState] {
        &self.partitions
    }

    pub(crate) fn schema_authority(&self) -> &super::RelationalBranchRootSchemaAuthority {
        &self.schema_authority
    }
}
