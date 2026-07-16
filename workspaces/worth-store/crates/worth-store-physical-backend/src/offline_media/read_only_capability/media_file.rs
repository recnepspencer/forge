use std::path::{Path, PathBuf};

use super::owned_allocation::path_owned_bytes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineMediaFileIdentity {
    pub(super) path: PathBuf,
    pub(super) length: u64,
    pub(super) metadata_fingerprint: [u8; 32],
    pub(super) physical_alias_group: u64,
    pub(super) physical_key: file_id::FileId,
    pub(super) physical_key_fingerprint: [u8; 32],
}

impl OfflineMediaFileIdentity {
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub const fn length(&self) -> u64 {
        self.length
    }
    pub const fn metadata_fingerprint(&self) -> [u8; 32] {
        self.metadata_fingerprint
    }
    pub const fn physical_alias_group(&self) -> u64 {
        self.physical_alias_group
    }
    pub const fn physical_key_fingerprint(&self) -> [u8; 32] {
        self.physical_key_fingerprint
    }

    pub fn owned_allocation_bytes(&self) -> Option<u64> {
        path_owned_bytes(&self.path)
    }
}

#[derive(Debug)]
pub(super) struct StableReadOnlyFile {
    pub(super) identity: OfflineMediaFileIdentity,
}
