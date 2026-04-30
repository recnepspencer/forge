use super::*;
use forge_relational::facade::bridge::bridge_snapshot_identity_for_commit;
use forge_relational::facade::identity::PartitionId;
use forge_relational::facade::transactions::CommitResult;
use serde_json::Map;

pub(super) fn snapshot_token_from_runtime(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
) -> String {
    runtime
        .publication()
        .latest_bundle()
        .map(|bundle| {
            bridge_snapshot_identity_for_commit(bundle.commit.commit_id, bundle.commit.version_id)
                .as_str()
                .to_string()
        })
        .unwrap_or_else(|| "relational-snapshot:empty:version:0".to_string())
}

pub(super) fn receipt_from_runtime_commit(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    result: CommitResult,
    collection: String,
    kind: ForgeQueryMutationKind,
    aspect_paths: Vec<String>,
) -> ForgeQueryMutationReceipt {
    let deltas = result
        .changed_records
        .iter()
        .filter_map(|record| match record {
            forge_relational::facade::transactions::RecordRef::Entity(entity) => {
                Some(ForgeQueryMutationDelta {
                    collection: collection.clone(),
                    entity_identity: entity_identity(*entity),
                    kind: kind.clone(),
                    aspect_paths: aspect_paths.clone(),
                })
            }
            forge_relational::facade::transactions::RecordRef::Relation(_) => None,
        })
        .collect::<Vec<_>>();
    ForgeQueryMutationReceipt {
        commit_identity: format!("commit-{}", result.commit.commit_id.0),
        snapshot_token: snapshot_token_from_runtime(runtime),
        deltas,
        bridge_authority: None,
    }
}

pub(super) fn entity_identity(entity: forge_relational::facade::identity::EntityId) -> String {
    format!(
        "entity:{}:{}:{}",
        entity.partition_id.0, entity.local_slot.0, entity.generation.0
    )
}

pub(super) fn parse_entity_identity(
    identity: &str,
) -> Result<forge_relational::facade::identity::EntityId, ForgeQueryWorkspaceError> {
    let mut parts = identity.split(':');
    if parts.next() != Some("entity") {
        return Err(ForgeQueryWorkspaceError::new("expected entity identity"));
    }
    let partition = parts
        .next()
        .ok_or_else(|| ForgeQueryWorkspaceError::new("missing partition"))?
        .parse::<u32>()
        .map_err(|_| ForgeQueryWorkspaceError::new("invalid partition"))?;
    let slot = parts
        .next()
        .ok_or_else(|| ForgeQueryWorkspaceError::new("missing slot"))?
        .parse::<u64>()
        .map_err(|_| ForgeQueryWorkspaceError::new("invalid slot"))?;
    let generation = parts
        .next()
        .ok_or_else(|| ForgeQueryWorkspaceError::new("missing generation"))?
        .parse::<u32>()
        .map_err(|_| ForgeQueryWorkspaceError::new("invalid generation"))?;
    Ok(forge_relational::facade::identity::EntityId::new(
        PartitionId(partition),
        slot,
        generation,
    ))
}

pub(super) fn set_json_path(
    target: &mut Value,
    path: &str,
    value: Value,
) -> Result<(), ForgeQueryWorkspaceError> {
    let mut parts = path.split('.').peekable();
    let mut current = target;
    while let Some(part) = parts.next() {
        if parts.peek().is_none() {
            let object = current
                .as_object_mut()
                .ok_or_else(|| ForgeQueryWorkspaceError::new("target payload is not an object"))?;
            object.insert(part.to_string(), value);
            return Ok(());
        }
        let object = current
            .as_object_mut()
            .ok_or_else(|| ForgeQueryWorkspaceError::new("target payload is not an object"))?;
        current = object
            .entry(part.to_string())
            .or_insert_with(|| Value::Object(Map::new()));
    }
    Err(ForgeQueryWorkspaceError::new("empty aspect path"))
}

pub(super) fn get_json_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    for part in path.split('.') {
        current = current.as_object()?.get(part)?;
    }
    Some(current)
}

pub(super) fn json_scalar_text(value: &Value) -> Option<String> {
    match value {
        Value::String(text) => Some(text.clone()),
        Value::Number(number) => Some(number.to_string()),
        Value::Bool(value) => Some(value.to_string()),
        Value::Null | Value::Array(_) | Value::Object(_) => None,
    }
}
