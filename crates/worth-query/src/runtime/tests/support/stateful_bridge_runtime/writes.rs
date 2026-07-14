use super::super::*;
use super::state::{NativeExternalRow, StatefulBridgeState};
use worth_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey};
use worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts;

use crate::memory_workspace::{WorthQueryCommitIdentity, WorthQuerySnapshotIdentity};
use crate::runtime::backend::build_bridge_authority_bundle;

use super::SharedState;

pub(super) fn execute_write(
    shared: &SharedState,
    mutation: WorthQueryBackendAdmissibleMutation,
) -> Result<WorthQueryMutationReceipt, WorthQueryWorkspaceError> {
    let mut state = shared.borrow_mut();
    let collection = mutation
        .declared_collection_identity()
        .map(|collection| collection.as_str().to_string())
        .or_else(|| {
            mutation.existing_truth_binding().and_then(|binding| {
                binding
                    .terminal_target_collection_projection()
                    .map(str::to_string)
            })
        })
        .or_else(|| {
            mutation
                .declared_entity_identity_ref()
                .and_then(|identity| {
                    state
                        .collection_by_identity
                        .get(&identity.terminal_projection_for_reporting())
                        .cloned()
                })
        })
        .ok_or_else(|| {
            WorthQueryWorkspaceError::new("stateful bridge could not resolve collection")
        })?;
    let (entity_identity, entity_identity_text) = match mutation.mutation_family() {
        WorthQueryMutationFamily::Insert => {
            state.next_entity_identity += 1;
            let identity = WorthQueryEntityIdentity::from_relational_record(
                RelationalBridgeRecordIdentityParts::entity(
                    1,
                    state.next_entity_identity as u64,
                    0,
                ),
            );
            let identity_text = identity.terminal_projection_for_reporting();
            (identity, identity_text)
        }
        _ => {
            let identity = mutation
                .declared_entity_identity_ref()
                .cloned()
                .or_else(|| {
                    mutation
                        .existing_truth_binding()
                        .map(|binding| binding.resolved_target_identity().clone())
                })
                .or_else(|| {
                    mutation.symbolic_target_reference().and_then(|reference| {
                        state.identity_by_symbol.get(reference.symbol()).cloned()
                    })
                })
                .ok_or_else(|| {
                    WorthQueryWorkspaceError::new(
                        "stateful bridge could not resolve target entity identity",
                    )
                })?;
            let identity_text = mutation
                .symbolic_target_reference()
                .and_then(|reference| state.identity_text_by_symbol.get(reference.symbol()))
                .cloned()
                .unwrap_or_else(|| identity.terminal_projection_for_reporting());
            (identity, identity_text)
        }
    };
    let mutation_kind = apply_command(
        &mut state,
        &mutation,
        &collection,
        &entity_identity,
        &entity_identity_text,
    )?;
    state.next_commit_identity += 1;
    state.next_snapshot_token += 1;
    let commit_identity =
        WorthQueryCommitIdentity::from_relational_commit_id(state.next_commit_identity as u64);
    let snapshot_identity = WorthQuerySnapshotIdentity::from_relational_snapshot(
        worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts::new(
            1,
            state.next_snapshot_token as u64,
        ),
    );
    let bridge_authority = build_bridge_authority_bundle(
        &state.bridge,
        &snapshot_identity,
        &mutation,
        &collection,
        &entity_identity,
        mutation_kind.clone(),
    )?;
    Ok(test_mutation_receipt_with_bridge_authority(
        commit_identity,
        snapshot_identity,
        collection,
        entity_identity,
        mutation_kind,
        mutation.declared_aspect_touches(),
        bridge_authority,
    ))
}

