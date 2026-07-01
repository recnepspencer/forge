pub use crate::workload_platform::retained_replay_workload::{
    canonical_retained_cancellation_chain_capture, AdmittedRetainedReplayCapture,
    CapturedRetainedWorkload, ReplayEvidenceKind, ReplayEvidenceRow, ReplayEvidenceSet,
    ReplayParityKind, ReplayParityReport, ReplayParityRow, ReplayReceiptSet, ReplayWorkload,
    ReplayWorkloadCounters, ReplayedWorkload, RetainedArtifactCaptureReceipt, RetainedWorkload,
    UnsupportedReplayReasonCode, UnsupportedReplayWorkload,
};
pub use crate::workload_platform::spatial_compiled_product_consumer_cutover::{
    admit_retained_replay_capture, build_retained_replay_parity_report,
};
