use crate::backend::records::EmbeddedCheckpointRecord;
use crate::failure::StoreError;
use crate::snapshot::{
    PublishedSnapshotHandle, SnapshotCaptureRequest, SnapshotId, SnapshotImageBundle,
    SnapshotReadRequest, SnapshotReadResult, SnapshotRestoreOutcome, SnapshotRestorePlan,
    SnapshotRestoreRequest,
};

use super::{dispatch_mut, dispatch_ref, StoreBackend};

impl StoreBackend {
    pub fn persist_embedded_checkpoint(
        &mut self,
        record: EmbeddedCheckpointRecord,
    ) -> Result<EmbeddedCheckpointRecord, StoreError> {
        dispatch_mut!(self, |backend| backend.persist_embedded_checkpoint(record))
    }
    pub fn fetch_embedded_checkpoint(
        &self,
        checkpoint_id: &str,
    ) -> Result<EmbeddedCheckpointRecord, StoreError> {
        dispatch_ref!(self, |backend| backend
            .fetch_embedded_checkpoint(checkpoint_id))
    }
    pub fn capture_snapshot(
        &mut self,
        request: SnapshotCaptureRequest,
    ) -> Result<PublishedSnapshotHandle, StoreError> {
        dispatch_mut!(self, |backend| backend.capture_snapshot(request))
    }
    pub fn read_snapshot(
        &self,
        request: SnapshotReadRequest,
    ) -> Result<SnapshotReadResult, StoreError> {
        dispatch_ref!(self, |backend| backend.read_snapshot(request))
    }
    pub fn plan_snapshot_restore(
        &self,
        request: SnapshotRestoreRequest,
    ) -> Result<SnapshotRestorePlan, StoreError> {
        dispatch_ref!(self, |backend| backend.plan_snapshot_restore(request))
    }
    pub fn execute_snapshot_restore(
        &self,
        plan: SnapshotRestorePlan,
    ) -> Result<SnapshotRestoreOutcome, StoreError> {
        dispatch_ref!(self, |backend| backend.execute_snapshot_restore(plan))
    }
    pub fn rebuild_snapshot(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotImageBundle, StoreError> {
        dispatch_ref!(self, |backend| backend.rebuild_snapshot(snapshot_id))
    }
    #[cfg(test)]
    pub fn remove_snapshot_image_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        dispatch_mut!(self, |backend| backend
            .remove_snapshot_image_for_test(snapshot_id))
    }
    #[cfg(test)]
    pub fn remove_snapshot_basis_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        dispatch_mut!(self, |backend| backend
            .remove_snapshot_basis_for_test(snapshot_id))
    }
    #[cfg(test)]
    pub fn clear_branch_heads_for_test(&mut self) -> Result<(), StoreError> {
        dispatch_mut!(self, |backend| backend.clear_branch_heads_for_test())
    }
    #[cfg(test)]
    pub fn corrupt_snapshot_basis_digest_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        dispatch_mut!(self, |backend| backend
            .corrupt_snapshot_basis_digest_for_test(snapshot_id))
    }
}
