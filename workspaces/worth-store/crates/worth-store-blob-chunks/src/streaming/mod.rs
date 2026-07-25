//! Streaming proof grammar:
//! - ingest: admit_stream → advance_frontier → verify_chunk_window → emit_ingest_receipt
//! - read: admit_read → observe_chunk_window → finish_verified_read
//! - resume: verify_request_match → resume_bounded_ingest → bind_resume_session
mod allocation;
mod chunk_streaming;
mod ingest;
mod operation_counters;
mod read;
mod resume;
mod window_denial;

pub use allocation::BlobStreamingAllocationObservation;
pub use chunk_streaming::{
    BlobChunkStreamingObservation, BlobChunkStreamingOperation, BlobChunkStreamingOperationKind,
    BlobChunkStreamingResidencyProof, BlobChunkStreamingWindow,
};
pub use ingest::{
    reject_allocation_denial_as_streaming_ingest, reject_full_blob_vec_as_streaming_ingest,
    reject_scalar_backend_api_as_streaming_ingest, BlobStreamingChunkWriter,
    BlobStreamingContentFrontier, BlobStreamingCounterBackedPerformanceReceipt,
    BlobStreamingIngest, BlobStreamingIngestCounterSnapshot, BlobStreamingIngestDenial,
    BlobStreamingIngestExecution, BlobStreamingIngestRequest, BlobStreamingPressureAdmission,
    BlobStreamingResidencyProof, BlobStreamingSourceFrame, BlobStreamingWindow,
    BlobStreamingWrittenChunk,
};
#[cfg(feature = "certification-test-authority")]
pub use ingest::{LargeRecordStreamingEnvelope, LargeRecordStreamingEnvelopeDenial};
pub use operation_counters::BlobChunkStreamingCounterSnapshot;
#[cfg(test)]
pub(crate) use read::test_support::layout_runtime_case;
pub use read::{
    reject_full_blob_vec_as_streaming_read, BlobStreamingReadAdmission,
    BlobStreamingReadCounterBackedPerformanceReceipt, BlobStreamingReadCounterSnapshot,
    BlobStreamingReadDenial, BlobStreamingReadExecution, BlobStreamingReadObservation,
    BlobStreamingReadObservedChunk, BlobStreamingReadRequest, BlobStreamingReadResidencyProof,
    BlobStreamingReadWindow, BlobStreamingVerifiedRead,
};
pub use resume::{
    run_resumable_streaming_ingest, BlobStreamingResumeAdmission, BlobStreamingResumePosture,
};
pub use window_denial::BlobChunkStreamingDenial;
