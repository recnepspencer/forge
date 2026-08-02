mod artifact_name;
mod segment_inspection;

pub use artifact_name::WalSegmentArtifactIdentity;
pub use segment_inspection::{
    inspect_complete_wal_segment, inspect_verified_wal_segment, VerifiedWalFramePayload,
    VerifiedWalSegment, WalSegmentInspection,
};

#[cfg(test)]
mod tests;
