use std::path::{Path, PathBuf};

use super::WalArtifactStoreDenial;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WalArtifactInventoryIdentity {
    pub(super) root: PathBuf,
    pub(super) segment_id: u64,
    pub(super) generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalArtifactInventory {
    pub(super) identity: WalArtifactInventoryIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalArtifactObservation {
    pub(super) path: PathBuf,
    pub(super) offset: u64,
    pub(super) byte_count: u64,
    pub(super) digest: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalArtifactObservationRead {
    bytes: Vec<u8>,
    bytes_read: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WalArtifactScanCounters {
    pub(super) directories_examined: u64,
    pub(super) artifacts_read: u64,
    pub(super) bytes_read: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalArtifactInventoryScan {
    pub(super) identity: WalArtifactInventoryIdentity,
    pub(super) artifacts: Vec<WalArtifactObservation>,
    pub(super) counters: WalArtifactScanCounters,
}

impl WalArtifactInventory {
    pub fn open(
        root: &Path,
        segment_id: u64,
        generation: u64,
    ) -> Result<Self, WalArtifactStoreDenial> {
        let root = std::fs::canonicalize(root).map_err(|_| WalArtifactStoreDenial::Io)?;
        Ok(Self {
            identity: WalArtifactInventoryIdentity {
                root,
                segment_id,
                generation,
            },
        })
    }

    pub const fn identity(&self) -> &WalArtifactInventoryIdentity {
        &self.identity
    }

    pub fn admits_path(&self, path: &Path) -> bool {
        std::fs::canonicalize(path)
            .is_ok_and(|path| super::scan::is_inventory_artifact(&self.identity, &path))
    }

    pub fn scan(&self) -> Result<WalArtifactInventoryScan, WalArtifactStoreDenial> {
        super::scan::scan(self)
    }
}

impl WalArtifactInventoryIdentity {
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

impl WalArtifactObservation {
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
    ) -> Result<WalArtifactObservationRead, WalArtifactStoreDenial> {
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
        Ok(WalArtifactObservationRead {
            bytes,
            bytes_read: self.byte_count,
        })
    }
}

impl WalArtifactObservationRead {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub const fn bytes_read(&self) -> u64 {
        self.bytes_read
    }
}

impl WalArtifactInventoryScan {
    pub const fn identity(&self) -> &WalArtifactInventoryIdentity {
        &self.identity
    }

    pub fn artifacts(&self) -> &[WalArtifactObservation] {
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
