use crate::{
    backend::records::StoreState,
    failure::{StoreError, StoreErrorKind},
    publication::{
        admit_local_snapshot_basis_source, admit_local_snapshot_image_source,
        classify_snapshot_publication, PublicationClassification,
    },
    snapshot::{stable_snapshot_digest, SnapshotId, SnapshotImageBundle},
};

impl StoreState {
    pub fn snapshot_image(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotImageBundle, StoreError> {
        let basis = self.snapshot_basis_records.get(&snapshot_id.0).cloned();
        let record = self.snapshot_image_records.get(&snapshot_id.0).cloned();
        let publication = classify_snapshot_publication(
            crate::media::DurableMediaReport::new(
                crate::media::DurableBackendFamily::InMemory,
                crate::media::DurabilityBarrierClass::FileContentDurable,
                crate::media::DurabilityBarrierClass::FileAndRequiredMetadataDurable,
                crate::media::DurabilityBarrierClass::FileContentDurable,
            ),
            basis.clone(),
            record.clone(),
        )?;
        match publication.classification() {
            PublicationClassification::RequireRebuild => {
                return Err(StoreError::new(
                    StoreErrorKind::SnapshotPublicationStateGap,
                    format!(
                        "snapshot {} image missing while basis exists",
                        snapshot_id.0
                    ),
                ));
            }
            PublicationClassification::RequireQuarantine => {
                return Err(StoreError::new(
                    StoreErrorKind::SnapshotPublicationStateGap,
                    format!(
                        "snapshot {} image exists without an admitted basis",
                        snapshot_id.0
                    ),
                ));
            }
            PublicationClassification::DiscardUnpublished => {
                return Err(StoreError::new(
                    StoreErrorKind::SnapshotBasisUnsupported,
                    format!("snapshot {} basis not found", snapshot_id.0),
                ));
            }
            PublicationClassification::FinishPublication
            | PublicationClassification::RetainTrusted => {}
        }
        let basis = admit_local_snapshot_basis_source(
            basis.expect("published snapshot should include basis record"),
        )?
        .into_inner();
        let record = admit_local_snapshot_image_source(
            record.expect("published snapshot should include image record"),
        )?
        .into_inner();
        let digest = stable_snapshot_digest(&record.image);
        if digest != basis.snapshot_image_digest {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotDigestMismatch,
                format!(
                    "snapshot {} image digest {} did not match basis {}",
                    snapshot_id.0, digest, basis.snapshot_image_digest
                ),
            ));
        }
        Ok(record.image.clone())
    }

    #[cfg(test)]
    pub fn remove_snapshot_image(&mut self, snapshot_id: SnapshotId) {
        self.snapshot_image_records.remove(&snapshot_id.0);
    }
}
