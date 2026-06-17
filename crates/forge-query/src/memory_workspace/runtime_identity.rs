use super::*;
use forge_runtime_bridge::facade::{
    RelationalBridgeRecordIdentityParts, RelationalBridgeSnapshotIdentityParts,
};

pub(super) fn snapshot_identity_from_runtime(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
) -> ForgeQuerySnapshotIdentity {
    runtime
        .publication()
        .latest_bundle()
        .map(|bundle| {
            ForgeQuerySnapshotIdentity::from_relational_snapshot(
                RelationalBridgeSnapshotIdentityParts::new(
                    bundle.commit.commit_id.0,
                    bundle.commit.version_id.0,
                ),
            )
        })
        .unwrap_or_else(ForgeQuerySnapshotIdentity::empty_relational_state)
}

pub(super) fn entity_identity(
    entity: forge_relational::facade::identity::EntityId,
) -> ForgeQueryEntityIdentity {
    ForgeQueryEntityIdentity::from_relational_record(RelationalBridgeRecordIdentityParts::entity(
        entity.partition_id.0,
        entity.local_slot.0,
        entity.generation.0,
    ))
}

pub(super) fn entity_id_from_identity(
    identity: ForgeQueryEntityIdentity,
) -> Result<forge_relational::facade::identity::EntityId, ForgeQueryWorkspaceError> {
    let Some(parts) = identity.relational_record_parts() else {
        return Err(ForgeQueryWorkspaceError::new(
            "memory workspace mutations require relational entity identities",
        ));
    };
    Ok(forge_relational::facade::identity::EntityId::new(
        forge_relational::facade::identity::PartitionId(parts.partition_id()),
        parts.local_slot(),
        parts.generation(),
    ))
}
