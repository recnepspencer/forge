use super::super::*;
use super::state::StatefulBridgeState;
use serde_json::Value;

pub(super) fn apply_command(
    state: &mut StatefulBridgeState,
    command: &ForgeQueryWriteCommand,
    collection: &str,
    entity_identity: &ForgeQueryEntityIdentity,
    entity_identity_text: &str,
) -> Result<ForgeQueryMutationKind, ForgeQueryWorkspaceError> {
    let entity_identity_key = entity_identity.to_string();
    match command.mutation_family() {
        ForgeQueryMutationFamily::Insert => {
            let external_row = external_row_from_command(state, command)?;
            state
                .rows_by_collection
                .entry(collection.to_string())
                .or_default()
                .insert(entity_identity_key.clone(), external_row);
            state
                .collection_by_identity
                .insert(entity_identity_key.clone(), collection.to_string());
            state
                .identity_by_storage_key
                .insert(entity_identity_key.clone(), entity_identity.clone());
            if let Some(reference) = command.symbolic_target_reference() {
                state
                    .identity_by_symbol
                    .insert(reference.symbol().to_string(), entity_identity.clone());
                state.identity_text_by_symbol.insert(
                    reference.symbol().to_string(),
                    entity_identity_text.to_string(),
                );
            }
            Ok(ForgeQueryMutationKind::Created)
        }
        ForgeQueryMutationFamily::Update | ForgeQueryMutationFamily::Assertion => {
            let resolved_aspects = resolved_aspects(state, command)?;
            let row = state
                .rows_by_collection
                .entry(collection.to_string())
                .or_default()
                .get_mut(&entity_identity_key)
                .ok_or_else(|| {
                    ForgeQueryWorkspaceError::new(format!(
                        "stateful bridge update could not find `{entity_identity_key}` in `{collection}`"
                    ))
                })?;
            apply_aspects_to_external_row(row, &resolved_aspects)?;
            Ok(ForgeQueryMutationKind::Updated)
        }
        ForgeQueryMutationFamily::Delete => {
            if let Some(rows) = state.rows_by_collection.get_mut(collection) {
                rows.remove(&entity_identity_key);
            }
            state.collection_by_identity.remove(&entity_identity_key);
            state.identity_by_storage_key.remove(&entity_identity_key);
            state.identity_by_symbol.retain(|_, resolved_identity| {
                resolved_identity.evidence_identity() != entity_identity.evidence_identity()
            });
            state
                .identity_text_by_symbol
                .retain(|_, resolved_identity| resolved_identity != entity_identity_text);
            Ok(ForgeQueryMutationKind::Deleted)
        }
    }
}

fn external_row_from_command(
    state: &StatefulBridgeState,
    command: &ForgeQueryWriteCommand,
) -> Result<Value, ForgeQueryWorkspaceError> {
    external_row_from_aspects(&resolved_aspects(state, command)?)
}

fn resolved_aspects(
    state: &StatefulBridgeState,
    command: &ForgeQueryWriteCommand,
) -> Result<Vec<ForgeQueryAspectValue>, ForgeQueryWorkspaceError> {
    let mut aspects = command.aspect_values().to_vec();
    for reference in command.symbolic_aspect_references() {
        let resolved_identity = state
            .identity_text_by_symbol
            .get(reference.reference().symbol())
            .cloned()
            .ok_or_else(|| {
                ForgeQueryWorkspaceError::new(format!(
                    "stateful bridge could not resolve symbolic aspect reference `{}`",
                    reference.reference().symbol()
                ))
            })?;
        aspects.push(ForgeQueryAspectValue::new_set(
            reference.aspect_path().to_string(),
            resolved_identity,
        )?);
    }
    Ok(aspects)
}

fn external_row_from_aspects(
    aspects: &[ForgeQueryAspectValue],
) -> Result<Value, ForgeQueryWorkspaceError> {
    let mut external_row = Value::Object(serde_json::Map::new());
    apply_aspects_to_external_row(&mut external_row, aspects)?;
    Ok(external_row)
}

fn apply_aspects_to_external_row(
    external_row: &mut Value,
    aspects: &[ForgeQueryAspectValue],
) -> Result<(), ForgeQueryWorkspaceError> {
    for aspect in aspects {
        set_external_row_path(external_row, aspect.aspect_path(), aspect.value().clone())?;
    }
    Ok(())
}

fn set_external_row_path(
    external_row: &mut Value,
    dotted_path: &str,
    value: Value,
) -> Result<(), ForgeQueryWorkspaceError> {
    let mut current = external_row;
    let mut parts = dotted_path.split('.').peekable();
    while let Some(part) = parts.next() {
        if parts.peek().is_none() {
            return match current {
                Value::Object(object) => {
                    object.insert(part.to_string(), value);
                    Ok(())
                }
                _ => Err(ForgeQueryWorkspaceError::new(format!(
                    "stateful bridge external row path `{dotted_path}` crossed a non-object boundary"
                ))),
            };
        }
        current = match current {
            Value::Object(object) => object
                .entry(part.to_string())
                .or_insert_with(|| Value::Object(serde_json::Map::new())),
            _ => {
                return Err(ForgeQueryWorkspaceError::new(format!(
                "stateful bridge external row path `{dotted_path}` crossed a non-object boundary"
            )))
            }
        };
    }
    Ok(())
}

pub(super) fn external_row_text(external_row: &Value, dotted_path: &str) -> Option<String> {
    let mut current = external_row;
    for part in dotted_path.split('.') {
        current = current.get(part)?;
    }
    match current {
        Value::String(text) => Some(text.clone()),
        Value::Number(number) => Some(number.to_string()),
        Value::Bool(flag) => Some(flag.to_string()),
        _ => None,
    }
}
