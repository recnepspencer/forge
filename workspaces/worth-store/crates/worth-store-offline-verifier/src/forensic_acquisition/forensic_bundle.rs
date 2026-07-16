use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForensicRangePosture {
    Acquired,
    Unreadable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForensicBundleRange {
    pub(crate) source_index: usize,
    pub(crate) source_offset: u64,
    pub(crate) byte_length: u64,
    pub(crate) output_name: Option<String>,
    pub(crate) digest: Option<[u8; 32]>,
    pub(crate) posture: ForensicRangePosture,
}

impl ForensicBundleRange {
    pub const fn source_index(&self) -> usize {
        self.source_index
    }

    pub const fn source_offset(&self) -> u64 {
        self.source_offset
    }

    pub const fn byte_length(&self) -> u64 {
        self.byte_length
    }

    pub fn output_name(&self) -> Option<&str> {
        self.output_name.as_deref()
    }

    pub const fn posture(&self) -> ForensicRangePosture {
        self.posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForensicCustodyRecord {
    pub(crate) observer_identity: String,
    pub(crate) acquisition_method: String,
    pub(crate) consistency_basis_identity: [u8; 32],
    pub(crate) source_media_fingerprints: Vec<[u8; 32]>,
}

impl ForensicCustodyRecord {
    pub fn observer_identity(&self) -> &str {
        &self.observer_identity
    }

    pub fn acquisition_method(&self) -> &str {
        &self.acquisition_method
    }
}

#[derive(Debug)]
pub struct ForensicBundle {
    pub(crate) root: PathBuf,
    pub(crate) bundle_identity: [u8; 32],
    pub(crate) ranges: Vec<ForensicBundleRange>,
    pub(crate) custody: ForensicCustodyRecord,
}

impl ForensicBundle {
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub const fn bundle_identity(&self) -> [u8; 32] {
        self.bundle_identity
    }

    pub fn ranges(&self) -> &[ForensicBundleRange] {
        &self.ranges
    }

    pub const fn custody(&self) -> &ForensicCustodyRecord {
        &self.custody
    }
}
