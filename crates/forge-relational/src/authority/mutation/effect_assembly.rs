use serde_json::json;

use crate::authority::mutation::aspect_versions::aspect_keys_for_payload;
use crate::authority::mutation::patch_details::{
    patch_detail_for_entity, patch_detail_for_relation,
};
use crate::diagnostics::data::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::publication::data::diff::{PatchRecord, PatchRecordKind};
use crate::transactions::data::RecordRef;

use super::outcomes::{MutationEvent, MutationOutcome, RecordMutation};
use super::{AdjacencyDelta, AdjacencyDeltaKind, MutationEffect, MutationWorkspace};

pub(crate) fn assemble_effect(
    outcome: MutationOutcome,
    workspace: &mut MutationWorkspace<'_>,
) -> MutationEffect {
    let patch_surface_policy = workspace.patch_surface_policy();
    let mut effect = MutationEffect::default();

    for change in outcome.changes {
        match change {
            RecordMutation::EntityCreated { entity_id, payload } => {
                let aspects = workspace.with_context(|context| {
                    aspect_keys_for_payload(Some(&payload), context.symbols)
                });
                effect
                    .publication
                    .changed_records
                    .push(RecordRef::Entity(entity_id));
                effect.publication.patch_records.push(PatchRecord {
                    kind: PatchRecordKind::Created,
                    target: RecordRef::Entity(entity_id),
                    aspects,
                    detail: patch_detail_for_entity(
                        patch_surface_policy,
                        PatchRecordKind::Created,
                        entity_id,
                        Some(&payload),
                    ),
                });
            }
            RecordMutation::EntityUpdated { entity_id, payload } => {
                let aspects = workspace.with_context(|context| {
                    aspect_keys_for_payload(Some(&payload), context.symbols)
                });
                effect
                    .publication
                    .changed_records
                    .push(RecordRef::Entity(entity_id));
                effect.publication.patch_records.push(PatchRecord {
                    kind: PatchRecordKind::Updated,
                    target: RecordRef::Entity(entity_id),
                    aspects,
                    detail: patch_detail_for_entity(
                        patch_surface_policy,
                        PatchRecordKind::Updated,
                        entity_id,
                        Some(&payload),
                    ),
                });
            }
            RecordMutation::EntityDeleted { entity_id } => {
                effect
                    .publication
                    .changed_records
                    .push(RecordRef::Entity(entity_id));
                effect.publication.patch_records.push(PatchRecord {
                    kind: PatchRecordKind::Deleted,
                    target: RecordRef::Entity(entity_id),
                    aspects: Vec::new(),
                    detail: patch_detail_for_entity(
                        patch_surface_policy,
                        PatchRecordKind::Deleted,
                        entity_id,
                        None,
                    ),
                });
            }
            RecordMutation::RelationCreated {
                relation_id,
                source,
                target,
                payload,
            } => {
                let aspects = workspace.with_context(|context| {
                    aspect_keys_for_payload(payload.as_ref(), context.symbols)
                });
                effect
                    .publication
                    .changed_records
                    .push(RecordRef::Relation(relation_id));
                effect.adjacency.deltas.push(AdjacencyDelta {
                    relation_id,
                    kind: AdjacencyDeltaKind::Created { source, target },
                });
                effect.publication.patch_records.push(PatchRecord {
                    kind: PatchRecordKind::Created,
                    target: RecordRef::Relation(relation_id),
                    aspects,
                    detail: patch_detail_for_relation(
                        patch_surface_policy,
                        PatchRecordKind::Created,
                        relation_id,
                        source,
                        target,
                        payload.as_ref(),
                    ),
                });
            }
            RecordMutation::RelationDeleted {
                relation_id,
                source,
                target,
            } => {
                effect
                    .publication
                    .changed_records
                    .push(RecordRef::Relation(relation_id));
                effect.adjacency.deltas.push(AdjacencyDelta {
                    relation_id,
                    kind: AdjacencyDeltaKind::Deleted { source, target },
                });
                effect.publication.patch_records.push(PatchRecord {
                    kind: PatchRecordKind::Deleted,
                    target: RecordRef::Relation(relation_id),
                    aspects: Vec::new(),
                    detail: patch_detail_for_relation(
                        patch_surface_policy,
                        PatchRecordKind::Deleted,
                        relation_id,
                        source,
                        target,
                        None,
                    ),
                });
            }
            RecordMutation::RelationRetainedForAudit {
                relation_id,
                source,
                target,
                payload,
            } => {
                effect
                    .publication
                    .changed_records
                    .push(RecordRef::Relation(relation_id));
                effect.publication.patch_records.push(PatchRecord {
                    kind: PatchRecordKind::RetainedForAudit,
                    target: RecordRef::Relation(relation_id),
                    aspects: Vec::new(),
                    detail: patch_detail_for_relation(
                        patch_surface_policy,
                        PatchRecordKind::RetainedForAudit,
                        relation_id,
                        source,
                        target,
                        payload.as_ref(),
                    ),
                });
            }
        }
    }

    for event in outcome.events {
        effect.diagnostics.entries.push(event_diagnostic(event));
    }

    effect
}

