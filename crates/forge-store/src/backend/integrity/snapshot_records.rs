use crate::{
    failure::{StoreError, StoreErrorKind},
    snapshot::stable_snapshot_digest,
};

use crate::backend::records::StoreState;

impl StoreState {
    pub fn verify_snapshot_record_family(&self) -> Result<(), StoreError> {
        for (snapshot_id, basis) in &self.snapshot_basis_records {
            if *snapshot_id != basis.snapshot_id.0 {
                return Err(StoreError::new(
                    StoreErrorKind::SnapshotIntegrityFailure,
                    "snapshot basis key does not match stored snapshot id",
                ));
            }
            if basis.snapshot_canonicalization_version != self.canonicalization_version {
                return Err(StoreError::new(
                    StoreErrorKind::SnapshotFamilyVersionUnsupported,
                    format!(
                        "snapshot {} canonicalization version {} does not match store {}",
                        snapshot_id,
                        basis.snapshot_canonicalization_version,
                        self.canonicalization_version
                    ),
                ));
            }
            let image = self
                .snapshot_image_records
                .get(snapshot_id)
                .ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::SnapshotPublicationStateGap,
                        format!("snapshot {} basis exists without image", snapshot_id),
                    )
                })?;
            let digest = stable_snapshot_digest(&image.image);
            if digest != basis.snapshot_image_digest {
                return Err(StoreError::new(
                    StoreErrorKind::SnapshotDigestMismatch,
                    format!(
                        "snapshot {} image digest {} did not match basis {}",
                        snapshot_id, digest, basis.snapshot_image_digest
                    ),
                ));
            }
        }

        for (snapshot_id, image) in &self.snapshot_image_records {
            if *snapshot_id != image.snapshot_id.0 {
                return Err(StoreError::new(
                    StoreErrorKind::SnapshotIntegrityFailure,
                    "snapshot image key does not match stored snapshot id",
                ));
            }
            if !self.snapshot_basis_records.contains_key(snapshot_id) {
                return Err(StoreError::new(
                    StoreErrorKind::SnapshotPublicationStateGap,
                    format!("snapshot {} image exists without basis", snapshot_id),
                ));
            }
        }
        Ok(())
    }
}
