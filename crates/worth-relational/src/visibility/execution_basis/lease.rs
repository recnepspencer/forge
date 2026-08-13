use std::sync::Arc;

use crate::runtime::ExecutionBasisRegistry;
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};

use super::RelationalExecutionBasisCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalExecutionBasisIdentity {
    runtime_instance_id: u64,
    branch_id: crate::history::data::BranchId,
    snapshot_id: SnapshotId,
    lease_ordinal: u64,
}

impl RelationalExecutionBasisIdentity {
    pub fn runtime_instance_id(&self) -> u64 {
        self.runtime_instance_id
    }

    pub fn snapshot_id(&self) -> SnapshotId {
        self.snapshot_id
    }

    pub fn branch_id(&self) -> &crate::history::data::BranchId {
        &self.branch_id
    }

    pub fn lease_ordinal(&self) -> u64 {
        self.lease_ordinal
    }
}

pub struct RelationalExecutionBasisLease {
    identity: RelationalExecutionBasisIdentity,
    handle: SnapshotHandle,
    current_at_admission: bool,
    registry: Option<Arc<ExecutionBasisRegistry>>,
    counters: RelationalExecutionBasisCounters,
}

impl RelationalExecutionBasisLease {
    pub(crate) fn new(
        handle: SnapshotHandle,
        current_at_admission: bool,
        lease_ordinal: u64,
        registry: Arc<ExecutionBasisRegistry>,
        counters: RelationalExecutionBasisCounters,
    ) -> Self {
        Self {
            identity: RelationalExecutionBasisIdentity {
                runtime_instance_id: handle.runtime_instance_id,
                branch_id: handle.branch_id.clone(),
                snapshot_id: handle.snapshot_id,
                lease_ordinal,
            },
            handle,
            current_at_admission,
            registry: Some(registry),
            counters,
        }
    }

    pub fn identity(&self) -> &RelationalExecutionBasisIdentity {
        &self.identity
    }

    pub fn snapshot_handle(&self) -> &SnapshotHandle {
        &self.handle
    }

    pub fn version_id(&self) -> crate::identity::data::VersionId {
        self.handle.version_id
    }

    pub fn read_policy(&self) -> SnapshotReadPolicy {
        self.handle.read_policy
    }

    pub fn was_current_at_admission(&self) -> bool {
        self.current_at_admission
    }

    pub fn counters(&self) -> &RelationalExecutionBasisCounters {
        &self.counters
    }

    pub fn is_live(&self) -> bool {
        self.registry.as_ref().is_some_and(|registry| {
            registry.retains(
                self.identity.snapshot_id,
                &self.identity.branch_id,
                self.handle.version_id,
                self.handle.read_policy,
                self.identity.lease_ordinal,
            )
        })
    }

    pub fn release(mut self) -> RelationalExecutionBasisReleaseReceipt {
        let released = self.registry.take().is_some_and(|registry| {
            registry.release(self.identity.snapshot_id, self.identity.lease_ordinal)
        });
        RelationalExecutionBasisReleaseReceipt {
            identity: self.identity.clone(),
            released,
        }
    }
}

impl Drop for RelationalExecutionBasisLease {
    fn drop(&mut self) {
        if let Some(registry) = self.registry.take() {
            let _ = registry.release(self.identity.snapshot_id, self.identity.lease_ordinal);
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalExecutionBasisReleaseReceipt {
    identity: RelationalExecutionBasisIdentity,
    released: bool,
}

impl RelationalExecutionBasisReleaseReceipt {
    pub fn identity(&self) -> &RelationalExecutionBasisIdentity {
        &self.identity
    }

    pub fn released(&self) -> bool {
        self.released
    }
}
