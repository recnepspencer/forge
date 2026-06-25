use crate::query_native_runtime_boundary::query_entity_identity_reporting_label;
use forge_query::facade::ForgeQueryEntityIdentity;
use forge_relational::facade::identity::{EntityId, PartitionId};
use forge_runtime_bridge::facade::RelationalBridgeRecordIdentityKind;

use super::TopologyMaterializationError;

pub(crate) fn entity_id_from_query_identity(
    identity: &ForgeQueryEntityIdentity,
) -> Result<EntityId, TopologyMaterializationError> {
    let parts = identity.relational_record_parts().ok_or_else(|| {
        TopologyMaterializationError::new(format!(
            "expected relational forge-query entity identity, found `{}`",
            query_entity_identity_reporting_label(identity)
        ))
    })?;
    if parts.kind() != RelationalBridgeRecordIdentityKind::Entity {
        return Err(TopologyMaterializationError::new(format!(
            "expected forge-query entity identity, found `{}`",
            query_entity_identity_reporting_label(identity)
        )));
    }
    Ok(EntityId::new(
        PartitionId(parts.partition_id()),
        parts.local_slot(),
        parts.generation(),
    ))
}

pub(crate) fn query_entity_identity(entity_id: EntityId) -> String {
    format!(
        "entity:{}:{}:{}",
        entity_id.partition_id.0, entity_id.local_slot.0, entity_id.generation.0
    )
}
