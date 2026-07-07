use forge_store_buffer_pool::AllocationDenial;
use forge_store_physical_backend::{
    BlobBackendChunkWriteObservation, BlobBackendChunkWriteObservationKind,
};

use crate::BlobStreamingIngestDenial;

pub fn reject_scalar_backend_api_as_streaming_ingest(
    observation: BlobBackendChunkWriteObservation,
) -> BlobStreamingIngestDenial {
    if observation.kind() == BlobBackendChunkWriteObservationKind::ScalarFramedRecordApi {
        BlobStreamingIngestDenial::ScalarBackendCertificationRejected
    } else {
        BlobStreamingIngestDenial::BackendWriteOrdinalMismatch {
            expected: 0,
            actual: observation.ordinal(),
        }
    }
}

pub fn reject_allocation_denial_as_streaming_ingest(
    denial: AllocationDenial,
) -> BlobStreamingIngestDenial {
    BlobStreamingIngestDenial::AllocationDenied(denial)
}
