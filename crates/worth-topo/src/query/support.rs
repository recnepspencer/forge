use forge_relational::facade::identity::{EntityId, PartitionId};
use serde_json::Value;
use worth_schema::facade::{WorthEntityKind, WorthRelationKind};

use crate::materialization::WorthTopologyMaterializationError;

pub(crate) fn parse_entity_kind(
    kind_name: &str,
) -> Result<WorthEntityKind, WorthTopologyMaterializationError> {
    WorthEntityKind::ALL
        .into_iter()
        .find(|kind| kind.kind_name() == kind_name)
        .ok_or_else(|| {
            WorthTopologyMaterializationError::new(format!(
                "unknown worth topology entity kind `{kind_name}` in query row"
            ))
        })
}

pub(crate) fn parse_relation_kind(
    kind_name: &str,
) -> Result<WorthRelationKind, WorthTopologyMaterializationError> {
    WorthRelationKind::ALL
        .into_iter()
        .find(|kind| kind.kind_name() == kind_name)
        .ok_or_else(|| {
            WorthTopologyMaterializationError::new(format!(
                "unknown worth topology relation kind `{kind_name}` in query row"
            ))
        })
}

pub(crate) fn required_text<'a>(
    payload: &'a Value,
    path: &str,
) -> Result<&'a str, WorthTopologyMaterializationError> {
    let mut current = payload;
    for part in path.split('.') {
        current = current.get(part).ok_or_else(|| {
            WorthTopologyMaterializationError::new(format!(
                "query truth row is missing required field `{path}`"
            ))
        })?;
    }
    current.as_str().ok_or_else(|| {
        WorthTopologyMaterializationError::new(format!(
            "query truth row field `{path}` must be a string"
        ))
    })
}

pub(crate) fn parse_entity_identity(
    identity: &str,
) -> Result<EntityId, WorthTopologyMaterializationError> {
    let mut parts = identity.split(':');
    if parts.next() != Some("entity") {
        return Err(WorthTopologyMaterializationError::new(format!(
            "expected forge-query entity identity, found `{identity}`"
        )));
    }
    let partition = parse_identity_part(parts.next(), "partition", identity)?;
    let slot = parse_identity_part(parts.next(), "slot", identity)?;
    let generation = parse_identity_part(parts.next(), "generation", identity)?;
    if parts.next().is_some() {
        return Err(WorthTopologyMaterializationError::new(format!(
            "unexpected trailing forge-query identity data in `{identity}`"
        )));
    }
    Ok(EntityId::new(PartitionId(partition), slot, generation))
}

fn parse_identity_part<T>(
    part: Option<&str>,
    label: &str,
    identity: &str,
) -> Result<T, WorthTopologyMaterializationError>
where
    T: std::str::FromStr,
{
    let value = part.ok_or_else(|| {
        WorthTopologyMaterializationError::new(format!(
            "missing {label} component in forge-query identity `{identity}`"
        ))
    })?;
    value.parse::<T>().map_err(|_| {
        WorthTopologyMaterializationError::new(format!(
            "invalid {label} component in forge-query identity `{identity}`"
        ))
    })
}
