use crate::{
    failure::{StoreError, StoreErrorKind},
    snapshot::{
        stable_snapshot_basis_authority_digest, stable_snapshot_digest, SNAPSHOT_BASIS_VERSION,
        SNAPSHOT_FAMILY_VERSION, SNAPSHOT_IMAGE_FORMAT_VERSION,
    },
};

use crate::backend::records::StoreState;

impl StoreState {
    pub fn verify_snapshot_record(&self, snapshot_id: u64) -> Result<(), StoreError> {
        let basis = self
            .snapshot_basis_records
            .get(&snapshot_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::SnapshotIntegrityFailure,
                    format!(
                        "missing snapshot basis for {snapshot_id} during targeted verification"
                    ),
                )
            })?;
        if snapshot_id != basis.snapshot_id.0 {
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
        if basis.snapshot_family_version != SNAPSHOT_FAMILY_VERSION
            || basis.snapshot_basis_version != SNAPSHOT_BASIS_VERSION
            || basis.snapshot_image_format_version != SNAPSHOT_IMAGE_FORMAT_VERSION
        {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotFamilyVersionUnsupported,
                format!(
                    "snapshot {} version tuple ({}, {}, {}) is unsupported",
                    snapshot_id,
                    basis.snapshot_family_version,
                    basis.snapshot_basis_version,
                    basis.snapshot_image_format_version
                ),
            ));
        }
        let expected_basis_digest = stable_snapshot_basis_authority_digest(
            &basis.snapshot_branch_id,
            basis.snapshot_frontier_commit_id,
            &basis.snapshot_history_range,
            basis.snapshot_canonicalization_version,
        );
        if expected_basis_digest != basis.snapshot_authority_digest {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotIntegrityFailure,
                format!(
                    "snapshot {} authority digest {} did not match expected {}",
                    snapshot_id, basis.snapshot_authority_digest, expected_basis_digest
                ),
            ));
        }
        let image = self
            .snapshot_image_records
            .get(&snapshot_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::SnapshotPublicationStateGap,
                    format!("snapshot {} basis exists without image", snapshot_id),
                )
            })?;
        if image.image.snapshot_family_version() != basis.snapshot_family_version
            || image.image.snapshot_basis_version() != basis.snapshot_basis_version
            || image.image.snapshot_image_format_version() != basis.snapshot_image_format_version
        {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotFamilyVersionUnsupported,
                format!(
                    "snapshot {} image version tuple did not match basis tuple",
                    snapshot_id
                ),
            ));
        }
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
        StoreState::from_authoritative_export_bundle(image.image.authoritative_export().clone())
            .map_err(|error| {
                StoreError::new(
                    StoreErrorKind::SnapshotIntegrityFailure,
                    format!(
                        "snapshot {} image did not rebuild into a valid authoritative state: {}",
                        snapshot_id,
                        error.message()
                    ),
                )
            })?;
        Ok(())
    }

    pub fn verify_snapshot_image_record(&self, snapshot_id: u64) -> Result<(), StoreError> {
        let image = self
            .snapshot_image_records
            .get(&snapshot_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::SnapshotIntegrityFailure,
                    format!(
                        "missing snapshot image for {snapshot_id} during targeted verification"
                    ),
                )
            })?;
        if snapshot_id != image.snapshot_id.0 {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotIntegrityFailure,
                "snapshot image key does not match stored snapshot id",
            ));
        }
        if !self.snapshot_basis_records.contains_key(&snapshot_id) {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotPublicationStateGap,
                format!("snapshot {} image exists without basis", snapshot_id),
            ));
        }
        Ok(())
    }

    pub fn verify_snapshot_record_family(&self) -> Result<(), StoreError> {
        for snapshot_id in self.snapshot_basis_records.keys() {
            self.verify_snapshot_record(*snapshot_id)?;
        }

        for snapshot_id in self.snapshot_image_records.keys() {
            self.verify_snapshot_image_record(*snapshot_id)?;
        }
        Ok(())
    }
}
