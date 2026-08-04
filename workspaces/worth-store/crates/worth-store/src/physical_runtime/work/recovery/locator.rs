use sha2::{Digest, Sha256};
use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, RecordArtifactFile, RecordFrameCoordinate,
};

use super::super::{PhysicalWorkOperationFamily, PhysicalWorkRecoveryDisposition};

mod codec;
pub(super) use codec::{encode_family, encode_target};

pub(super) const RECOVERY_RECORD_BYTES: usize = 160;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkRecoveryTarget {
    Range(RecordFrameCoordinate),
    WalArtifactInterval {
        segment: u64,
        generation: u64,
        offset: u64,
        byte_count: u64,
    },
    Checkpoint {
        sequence: u64,
        action: PhysicalCheckpointRecoveryAction,
    },
    WalSegmentReclamation {
        segment: u64,
        generation: u64,
    },
    ArtifactFileSynchronization(RecordArtifactFile),
    ArtifactParentSynchronization(RecordArtifactFile),
    CatalogReplacement(RecordArtifactFile),
    RecordNamespaceSynchronization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkRecoveryLocator {
    store: StableStoreIdentity,
    runtime: u64,
    generation: u64,
    operation: u64,
    family: PhysicalWorkOperationFamily,
    target: PhysicalWorkRecoveryTarget,
    payload_digest: Option<[u8; 32]>,
    recovery: PhysicalWorkRecoveryDisposition,
}

pub(super) fn decode_locator(
    expected_store: StableStoreIdentity,
    file_name: &str,
    record: &[u8],
) -> Option<PhysicalWorkRecoveryLocator> {
    if record.len() != RECOVERY_RECORD_BYTES
        || &record[..8] != b"WPEFFECT"
        || record[16..32] != expected_store.bytes()
        || record[10..16].iter().any(|byte| *byte != 0)
    {
        return None;
    }
    let checksum: [u8; 32] = Sha256::digest(&record[..128]).into();
    if checksum != record[128..] {
        return None;
    }
    let family = codec::decode_family(record[9])?;
    if matches!(
        family,
        PhysicalWorkOperationFamily::ArtifactMetadataRead
            | PhysicalWorkOperationFamily::ArtifactRangeRead
    ) {
        return None;
    }
    let runtime = read_u64(record, 32)?;
    let generation = read_u64(record, 40)?;
    let operation = read_u64(record, 48)?;
    if runtime == 0 || generation == 0 || operation == 0 {
        return None;
    }
    let expected_name = format!("effect-{runtime:016x}-{generation:016x}-{operation:016x}.pending");
    if file_name != expected_name {
        return None;
    }
    if record[8] != 6 {
        return None;
    }
    let (target, payload_digest) = codec::decode_target(record)?;
    Some(PhysicalWorkRecoveryLocator {
        store: expected_store,
        runtime,
        generation,
        operation,
        family,
        target,
        payload_digest,
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalCheckpointRecoveryAction {
    CreateCandidate { byte_count: u64 },
    AppendCandidate { offset: u64, byte_count: u64 },
    SynchronizeCandidate,
    RemoveCandidate,
    PublishCandidate,
    SynchronizeNamespace,
}

impl From<super::super::PhysicalCheckpointWorkAction> for PhysicalCheckpointRecoveryAction {
    fn from(action: super::super::PhysicalCheckpointWorkAction) -> Self {
        match action {
            super::super::PhysicalCheckpointWorkAction::CreateCandidate { byte_count } => {
                Self::CreateCandidate { byte_count }
            }
            super::super::PhysicalCheckpointWorkAction::AppendCandidate { offset, byte_count } => {
                Self::AppendCandidate { offset, byte_count }
            }
            super::super::PhysicalCheckpointWorkAction::SynchronizeCandidate => {
                Self::SynchronizeCandidate
            }
            super::super::PhysicalCheckpointWorkAction::RemoveCandidate => Self::RemoveCandidate,
            super::super::PhysicalCheckpointWorkAction::PublishCandidate => Self::PublishCandidate,
            super::super::PhysicalCheckpointWorkAction::SynchronizeNamespace => {
                Self::SynchronizeNamespace
            }
        }
    }
}

fn read_u64(record: &[u8], offset: usize) -> Option<u64> {
    Some(u64::from_le_bytes(
        record.get(offset..offset + 8)?.try_into().ok()?,
    ))
}

impl PhysicalWorkRecoveryLocator {
    pub const fn store(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn runtime(self) -> u64 {
        self.runtime
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }

    pub const fn operation(self) -> u64 {
        self.operation
    }

    pub const fn family(self) -> PhysicalWorkOperationFamily {
        self.family
    }

    pub const fn target(self) -> PhysicalWorkRecoveryTarget {
        self.target
    }

    pub const fn coordinate(self) -> Option<RecordFrameCoordinate> {
        match self.target {
            PhysicalWorkRecoveryTarget::Range(coordinate) => Some(coordinate),
            _ => None,
        }
    }

    pub const fn payload_digest(self) -> Option<[u8; 32]> {
        self.payload_digest
    }

    pub const fn recovery_disposition(self) -> PhysicalWorkRecoveryDisposition {
        self.recovery
    }
}
