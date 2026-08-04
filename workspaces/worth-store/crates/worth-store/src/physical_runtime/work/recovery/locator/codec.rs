use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::{PhysicalCheckpointRecoveryAction, PhysicalWorkRecoveryTarget, RECOVERY_RECORD_BYTES};
use crate::physical_runtime::work::PhysicalWorkOperationFamily;

const TARGET_RANGE: u8 = 1;
const TARGET_FILE_SYNC: u8 = 2;
const TARGET_PARENT_SYNC: u8 = 3;
const TARGET_CATALOG_REPLACE: u8 = 4;
const TARGET_RECORD_NAMESPACE_SYNC: u8 = 5;
const TARGET_WAL_INTERVAL: u8 = 6;
const TARGET_CHECKPOINT: u8 = 7;
const TARGET_WAL_RECLAMATION: u8 = 8;

pub(in crate::physical_runtime::work::recovery) const fn encode_family(
    family: PhysicalWorkOperationFamily,
) -> u8 {
    match family {
        PhysicalWorkOperationFamily::ArtifactRangeRead => 1,
        PhysicalWorkOperationFamily::ArtifactRangeWrite => 2,
        PhysicalWorkOperationFamily::ArtifactPublication => 3,
        PhysicalWorkOperationFamily::ArtifactMetadataRead => 4,
        PhysicalWorkOperationFamily::WalAppend => 5,
        PhysicalWorkOperationFamily::DurabilityBarrier => 6,
        PhysicalWorkOperationFamily::CheckpointCapture => 7,
        PhysicalWorkOperationFamily::WalReclamation => 8,
        PhysicalWorkOperationFamily::RootPublication => 9,
    }
}

pub(super) const fn decode_family(value: u8) -> Option<PhysicalWorkOperationFamily> {
    match value {
        1 => Some(PhysicalWorkOperationFamily::ArtifactRangeRead),
        2 => Some(PhysicalWorkOperationFamily::ArtifactRangeWrite),
        3 => Some(PhysicalWorkOperationFamily::ArtifactPublication),
        4 => Some(PhysicalWorkOperationFamily::ArtifactMetadataRead),
        5 => Some(PhysicalWorkOperationFamily::WalAppend),
        6 => Some(PhysicalWorkOperationFamily::DurabilityBarrier),
        7 => Some(PhysicalWorkOperationFamily::CheckpointCapture),
        8 => Some(PhysicalWorkOperationFamily::WalReclamation),
        9 => Some(PhysicalWorkOperationFamily::RootPublication),
        _ => None,
    }
}

pub(in crate::physical_runtime::work::recovery) fn encode_target(
    target: PhysicalWorkRecoveryTarget,
    record: &mut [u8; RECOVERY_RECORD_BYTES],
) {
    match target {
        PhysicalWorkRecoveryTarget::Range(coordinate) => {
            record[104] = TARGET_RANGE;
            encode_interval(record, coordinate.offset(), u64::from(coordinate.length()));
            encode_artifact(record, coordinate.artifact());
        }
        PhysicalWorkRecoveryTarget::WalArtifactInterval {
            segment,
            generation,
            offset,
            byte_count,
        } => {
            record[104] = TARGET_WAL_INTERVAL;
            encode_interval(record, offset, byte_count);
            encode_pair(record, segment, generation);
        }
        PhysicalWorkRecoveryTarget::Checkpoint { sequence, action } => {
            record[104] = TARGET_CHECKPOINT;
            encode_pair(record, sequence, 0);
            encode_checkpoint_action(record, action);
        }
        PhysicalWorkRecoveryTarget::WalSegmentReclamation {
            segment,
            generation,
        } => {
            record[104] = TARGET_WAL_RECLAMATION;
            encode_pair(record, segment, generation);
        }
        PhysicalWorkRecoveryTarget::ArtifactFileSynchronization(artifact) => {
            record[104] = TARGET_FILE_SYNC;
            encode_artifact(record, artifact);
        }
        PhysicalWorkRecoveryTarget::ArtifactParentSynchronization(artifact) => {
            record[104] = TARGET_PARENT_SYNC;
            encode_artifact(record, artifact);
        }
        PhysicalWorkRecoveryTarget::CatalogReplacement(artifact) => {
            record[104] = TARGET_CATALOG_REPLACE;
            encode_artifact(record, artifact);
        }
        PhysicalWorkRecoveryTarget::RecordNamespaceSynchronization => {
            record[104] = TARGET_RECORD_NAMESPACE_SYNC;
        }
    }
}

