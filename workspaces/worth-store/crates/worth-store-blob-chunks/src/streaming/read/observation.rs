use worth_store_physical_format::PhysicalChunkPayloadIntegrityWitness;

use crate::{
    BlobChunkByteRange, BlobChunkOrdinal, BlobStreamingReadDenial, BlobStreamingReadWindow,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobStreamingReadObservedChunk {
    ordinal: BlobChunkOrdinal,
    byte_range: BlobChunkByteRange,
    payload: PhysicalChunkPayloadIntegrityWitness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobStreamingReadObservation {
    Chunk(BlobStreamingReadObservedChunk),
    ColdUnavailable {
        ordinal: BlobChunkOrdinal,
        byte_range: BlobChunkByteRange,
    },
}

impl BlobStreamingReadObservedChunk {
    pub fn from_store_payload(
        ordinal: BlobChunkOrdinal,
        start: u64,
        payload: PhysicalChunkPayloadIntegrityWitness,
        window: BlobStreamingReadWindow,
    ) -> Result<Self, BlobStreamingReadDenial> {
        let bytes = payload.bytes_checked();
        if bytes == 0 {
            return Err(BlobStreamingReadDenial::EmptyObservedReadChunk);
        }
        if bytes > window.max_resident_bytes() {
            return Err(BlobStreamingReadDenial::ReadWindowExceedsResidentEnvelope {
                window_bytes: bytes,
                envelope_bytes: window.max_resident_bytes(),
            });
        }
        Ok(Self {
            ordinal,
            byte_range: BlobChunkByteRange::new(start, bytes)
                .map_err(|_| BlobStreamingReadDenial::EmptyObservedReadChunk)?,
            payload,
        })
    }

    pub const fn ordinal(&self) -> BlobChunkOrdinal {
        self.ordinal
    }

    pub const fn byte_range(&self) -> BlobChunkByteRange {
        self.byte_range
    }

    pub const fn payload(&self) -> &PhysicalChunkPayloadIntegrityWitness {
        &self.payload
    }
}

impl BlobStreamingReadObservation {
    pub const fn from_chunk(chunk: BlobStreamingReadObservedChunk) -> Self {
        Self::Chunk(chunk)
    }

    pub const fn cold_unavailable(
        ordinal: BlobChunkOrdinal,
        byte_range: BlobChunkByteRange,
    ) -> Self {
        Self::ColdUnavailable {
            ordinal,
            byte_range,
        }
    }
}
