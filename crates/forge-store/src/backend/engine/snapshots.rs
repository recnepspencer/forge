use crate::failure::{StoreError, StoreErrorKind};
use crate::snapshot::{
    PublishedSnapshotHandle, SnapshotCaptureRequest, SnapshotId, SnapshotImageBundle,
    SnapshotReadRequest, SnapshotReadResult, SnapshotRestoreOutcome, SnapshotRestorePlan,
    SnapshotRestoreRequest,
};

use super::{core::verify_durable_barrier, StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn capture_snapshot(
        &mut self,
        request: SnapshotCaptureRequest,
    ) -> Result<PublishedSnapshotHandle, StoreError> {
        let (applied, handle, record_count, byte_count) =
            self.state.apply_snapshot_capture_in_place(request)?;
        if let Err(error) = self.state.verify_applied_snapshot_capture(&applied) {
            self.state.rollback_snapshot_capture(applied);
            return Err(error);
        }
        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state.rollback_snapshot_capture(applied);
                return Err(error);
            }
        };
        if let Err(error) = verify_durable_barrier(&mut self.counters, &report) {
            self.state.rollback_snapshot_capture(applied);
            return Err(error);
        }
        self.counters.record_state_delta_apply(2, 2);
        self.counters.record_snapshot_capture(record_count, byte_count);
        Ok(handle)
    }

    pub fn read_snapshot(
        &self,
        request: SnapshotReadRequest,
    ) -> Result<SnapshotReadResult, StoreError> {
        match self.state.read_snapshot(request) {
            Ok((result, record_count, tail_commit_count, tail_replay_count)) => {
                self.counters.record_snapshot_read(
                    record_count,
                    tail_commit_count,
                    tail_replay_count,
                );
                Ok(result)
            }
            Err(error) => {
                record_snapshot_error_counters(&self.counters, &error);
                Err(error)
            }
        }
    }

    pub fn plan_snapshot_restore(
        &self,
        request: SnapshotRestoreRequest,
    ) -> Result<SnapshotRestorePlan, StoreError> {
        match self.state.plan_snapshot_restore(request) {
            Ok(plan) => Ok(plan),
            Err(error) => {
                if matches!(
                    error.kind(),
                    StoreErrorKind::SnapshotReadBasisMismatch
                        | StoreErrorKind::SnapshotRestoreTargetIllegal
                        | StoreErrorKind::SnapshotTailRangeGap
                ) {
                    self.counters.record_snapshot_basis_mismatch();
                }
                Err(error)
            }
        }
    }

    pub fn execute_snapshot_restore(
        &self,
        plan: SnapshotRestorePlan,
    ) -> Result<SnapshotRestoreOutcome, StoreError> {
        match self.state.execute_snapshot_restore(plan) {
            Ok((outcome, tail_commit_count, tail_replay_count)) => {
                self.counters
                    .record_snapshot_restore(tail_commit_count, tail_replay_count);
                Ok(outcome)
            }
            Err(error) => {
                if matches!(
                    error.kind(),
                    StoreErrorKind::SnapshotReadBasisMismatch
                        | StoreErrorKind::SnapshotRestoreTargetIllegal
                        | StoreErrorKind::SnapshotTailRangeGap
                ) {
                    self.counters.record_snapshot_basis_mismatch();
                }
                Err(error)
            }
        }
    }

    pub fn rebuild_snapshot(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotImageBundle, StoreError> {
        match self.state.rebuild_snapshot(snapshot_id) {
            Ok((image, record_count)) => {
                self.counters.record_snapshot_rebuild(record_count);
                Ok(image)
            }
            Err(error) => {
                if matches!(
                    error.kind(),
                    StoreErrorKind::SnapshotDigestMismatch
                        | StoreErrorKind::SnapshotIntegrityFailure
                        | StoreErrorKind::SnapshotPublicationStateGap
                        | StoreErrorKind::SnapshotRebuildParityViolation
                ) {
                    self.counters.record_snapshot_integrity_failure();
                }
                Err(error)
            }
        }
    }

    #[cfg(test)]
    pub fn remove_snapshot_image_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        let mut next = self.state.clone();
        next.remove_snapshot_image(snapshot_id);
        let _ = self.persistence.persist_state(&next)?;
        self.state = next;
        Ok(())
    }

    #[cfg(test)]
    pub fn remove_snapshot_basis_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        let mut next = self.state.clone();
        next.remove_snapshot_basis(snapshot_id);
        let _ = self.persistence.persist_state(&next)?;
        self.state = next;
        Ok(())
    }

    #[cfg(test)]
    pub fn corrupt_snapshot_basis_digest_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        let mut next = self.state.clone();
        let basis = next
            .snapshot_basis_records
            .get_mut(&snapshot_id.0)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::SnapshotBasisUnsupported,
                    format!("snapshot {} basis not found", snapshot_id.0),
                )
            })?;
        basis.snapshot_image_digest.push_str("-corrupt");
        let _ = self.persistence.persist_state(&next)?;
        self.state = next;
        Ok(())
    }

    #[cfg(test)]
    pub fn clear_branch_heads_for_test(&mut self) -> Result<(), StoreError> {
        let mut next = self.state.clone();
        next.branch_head_records.clear();
        let _ = self.persistence.persist_state(&next)?;
        self.state = next;
        Ok(())
    }
}

fn record_snapshot_error_counters(counters: &crate::evidence::StoreCounters, error: &StoreError) {
    if matches!(
        error.kind(),
        StoreErrorKind::SnapshotReadBasisMismatch
            | StoreErrorKind::SnapshotRestoreTargetIllegal
            | StoreErrorKind::SnapshotTailRangeGap
    ) {
        counters.record_snapshot_basis_mismatch();
    }
    if matches!(
        error.kind(),
        StoreErrorKind::SnapshotDigestMismatch
            | StoreErrorKind::SnapshotIntegrityFailure
            | StoreErrorKind::SnapshotPublicationStateGap
    ) {
        counters.record_snapshot_integrity_failure();
    }
}
