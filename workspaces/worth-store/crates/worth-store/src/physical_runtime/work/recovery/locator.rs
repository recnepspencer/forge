use sha2::{Digest, Sha256};
use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, RecordArtifactFile, RecordFrameCoordinate,
};

use super::super::{PhysicalWorkOperationFamily, PhysicalWorkRecoveryDisposition};

pub(super) const RECOVERY_RECORD_BYTES: usize = 160;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkRecoveryTarget {
    Range(RecordFrameCoordinate),
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
    let family = decode_family(record[9])?;
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
    let (target, payload_digest) = match record[8] {
        2 => decode_v2_target(record)?,
        3 => decode_v3_target(record)?,
        _ => return None,
    };
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

fn decode_v2_target(record: &[u8]) -> Option<(PhysicalWorkRecoveryTarget, Option<[u8; 32]>)> {
    if record[69..72].iter().any(|byte| *byte != 0)
        || record[105..112].iter().any(|byte| *byte != 0)
        || record[68] != 1
    {
        return None;
    }
    let coordinate = decode_coordinate(record)?;
    let digest = record[72..104].try_into().ok()?;
    Some((PhysicalWorkRecoveryTarget::Range(coordinate), Some(digest)))
}

fn decode_v3_target(record: &[u8]) -> Option<(PhysicalWorkRecoveryTarget, Option<[u8; 32]>)> {
    if record[70..72].iter().any(|byte| *byte != 0)
        || record[105..112].iter().any(|byte| *byte != 0)
        || record[68] > 1
    {
        return None;
    }
    let artifact = decode_artifact(record[104], read_u64(record, 112)?, read_u64(record, 120)?)?;
    let digest = (record[68] == 1)
        .then(|| record[72..104].try_into().ok())
        .flatten();
    let offset = read_u64(record, 56)?;
    let length = read_u32(record, 64)?;
    let target = match record[69] {
        1 if digest.is_some() => {
            PhysicalWorkRecoveryTarget::Range(RecordFrameCoordinate::new(artifact, offset, length)?)
        }
        2 if offset == 0 && length == 0 && digest.is_none() => {
            PhysicalWorkRecoveryTarget::ArtifactFileSynchronization(artifact)
        }
        3 if offset == 0 && length == 0 && digest.is_none() => {
            PhysicalWorkRecoveryTarget::ArtifactParentSynchronization(artifact)
        }
        4 if offset == 0 && length == 0 && digest.is_none() => {
            if !matches!(artifact, RecordArtifactFile::CatalogCandidate { .. }) {
                return None;
            }
            PhysicalWorkRecoveryTarget::CatalogReplacement(artifact)
        }
        5 if offset == 0
            && length == 0
            && digest.is_none()
            && artifact == RecordArtifactFile::BootstrapCatalog =>
        {
            PhysicalWorkRecoveryTarget::RecordNamespaceSynchronization
        }
        _ => return None,
    };
    Some((target, digest))
}

fn decode_coordinate(record: &[u8]) -> Option<RecordFrameCoordinate> {
    let artifact = decode_artifact(record[104], read_u64(record, 112)?, read_u64(record, 120)?)?;
    RecordFrameCoordinate::new(artifact, read_u64(record, 56)?, read_u32(record, 64)?)
}

pub(super) fn encode_target(
    target: PhysicalWorkRecoveryTarget,
    record: &mut [u8; RECOVERY_RECORD_BYTES],
) {
    let (tag, artifact, offset, length) = match target {
        PhysicalWorkRecoveryTarget::Range(coordinate) => (
            1,
            coordinate.artifact(),
            coordinate.offset(),
            coordinate.length(),
        ),
        PhysicalWorkRecoveryTarget::ArtifactFileSynchronization(artifact) => (2, artifact, 0, 0),
        PhysicalWorkRecoveryTarget::ArtifactParentSynchronization(artifact) => (3, artifact, 0, 0),
        PhysicalWorkRecoveryTarget::CatalogReplacement(candidate) => (4, candidate, 0, 0),
        PhysicalWorkRecoveryTarget::RecordNamespaceSynchronization => {
            (5, RecordArtifactFile::BootstrapCatalog, 0, 0)
        }
    };
    record[69] = tag;
    record[56..64].copy_from_slice(&offset.to_le_bytes());
    record[64..68].copy_from_slice(&length.to_le_bytes());
    encode_artifact(artifact, record);
}

pub(super) fn encode_artifact(artifact: RecordArtifactFile, record: &mut [u8; 160]) {
    let (tag, first, second) = match artifact {
        RecordArtifactFile::BootstrapCatalog => (1, 0, 0),
        RecordArtifactFile::CatalogCandidate { publication } => (2, publication, 0),
        RecordArtifactFile::RootManifest { generation } => (3, generation, 0),
        RecordArtifactFile::RootRoutingBlock { generation, block } => (4, generation, block),
        RecordArtifactFile::Segment {
            segment,
            generation,
        } => (5, segment, generation),
        RecordArtifactFile::SegmentManifest {
            segment,
            generation,
        } => (6, segment, generation),
        RecordArtifactFile::SegmentMembershipBlock { generation, block } => (7, generation, block),
        RecordArtifactFile::Extent { extent, generation } => (8, extent, generation),
        RecordArtifactFile::ExtentManifest { extent, generation } => (9, extent, generation),
        RecordArtifactFile::FreeSpaceManifest { generation } => (10, generation, 0),
        RecordArtifactFile::FreeSpaceMembershipBlock { generation, block } => {
            (11, generation, block)
        }
    };
    record[104] = tag;
    record[112..120].copy_from_slice(&first.to_le_bytes());
    record[120..128].copy_from_slice(&second.to_le_bytes());
}

const fn decode_family(value: u8) -> Option<PhysicalWorkOperationFamily> {
    match value {
        1 => Some(PhysicalWorkOperationFamily::ArtifactRangeRead),
        2 => Some(PhysicalWorkOperationFamily::ArtifactRangeWrite),
        3 => Some(PhysicalWorkOperationFamily::ArtifactPublication),
        4 => Some(PhysicalWorkOperationFamily::ArtifactMetadataRead),
        _ => None,
    }
}

const fn decode_artifact(tag: u8, first: u64, second: u64) -> Option<RecordArtifactFile> {
    match tag {
        1 if first == 0 && second == 0 => Some(RecordArtifactFile::BootstrapCatalog),
        2 if second == 0 => Some(RecordArtifactFile::CatalogCandidate { publication: first }),
        3 if second == 0 => Some(RecordArtifactFile::RootManifest { generation: first }),
        4 => Some(RecordArtifactFile::RootRoutingBlock {
            generation: first,
            block: second,
        }),
        5 => Some(RecordArtifactFile::Segment {
            segment: first,
            generation: second,
        }),
        6 => Some(RecordArtifactFile::SegmentManifest {
            segment: first,
            generation: second,
        }),
        7 => Some(RecordArtifactFile::SegmentMembershipBlock {
            generation: first,
            block: second,
        }),
        8 => Some(RecordArtifactFile::Extent {
            extent: first,
            generation: second,
        }),
        9 => Some(RecordArtifactFile::ExtentManifest {
            extent: first,
            generation: second,
        }),
        10 if second == 0 => Some(RecordArtifactFile::FreeSpaceManifest { generation: first }),
        11 => Some(RecordArtifactFile::FreeSpaceMembershipBlock {
            generation: first,
            block: second,
        }),
        _ => None,
    }
}

fn read_u64(record: &[u8], offset: usize) -> Option<u64> {
    Some(u64::from_le_bytes(
        record.get(offset..offset + 8)?.try_into().ok()?,
    ))
}

fn read_u32(record: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_le_bytes(
        record.get(offset..offset + 4)?.try_into().ok()?,
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
