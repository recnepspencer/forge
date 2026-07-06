use forge_store_physical_format::PhysicalChunkPayloadIntegrityWitness;

use crate::{BlobChunkOrdinal, BlobStreamingIngestDenial, BlobStreamingWrittenChunk};

pub(crate) fn verify_payload_matches_pending(
    ordinal: BlobChunkOrdinal,
    written: &BlobStreamingWrittenChunk,
    pending: &[u8],
) -> Result<(), BlobStreamingIngestDenial> {
    if written.payload_bytes() == pending {
        Ok(())
    } else {
        Err(BlobStreamingIngestDenial::BackendWritePayloadMismatch {
            ordinal: ordinal.get(),
        })
    }
}

pub(crate) fn verify_backend_observation(
    ordinal: BlobChunkOrdinal,
    written: BlobStreamingWrittenChunk,
) -> Result<(PhysicalChunkPayloadIntegrityWitness, u64), BlobStreamingIngestDenial> {
    let (payload, backend_write) = written.into_parts();
    if backend_write.ordinal() != ordinal.get() {
        return Err(BlobStreamingIngestDenial::BackendWriteOrdinalMismatch {
            expected: ordinal.get(),
            actual: backend_write.ordinal(),
        });
    }
    let bytes = payload.bytes_checked();
    if backend_write.bytes_written() != bytes {
        return Err(BlobStreamingIngestDenial::BackendWriteBytesMismatch {
            expected: bytes,
            actual: backend_write.bytes_written(),
        });
    }
    Ok((payload, bytes))
}