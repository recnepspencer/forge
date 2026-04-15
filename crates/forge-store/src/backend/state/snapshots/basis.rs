use crate::{
    backend::{
        records::{SnapshotBasisRecord, SnapshotImageRecord, StoreState},
        state::snapshots::image::snapshot_image_record_count,
    },
    failure::StoreError,
    snapshot::{
        stable_snapshot_basis_authority_digest, stable_snapshot_digest, PublishedSnapshotHandle,
        SnapshotCaptureRequest, SnapshotId, SNAPSHOT_BASIS_VERSION, SNAPSHOT_FAMILY_VERSION,
        SNAPSHOT_IMAGE_FORMAT_VERSION,
    },
};

#[derive(Debug)]
pub(crate) struct AppliedSnapshotCapture {
    snapshot_id: SnapshotId,
    previous_next_snapshot_id: u64,
}

impl StoreState {
    pub fn apply_snapshot_capture_in_place(
        &mut self,
        request: SnapshotCaptureRequest,
    ) -> Result<
        (
            AppliedSnapshotCapture,
            PublishedSnapshotHandle,
            usize,
            usize,
        ),
        StoreError,
    > {
        let image = self.build_snapshot_image(
            &request.snapshot_branch_id,
            request.snapshot_frontier_commit_id,
        )?;
        let snapshot_id = SnapshotId(self.next_snapshot_id);
        let history_range = self.snapshot_history_range(request.snapshot_frontier_commit_id)?;
        let basis = SnapshotBasisRecord {
            snapshot_id,
            snapshot_family_version: SNAPSHOT_FAMILY_VERSION,
            snapshot_basis_version: SNAPSHOT_BASIS_VERSION,
            snapshot_image_format_version: SNAPSHOT_IMAGE_FORMAT_VERSION,
            snapshot_branch_id: request.snapshot_branch_id.clone(),
            snapshot_frontier_commit_id: request.snapshot_frontier_commit_id,
            snapshot_history_range: history_range.clone(),
            snapshot_canonicalization_version: self.canonicalization_version,
            snapshot_authority_digest: stable_snapshot_basis_authority_digest(
                &request.snapshot_branch_id,
                request.snapshot_frontier_commit_id,
                &history_range,
                self.canonicalization_version,
            ),
            snapshot_image_digest: stable_snapshot_digest(&image),
        };
        let image_record = SnapshotImageRecord {
            snapshot_id,
            image: image.clone(),
        };

        let previous_next_snapshot_id = self.next_snapshot_id;
        self.next_snapshot_id += 1;
        self.snapshot_basis_records
            .insert(snapshot_id.0, basis.clone());
        self.snapshot_image_records
            .insert(snapshot_id.0, image_record);

        let handle = PublishedSnapshotHandle {
            snapshot_id,
            snapshot_family_version: basis.snapshot_family_version,
            snapshot_basis_version: basis.snapshot_basis_version,
            snapshot_image_format_version: basis.snapshot_image_format_version,
            snapshot_branch_id: basis.snapshot_branch_id,
            snapshot_frontier_commit_id: basis.snapshot_frontier_commit_id,
            snapshot_authority_digest: basis.snapshot_authority_digest,
            snapshot_image_digest: basis.snapshot_image_digest,
        };
        let record_count = snapshot_image_record_count(&image);
        let byte_count = image.canonical_json().len();
        Ok((
            AppliedSnapshotCapture {
                snapshot_id,
                previous_next_snapshot_id,
            },
            handle,
            record_count,
            byte_count,
        ))
    }

    pub fn rollback_snapshot_capture(&mut self, applied: AppliedSnapshotCapture) {
        self.next_snapshot_id = applied.previous_next_snapshot_id;
        self.snapshot_basis_records.remove(&applied.snapshot_id.0);
        self.snapshot_image_records.remove(&applied.snapshot_id.0);
    }

    pub fn verify_applied_snapshot_capture(
        &self,
        applied: &AppliedSnapshotCapture,
    ) -> Result<(), StoreError> {
        if !self
            .snapshot_basis_records
            .contains_key(&applied.snapshot_id.0)
        {
            return Err(StoreError::backend_integrity(format!(
                "snapshot {} basis missing after in-place capture",
                applied.snapshot_id.0
            )));
        }
        if !self
            .snapshot_image_records
            .contains_key(&applied.snapshot_id.0)
        {
            return Err(StoreError::backend_integrity(format!(
                "snapshot {} image missing after in-place capture",
                applied.snapshot_id.0
            )));
        }
        self.verify_snapshot_record(applied.snapshot_id.0)?;
        self.verify_snapshot_image_record(applied.snapshot_id.0)
    }

    pub fn snapshot_basis(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotBasisRecord, StoreError> {
        self.snapshot_basis_records
            .get(&snapshot_id.0)
            .cloned()
            .ok_or_else(|| {
                crate::StoreError::new(
                    crate::StoreErrorKind::SnapshotBasisUnsupported,
                    format!("snapshot basis {} not found", snapshot_id.0),
                )
            })
    }

    #[cfg(test)]
    pub fn remove_snapshot_basis(&mut self, snapshot_id: SnapshotId) {
        self.snapshot_basis_records.remove(&snapshot_id.0);
    }
}
