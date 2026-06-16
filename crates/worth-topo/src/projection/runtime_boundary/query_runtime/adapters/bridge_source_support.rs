use forge_foundational::facade::AspectValue;
use forge_relational::facade::history::CommitId;
use forge_relational::facade::identity::{EntityId, PartitionId, RelationId, VersionId};
use forge_relational::facade::runtime::{EntityReadRecord, RelationReadRecord};
use forge_relational::facade::snapshots::SnapshotId;
use forge_relational::facade::transactions::RecordRef;
use forge_runtime_bridge::facade::{
    BridgeSnapshotReadError, RelationalBridgeRecordIdentityKind,
    RelationalBridgeRecordIdentityParts, RelationalBridgeSourceError, TruthCommitIdentity,
    TruthSnapshotIdentity,
};
use schema::facade::platform::entities::EntityKind;
use schema::facade::platform::relations::RelationKind;

use crate::relational_aspect_boundary::entity_record_domain_label;

pub(super) fn bridge_commit_id(
    identity: &TruthCommitIdentity,
) -> Result<CommitId, RelationalBridgeSourceError> {
    identity
        .relational_commit_id()
        .map(CommitId)
        .ok_or_else(|| {
            RelationalBridgeSourceError::new(
                "unsupported topology bridge commit identity; expected typed relational commit",
            )
        })
}

pub(super) fn parse_bridge_snapshot_identity(
    identity: &TruthSnapshotIdentity,
) -> Result<(SnapshotId, VersionId), RelationalBridgeSourceError> {
    let Some(parts) = identity.relational_snapshot_parts() else {
        return Err(RelationalBridgeSourceError::new(
            "unsupported topology bridge snapshot identity; expected typed relational snapshot",
        ));
    };
    Ok((
        SnapshotId(parts.snapshot_id()),
        VersionId(parts.version_id()),
    ))
}

pub(super) fn bridge_record_ref(
    identity: RelationalBridgeRecordIdentityParts,
) -> Result<RecordRef, RelationalBridgeSourceError> {
    let partition_id = PartitionId(identity.partition_id());
    Ok(match identity.kind() {
        RelationalBridgeRecordIdentityKind::Entity => RecordRef::Entity(EntityId::new(
            partition_id,
            identity.local_slot(),
            identity.generation(),
        )),
        RelationalBridgeRecordIdentityKind::Relation => RecordRef::Relation(RelationId::new(
            partition_id,
            identity.local_slot(),
            identity.generation(),
        )),
    })
}

pub(super) fn missing_record_error(
    kind: &str,
    identity: &str,
    snapshot_identity: &TruthSnapshotIdentity,
) -> BridgeSnapshotReadError {
    BridgeSnapshotReadError::new(format!(
        "topology snapshot reader could not find {kind} `{identity}` in authoritative snapshot `{}`",
        snapshot_identity
            .bridge_admission_evidence()
            .terminal_projection_for_reporting()
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
        snapshot_identity
            .bridge_admission_evidence()
            .terminal_projection_for_reporting()
    ))
}

pub(super) fn snapshot_aspect_value_for_entity_aspect(
    record: &EntityReadRecord,
    aspect_label: &str,
) -> Option<AspectValue> {
    match aspect_label {
        "identity.id" => Some(entity_identity_value(record.entity_id)),
        "lineage.provenance" => Some(entity_identity_value(record.entity_id)),
        "topology.kind" => Some(aspect_string(
            EntityKind::from_kind_id(record.kind.kind_id)?
                .kind_name()
                .to_string(),
        )),
        "topology.structure" | "naming.persistent_name" => {
            Some(aspect_string(entity_record_domain_label(record)?))
        }
        _ => return None,
    }
}

pub(super) fn snapshot_aspect_value_for_relation_aspect(
    record: &RelationReadRecord,
    aspect_label: &str,
) -> Option<AspectValue> {
    if aspect_label == "identity.id" {
        Some(relation_identity_value(record.relation_id))
    } else if aspect_label == "lineage.provenance" {
        Some(relation_identity_value(record.relation_id))
    } else if aspect_label == "topology.kind" {
        Some(aspect_string(
            RelationKind::from_kind_id(record.kind.kind_id)?
                .kind_name()
                .to_string(),
        ))
    } else if aspect_label == "topology.source_identity" {
        Some(entity_identity_value(record.source))
    } else if aspect_label == "topology.target_identity" {
        Some(entity_identity_value(record.target))
    } else if aspect_label == "naming.source_identity" {
        Some(entity_identity_value(record.source))
    } else if aspect_label == "naming.target_identity" {
        Some(entity_identity_value(record.target))
    } else {
        return None;
    }
}

fn entity_identity_value(entity_id: EntityId) -> AspectValue {
    aspect_string(format!(
        "entity:{}:{}:{}",
        entity_id.partition_id.0, entity_id.local_slot.0, entity_id.generation.0
    ))
}

fn relation_identity_value(relation_id: RelationId) -> AspectValue {
    aspect_string(format!(
        "relation:{}:{}:{}",
        relation_id.partition_id.0, relation_id.local_slot.0, relation_id.generation.0
    ))
}

fn aspect_string(value: impl Into<String>) -> AspectValue {
    AspectValue::String(value.into().into())
}
