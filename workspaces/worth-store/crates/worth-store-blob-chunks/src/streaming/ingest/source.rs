use worth_store_physical_backend::{
    BlobBackendChunkWriteObservation, BlobBackendChunkWriteObservationKind,
};
use worth_store_physical_format::PhysicalChunkPayloadIntegrityWitness;

use crate::{BlobChunkOrdinal, BlobStreamingIngestDenial, BlobStreamingWindow};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobStreamingSourceFrame {
    bytes: Vec<u8>,
}

impl BlobStreamingSourceFrame {
    pub fn from_bounded_bytes(
        bytes: impl Into<Vec<u8>>,
        window: BlobStreamingWindow,
    ) -> Result<Self, BlobStreamingIngestDenial> {
        let bytes = bytes.into();
        if bytes.is_empty() {
            return Err(BlobStreamingIngestDenial::EmptySourceFrame);
        }
        let frame_bytes = bytes.len() as u64;
        if frame_bytes > window.max_resident_bytes() {
            return Err(
                BlobStreamingIngestDenial::SourceWindowExceedsResidentEnvelope {
                    window_bytes: frame_bytes,
                    envelope_bytes: window.max_resident_bytes(),
                },
            );
        }
        Ok(Self { bytes })
    }

    pub(crate) fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

pub trait BlobStreamingChunkWriter {
    fn write_streaming_chunk(
        &mut self,
        ordinal: BlobChunkOrdinal,
        bytes: &[u8],
    ) -> Result<BlobStreamingWrittenChunk, BlobStreamingIngestDenial>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobStreamingWrittenChunk {
    payload: PhysicalChunkPayloadIntegrityWitness,
    backend_write: BlobBackendChunkWriteObservation,
}

impl BlobStreamingWrittenChunk {
    pub fn from_store_chunk_write(
        payload: PhysicalChunkPayloadIntegrityWitness,
        backend_write: BlobBackendChunkWriteObservation,
    ) -> Result<Self, BlobStreamingIngestDenial> {
        if backend_write.kind() == BlobBackendChunkWriteObservationKind::ScalarFramedRecordApi {
            return Err(BlobStreamingIngestDenial::ScalarBackendCertificationRejected);
        }
        if backend_write.bytes_written() != payload.bytes_checked() {
            return Err(BlobStreamingIngestDenial::BackendWriteBytesMismatch {
                expected: payload.bytes_checked(),
                actual: backend_write.bytes_written(),
            });
        }
        Ok(Self {
            payload,
            backend_write,
        })
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        PhysicalChunkPayloadIntegrityWitness,
        BlobBackendChunkWriteObservation,
    ) {
        (self.payload, self.backend_write)
    }

    pub(crate) fn payload_bytes(&self) -> &[u8] {
        self.payload.payload_bytes()
    }
}
