use std::ops::Range;
use worth_store_physical_format::{
    decode_extent_chunk, DurableExtentManifest, ExtentChunkCoordinate, ExtentFrameDenial,
    PhysicalRecordFormatDeclaration, RecordArtifactFile, DURABLE_EXTENT_FRAME_HEADER_BYTES,
    EXTENT_CHUNK_METADATA_BYTES,
};

use super::super::{RecordReadObservation, RecordStreamFailure, RecordStreamFailureKind};

pub(in crate::physical_runtime::record_serving) struct ExtentReadState {
    artifacts: super::super::residency::record_frame_reader::RecordFrameReader<'static>,
    artifact: RecordArtifactFile,
    manifest: DurableExtentManifest,
    format: PhysicalRecordFormatDeclaration,
    next_ordinal: u32,
    logical_offset: u64,
    artifact_offset: u64,
    frame: Option<super::super::residency::frame_loading::LoadedPhysicalFrame>,
    payload: Range<usize>,
    payload_offset: usize,
}

impl ExtentReadState {
    pub(in crate::physical_runtime::record_serving) fn new(
        artifacts: super::super::residency::record_frame_reader::RecordFrameReader<'static>,
        artifact: RecordArtifactFile,
        manifest: DurableExtentManifest,
        format: PhysicalRecordFormatDeclaration,
    ) -> Self {
        Self {
            artifacts,
            artifact,
            manifest,
            format,
            next_ordinal: 1,
            logical_offset: 0,
            artifact_offset: 0,
            frame: None,
            payload: 0..0,
            payload_offset: 0,
        }
    }

    pub(in crate::physical_runtime::record_serving) fn read_next(
        &mut self,
        target: &mut [u8],
        observation: &mut RecordReadObservation,
    ) -> Result<usize, RecordStreamFailure> {
        if self.payload_offset == self.payload.len() {
            if self.logical_offset == self.manifest.logical_bytes() {
                return Ok(0);
            }
            self.load_chunk(observation)?;
        }
        let count = target.len().min(self.payload.len() - self.payload_offset);
        let start = self.payload.start + self.payload_offset;
        let frame = self.frame.as_ref().expect("loaded extent frame is present");
        frame.copy_range_into(start..start + count, &mut target[..count]);
        self.payload_offset += count;
        Ok(count)
    }

    fn load_chunk(
        &mut self,
        observation: &mut RecordReadObservation,
    ) -> Result<(), RecordStreamFailure> {
        let completed = self.delivered_bytes();
        self.frame = None;
        let chunk_bytes = (self.manifest.logical_bytes() - self.logical_offset)
            .min(u64::from(self.manifest.chunk_payload_capacity()))
            as usize;
        let frame_bytes =
            DURABLE_EXTENT_FRAME_HEADER_BYTES + EXTENT_CHUNK_METADATA_BYTES + chunk_bytes;
        let frame = self
            .artifacts
            .load_exact(self.artifact, self.artifact_offset, frame_bytes as u32)
            .map_err(|failure| {
                observation.observe_physical_work(failure.work_trace());
                RecordStreamFailure::during_read(
                    RecordStreamFailureKind::ArtifactDamaged,
                    completed,
                )
            })?;
        observation.observe_physical_work(frame.work_trace());
        observation.observe_transfer(frame.len());
        let coordinate = ExtentChunkCoordinate::new(
            self.manifest.record(),
            self.manifest.extent_cell(),
            self.manifest.logical_bytes(),
            self.logical_offset,
            self.next_ordinal,
        )
        .ok_or_else(|| {
            RecordStreamFailure::during_read(RecordStreamFailureKind::ArtifactDamaged, completed)
        })?;
        let decoded = decode_extent_chunk(&frame, coordinate);
        match &decoded {
            Ok(_) => {
                observation.check_generation(true);
            }
            Err(ExtentFrameDenial::GenerationMismatch) => {
                observation.check_generation(false);
            }
            Err(_) => {}
        }
        let (chunk, format) = decoded.map_err(|denial| {
            let kind = if denial == ExtentFrameDenial::GenerationMismatch {
                RecordStreamFailureKind::StalePlacement
            } else {
                RecordStreamFailureKind::ArtifactDamaged
            };
            RecordStreamFailure::during_read(kind, completed)
        })?;
        if format != self.format || chunk.len() != chunk_bytes {
            return Err(RecordStreamFailure::during_read(
                RecordStreamFailureKind::FormatMismatch,
                completed,
            ));
        }
        self.payload = DURABLE_EXTENT_FRAME_HEADER_BYTES + EXTENT_CHUNK_METADATA_BYTES
            ..DURABLE_EXTENT_FRAME_HEADER_BYTES + EXTENT_CHUNK_METADATA_BYTES + chunk_bytes;
        self.payload_offset = 0;
        self.frame = Some(frame);
        self.artifact_offset += frame_bytes as u64;
        self.logical_offset += chunk_bytes as u64;
        if self.logical_offset < self.manifest.logical_bytes() {
            self.next_ordinal = self.next_ordinal.checked_add(1).ok_or_else(|| {
                RecordStreamFailure::during_read(
                    RecordStreamFailureKind::ArtifactDamaged,
                    completed,
                )
            })?;
        }
        Ok(())
    }

    fn delivered_bytes(&self) -> u64 {
        self.logical_offset
            .saturating_sub(self.payload.len() as u64)
            .saturating_add(self.payload_offset as u64)
    }
}
