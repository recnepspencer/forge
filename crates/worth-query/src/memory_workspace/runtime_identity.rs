use super::*;
use worth_runtime_bridge::facade::{
    RelationalBridgeRecordIdentityParts, RelationalBridgeSnapshotIdentityParts,
};

pub(super) fn snapshot_identity_from_runtime(
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
) -> WorthQuerySnapshotIdentity {
    runtime
        .publication()
        .latest_bundle()
        .map(|bundle| {
            WorthQuerySnapshotIdentity::from_relational_snapshot(
                RelationalBridgeSnapshotIdentityParts::new(
                    bundle.commit.commit_id.0,
                    bundle.commit.version_id.0,
                ),
            )
        })
        .unwrap_or_else(WorthQuerySnapshotIdentity::empty_relational_state)
}

pub(super) fn entity_identity(
    entity: worth_relational::facade::identity::EntityId,
) -> WorthQueryEntityIdentity {
    WorthQueryEntityIdentity::from_relational_record(RelationalBridgeRecordIdentityParts::entity(
        entity.partition_id.0,
        entity.local_slot.0,
        entity.generation.0,
    ))
}

pub(super) fn entity_id_from_identity(
    identity: WorthQueryEntityIdentity,
) -> Result<worth_relational::facade::identity::EntityId, WorthQueryWorkspaceError> {
    let Some(parts) = identity.relational_record_parts() else {
        return Err(WorthQueryWorkspaceError::new(
            "memory workspace mutations require relational entity identities",
        ));
    };
    Ok(worth_relational::facade::identity::EntityId::new(
        worth_relational::facade::identity::PartitionId(parts.partition_id()),
        parts.local_slot(),
        parts.generation(),
    ))
}
