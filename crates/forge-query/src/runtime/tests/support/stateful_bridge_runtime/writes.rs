use super::super::*;
use super::state::{NativeExternalRow, StatefulBridgeState};
use forge_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey};

pub(super) fn apply_command(
    state: &mut StatefulBridgeState,
    mutation: &ForgeQueryBackendAdmissibleMutation,
    collection: &str,
    entity_identity: &ForgeQueryEntityIdentity,
    entity_identity_text: &str,
) -> Result<ForgeQueryMutationKind, ForgeQueryWorkspaceError> {
    let entity_identity_key = entity_identity
        .terminal_projection_for_reporting()
        .to_string();
    match mutation.mutation_family() {
        ForgeQueryMutationFamily::Insert => {
            let external_row = external_row_from_mutation(state, mutation)?;
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
            if let Some(reference) = mutation.symbolic_target_reference() {
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
            let resolved_aspects = resolved_aspects(state, mutation)?;
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

fn external_row_from_mutation(
    state: &StatefulBridgeState,
    mutation: &ForgeQueryBackendAdmissibleMutation,
) -> Result<NativeExternalRow, ForgeQueryWorkspaceError> {
    external_row_from_aspects(&resolved_aspects(state, mutation)?)
}

fn resolved_aspects(
    state: &StatefulBridgeState,
    mutation: &ForgeQueryBackendAdmissibleMutation,
) -> Result<Vec<ForgeQueryAspectValue>, ForgeQueryWorkspaceError> {
    let mut aspects = mutation.admitted_aspect_values().to_vec();
    for reference in mutation.symbolic_aspect_references() {
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
            reference.aspect_touch().clone(),
            AspectValue::String(resolved_identity.into()),
        )?);
    }
    Ok(aspects)
}

fn external_row_from_aspects(
    aspects: &[ForgeQueryAspectValue],
) -> Result<NativeExternalRow, ForgeQueryWorkspaceError> {
    let mut external_row = NativeExternalRow::new();
    apply_aspects_to_external_row(&mut external_row, aspects)?;
    Ok(external_row)
}

fn apply_aspects_to_external_row(
    external_row: &mut NativeExternalRow,
    aspects: &[ForgeQueryAspectValue],
) -> Result<(), ForgeQueryWorkspaceError> {
    for aspect in aspects {
        let aspect_touch = aspect.aspect_touch();
        set_external_row_touch(
            external_row,
            &aspect_touch,
            aspect
                .foundational_value()
                .cloned()
                .unwrap_or(AspectValue::Null),
        )?;
    }
    Ok(())
}

fn set_external_row_touch(
    external_row: &mut NativeExternalRow,
    aspect_touch: &ForgeQueryAspectTouch,
    value: AspectValue,
) -> Result<(), ForgeQueryWorkspaceError> {
    external_row.insert(native_external_field_path_for_touch(aspect_touch)?, value);
    Ok(())
}

pub(super) fn native_external_field_path_for_touch(
    aspect_touch: &ForgeQueryAspectTouch,
) -> Result<CanonicalFieldPath, ForgeQueryWorkspaceError> {
    let mut fields = vec![
        FieldKey::new(aspect_touch.native_aspect_key().as_str().to_string()).ok_or_else(|| {
            ForgeQueryWorkspaceError::new(format!(
                "stateful bridge could not use native aspect `{}` as an external field",
                aspect_touch.native_aspect_key().as_str()
            ))
        })?,
    ];
    if let Some(field_path) = aspect_touch.native_field_path() {
        fields.extend(field_path.fields().iter().cloned());
    }
    CanonicalFieldPath::new(fields).ok_or_else(|| {
        ForgeQueryWorkspaceError::new(format!(
            "stateful bridge could not derive external field path for `{}`",
            aspect_touch.admitted_touch_digest_part()
        ))
    })
}

pub(super) fn external_row_text(
    external_row: &NativeExternalRow,
    dotted_path: &str,
) -> Option<String> {
    let field_path = native_external_field_path(dotted_path).ok()?;
    match external_row.get(&field_path)? {
        AspectValue::String(value) => Some(match value {
            forge_foundational::facade::InternedString::Raw(value) => value.clone(),
            forge_foundational::facade::InternedString::Symbol(symbol) => {
                format!("symbol:{}", symbol.0)
            }
        }),
        AspectValue::Int8(value) => Some(value.to_string()),
        AspectValue::Int16(value) => Some(value.to_string()),
        AspectValue::Int32(value) => Some(value.to_string()),
        AspectValue::Int64(value) => Some(value.to_string()),
        AspectValue::UInt8(value) => Some(value.to_string()),
        AspectValue::UInt16(value) => Some(value.to_string()),
        AspectValue::UInt32(value) => Some(value.to_string()),
        AspectValue::UInt64(value) => Some(value.to_string()),
        AspectValue::Bool(value) => Some(value.to_string()),
        _ => None,
    }
}

pub(super) fn native_external_field_path(
    dotted_path: &str,
) -> Result<CanonicalFieldPath, ForgeQueryWorkspaceError> {
    CanonicalFieldPath::new(
        dotted_path
            .split('.')
            .map(|part| FieldKey::new(part.to_string()))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| {
                ForgeQueryWorkspaceError::new(format!(
                    "stateful bridge external row path `{dotted_path}` was not a valid field path"
                ))
            })?,
    )
    .ok_or_else(|| {
        ForgeQueryWorkspaceError::new(format!(
            "stateful bridge external row path `{dotted_path}` was empty"
        ))
    })
}
