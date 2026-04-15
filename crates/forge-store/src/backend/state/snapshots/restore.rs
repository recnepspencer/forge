use forge_relational::facade::history::CommitId;

use crate::{
    backend::{
        records::{SnapshotBasisRecord, StoreState},
        state::snapshots::image::snapshot_image_record_count,
    },
    failure::{StoreError, StoreErrorKind},
    snapshot::{
        SnapshotId, SnapshotImageBundle, SnapshotRestoreOutcome, SnapshotRestorePlan,
        SnapshotRestoreRequest,
    },
};

impl StoreState {
    pub fn plan_snapshot_restore(
        &self,
        request: SnapshotRestoreRequest,
    ) -> Result<SnapshotRestorePlan, StoreError> {
        let basis = self.snapshot_basis(request.snapshot_id)?;
        self.require_snapshot_restore_target(&basis, request.target_commit_id)?;
        let tail_commit_ids = self
            .snapshot_history_range(request.target_commit_id)?
            .into_iter()
            .filter(|commit_id| !basis.snapshot_history_range.contains(commit_id))
            .collect();
        Ok(SnapshotRestorePlan::new(
            request.snapshot_id,
            basis.snapshot_branch_id,
            request.target_commit_id,
            tail_commit_ids,
        ))
    }

    pub fn execute_snapshot_restore(
        &self,
        plan: SnapshotRestorePlan,
    ) -> Result<(SnapshotRestoreOutcome, usize, usize), StoreError> {
        let (image, tail_commit_count) =
            self.build_snapshot_tail_image(plan.snapshot_id(), plan.target_commit_id())?;
        Ok((
            SnapshotRestoreOutcome {
                snapshot_id: plan.snapshot_id(),
                restored_branch_id: plan.snapshot_branch_id().clone(),
                restored_frontier_commit_id: plan.target_commit_id(),
                restored_image: image,
            },
            tail_commit_count,
            tail_commit_count,
        ))
    }

    pub fn rebuild_snapshot(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<(SnapshotImageBundle, usize), StoreError> {
        let basis = self.snapshot_basis(snapshot_id)?;
        let image = self
            .build_snapshot_image(&basis.snapshot_branch_id, basis.snapshot_frontier_commit_id)?;
        let digest = crate::snapshot::stable_snapshot_digest(&image);
        if digest != basis.snapshot_image_digest {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotRebuildParityViolation,
                format!(
                    "rebuilt snapshot {} image digest {} did not match basis {}",
                    snapshot_id.0, digest, basis.snapshot_image_digest
                ),
            ));
        }
        let record_count = snapshot_image_record_count(&image);
        Ok((image, record_count))
    }

    pub(super) fn require_snapshot_restore_target(
        &self,
        basis: &SnapshotBasisRecord,
        target_commit_id: CommitId,
    ) -> Result<(), StoreError> {
        let target = self.commit_record(target_commit_id).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::SnapshotRestoreTargetIllegal,
                format!("target commit {} does not exist", target_commit_id.0),
            )
        })?;
        if target.envelope.branch_context != basis.snapshot_branch_id {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotRestoreTargetIllegal,
                format!(
                    "target commit {} is on branch `{}` not snapshot branch `{}`",
                    target_commit_id.0,
                    target.envelope.branch_context.0,
                    basis.snapshot_branch_id.0
                ),
            ));
        }
        if target_commit_id == basis.snapshot_frontier_commit_id {
            return Ok(());
        }
        if basis.snapshot_history_range.contains(&target_commit_id) {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotRestoreTargetIllegal,
                format!(
                    "target commit {} predates snapshot frontier {}",
                    target_commit_id.0, basis.snapshot_frontier_commit_id.0
                ),
            ));
        }
        if !self.is_descendant_of(target_commit_id, basis.snapshot_frontier_commit_id)? {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotTailRangeGap,
                format!(
                    "target commit {} is not a descendant of snapshot frontier {}",
                    target_commit_id.0, basis.snapshot_frontier_commit_id.0
                ),
            ));
        }
        Ok(())
    }
}
