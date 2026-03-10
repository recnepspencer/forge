use serde_json::json;
use std::collections::BTreeSet;

use crate::config::data::PatchSurfacePolicy;
use crate::diagnostics::data::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::publication::data::diff::{AspectKey, PatchRecord, PatchRecordKind};
use crate::schema::data::RelationalSchemaRegistry;
use crate::symbols::data::{InternedString, StringInterner};
use crate::transactions::data::{
    AuthoritativeApplyPlan, RecordRef, RelationSpec, TransactionIntent,
};

use super::apply_mutation::{
    allocate_entity, allocate_relation, delete_entity_with_cascade, delete_relation,
    reserve_bulk_entity_capacity, reserve_bulk_relation_capacity,
};
use super::apply_patching::{patch_detail_for_entity, patch_detail_for_relation};
use crate::logic::runtime::WorkingState;

pub(crate) fn apply_plan_to_staged_state(
    staged: &mut WorkingState,
    apply_plan: &AuthoritativeApplyPlan,
    patch_surface_policy: PatchSurfacePolicy,
    schema_registry: &RelationalSchemaRegistry,
    symbols: &mut StringInterner,
    cascade_delete_policy: crate::config::data::CascadeDeletePolicy,
) -> (
    Vec<RecordRef>,
    Vec<PatchRecord>,
    Vec<RelationalDiagnosticsEntry>,
) {
    let mut changed_records = Vec::new();
    let mut patch_records = Vec::new();
    let mut diagnostics = Vec::new();

    for intent in &apply_plan.merged_intents {
        match intent.clone() {
            TransactionIntent::CreateEntity(spec) => {
                let entity_id = allocate_entity(
                    staged,
                    apply_plan.version_id,
                    spec.partition_id,
                    spec.kind_id,
                    spec.payload.clone(),
                );
                staged.mark_entity_slot_touched(
                    entity_id.partition_id,
                    entity_id.local_slot.0 as usize,
                );
                write_entity_aspect_versions(
                    staged,
                    entity_id,
                    apply_plan.version_id,
                    &spec.payload,
                    symbols,
                );
                changed_records.push(RecordRef::Entity(entity_id));
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::EntityCreated,
                    message: "entity created".to_string(),
                    fields: json!({
                        "partition_id": entity_id.partition_id.0,
                        "entity_slot": entity_id.local_slot.0,
                        "kind_id": spec.kind_id.0,
                    }),
                });
                patch_records.push(PatchRecord {
                    kind: PatchRecordKind::EntityCreated,
                    entity_id: Some(entity_id),
                    relation_id: None,
                    aspects: aspect_keys_for_payload(Some(&spec.payload), symbols),
                    detail: patch_detail_for_entity(
                        patch_surface_policy,
                        PatchRecordKind::EntityCreated,
                        entity_id,
                        Some(&spec.payload),
                    ),
                });
            }
            TransactionIntent::BulkCreateEntities {
                partition_id,
                kind_id,
                client_keys: _,
                payloads,
            } => {
                reserve_bulk_entity_capacity(staged, partition_id, payloads.len());
                for payload in payloads {
                    let entity_id = allocate_entity(
                        staged,
                        apply_plan.version_id,
                        partition_id,
                        kind_id,
                        payload.clone(),
                    );
                    staged.mark_entity_slot_touched(
                        entity_id.partition_id,
                        entity_id.local_slot.0 as usize,
                    );
                    write_entity_aspect_versions(
                        staged,
                        entity_id,
                        apply_plan.version_id,
                        &payload,
                        symbols,
                    );
                    changed_records.push(RecordRef::Entity(entity_id));
                    patch_records.push(PatchRecord {
                        kind: PatchRecordKind::EntityCreated,
                        entity_id: Some(entity_id),
                        relation_id: None,
                        aspects: aspect_keys_for_payload(Some(&payload), symbols),
                        detail: patch_detail_for_entity(
                            patch_surface_policy,
                            PatchRecordKind::EntityCreated,
                            entity_id,
                            Some(&payload),
                        ),
                    });
                }
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::EntityCreated,
                    message: "bulk entities created".to_string(),
                    fields: json!({
                        "partition_id": partition_id.0,
                        "kind_id": kind_id.0,
                    }),
                });
            }
            TransactionIntent::UpdateEntity { entity_id, payload } => {
                let payload = payload.canonicalized();
                let slot = entity_id.local_slot.0 as usize;
                staged.mark_entity_slot_touched(entity_id.partition_id, slot);
                let partition = staged.get_partition_mut(entity_id.partition_id);
                partition.entity_arena.apply_payload_update(
                    slot,
                    payload.clone(),
                    apply_plan.version_id,
                );
                write_entity_aspect_versions(
                    staged,
                    entity_id,
                    apply_plan.version_id,
                    &payload,
                    symbols,
                );
                changed_records.push(RecordRef::Entity(entity_id));
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::EntityUpdated,
                    message: "entity updated".to_string(),
                    fields: json!({
                        "partition_id": entity_id.partition_id.0,
                        "entity_slot": entity_id.local_slot.0,
                    }),
                });
                patch_records.push(PatchRecord {
                    kind: PatchRecordKind::EntityUpdated,
                    entity_id: Some(entity_id),
                    relation_id: None,
                    aspects: aspect_keys_for_payload(Some(&payload), symbols),
                    detail: patch_detail_for_entity(
                        patch_surface_policy,
                        PatchRecordKind::EntityUpdated,
                        entity_id,
                        Some(&payload),
                    ),
                });
            }
            TransactionIntent::ReplaceEntity {
                entity_id,
                replacement,
            } => {
                delete_entity_with_cascade(
                    staged,
                    apply_plan.version_id,
                    entity_id,
                    patch_surface_policy,
                    schema_registry,
                    cascade_delete_policy,
                    &mut changed_records,
                    &mut patch_records,
                );
                let replacement_id = allocate_entity(
                    staged,
                    apply_plan.version_id,
                    replacement.partition_id,
                    replacement.kind_id,
                    replacement.payload.clone(),
                );
                staged.mark_entity_slot_touched(
                    replacement_id.partition_id,
                    replacement_id.local_slot.0 as usize,
                );
                write_entity_aspect_versions(
                    staged,
                    replacement_id,
                    apply_plan.version_id,
                    &replacement.payload,
                    symbols,
                );
                changed_records.push(RecordRef::Entity(replacement_id));
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::EntityUpdated,
                    message: "entity replaced".to_string(),
                    fields: json!({
                        "replaced_partition_id": entity_id.partition_id.0,
                        "replaced_entity_slot": entity_id.local_slot.0,
                        "replacement_partition_id": replacement_id.partition_id.0,
                        "replacement_entity_slot": replacement_id.local_slot.0,
                        "kind_id": replacement.kind_id.0,
                    }),
                });
                patch_records.push(PatchRecord {
                    kind: PatchRecordKind::EntityCreated,
                    entity_id: Some(replacement_id),
                    relation_id: None,
                    aspects: aspect_keys_for_payload(Some(&replacement.payload), symbols),
                    detail: patch_detail_for_entity(
                        patch_surface_policy,
                        PatchRecordKind::EntityCreated,
                        replacement_id,
                        Some(&replacement.payload),
                    ),
                });
            }
            TransactionIntent::DeleteEntity { entity_id } => {
                delete_entity_with_cascade(
                    staged,
                    apply_plan.version_id,
                    entity_id,
                    patch_surface_policy,
                    schema_registry,
                    cascade_delete_policy,
                    &mut changed_records,
                    &mut patch_records,
                );
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::EntityDeleted,
                    message: "entity deleted".to_string(),
                    fields: json!({
                        "partition_id": entity_id.partition_id.0,
                        "entity_slot": entity_id.local_slot.0,
                    }),
                });
            }
            TransactionIntent::CreateRelation(spec) => {
                let relation_id = allocate_relation(staged, apply_plan.version_id, &spec);
                staged.mark_relation_slot_touched(
                    relation_id.partition_id,
                    relation_id.local_slot.0 as usize,
                );
                staged.mark_adjacency_slot_touched(
                    spec.source.partition_id,
                    spec.source.local_slot.0 as usize,
                );
                staged.mark_reverse_adjacency_slot_touched(
                    spec.target.partition_id,
                    spec.target.local_slot.0 as usize,
                );
                changed_records.push(RecordRef::Relation(relation_id));
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::RelationCreated,
                    message: "relation created".to_string(),
                    fields: json!({
                        "partition_id": relation_id.partition_id.0,
                        "relation_slot": relation_id.local_slot.0,
                        "source_partition": spec.source.partition_id.0,
                        "source_slot": spec.source.local_slot.0,
                        "target_partition": spec.target.partition_id.0,
                        "target_slot": spec.target.local_slot.0,
                        "kind_id": spec.kind_id.0,
                    }),
                });
                patch_records.push(PatchRecord {
                    kind: PatchRecordKind::RelationCreated,
                    entity_id: None,
                    relation_id: Some(relation_id),
                    aspects: aspect_keys_for_payload(spec.payload.as_ref(), symbols),
                    detail: patch_detail_for_relation(
                        patch_surface_policy,
                        PatchRecordKind::RelationCreated,
                        relation_id,
                        spec.source,
                        spec.target,
                        spec.payload.as_ref(),
                    ),
                });
                write_relation_aspect_versions(
                    staged,
                    relation_id,
                    apply_plan.version_id,
                    spec.payload.as_ref(),
                    symbols,
                );
            }
            TransactionIntent::BulkCreateRelations {
                partition_id,
                kind_id,
                client_keys: _,
                endpoints,
                payloads,
            } => {
                reserve_bulk_relation_capacity(staged, partition_id, endpoints.len());
                for (index, (source, target)) in endpoints.into_iter().enumerate() {
                    let spec = RelationSpec {
                        partition_id,
                        kind_id,
                        client_key: crate::symbols::data::InternedString::from("bulk"),
                        source,
                        target,
                        payload: payloads.get(index).cloned().unwrap_or(None),
                    };
                    let relation_id = allocate_relation(staged, apply_plan.version_id, &spec);
                    staged.mark_relation_slot_touched(
                        relation_id.partition_id,
                        relation_id.local_slot.0 as usize,
                    );
                    staged.mark_adjacency_slot_touched(
                        spec.source.partition_id,
                        spec.source.local_slot.0 as usize,
                    );
                    staged.mark_reverse_adjacency_slot_touched(
                        spec.target.partition_id,
                        spec.target.local_slot.0 as usize,
                    );
                    changed_records.push(RecordRef::Relation(relation_id));
                    patch_records.push(PatchRecord {
                        kind: PatchRecordKind::RelationCreated,
                        entity_id: None,
                        relation_id: Some(relation_id),
                        aspects: aspect_keys_for_payload(spec.payload.as_ref(), symbols),
                        detail: patch_detail_for_relation(
                            patch_surface_policy,
                            PatchRecordKind::RelationCreated,
                            relation_id,
                            spec.source,
                            spec.target,
                            spec.payload.as_ref(),
                        ),
                    });
                    write_relation_aspect_versions(
                        staged,
                        relation_id,
                        apply_plan.version_id,
                        spec.payload.as_ref(),
                        symbols,
                    );
                }
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::RelationCreated,
                    message: "bulk relations created".to_string(),
                    fields: json!({"partition_id": partition_id.0, "kind_id": kind_id.0}),
                });
            }
            TransactionIntent::DeleteRelation { relation_id } => {
                delete_relation(
                    staged,
                    apply_plan.version_id,
                    relation_id,
                    patch_surface_policy,
                    &mut changed_records,
                    &mut patch_records,
                );
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::RelationDeleted,
                    message: "relation deleted".to_string(),
                    fields: json!({
                        "partition_id": relation_id.partition_id.0,
                        "relation_slot": relation_id.local_slot.0,
                    }),
                });
            }
        }
    }

    (changed_records, patch_records, diagnostics)
}

