#[cfg(feature = "certification-authority")]
mod append_planner;
mod exact_frontier_prefix;
mod frame_codec;
mod offline_segment_verification;
mod prefix_scan;
mod scan;

#[cfg(all(test, feature = "certification-authority"))]
mod append_planner_tests;
#[cfg(test)]
mod offline_segment_verification_tests;

use crate::AdmittedWalAppendReceipt;
use std::path::{Path, PathBuf};

#[cfg(feature = "certification-authority")]
pub use append_planner::WalAppendPlanner;
pub use exact_frontier_prefix::{
    inspect_wal_exact_frontier_prefix, WalExactFrontierPrefix, WalExactFrontierPrefixDenial,
    WalExactFrontierPrefixRequest,
};
pub use offline_segment_verification::{
    verify_bounded_wal_segment, verify_bounded_wal_segment_from_reader, BoundedWalSegmentDenial,
    BoundedWalSegmentObservation, BoundedWalSegmentVerificationRequest,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WalStoreIdentity {
    root: PathBuf,
    segment_id: u64,
    generation: u64,
}

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
pub struct AdmittedWalArtifactStore {
    identity: WalStoreIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalPersistedArtifact {
    path: PathBuf,
    offset: u64,
    byte_count: u64,
    digest: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalPersistedArtifactRead {
    bytes: Vec<u8>,
    bytes_read: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalFrameAppendPlan {
    relative_path: PathBuf,
    encoded_frame: Vec<u8>,
    valid_prefix_bytes: u64,
    observed_file_bytes: u64,
    prefix_bytes_scanned: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WalArtifactScanCounters {
    directories_examined: u64,
    artifacts_read: u64,
    bytes_read: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalPersistedArtifactSet {
    store: WalStoreIdentity,
    artifacts: Vec<WalPersistedArtifact>,
    counters: WalArtifactScanCounters,
}

impl AdmittedWalArtifactStore {
    pub fn open(anchor: &AdmittedWalAppendReceipt) -> Result<Self, WalArtifactStoreDenial> {
        let artifact_directory = anchor
            .persisted_path()
            .parent()
            .ok_or(WalArtifactStoreDenial::InvalidArtifactPath)?;
        let root = artifact_directory
            .parent()
            .ok_or(WalArtifactStoreDenial::InvalidArtifactPath)?;
        let root = std::fs::canonicalize(root).map_err(|_| WalArtifactStoreDenial::Io)?;
        let persisted = std::fs::canonicalize(anchor.persisted_path())
            .map_err(|_| WalArtifactStoreDenial::Io)?;
        if !scan::is_segment_artifact(&root, &persisted) {
            return Err(WalArtifactStoreDenial::InvalidArtifactPath);
        }
        Ok(Self {
            identity: WalStoreIdentity {
                root,
                segment_id: anchor.scope().segment_id(),
                generation: anchor.scope().generation(),
            },
        })
    }

    pub fn identity(&self) -> &WalStoreIdentity {
        &self.identity
    }

    pub fn admits_append(&self, receipt: &AdmittedWalAppendReceipt) -> bool {
        receipt.scope().segment_id() == self.identity.segment_id
            && receipt.scope().generation() == self.identity.generation
            && std::fs::canonicalize(receipt.persisted_path())
                .is_ok_and(|path| scan::is_segment_artifact(&self.identity.root, &path))
    }

    pub fn admits_persisted_path(&self, path: &Path) -> bool {
        std::fs::canonicalize(path).is_ok_and(|path| {
            scan::is_segment_artifact(&self.identity.root, &path)
                || scan::is_checkpoint_artifact(&self.identity.root, &path)
        })
    }

    pub fn scan(&self) -> Result<WalPersistedArtifactSet, WalArtifactStoreDenial> {
        scan::scan(self)
    }
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

pub(crate) fn validate_persisted_wal_frame(
    path: &Path,
    encoded_offset: u64,
    encoded_bytes: u64,
    scope: &crate::WalFrameDurablePublicationScope,
) -> Result<(u64, u64), WalArtifactStoreDenial> {
    frame_codec::validate_persisted_frame(path, encoded_offset, encoded_bytes, scope)
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

impl WalStoreIdentity {
    pub fn stable_binding(&self) -> String {
        format!(
            "{}:{}:{}",
            self.root.to_string_lossy(),
            self.segment_id,
            self.generation
        )
    }

    pub const fn segment_id(&self) -> u64 {
        self.segment_id
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }
}

impl WalPersistedArtifact {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub const fn offset(&self) -> u64 {
        self.offset
    }

    pub const fn byte_count(&self) -> u64 {
        self.byte_count
    }

    pub fn read_bounded(
        &self,
        maximum_bytes: u64,
    ) -> Result<WalPersistedArtifactRead, WalArtifactStoreDenial> {
        use std::io::{Read, Seek, SeekFrom};

        use sha2::{Digest, Sha256};

        if self.byte_count > maximum_bytes {
            return Err(WalArtifactStoreDenial::ArtifactReadBudgetExceeded {
                bytes: self.byte_count,
                maximum: maximum_bytes,
            });
        }
        let allocation = usize::try_from(self.byte_count).map_err(|_| {
            WalArtifactStoreDenial::ArtifactReadBudgetExceeded {
                bytes: self.byte_count,
                maximum: maximum_bytes,
            }
        })?;
        let mut file = std::fs::File::open(&self.path).map_err(|_| WalArtifactStoreDenial::Io)?;
        let mut bytes = vec![0; allocation];
        file.seek(SeekFrom::Start(self.offset))
            .map_err(|_| WalArtifactStoreDenial::Io)?;
        file.read_exact(&mut bytes)
            .map_err(|_| WalArtifactStoreDenial::Io)?;
        if <[u8; 32]>::from(Sha256::digest(&bytes)) != self.digest {
            return Err(WalArtifactStoreDenial::DigestMismatch);
        }
        Ok(WalPersistedArtifactRead {
            bytes,
            bytes_read: self.byte_count,
        })
    }
}

impl WalPersistedArtifactRead {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub const fn bytes_read(&self) -> u64 {
        self.bytes_read
    }
}

impl WalPersistedArtifactSet {
    pub fn store(&self) -> &WalStoreIdentity {
        &self.store
    }

    pub fn artifacts(&self) -> &[WalPersistedArtifact] {
        &self.artifacts
    }

    pub const fn counters(&self) -> WalArtifactScanCounters {
        self.counters
    }
}

impl WalArtifactScanCounters {
    pub const fn directories_examined(self) -> u64 {
        self.directories_examined
    }

    pub const fn artifacts_read(self) -> u64 {
        self.artifacts_read
    }

    pub const fn bytes_read(self) -> u64 {
        self.bytes_read
    }
}
