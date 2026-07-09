use crate::{
    failure::StoreError,
    snapshot::{
        PublishedSnapshotHandle, SnapshotCaptureRequest, SnapshotId, SnapshotImageBundle,
        SnapshotReadRequest, SnapshotReadResult, SnapshotRestoreOutcome, SnapshotRestorePlan,
        SnapshotRestoreRequest,
    },
};
use worth_relational::facade::history::CommitId;

use super::WORTHStore;

impl WORTHStore {
    pub fn capture_snapshot(
        &mut self,
        request: SnapshotCaptureRequest,
    ) -> Result<PublishedSnapshotHandle, StoreError> {
        self.backend.capture_snapshot(request)
    }

    pub fn read_snapshot(
        &self,
        request: SnapshotReadRequest,
    ) -> Result<SnapshotReadResult, StoreError> {
        self.backend.read_snapshot(request)
    }

    pub fn plan_snapshot_restore(
        &self,
        request: SnapshotRestoreRequest,
    ) -> Result<SnapshotRestorePlan, StoreError> {
        self.backend.plan_snapshot_restore(request)
    }

    pub fn execute_snapshot_restore(
        &self,
        plan: SnapshotRestorePlan,
    ) -> Result<SnapshotRestoreOutcome, StoreError> {
        self.backend.execute_snapshot_restore(plan)
    }

    pub fn restore_snapshot(
        &self,
        snapshot_id: SnapshotId,
        target_commit_id: CommitId,
    ) -> Result<SnapshotRestoreOutcome, StoreError> {
        let plan =
            self.plan_snapshot_restore(SnapshotRestoreRequest::new(snapshot_id, target_commit_id))?;
        self.execute_snapshot_restore(plan)
    }

    pub fn rebuild_snapshot(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotImageBundle, StoreError> {
        self.backend.rebuild_snapshot(snapshot_id)
    }

    #[cfg(test)]
    pub(crate) fn remove_snapshot_image_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        self.backend.remove_snapshot_image_for_test(snapshot_id)
    }

    #[cfg(test)]
    pub(crate) fn remove_snapshot_basis_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        self.backend.remove_snapshot_basis_for_test(snapshot_id)
    }

    #[cfg(test)]
    pub(crate) fn clear_branch_heads_for_test(&mut self) -> Result<(), StoreError> {
        self.backend.clear_branch_heads_for_test()
    }

    #[cfg(test)]
    pub(crate) fn corrupt_snapshot_basis_digest_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        self.backend
            .corrupt_snapshot_basis_digest_for_test(snapshot_id)
    }
}
