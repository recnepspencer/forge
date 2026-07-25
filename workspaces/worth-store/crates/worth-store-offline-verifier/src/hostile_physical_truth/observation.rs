use super::super::{OfflineDurableManifestDenial, OfflineDurableManifestWalk};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineHostilePhysicalTruthBudgetDenial {
    ZeroFileLimit,
    ZeroByteLimit,
    PrefixExceedsByteLimit,
    PrefixExceedsUsize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OfflineHostilePhysicalTruthBudget {
    max_files: usize,
    max_total_bytes: u64,
    prefix_bytes: usize,
}

impl OfflineHostilePhysicalTruthBudget {
    pub fn new(
        max_files: usize,
        max_total_bytes: u64,
        prefix_bytes: u64,
    ) -> Result<Self, OfflineHostilePhysicalTruthBudgetDenial> {
        if max_files == 0 {
            return Err(OfflineHostilePhysicalTruthBudgetDenial::ZeroFileLimit);
        }
        if max_total_bytes == 0 {
            return Err(OfflineHostilePhysicalTruthBudgetDenial::ZeroByteLimit);
        }
        if prefix_bytes > max_total_bytes {
            return Err(OfflineHostilePhysicalTruthBudgetDenial::PrefixExceedsByteLimit);
        }
        let prefix_bytes = usize::try_from(prefix_bytes)
            .map_err(|_| OfflineHostilePhysicalTruthBudgetDenial::PrefixExceedsUsize)?;
        Ok(Self {
            max_files,
            max_total_bytes,
            prefix_bytes,
        })
    }

    pub(super) const fn max_files(self) -> usize {
        self.max_files
    }

    pub(super) const fn max_total_bytes(self) -> u64 {
        self.max_total_bytes
    }

    pub(super) const fn prefix_bytes(self) -> usize {
        self.prefix_bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineHostilePhysicalTruthDenial {
    RootUnavailable(std::io::ErrorKind),
    ArtifactUnavailable(std::io::ErrorKind),
    SymbolicLinkEncountered,
    NonUnicodeArtifactPath,
    FileBudgetExceeded,
    ByteBudgetExceeded,
    ArtifactChangedDuringObservation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineHostileArtifactObservation {
    path: Box<str>,
    byte_length: u64,
    digest: [u8; 32],
    prefix: Box<[u8]>,
    recovery_obligation: bool,
}

impl OfflineHostileArtifactObservation {
    pub(super) fn new(
        path: Box<str>,
        byte_length: u64,
        digest: [u8; 32],
        prefix: Box<[u8]>,
    ) -> Self {
        let recovery_obligation =
            path.starts_with("families/physical-work/") && path.ends_with(".pending");
        Self {
            path,
            byte_length,
            digest,
            prefix,
            recovery_obligation,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub const fn byte_length(&self) -> u64 {
        self.byte_length
    }

    pub const fn digest(&self) -> [u8; 32] {
        self.digest
    }

    pub fn prefix(&self) -> &[u8] {
        &self.prefix
    }

    pub const fn is_recovery_obligation(&self) -> bool {
        self.recovery_obligation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OfflineHostileCurrentRecordTruth {
    store_identity: [u8; 16],
    root_generation: u64,
    records: usize,
    payload_bytes: u64,
    payload_digest: [u8; 32],
}

impl OfflineHostileCurrentRecordTruth {
    pub(super) fn from_walk(walk: OfflineDurableManifestWalk) -> Self {
        Self {
            store_identity: walk.store_identity(),
            root_generation: walk.root_generation(),
            records: walk.placements().len(),
            payload_bytes: walk.payload_bytes(),
            payload_digest: walk.payload_digest(),
        }
    }

    pub const fn store_identity(self) -> [u8; 16] {
        self.store_identity
    }

    pub const fn root_generation(self) -> u64 {
        self.root_generation
    }

    pub const fn records(self) -> usize {
        self.records
    }

    pub const fn payload_bytes(self) -> u64 {
        self.payload_bytes
    }

    pub const fn payload_digest(self) -> [u8; 32] {
        self.payload_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineHostilePhysicalTruthObservation {
    artifacts: Box<[OfflineHostileArtifactObservation]>,
    total_bytes: u64,
    recovery_obligations: usize,
    current: Result<OfflineHostileCurrentRecordTruth, OfflineDurableManifestDenial>,
}

impl OfflineHostilePhysicalTruthObservation {
    pub(super) fn new(
        artifacts: Vec<OfflineHostileArtifactObservation>,
        current: Result<OfflineHostileCurrentRecordTruth, OfflineDurableManifestDenial>,
    ) -> Self {
        let total_bytes = artifacts
            .iter()
            .map(OfflineHostileArtifactObservation::byte_length)
            .sum();
        let recovery_obligations = artifacts
            .iter()
            .filter(|artifact| artifact.is_recovery_obligation())
            .count();
        Self {
            artifacts: artifacts.into_boxed_slice(),
            total_bytes,
            recovery_obligations,
            current,
        }
    }

    pub fn artifacts(&self) -> &[OfflineHostileArtifactObservation] {
        &self.artifacts
    }

    pub const fn total_bytes(&self) -> u64 {
        self.total_bytes
    }

    pub const fn recovery_obligations(&self) -> usize {
        self.recovery_obligations
    }

    pub const fn current(
        &self,
    ) -> Result<OfflineHostileCurrentRecordTruth, OfflineDurableManifestDenial> {
        self.current
    }
}
