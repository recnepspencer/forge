#[cfg(feature = "certification-authority")]
mod append_planner;
mod artifact_observation;
mod exact_frontier_prefix;
mod frame_codec;
mod inventory;
mod offline_segment_verification;
mod prefix_scan;
mod scan;
mod segment_inventory;

#[cfg(all(test, feature = "certification-authority"))]
mod append_planner_tests;
#[cfg(test)]
mod offline_segment_verification_tests;

use std::path::{Path, PathBuf};

#[cfg(feature = "certification-authority")]
pub use append_planner::{WalAppendPlanner, WalAppendPlannerDenial};
pub use artifact_observation::{
    observe_checkpoint_artifact, observe_wal_frame_artifact, CheckpointArtifactObservation,
    WalFrameArtifactObservation,
};
pub use exact_frontier_prefix::{
    inspect_wal_exact_frontier_prefix, WalExactFrontierPrefix, WalExactFrontierPrefixDenial,
    WalExactFrontierPrefixRequest,
};
pub use inventory::{
    WalArtifactInventory, WalArtifactInventoryIdentity, WalArtifactInventoryScan,
    WalArtifactObservation, WalArtifactObservationRead, WalArtifactScanCounters,
};
pub use offline_segment_verification::{
    verify_bounded_wal_segment, verify_bounded_wal_segment_from_reader, BoundedWalSegmentDenial,
    BoundedWalSegmentObservation, BoundedWalSegmentVerificationRequest,
};
pub use segment_inventory::{
    inspect_bounded_wal_active_tail_with_evidence, inspect_complete_wal_segment,
    inspect_interrupted_wal_segment_start, inspect_verified_wal_active_tail,
    inspect_verified_wal_segment, InterruptedWalSegmentStart, InterruptedWalTail,
    VerifiedWalActiveTail, VerifiedWalArtifact, VerifiedWalFrame, VerifiedWalFramePayload,
    VerifiedWalSegment, WalActiveTailInspectionDenial, WalActiveTailInspectionFailure,
    WalSegmentArtifactIdentity, WalSegmentInspection,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalArtifactStoreDenial {
    InvalidArtifactPath,
    StoreBindingMismatch,
    InvalidFrame,
    DigestMismatch,
    NonContiguousLsn,
    ArtifactReadBudgetExceeded { bytes: u64, maximum: u64 },
    Io,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalFrameAppendPlan {
    relative_path: PathBuf,
    encoded_frame: Vec<u8>,
    valid_prefix_bytes: u64,
    observed_file_bytes: u64,
    prefix_bytes_scanned: u64,
}

pub fn prepare_wal_frame_append(
    root: &Path,
    segment_id: u64,
    generation: u64,
    lsn_start: u64,
    lsn_end: u64,
    declared_digest: &str,
    payload: &[u8],
) -> Result<WalFrameAppendPlan, WalArtifactStoreDenial> {
    frame_codec::prepare_append(
        root,
        segment_id,
        generation,
        lsn_start,
        lsn_end,
        declared_digest,
        payload,
    )
}

pub(crate) fn encode_wal_frame_at_frontier(
    segment_id: u64,
    generation: u64,
    lsn_start: u64,
    lsn_end: u64,
    declared_digest: &str,
    payload: &[u8],
    valid_prefix_bytes: u64,
    last_lsn_end: Option<u64>,
) -> Result<WalFrameAppendPlan, WalArtifactStoreDenial> {
    frame_codec::encode_append(
        segment_id,
        generation,
        lsn_start,
        lsn_end,
        declared_digest,
        payload,
        prefix_scan::WalPrefixScan {
            valid_prefix_bytes,
            observed_file_bytes: valid_prefix_bytes,
            last_lsn_end,
            bytes_scanned: 0,
        },
    )
}

impl WalFrameAppendPlan {
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    pub fn encoded_frame(&self) -> &[u8] {
        &self.encoded_frame
    }

    pub const fn valid_prefix_bytes(&self) -> u64 {
        self.valid_prefix_bytes
    }

    pub const fn observed_file_bytes(&self) -> u64 {
        self.observed_file_bytes
    }

    /// Bytes read while establishing the pre-append durable prefix.
    /// A reused reconstructive planner reports only newly observed suffix bytes.
    pub const fn prefix_bytes_scanned(&self) -> u64 {
        self.prefix_bytes_scanned
    }
}
