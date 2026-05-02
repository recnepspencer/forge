use serde_json::json;

use crate::authority::mutation::aspect_versions::write_aspect_versions_for_delta;
use crate::authority::mutation::canonical_deltas::canonical_delta_for_mutation;
use crate::authority::mutation::patch_details::{
    patch_detail_for_entity, patch_detail_for_relation, EntityPatchDetailKind,
    RelationPatchDetailKind,
};
use crate::diagnostics::data::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::publication::data::diff::{PatchRecord, PatchRecordKind, RecordStructuralChange};
use crate::transactions::data::RecordRef;

use super::outcomes::{MutationEvent, MutationOutcome, RecordMutation};
use super::{AdjacencyDelta, AdjacencyDeltaKind, MutationEffect, MutationWorkspace};

pub(crate) fn assemble_effect(
    outcome: MutationOutcome,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationEffect, crate::transactions::data::CommitConflict> {
    let patch_surface_policy = workspace.patch_surface_policy();
    let version_id = workspace.version_id();
    let mut effect = MutationEffect::with_capacity(outcome.changes.len(), outcome.events.len());

    for change in outcome.changes {
        let canonical_delta = canonical_delta_for_mutation(&change, workspace)
            .map_err(|error| error.to_commit_conflict())?;
        workspace
            .with_context(|context| {
                write_aspect_versions_for_delta(
                    context.state,
                    &canonical_delta,
                    version_id,
                    context.symbols,
                )
            })
            .map_err(|error| error.to_commit_conflict())?;
        let patch_aspects = canonical_delta.changed_aspects.clone();
        let contains_degraded_precision = canonical_delta.contains_degraded_precision;
        match change {
            RecordMutation::EntityCreated {
                entity_id, payload, ..
            } => {
                effect
                    .publication
                    .changed_records
                    .push(RecordRef::Entity(entity_id));
                effect.publication.canonical_deltas.push(canonical_delta);
                effect.publication.patch_records.push(PatchRecord {
                    kind: PatchRecordKind::Created,
                    target: RecordRef::Entity(entity_id),
                    structural_change: RecordStructuralChange::Created,
                    aspects: patch_aspects,
                    contains_degraded_precision,
                    detail: patch_detail_for_entity(
                        patch_surface_policy,
                        EntityPatchDetailKind::Created,
                        entity_id,
                        Some(&payload),
                    ),
                });
            }
            RecordMutation::EntityUpdated {
                entity_id,
                new_payload,
                ..
            } => {
                effect
                    .publication
                    .changed_records
                    .push(RecordRef::Entity(entity_id));
                effect.publication.canonical_deltas.push(canonical_delta);
                effect.publication.patch_records.push(PatchRecord {
                    kind: PatchRecordKind::Updated,
                    target: RecordRef::Entity(entity_id),
                    structural_change: RecordStructuralChange::Updated,
                    aspects: patch_aspects,
                    contains_degraded_precision,
                    detail: patch_detail_for_entity(
                        patch_surface_policy,
                        EntityPatchDetailKind::Updated,
                        entity_id,
                        Some(&new_payload),
                    ),
                });
            }
            RecordMutation::EntityDeleted { entity_id, .. } => {
                effect
                    .publication
                    .changed_records
                    .push(RecordRef::Entity(entity_id));
                effect.publication.canonical_deltas.push(canonical_delta);
                effect.publication.patch_records.push(PatchRecord {
                    kind: PatchRecordKind::Deleted,
                    target: RecordRef::Entity(entity_id),
                    structural_change: RecordStructuralChange::Deleted,
                    aspects: patch_aspects,
                    contains_degraded_precision,
                    detail: patch_detail_for_entity(
                        patch_surface_policy,
                        EntityPatchDetailKind::Deleted,
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
                ..
            } => {
                effect
                    .publication
                    .changed_records
                    .push(RecordRef::Relation(relation_id));
                effect.publication.canonical_deltas.push(canonical_delta);
                effect.adjacency.deltas.push(AdjacencyDelta {
                    relation_id,
                    kind: AdjacencyDeltaKind::Created { source, target },
                });
                effect.publication.patch_records.push(PatchRecord {
                    kind: PatchRecordKind::Created,
                    target: RecordRef::Relation(relation_id),
                    structural_change: RecordStructuralChange::Created,
                    aspects: patch_aspects,
                    contains_degraded_precision,
                    detail: patch_detail_for_relation(
                        patch_surface_policy,
                        RelationPatchDetailKind::Created,
                        relation_id,
                        source,
                        target,
                        payload.as_ref(),
                    ),
                });
            }
            RecordMutation::RelationUpdated {
                relation_id,
                old_source,
                old_target,
                new_source,
                new_target,
                payload,
                ..
            } => {
                effect
                    .publication
                    .changed_records
                    .push(RecordRef::Relation(relation_id));
                effect.publication.canonical_deltas.push(canonical_delta);
                effect.adjacency.deltas.push(AdjacencyDelta {
                    relation_id,
                    kind: AdjacencyDeltaKind::Deleted {
                        source: old_source,
                        target: old_target,
                    },
                });
                effect.adjacency.deltas.push(AdjacencyDelta {
                    relation_id,
                    kind: AdjacencyDeltaKind::Created {
                        source: new_source,
                        target: new_target,
                    },
                });
                effect.publication.patch_records.push(PatchRecord {
                    kind: PatchRecordKind::Updated,
                    target: RecordRef::Relation(relation_id),
                    structural_change: RecordStructuralChange::Updated,
                    aspects: patch_aspects,
                    contains_degraded_precision,
                    detail: patch_detail_for_relation(
                        patch_surface_policy,
                        RelationPatchDetailKind::Updated,
                        relation_id,
                        new_source,
                        new_target,
                        payload.as_ref(),
                    ),
                });
            }
            RecordMutation::RelationDeleted {
                relation_id,
                source,
                target,
                ..
            } => {
                effect
                    .publication
                    .changed_records
                    .push(RecordRef::Relation(relation_id));
                effect.publication.canonical_deltas.push(canonical_delta);
                effect.adjacency.deltas.push(AdjacencyDelta {
                    relation_id,
                    kind: AdjacencyDeltaKind::Deleted { source, target },
                });
                effect.publication.patch_records.push(PatchRecord {
                    kind: PatchRecordKind::Deleted,
                    target: RecordRef::Relation(relation_id),
                    structural_change: RecordStructuralChange::Deleted,
                    aspects: patch_aspects,
                    contains_degraded_precision,
                    detail: patch_detail_for_relation(
                        patch_surface_policy,
                        RelationPatchDetailKind::Deleted,
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
                ..
            } => {
                effect
                    .publication
                    .changed_records
                    .push(RecordRef::Relation(relation_id));
                effect.publication.canonical_deltas.push(canonical_delta);
                effect.publication.patch_records.push(PatchRecord {
                    kind: PatchRecordKind::RetainedForAudit,
                    target: RecordRef::Relation(relation_id),
                    structural_change: RecordStructuralChange::RetainedForAudit,
                    aspects: patch_aspects,
                    contains_degraded_precision,
                    detail: patch_detail_for_relation(
                        patch_surface_policy,
                        RelationPatchDetailKind::RetainedForAudit,
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

    Ok(effect)
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
        MutationEvent::RelationUpdated { relation_id } => RelationalDiagnosticsEntry {
            code: DiagnosticCode::RelationUpdated,
            message: "relation updated".to_string(),
            fields: json!({
                "partition_id": relation_id.partition_id.0,
                "relation_slot": relation_id.local_slot.0,
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
