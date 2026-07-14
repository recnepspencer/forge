use crate::{
    backend::{records::StoreState, state::snapshots::image::snapshot_image_record_count},
    failure::StoreError,
    snapshot::{SnapshotReadMode, SnapshotReadRequest, SnapshotReadResult},
};

impl StoreState {
    pub fn read_snapshot(
        &self,
        request: SnapshotReadRequest,
    ) -> Result<(SnapshotReadResult, usize, usize, usize), StoreError> {
        let basis = self.snapshot_basis(request.snapshot_id)?;
        let persisted_snapshot_image = self.snapshot_image(request.snapshot_id)?;
        let (image, tail_commit_count, tail_replay_count) = match request.mode {
            SnapshotReadMode::PureSnapshot => {
                if request.target_commit_id != basis.snapshot_frontier_commit_id {
                    return Err(crate::StoreError::new(
                        crate::StoreErrorKind::SnapshotReadBasisMismatch,
                        format!(
                            "pure snapshot read requires target frontier {}; got {}",
                            basis.snapshot_frontier_commit_id.0, request.target_commit_id.0
                        ),
                    ));
                }
                (persisted_snapshot_image.clone(), 0, 0)
            }
            SnapshotReadMode::SnapshotPlusTail => self
                .build_snapshot_tail_image(request.snapshot_id, request.target_commit_id)
                .map(|(image, tail_commit_count)| (image, tail_commit_count, tail_commit_count))?,
        };
        let record_count = snapshot_image_record_count(&persisted_snapshot_image);

        Ok((
            SnapshotReadResult {
                snapshot_id: request.snapshot_id,
                target_commit_id: request.target_commit_id,
                mode: request.mode,
                image,
            },
            record_count,
            tail_commit_count,
            tail_replay_count,
        ))
    }
}
