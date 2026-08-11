mod artifact_name;
mod segment_inspection;

pub use artifact_name::WalSegmentArtifactIdentity;
pub use segment_inspection::{
    inspect_bounded_wal_active_tail_with_evidence, inspect_complete_wal_segment,
    inspect_interrupted_wal_segment_start, inspect_verified_wal_active_tail,
    inspect_verified_wal_segment, InterruptedWalSegmentStart, InterruptedWalTail,
    VerifiedWalActiveTail, VerifiedWalFrame, VerifiedWalFramePayload, VerifiedWalSegment,
    WalActiveTailInspectionDenial, WalActiveTailInspectionFailure, WalSegmentInspection,
};

#[cfg(test)]
mod tests;
