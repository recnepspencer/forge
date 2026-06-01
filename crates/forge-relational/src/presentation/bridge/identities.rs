use crate::history::data::CommitId;
use crate::identity::data::{EntityId, PartitionId, RelationId, VersionId};
use crate::snapshots::data::{SnapshotHandle, SnapshotId};
use crate::transactions::data::RecordRef;
use forge_runtime_bridge::facade::{RelationalBridgeSourceError, TruthSnapshotIdentity};

const RELATIONAL_BRIDGE_SNAPSHOT_PREFIX: &str = "relational-snapshot";
const RELATIONAL_BRIDGE_SNAPSHOT_VERSION_SEGMENT: &str = "version";

pub fn bridge_snapshot_identity_for_handle(handle: &SnapshotHandle) -> TruthSnapshotIdentity {
    bridge_snapshot_identity_for_binding(handle.snapshot_id, handle.version_id)
}

pub fn bridge_snapshot_identity_for_commit(
    commit_id: CommitId,
    version_id: VersionId,
) -> TruthSnapshotIdentity {
    bridge_snapshot_identity_for_binding(SnapshotId(commit_id.0), version_id)
}

pub(crate) fn record_ref_identity(record: &RecordRef) -> String {
    match record {
        RecordRef::Entity(entity) => format!(
            "entity:{}:{}:{}",
            entity.partition_id.0, entity.local_slot.0, entity.generation.0
        ),
        RecordRef::Relation(relation) => format!(
            "relation:{}:{}:{}",
            relation.partition_id.0, relation.local_slot.0, relation.generation.0
        ),
    }
}

pub(crate) fn parse_bridge_record_identity(
    identity: &str,
) -> Result<RecordRef, RelationalBridgeSourceError> {
    let mut parts = identity.split(':');
    let kind = parts
        .next()
        .ok_or_else(|| RelationalBridgeSourceError::new("missing bridge record kind"))?;
    let partition_id = PartitionId(
        parts
            .next()
            .ok_or_else(|| RelationalBridgeSourceError::new("missing bridge partition id"))?
            .parse::<u32>()
            .map_err(|_| RelationalBridgeSourceError::new("invalid bridge partition id"))?,
    );
    let local_slot = parts
        .next()
        .ok_or_else(|| RelationalBridgeSourceError::new("missing bridge local slot"))?
        .parse::<u64>()
        .map_err(|_| RelationalBridgeSourceError::new("invalid bridge local slot"))?;
    let generation = parts
        .next()
        .ok_or_else(|| RelationalBridgeSourceError::new("missing bridge generation"))?
        .parse::<u32>()
        .map_err(|_| RelationalBridgeSourceError::new("invalid bridge generation"))?;
    if parts.next().is_some() {
        return Err(RelationalBridgeSourceError::new(
            "bridge record identity had too many fields",
        ));
    }

    Ok(match kind {
        "entity" => RecordRef::Entity(EntityId::new(partition_id, local_slot, generation)),
        "relation" => RecordRef::Relation(RelationId::new(partition_id, local_slot, generation)),
        _ => {
            return Err(RelationalBridgeSourceError::new(format!(
                "unsupported bridge record kind `{kind}`"
            )))
        }
    })
}

pub(crate) fn parse_bridge_commit_identity(
    identity: &str,
) -> Result<CommitId, RelationalBridgeSourceError> {
    let raw = identity.strip_prefix("commit-").ok_or_else(|| {
        RelationalBridgeSourceError::new(format!(
            "unsupported relational bridge commit identity `{identity}`"
        ))
    })?;
    let commit_id = raw.parse::<u64>().map_err(|_| {
        RelationalBridgeSourceError::new(format!(
            "invalid relational bridge commit identity `{identity}`"
        ))
    })?;
    Ok(CommitId(commit_id))
}

fn bridge_snapshot_identity_for_binding(
    snapshot_id: SnapshotId,
    version_id: VersionId,
) -> TruthSnapshotIdentity {
    TruthSnapshotIdentity::new(format!(
        "{RELATIONAL_BRIDGE_SNAPSHOT_PREFIX}:{}:{RELATIONAL_BRIDGE_SNAPSHOT_VERSION_SEGMENT}:{}",
        snapshot_id.0, version_id.0
    ))
}

pub(crate) fn parse_bridge_snapshot_identity(
    identity: &TruthSnapshotIdentity,
) -> Result<(SnapshotId, VersionId), RelationalBridgeSourceError> {
    let mut parts = identity.as_str().split(':');
    let prefix = parts
        .next()
        .ok_or_else(|| RelationalBridgeSourceError::new("missing bridge snapshot prefix"))?;
    if prefix != RELATIONAL_BRIDGE_SNAPSHOT_PREFIX {
        return Err(RelationalBridgeSourceError::new(format!(
            "unsupported relational bridge snapshot identity `{}`",
            identity.as_str()
        )));
    }

    let snapshot_id = SnapshotId(
        parts
            .next()
            .ok_or_else(|| RelationalBridgeSourceError::new("missing relational snapshot id"))?
            .parse::<u64>()
            .map_err(|_| RelationalBridgeSourceError::new("invalid relational snapshot id"))?,
    );
    let version_segment = parts
        .next()
        .ok_or_else(|| RelationalBridgeSourceError::new("missing relational version segment"))?;
    if version_segment != RELATIONAL_BRIDGE_SNAPSHOT_VERSION_SEGMENT {
        return Err(RelationalBridgeSourceError::new(format!(
            "unsupported relational bridge snapshot version segment `{version_segment}`"
        )));
    }
    let version_id = VersionId(
        parts
            .next()
            .ok_or_else(|| RelationalBridgeSourceError::new("missing relational version id"))?
            .parse::<u64>()
            .map_err(|_| RelationalBridgeSourceError::new("invalid relational version id"))?,
    );
    if parts.next().is_some() {
        return Err(RelationalBridgeSourceError::new(
            "relational bridge snapshot identity had too many fields",
        ));
    }

    Ok((snapshot_id, version_id))
}
