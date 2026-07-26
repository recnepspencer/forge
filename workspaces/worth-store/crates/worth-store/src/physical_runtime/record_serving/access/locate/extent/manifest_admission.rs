use worth_store_physical_format::{
    DurableExtentManifest, DurableExtentRecordPlacement, RecordArtifactFile,
    DURABLE_EXTENT_FRAME_HEADER_BYTES, EXTENT_CHUNK_METADATA_BYTES,
};

use super::super::failure_classification::read_failure;
use super::super::PhysicalRecordReader;
use crate::physical_runtime::record_serving::{
    residency::record_frame_reader::RecordFrameReader, PhysicalRecordId, RecordReadDenial,
    RecordReadObservation, StalePhysicalRecordPlacement,
};

pub(super) struct AdmittedExtentManifest {
    pub(super) artifact: RecordArtifactFile,
    pub(super) manifest: DurableExtentManifest,
}

pub(super) struct ExtentManifestAdmission<'admission, 'media> {
    pub(super) reader: &'admission PhysicalRecordReader,
    pub(super) record: PhysicalRecordId,
    pub(super) placement: DurableExtentRecordPlacement,
    pub(super) observation: &'admission mut RecordReadObservation,
    pub(super) allocation: &'admission worth_store_buffer_pool::OperationAllocationGrant,
    pub(super) artifacts: &'admission RecordFrameReader<'media>,
}

pub(super) fn admit_extent_manifest(
    mut admission: ExtentManifestAdmission<'_, '_>,
) -> Result<AdmittedExtentManifest, RecordReadDenial> {
    let manifest = admission.load_manifest()?;
    let artifact = RecordArtifactFile::Extent {
        extent: admission.placement.extent().get(),
        generation: admission.placement.extent_generation(),
    };
    admission.require_complete_extent(artifact, &manifest)?;
    Ok(AdmittedExtentManifest { artifact, manifest })
}

impl ExtentManifestAdmission<'_, '_> {
    fn load_manifest(&mut self) -> Result<DurableExtentManifest, RecordReadDenial> {
        let bytes = self
            .artifacts
            .load_bounded(
                self.allocation,
                RecordArtifactFile::ExtentManifest {
                    extent: self.placement.extent().get(),
                    generation: self.placement.extent_generation(),
                },
                self.reader
                    .access
                    .transfer_limit()
                    .get()
                    .min(self.reader.format.declaration().page_size().bytes()),
            )
            .map_err(|failure| {
                self.observation.observe_physical_work(failure.work_trace());
                read_failure(failure)
            })?;
        self.observation.observe_physical_work(bytes.work_trace());
        self.observation.observe_manifest_block(bytes.len());
        self.observation.observe_transfer(bytes.len());
        match self.project_manifest(&bytes) {
            Ok(manifest) => Ok(manifest),
            Err(denial) => {
                bytes.reject_projection_failure();
                Err(denial)
            }
        }
    }

    fn project_manifest(
        &mut self,
        bytes: &[u8],
    ) -> Result<DurableExtentManifest, RecordReadDenial> {
        let (manifest, format) =
            DurableExtentManifest::decode(bytes).map_err(|_| RecordReadDenial::ArtifactDamaged)?;
        if format != self.reader.format.declaration()
            || manifest.record() != self.record.persisted()
            || manifest.logical_bytes() != self.placement.payload_bytes()
        {
            return Err(RecordReadDenial::FormatMismatch);
        }
        if !self
            .observation
            .check_generation(manifest.extent_cell() == self.placement.extent_cell())
        {
            return Err(RecordReadDenial::StalePlacement(
                StalePhysicalRecordPlacement::ExtentMembership,
            ));
        }
        Ok(manifest)
    }

    fn require_complete_extent(
        &mut self,
        artifact: RecordArtifactFile,
        manifest: &DurableExtentManifest,
    ) -> Result<(), RecordReadDenial> {
        let expected = manifest.logical_bytes()
            + u64::from(manifest.chunk_count())
                * (DURABLE_EXTENT_FRAME_HEADER_BYTES + EXTENT_CHUNK_METADATA_BYTES) as u64;
        let extent_length = self.artifacts.file_length(artifact).map_err(|failure| {
            self.observation.observe_physical_work(failure.work_trace());
            read_failure(failure)
        })?;
        self.observation
            .observe_physical_work(extent_length.work_trace());
        if extent_length.bytes() != expected {
            extent_length.reject_structural_damage();
            return Err(RecordReadDenial::ArtifactDamaged);
        }
        Ok(())
    }
}