pub(super) fn apply_command(
    state: &mut StatefulBridgeState,
    mutation: &WorthQueryBackendAdmissibleMutation,
    collection: &str,
    entity_identity: &WorthQueryEntityIdentity,
    entity_identity_text: &str,
) -> Result<WorthQueryMutationKind, WorthQueryWorkspaceError> {
    let entity_identity_key = entity_identity
        .terminal_projection_for_reporting()
        .to_string();
    match mutation.mutation_family() {
        WorthQueryMutationFamily::Insert => {
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
            Ok(WorthQueryMutationKind::Created)
        }
        WorthQueryMutationFamily::Update | WorthQueryMutationFamily::Assertion => {
            let resolved_aspects = resolved_aspects(state, mutation)?;
            let row = state
                .rows_by_collection
                .entry(collection.to_string())
                .or_default()
                .get_mut(&entity_identity_key)
                .ok_or_else(|| {
                    WorthQueryWorkspaceError::new(format!(
                        "stateful bridge update could not find `{entity_identity_key}` in `{collection}`"
                    ))
                })?;
            apply_aspects_to_external_row(row, &resolved_aspects)?;
            Ok(WorthQueryMutationKind::Updated)
        }
        WorthQueryMutationFamily::Delete => {
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
            Ok(WorthQueryMutationKind::Deleted)
        }
    }
}

fn external_row_from_mutation(
    state: &StatefulBridgeState,
    mutation: &WorthQueryBackendAdmissibleMutation,
) -> Result<NativeExternalRow, WorthQueryWorkspaceError> {
    external_row_from_aspects(&resolved_aspects(state, mutation)?)
}

fn resolved_aspects(
    state: &StatefulBridgeState,
    mutation: &WorthQueryBackendAdmissibleMutation,
) -> Result<Vec<WorthQueryAdmittedAspectValue>, WorthQueryWorkspaceError> {
    let mut aspects = mutation.admitted_aspect_values().to_vec();
    for reference in mutation.symbolic_aspect_references() {
        let resolved_identity = state
            .identity_text_by_symbol
            .get(reference.reference().symbol())
            .cloned()
            .ok_or_else(|| {
                WorthQueryWorkspaceError::new(format!(
                    "stateful bridge could not resolve symbolic aspect reference `{}`",
                    reference.reference().symbol()
                ))
            })?;
        aspects.push(WorthQueryAdmittedAspectValue::new_set(
            reference.aspect_touch().clone(),
            crate::runtime::WorthQueryAdmittedAspectValue::native_string_value(resolved_identity),
        )?);
    }
    Ok(aspects)
}

fn external_row_from_aspects(
    aspects: &[WorthQueryAdmittedAspectValue],
) -> Result<NativeExternalRow, WorthQueryWorkspaceError> {
    let mut external_row = NativeExternalRow::new();
    apply_aspects_to_external_row(&mut external_row, aspects)?;
    Ok(external_row)
}

fn apply_aspects_to_external_row(
    external_row: &mut NativeExternalRow,
    aspects: &[WorthQueryAdmittedAspectValue],
) -> Result<(), WorthQueryWorkspaceError> {
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
    aspect_touch: &WorthQueryAspectTouch,
    value: AspectValue,
) -> Result<(), WorthQueryWorkspaceError> {
    external_row.insert(native_external_field_path_for_touch(aspect_touch)?, value);
    Ok(())
}

pub(super) fn native_external_field_path_for_touch(
    aspect_touch: &WorthQueryAspectTouch,
) -> Result<CanonicalFieldPath, WorthQueryWorkspaceError> {
    let mut fields = vec![
        FieldKey::new(aspect_touch.native_aspect_key().as_str()).ok_or_else(|| {
            WorthQueryWorkspaceError::new(format!(
                "stateful bridge could not use native aspect `{}` as an external field",
                aspect_touch.native_aspect_key().as_str()
            ))
        })?,
    ];
    if let Some(field_path) = aspect_touch.native_field_path() {
        fields.extend(field_path.fields().iter().cloned());
    }
    CanonicalFieldPath::new(fields).ok_or_else(|| {
        WorthQueryWorkspaceError::new(format!(
            "stateful bridge could not derive external field path for `{}`",
            aspect_touch.admitted_touch_digest_part()
        ))
    })
}

pub(super) fn external_row_text_at_path(
    external_row: &NativeExternalRow,
    field_path: &CanonicalFieldPath,
) -> Option<String> {
    match external_row.get(&field_path)? {
        AspectValue::String(value) => Some(match value {
            worth_foundational::facade::InternedString::Raw(value) => value.clone(),
            worth_foundational::facade::InternedString::Symbol(symbol) => {
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
