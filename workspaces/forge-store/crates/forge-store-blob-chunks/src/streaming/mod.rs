//! Streaming proof grammar:
//! - ingest: admit_stream → advance_frontier → verify_chunk_window → emit_ingest_receipt
//! - read: admit_read → observe_chunk_window → finish_verified_read
//! - resume: verify_request_match → resume_bounded_ingest → bind_resume_session
mod chunk_streaming;
mod ingest;
mod operation_counters;
mod read;
mod resume;
mod window_denial;

pub use chunk_streaming::{
    BlobChunkStreamingObservation, BlobChunkStreamingOperation, BlobChunkStreamingOperationKind,
    BlobChunkStreamingResidencyProof, BlobChunkStreamingWindow,
};
pub use ingest::{
    reject_allocation_denial_as_streaming_ingest, reject_full_blob_vec_as_streaming_ingest,
    reject_scalar_backend_api_as_streaming_ingest, BlobStreamingChunkWriter,
    BlobStreamingContentFrontier, BlobStreamingCounterBackedPerformanceReceipt,
    BlobStreamingIngest, BlobStreamingIngestCounterSnapshot, BlobStreamingIngestDenial,
    BlobStreamingIngestRequest, BlobStreamingPressureAdmission, BlobStreamingResidencyProof,
    BlobStreamingSourceFrame, BlobStreamingWindow, BlobStreamingWrittenChunk,
    LargeRecordStreamingEnvelope, LargeRecordStreamingEnvelopeDenial,
};
pub use operation_counters::BlobChunkStreamingCounterSnapshot;
pub use read::{
    reject_full_blob_vec_as_streaming_read, BlobStreamingReadAdmission,
    BlobStreamingReadCounterBackedPerformanceReceipt, BlobStreamingReadCounterSnapshot,
    BlobStreamingReadDenial, BlobStreamingReadObservation, BlobStreamingReadObservedChunk,
    BlobStreamingReadRequest, BlobStreamingReadWindow, BlobStreamingVerifiedRead,
};
pub use resume::{
    run_resumable_streaming_ingest, BlobStreamingResumeAdmission, BlobStreamingResumePosture,
};
pub use window_denial::BlobChunkStreamingDenial;