pub(super) fn decode_target(
    record: &[u8],
) -> Option<(PhysicalWorkRecoveryTarget, Option<[u8; 32]>)> {
    if record[107..112].iter().any(|byte| *byte != 0) || record[105] > 1 {
        return None;
    }
    let digest = (record[105] == 1)
        .then(|| record[72..104].try_into().ok())
        .flatten();
    let offset = read_u64(record, 56)?;
    let byte_count = read_u64(record, 64)?;
    let first = read_u64(record, 112)?;
    let second = read_u64(record, 120)?;
    let target = match record[104] {
        TARGET_RANGE if digest.is_some() => {
            let length = u32::try_from(byte_count).ok()?;
            PhysicalWorkRecoveryTarget::Range(RecordFrameCoordinate::new(
                decode_artifact(record[106], first, second)?,
                offset,
                length,
            )?)
        }
        TARGET_FILE_SYNC if empty_interval(offset, byte_count) && digest.is_none() => {
            PhysicalWorkRecoveryTarget::ArtifactFileSynchronization(decode_artifact(
                record[106],
                first,
                second,
            )?)
        }
        TARGET_PARENT_SYNC if empty_interval(offset, byte_count) && digest.is_none() => {
            PhysicalWorkRecoveryTarget::ArtifactParentSynchronization(decode_artifact(
                record[106],
                first,
                second,
            )?)
        }
        TARGET_CATALOG_REPLACE if empty_interval(offset, byte_count) && digest.is_none() => {
            let artifact = decode_artifact(record[106], first, second)?;
            matches!(artifact, RecordArtifactFile::CatalogCandidate { .. }).then_some(())?;
            PhysicalWorkRecoveryTarget::CatalogReplacement(artifact)
        }
        TARGET_RECORD_NAMESPACE_SYNC
            if empty_interval(offset, byte_count)
                && digest.is_none()
                && record[106] == 0
                && first == 0
                && second == 0 =>
        {
            PhysicalWorkRecoveryTarget::RecordNamespaceSynchronization
        }
        TARGET_WAL_INTERVAL
            if record[106] == 0
                && first > 0
                && second > 0
                && valid_interval(offset, byte_count)
                && digest.is_some() =>
        {
            PhysicalWorkRecoveryTarget::WalArtifactInterval {
                segment: first,
                generation: second,
                offset,
                byte_count,
            }
        }
        TARGET_CHECKPOINT if first > 0 && second == 0 => PhysicalWorkRecoveryTarget::Checkpoint {
            sequence: first,
            action: decode_checkpoint_action(record[106], offset, byte_count, digest)?,
        },
        TARGET_WAL_RECLAMATION
            if empty_interval(offset, byte_count)
                && digest.is_none()
                && record[106] == 0
                && first > 0
                && second > 0 =>
        {
            PhysicalWorkRecoveryTarget::WalSegmentReclamation {
                segment: first,
                generation: second,
            }
        }
        _ => return None,
    };
    Some((target, digest))
}

fn encode_checkpoint_action(
    record: &mut [u8; RECOVERY_RECORD_BYTES],
    action: PhysicalCheckpointRecoveryAction,
) {
    match action {
        PhysicalCheckpointRecoveryAction::CreateCandidate { byte_count } => {
            record[106] = 1;
            encode_interval(record, 0, byte_count);
        }
        PhysicalCheckpointRecoveryAction::AppendCandidate { offset, byte_count } => {
            record[106] = 2;
            encode_interval(record, offset, byte_count);
        }
        PhysicalCheckpointRecoveryAction::SynchronizeCandidate => record[106] = 3,
        PhysicalCheckpointRecoveryAction::RemoveCandidate => record[106] = 4,
        PhysicalCheckpointRecoveryAction::PublishCandidate => record[106] = 5,
        PhysicalCheckpointRecoveryAction::SynchronizeNamespace => record[106] = 6,
    }
}

fn decode_checkpoint_action(
    tag: u8,
    offset: u64,
    byte_count: u64,
    digest: Option<[u8; 32]>,
) -> Option<PhysicalCheckpointRecoveryAction> {
    match tag {
        1 if offset == 0 && byte_count > 0 && digest.is_some() => {
            Some(PhysicalCheckpointRecoveryAction::CreateCandidate { byte_count })
        }
        2 if valid_interval(offset, byte_count) && digest.is_some() => {
            Some(PhysicalCheckpointRecoveryAction::AppendCandidate { offset, byte_count })
        }
        3 if empty_interval(offset, byte_count) && digest.is_none() => {
            Some(PhysicalCheckpointRecoveryAction::SynchronizeCandidate)
        }
        4 if empty_interval(offset, byte_count) && digest.is_none() => {
            Some(PhysicalCheckpointRecoveryAction::RemoveCandidate)
        }
        5 if empty_interval(offset, byte_count) && digest.is_none() => {
            Some(PhysicalCheckpointRecoveryAction::PublishCandidate)
        }
        6 if empty_interval(offset, byte_count) && digest.is_none() => {
            Some(PhysicalCheckpointRecoveryAction::SynchronizeNamespace)
        }
        _ => None,
    }
}

fn encode_interval(record: &mut [u8; RECOVERY_RECORD_BYTES], offset: u64, byte_count: u64) {
    record[56..64].copy_from_slice(&offset.to_le_bytes());
    record[64..72].copy_from_slice(&byte_count.to_le_bytes());
}

fn encode_artifact(record: &mut [u8; RECOVERY_RECORD_BYTES], artifact: RecordArtifactFile) {
    let (tag, first, second) = artifact_parts(artifact);
    record[106] = tag;
    encode_pair(record, first, second);
}

fn encode_pair(record: &mut [u8; RECOVERY_RECORD_BYTES], first: u64, second: u64) {
    record[112..120].copy_from_slice(&first.to_le_bytes());
    record[120..128].copy_from_slice(&second.to_le_bytes());
}

fn artifact_parts(artifact: RecordArtifactFile) -> (u8, u64, u64) {
    match artifact {
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
    }
}

fn decode_artifact(tag: u8, first: u64, second: u64) -> Option<RecordArtifactFile> {
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

fn valid_interval(offset: u64, byte_count: u64) -> bool {
    byte_count > 0 && offset.checked_add(byte_count).is_some()
}

const fn empty_interval(offset: u64, byte_count: u64) -> bool {
    offset == 0 && byte_count == 0
}

fn read_u64(record: &[u8], offset: usize) -> Option<u64> {
    Some(u64::from_le_bytes(
        record.get(offset..offset + 8)?.try_into().ok()?,
    ))
}

#[cfg(test)]
mod tests;
