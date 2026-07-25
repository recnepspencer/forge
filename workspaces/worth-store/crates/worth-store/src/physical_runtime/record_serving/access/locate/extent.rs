use worth_store_physical_format::{
    DurableExtentManifest, DurableExtentRecordPlacement, RecordArtifactFile,
    DURABLE_EXTENT_FRAME_HEADER_BYTES, EXTENT_CHUNK_METADATA_BYTES,
};

use super::failure_classification::read_failure;
use super::{PhysicalRecordReader, ReadPlacement, RecordReadSession};
use crate::physical_runtime::record_serving::{
    access::extent_read_session::ExtentReadState,
    residency::record_frame_reader::RecordFrameReader, PhysicalRecordId, RecordReadDenial,
    RecordReadObservation, StalePhysicalRecordPlacement,
};

impl PhysicalRecordReader {
    pub(super) fn open_extent(
        &self,
        record: PhysicalRecordId,
        placement: DurableExtentRecordPlacement,
        observation: &mut RecordReadObservation,
        allocation: worth_store_buffer_pool::OperationAllocationGrant,
    ) -> Result<RecordReadSession, RecordReadDenial> {
        let runtime = self
            .runtime
            .upgrade()
            .ok_or(RecordReadDenial::ServingRequiresInspection)?;
        let health_permit = runtime
            .health
            .permit()
            .map_err(|_| RecordReadDenial::ServingRequiresInspection)?;
        let artifacts = RecordFrameReader::serving(self.frame_ports.clone(), self.source.clone());
        let bytes = artifacts
            .load_bounded(
                RecordArtifactFile::ExtentManifest {
                    extent: placement.extent().get(),
                    generation: placement.extent_generation(),
                },
                self.access
                    .transfer_limit()
                    .get()
                    .min(self.format.declaration().page_size().bytes()),
            )
            .map_err(|failure| {
                observation.observe_physical_work(failure.work_trace());
                read_failure(failure)
            })?;
        observation.observe_physical_work(bytes.work_trace());
        observation.observe_manifest_block(bytes.len());
        observation.observe_transfer(bytes.len());
        let (manifest, format) =
            DurableExtentManifest::decode(&bytes).map_err(|_| RecordReadDenial::ArtifactDamaged)?;
        if format != self.format.declaration()
            || manifest.record() != record.persisted()
            || manifest.logical_bytes() != placement.payload_bytes()
        {
            return Err(RecordReadDenial::FormatMismatch);
        }
        if !observation.check_generation(manifest.extent_cell() == placement.extent_cell()) {
            return Err(RecordReadDenial::StalePlacement(
                StalePhysicalRecordPlacement::ExtentMembership,
            ));
        }
        let expected = manifest.logical_bytes()
            + u64::from(manifest.chunk_count())
                * (DURABLE_EXTENT_FRAME_HEADER_BYTES + EXTENT_CHUNK_METADATA_BYTES) as u64;
        let artifact = RecordArtifactFile::Extent {
            extent: placement.extent().get(),
            generation: placement.extent_generation(),
        };
        let extent_length = artifacts.file_length(artifact).map_err(|failure| {
            observation.observe_physical_work(failure.work_trace());
            read_failure(failure)
        })?;
        observation.observe_physical_work(extent_length.work_trace());
        if extent_length.bytes() != expected {
            return Err(RecordReadDenial::ArtifactDamaged);
        }
        observation.touched_extents = 1;
        Ok(RecordReadSession {
            placement: ReadPlacement::Extent(Box::new(ExtentReadState::new(
                artifacts,
                artifact,
                manifest,
                self.format.declaration(),
            ))),
            observation: *observation,
            runtime: self.runtime.clone(),
            health_permit,
            _lifecycle: self.lifecycle.read_session(),
            _allocation: allocation,
        })
    }
}
