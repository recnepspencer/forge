mod scan;

use crate::AdmittedWalAppendReceipt;
use std::path::{Path, PathBuf};

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
    Io,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedWalArtifactStore {
    identity: WalStoreIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalPersistedArtifact {
    path: PathBuf,
    bytes: Vec<u8>,
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
        if !scan::is_durability_artifact(&root, &persisted) {
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
                .is_ok_and(|path| scan::is_durability_artifact(&self.identity.root, &path))
    }

    pub fn admits_persisted_path(&self, path: &Path) -> bool {
        std::fs::canonicalize(path)
            .is_ok_and(|path| scan::is_durability_artifact(&self.identity.root, &path))
    }

    pub fn scan(&self) -> Result<WalPersistedArtifactSet, WalArtifactStoreDenial> {
        scan::scan(self)
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

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
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
