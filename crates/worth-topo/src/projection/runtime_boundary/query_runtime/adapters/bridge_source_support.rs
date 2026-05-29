use forge_relational::facade::history::CommitId;
use forge_relational::facade::identity::{EntityId, PartitionId, RelationId, VersionId};
use forge_relational::facade::runtime::{EntityReadRecord, RelationReadRecord};
use forge_relational::facade::snapshots::SnapshotId;
use forge_relational::facade::transactions::RecordRef;
use forge_runtime_bridge::facade::{
    BridgeSnapshotReadError, RelationalBridgeSourceError, TruthSnapshotIdentity,
};
use serde_json::Value;

pub(super) fn parse_bridge_commit_identity(
    identity: &str,
) -> Result<CommitId, RelationalBridgeSourceError> {
    let raw = identity.strip_prefix("commit-").ok_or_else(|| {
        RelationalBridgeSourceError::new(format!(
            "unsupported topology bridge commit identity `{identity}`"
        ))
    })?;
    let commit_id = raw.parse::<u64>().map_err(|_| {
        RelationalBridgeSourceError::new(format!(
            "invalid topology bridge commit identity `{identity}`"
        ))
    })?;
    Ok(CommitId(commit_id))
}

pub(super) fn parse_bridge_snapshot_identity(
    identity: &TruthSnapshotIdentity,
) -> Result<(SnapshotId, VersionId), RelationalBridgeSourceError> {
    let mut parts = identity.as_str().split(':');
    let prefix = parts
        .next()
        .ok_or_else(|| RelationalBridgeSourceError::new("missing bridge snapshot prefix"))?;
    if prefix != "relational-snapshot" {
        return Err(RelationalBridgeSourceError::new(format!(
            "unsupported topology bridge snapshot identity `{}`",
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
    if version_segment != "version" {
        return Err(RelationalBridgeSourceError::new(format!(
            "unsupported topology bridge snapshot version segment `{version_segment}`"
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
            "topology bridge snapshot identity had too many fields",
        ));
    }
    Ok((snapshot_id, version_id))
}

pub(super) fn parse_bridge_record_identity(
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

pub(super) fn missing_record_error(
    kind: &str,
    identity: &str,
    snapshot_identity: &TruthSnapshotIdentity,
) -> BridgeSnapshotReadError {
    BridgeSnapshotReadError::new(format!(
        "topology snapshot reader could not find {kind} `{identity}` in authoritative snapshot `{}`",
        snapshot_identity.as_str()
    ))
}

pub(super) fn missing_aspect_error(
    kind: &str,
    aspect: &str,
    identity: &str,
    snapshot_identity: &TruthSnapshotIdentity,
) -> BridgeSnapshotReadError {
    BridgeSnapshotReadError::new(format!(
        "topology snapshot reader could not resolve aspect `{aspect}` on {kind} `{identity}` in authoritative snapshot `{}`",
        snapshot_identity.as_str()
    ))
}

pub(super) fn payload_bytes_for_entity_aspect(
    record: &EntityReadRecord,
    aspect_label: &str,
) -> Option<Vec<u8>> {
    payload_bytes_for_aspect(record.payload.as_json(), aspect_label, || {
        Some(Value::String(format!(
            "entity:{}:{}:{}",
            record.entity_id.partition_id.0,
            record.entity_id.local_slot.0,
            record.entity_id.generation.0
        )))
    })
}

pub(super) fn payload_bytes_for_relation_aspect(
    record: &RelationReadRecord,
    aspect_label: &str,
) -> Option<Vec<u8>> {
    payload_bytes_for_aspect(
        record.payload.as_ref().and_then(|value| value.as_json()),
        aspect_label,
        || {
            Some(Value::String(format!(
                "relation:{}:{}:{}",
                record.relation_id.partition_id.0,
                record.relation_id.local_slot.0,
                record.relation_id.generation.0
            )))
        },
    )
}

fn payload_bytes_for_aspect(
    payload: Option<&Value>,
    aspect_label: &str,
    identity_value: impl FnOnce() -> Option<Value>,
) -> Option<Vec<u8>> {
    let value = if aspect_label == "identity.id" {
        identity_value()?
    } else {
        let mut current = payload?;
        for segment in aspect_label.split('.') {
            current = current.get(segment)?;
        }
        current.clone()
    };
    serde_json::to_vec(&value).ok()
}




