// --- Capabilities (admission handles, next-step types) ---
#[cfg(feature = "certification-test-authority")]
pub use crate::streaming::LargeRecordStreamingEnvelope;
pub use crate::streaming::{
    run_resumable_streaming_ingest, BlobChunkStreamingOperation, BlobChunkStreamingOperationKind,
    BlobChunkStreamingWindow, BlobStreamingChunkWriter, BlobStreamingContentFrontier,
    BlobStreamingIngest, BlobStreamingIngestExecution, BlobStreamingIngestRequest,
    BlobStreamingPressureAdmission, BlobStreamingReadAdmission, BlobStreamingReadExecution,
    BlobStreamingReadRequest, BlobStreamingResumeAdmission,
};
// --- Outcomes (transition receipts) ---
pub use crate::streaming::{
    BlobChunkStreamingObservation, BlobChunkStreamingResidencyProof,
    BlobStreamingAllocationObservation, BlobStreamingCounterBackedPerformanceReceipt,
    BlobStreamingReadCounterBackedPerformanceReceipt, BlobStreamingReadObservation,
    BlobStreamingReadObservedChunk, BlobStreamingReadResidencyProof, BlobStreamingReadWindow,
    BlobStreamingResidencyProof, BlobStreamingResumePosture, BlobStreamingSourceFrame,
    BlobStreamingVerifiedRead, BlobStreamingWindow, BlobStreamingWrittenChunk,
};
// --- Denials (classified failure enums) ---
#[cfg(feature = "certification-test-authority")]
pub use crate::streaming::LargeRecordStreamingEnvelopeDenial;
pub use crate::streaming::{
    BlobChunkStreamingDenial, BlobStreamingIngestDenial, BlobStreamingReadDenial,
};
// --- Counter witnesses (read-only snapshots) ---
pub use crate::streaming::{
    BlobChunkStreamingCounterSnapshot, BlobStreamingIngestCounterSnapshot,
    BlobStreamingReadCounterSnapshot,
};