fn event_diagnostic(event: MutationEvent) -> RelationalDiagnosticsEntry {
    match event {
        MutationEvent::EntityCreated { entity_id, kind_id } => RelationalDiagnosticsEntry {
            code: DiagnosticCode::EntityCreated,
            message: "entity created".to_string(),
            fields: json!({
                "partition_id": entity_id.partition_id.0,
                "entity_slot": entity_id.local_slot.0,
                "kind_id": kind_id.0,
            }),
        },
        MutationEvent::BulkEntitiesCreated {
            partition_id,
            kind_id,
            count,
        } => RelationalDiagnosticsEntry {
            code: DiagnosticCode::EntityCreated,
            message: "bulk entities created".to_string(),
            fields: json!({
                "partition_id": partition_id.0,
                "kind_id": kind_id.0,
                "count": count,
            }),
        },
        MutationEvent::EntityUpdated { entity_id } => RelationalDiagnosticsEntry {
            code: DiagnosticCode::EntityUpdated,
            message: "entity updated".to_string(),
            fields: json!({
                "partition_id": entity_id.partition_id.0,
                "entity_slot": entity_id.local_slot.0,
            }),
        },
        MutationEvent::EntityReplaced {
            replaced_entity_id,
            replacement_entity_id,
            kind_id,
        } => RelationalDiagnosticsEntry {
            code: DiagnosticCode::EntityUpdated,
            message: "entity replaced".to_string(),
            fields: json!({
                "replaced_partition_id": replaced_entity_id.partition_id.0,
                "replaced_entity_slot": replaced_entity_id.local_slot.0,
                "replacement_partition_id": replacement_entity_id.partition_id.0,
                "replacement_entity_slot": replacement_entity_id.local_slot.0,
                "kind_id": kind_id.0,
            }),
        },
        MutationEvent::EntityDeleted { entity_id } => RelationalDiagnosticsEntry {
            code: DiagnosticCode::EntityDeleted,
            message: "entity deleted".to_string(),
            fields: json!({
                "partition_id": entity_id.partition_id.0,
                "entity_slot": entity_id.local_slot.0,
            }),
        },
        MutationEvent::RelationCreated {
            relation_id,
            source,
            target,
            kind_id,
        } => RelationalDiagnosticsEntry {
            code: DiagnosticCode::RelationCreated,
            message: "relation created".to_string(),
            fields: json!({
                "partition_id": relation_id.partition_id.0,
                "relation_slot": relation_id.local_slot.0,
                "source_partition": source.partition_id.0,
                "source_slot": source.local_slot.0,
                "target_partition": target.partition_id.0,
                "target_slot": target.local_slot.0,
                "kind_id": kind_id.0,
            }),
        },
        MutationEvent::BulkRelationsCreated {
            partition_id,
            kind_id,
            count,
        } => RelationalDiagnosticsEntry {
            code: DiagnosticCode::RelationCreated,
            message: "bulk relations created".to_string(),
            fields: json!({
                "partition_id": partition_id.0,
                "kind_id": kind_id.0,
                "count": count,
            }),
        },
        MutationEvent::RelationDeleted { relation_id } => RelationalDiagnosticsEntry {
            code: DiagnosticCode::RelationDeleted,
            message: "relation deleted".to_string(),
            fields: json!({
                "partition_id": relation_id.partition_id.0,
                "relation_slot": relation_id.local_slot.0,
            }),
        },
    }
}
