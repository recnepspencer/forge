use forge_query::facade::ForgeQueryEntityIdentity;
use forge_relational::facade::identity::{EntityId, PartitionId};
use forge_runtime_bridge::facade::RelationalBridgeRecordIdentityKind;
use schema::facade::platform::entities::EntityKind;
use schema::facade::platform::relations::RelationKind;
use serde_json::Value;

use super::TopologyMaterializationError;

pub(crate) fn parse_entity_kind(
    kind_name: &str,
) -> Result<EntityKind, TopologyMaterializationError> {
    EntityKind::ALL
        .into_iter()
        .find(|kind| kind.kind_name() == kind_name)
        .ok_or_else(|| {
            TopologyMaterializationError::new(format!(
                "unknown topology entity kind `{kind_name}` in query row"
            ))
        })
}

pub(crate) fn parse_relation_kind(
    kind_name: &str,
) -> Result<RelationKind, TopologyMaterializationError> {
    RelationKind::ALL
        .into_iter()
        .find(|kind| kind.kind_name() == kind_name)
        .ok_or_else(|| {
            TopologyMaterializationError::new(format!(
                "unknown topology relation kind `{kind_name}` in query row"
            ))
        })
}

pub(crate) fn required_text<'a>(
    payload: &'a Value,
    path: &str,
) -> Result<&'a str, TopologyMaterializationError> {
    let mut current = payload;
    for part in path.split('.') {
        current = current.get(part).ok_or_else(|| {
            TopologyMaterializationError::new(format!(
                "query truth row is missing required field `{path}`"
            ))
        })?;
    }
    current.as_str().ok_or_else(|| {
        TopologyMaterializationError::new(format!(
            "query truth row field `{path}` must be a string"
        ))
    })
}

pub(crate) fn parse_entity_identity(
    identity: &str,
) -> Result<EntityId, TopologyMaterializationError> {
    let mut parts = identity.split(':');
    if parts.next() != Some("entity") {
        return Err(TopologyMaterializationError::new(format!(
            "expected forge-query entity identity, found `{identity}`"
        )));
    }
    let partition = parse_identity_part(parts.next(), "partition", identity)?;
    let slot = parse_identity_part(parts.next(), "slot", identity)?;
    let generation = parse_identity_part(parts.next(), "generation", identity)?;
    if parts.next().is_some() {
        return Err(TopologyMaterializationError::new(format!(
            "unexpected trailing forge-query identity data in `{identity}`"
        )));
    }
    Ok(EntityId::new(PartitionId(partition), slot, generation))
}

pub(crate) fn entity_id_from_query_identity(
    identity: &ForgeQueryEntityIdentity,
) -> Result<EntityId, TopologyMaterializationError> {
    let parts = identity.relational_record_parts().ok_or_else(|| {
        TopologyMaterializationError::new(format!(
            "expected relational forge-query entity identity, found `{identity}`"
        ))
    })?;
    if parts.kind() != RelationalBridgeRecordIdentityKind::Entity {
        return Err(TopologyMaterializationError::new(format!(
            "expected forge-query entity identity, found `{identity}`"
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

fn parse_identity_part<T>(
    part: Option<&str>,
    label: &str,
    identity: &str,
) -> Result<T, TopologyMaterializationError>
where
    T: std::str::FromStr,
{
    let value = part.ok_or_else(|| {
        TopologyMaterializationError::new(format!(
            "missing {label} component in forge-query identity `{identity}`"
        ))
    })?;
    value.parse::<T>().map_err(|_| {
        TopologyMaterializationError::new(format!(
            "invalid {label} component in forge-query identity `{identity}`"
        ))
    })
}