fn aspect_keys_for_payload(
    payload: Option<&crate::payloads::data::RecordPayload>,
    _symbols: &mut StringInterner,
) -> Vec<AspectKey> {
    aspect_names_for_payload(payload)
        .into_iter()
        .map(|name| AspectKey(InternedString::Raw(name)))
        .collect()
}

fn aspect_names_for_payload(payload: Option<&crate::payloads::data::RecordPayload>) -> Vec<String> {
    let mut aspects = BTreeSet::new();
    match payload {
        Some(crate::payloads::data::RecordPayload::StructuredJson(value)) => {
            if let Some(object) = value.as_object() {
                for key in object.keys() {
                    aspects.insert(key.clone());
                }
            }
        }
        Some(crate::payloads::data::RecordPayload::OpaqueBytes(_)) => {
            aspects.insert("opaque_payload".to_string());
        }
        None => {}
    }
    aspects.into_iter().collect()
}

fn write_entity_aspect_versions(
    staged: &mut WorkingState,
    entity_id: crate::identity::data::EntityId,
    version_id: crate::identity::data::VersionId,
    payload: &crate::payloads::data::RecordPayload,
    symbols: &mut StringInterner,
) {
    let slot = entity_id.local_slot.0 as usize;
    let partition = staged.get_partition_mut(entity_id.partition_id);
    let versions = &mut partition.entity_arena.aspect_versions[slot];
    for name in aspect_names_for_payload(Some(payload)) {
        versions.insert(symbols.intern(&name), version_id.0);
    }
}

fn write_relation_aspect_versions(
    staged: &mut WorkingState,
    relation_id: crate::identity::data::RelationId,
    version_id: crate::identity::data::VersionId,
    payload: Option<&crate::payloads::data::RecordPayload>,
    symbols: &mut StringInterner,
) {
    let slot = relation_id.local_slot.0 as usize;
    let partition = staged.get_partition_mut(relation_id.partition_id);
    let versions = &mut partition.relation_arena.aspect_versions[slot];
    for name in aspect_names_for_payload(payload) {
        versions.insert(symbols.intern(&name), version_id.0);
    }
}
