use std::collections::{BTreeMap, BTreeSet};

use serde_json::json;

use crate::data::diagnostics::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::data::diff::{PatchRecord, PatchRecordKind};
use crate::data::identity::{EntityId, KindId, RelationId};
use crate::data::transaction::{
    AuthoritativeApplyPlan, RecordRef, RelationSpec, TransactionIntent,
};
use crate::logic::runtime::RecordLifecycleState;

use super::state::{RelationEndpoints, VersionedValue, WorkingState};

pub(super) fn apply_plan_to_staged_state(
    staged: &mut WorkingState,
    apply_plan: &AuthoritativeApplyPlan,
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
                    spec.kind_id,
                    spec.payload.clone(),
                );
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::EntityCreated,
                    message: "entity created".to_string(),
                    fields: json!({ "entity_slot": entity_id.slot.0, "kind_id": spec.kind_id.0, "client_key": spec.client_key }),
                });
                changed_records.push(RecordRef::Entity(entity_id));
                patch_records.push(PatchRecord {
                    kind: PatchRecordKind::EntityCreated,
                    entity_id: Some(entity_id),
                    relation_id: None,
                    detail: json!({ "kind_id": spec.kind_id.0, "client_key": spec.client_key }),
                });
            }
            TransactionIntent::UpdateEntity { entity_id, payload } => {
                let slot = entity_id.slot.0 as usize;
                staged.entity_arena.payloads[slot] = Some(payload.clone());
                if let Some(current) = staged.entity_arena.payload_history[slot].last_mut() {
                    current.retired_at = Some(apply_plan.version_id);
                }
                staged.entity_arena.payload_history[slot].push(VersionedValue {
                    effective_at: apply_plan.version_id,
                    retired_at: None,
                    value: payload,
                });
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::EntityUpdated,
                    message: "entity updated".to_string(),
                    fields: json!({ "entity_slot": entity_id.slot.0 }),
                });
                changed_records.push(RecordRef::Entity(entity_id));
                patch_records.push(PatchRecord {
                    kind: PatchRecordKind::EntityUpdated,
                    entity_id: Some(entity_id),
                    relation_id: None,
                    detail: json!({}),
                });
            }
            TransactionIntent::DeleteEntity { entity_id } => {
                let slot = entity_id.slot.0 as usize;
                staged.entity_arena.retired_at[slot] = Some(apply_plan.version_id);
                staged.entity_arena.lifecycle[slot] = RecordLifecycleState::DeletedRetained;
                if let Some(current) = staged.entity_arena.payload_history[slot].last_mut() {
                    current.retired_at = Some(apply_plan.version_id);
                }
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::EntityDeleted,
                    message: "entity deleted".to_string(),
                    fields: json!({ "entity_slot": entity_id.slot.0 }),
                });
                changed_records.push(RecordRef::Entity(entity_id));
                patch_records.push(PatchRecord {
                    kind: PatchRecordKind::EntityDeleted,
                    entity_id: Some(entity_id),
                    relation_id: None,
                    detail: json!({}),
                });
            }
            TransactionIntent::CreateRelation(spec) => {
                let relation_id = allocate_relation(staged, apply_plan.version_id, &spec);
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::RelationCreated,
                    message: "relation created".to_string(),
                    fields: json!({ "relation_slot": relation_id.slot.0, "source_slot": spec.source.slot.0, "target_slot": spec.target.slot.0, "kind_id": spec.kind_id.0 }),
                });
                changed_records.push(RecordRef::Relation(relation_id));
                patch_records.push(PatchRecord {
                    kind: PatchRecordKind::RelationCreated,
                    entity_id: None,
                    relation_id: Some(relation_id),
                    detail: json!({ "kind_id": spec.kind_id.0, "client_key": spec.client_key }),
                });
            }
            TransactionIntent::DeleteRelation { relation_id } => {
                let slot = relation_id.slot.0 as usize;
                staged.relation_arena.retired_at[slot] = Some(apply_plan.version_id);
                staged.relation_arena.lifecycle[slot] = RecordLifecycleState::DeletedRetained;
                if let Some(current) = staged.relation_arena.payload_history[slot].last_mut() {
                    current.retired_at = Some(apply_plan.version_id);
                }
                if let Some(endpoints) = staged.relation_arena.endpoints[slot].as_ref() {
                    staged.adjacency[endpoints.source.slot.0 as usize].remove(&relation_id);
                }
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::RelationDeleted,
                    message: "relation deleted".to_string(),
                    fields: json!({ "relation_slot": relation_id.slot.0 }),
                });
                changed_records.push(RecordRef::Relation(relation_id));
                patch_records.push(PatchRecord {
                    kind: PatchRecordKind::RelationDeleted,
                    entity_id: None,
                    relation_id: Some(relation_id),
                    detail: json!({}),
                });
            }
        }
    }

    (changed_records, patch_records, diagnostics)
}

fn allocate_entity(
    staged: &mut WorkingState,
    version_id: crate::data::identity::VersionId,
    kind_id: KindId,
    payload: serde_json::Value,
) -> EntityId {
    if let Some(slot) = staged.entity_arena.free_list.pop() {
        let idx = slot as usize;
        staged.entity_arena.lifecycle[idx] = RecordLifecycleState::Live;
        staged.entity_arena.kind_ids[idx] = Some(kind_id);
        staged.entity_arena.payloads[idx] = Some(payload);
        staged.entity_arena.payload_history[idx] = vec![VersionedValue {
            effective_at: version_id,
            retired_at: None,
            value: staged.entity_arena.payloads[idx]
                .clone()
                .expect("payload stored for reused entity"),
        }];
        staged.entity_arena.created_at[idx] = version_id;
        staged.entity_arena.retired_at[idx] = None;
        staged.entity_arena.generations[idx] += 1;
        return EntityId::new(slot, staged.entity_arena.generations[idx]);
    }
    let slot = staged.entity_arena.generations.len() as u64;
    staged.entity_arena.generations.push(1);
    staged
        .entity_arena
        .lifecycle
        .push(RecordLifecycleState::Live);
    staged.entity_arena.kind_ids.push(Some(kind_id));
    staged.entity_arena.payloads.push(Some(payload));
    staged
        .entity_arena
        .payload_history
        .push(vec![VersionedValue {
            effective_at: version_id,
            retired_at: None,
            value: staged.entity_arena.payloads[slot as usize]
                .clone()
                .expect("payload stored for entity"),
        }]);
    staged.entity_arena.created_at.push(version_id);
    staged.entity_arena.retired_at.push(None);
    staged.entity_arena.aspect_versions.push(BTreeMap::new());
    staged.entity_arena.structural_fingerprints.push(None);
    staged.entity_arena.lineage_ids.push(None);
    staged
        .entity_arena
        .diagnostics_enrichment
        .push(BTreeMap::new());
    staged.entity_arena.branch_pins.push(0);
    staged.entity_arena.replay_pins.push(0);
    staged.entity_arena.snapshot_pins.push(0);
    staged.adjacency.push(BTreeSet::new());
    EntityId::new(slot, 1)
}

fn allocate_relation(
    staged: &mut WorkingState,
    version_id: crate::data::identity::VersionId,
    spec: &RelationSpec,
) -> RelationId {
    if let Some(slot) = staged.relation_arena.free_list.pop() {
        let idx = slot as usize;
        staged.relation_arena.lifecycle[idx] = RecordLifecycleState::Live;
        staged.relation_arena.kind_ids[idx] = Some(spec.kind_id);
        staged.relation_arena.payloads[idx] = Some(spec.payload.clone());
        staged.relation_arena.payload_history[idx] = vec![VersionedValue {
            effective_at: version_id,
            retired_at: None,
            value: spec.payload.clone(),
        }];
        staged.relation_arena.created_at[idx] = version_id;
        staged.relation_arena.retired_at[idx] = None;
        staged.relation_arena.endpoints[idx] = Some(RelationEndpoints {
            source: spec.source,
            target: spec.target,
        });
        staged.relation_arena.generations[idx] += 1;
        let relation_id = RelationId::new(slot, staged.relation_arena.generations[idx]);
        staged.adjacency[spec.source.slot.0 as usize].insert(relation_id);
        return relation_id;
    }
    let slot = staged.relation_arena.generations.len() as u64;
    staged.relation_arena.generations.push(1);
    staged
        .relation_arena
        .lifecycle
        .push(RecordLifecycleState::Live);
    staged.relation_arena.kind_ids.push(Some(spec.kind_id));
    staged
        .relation_arena
        .payloads
        .push(Some(spec.payload.clone()));
    staged
        .relation_arena
        .payload_history
        .push(vec![VersionedValue {
            effective_at: version_id,
            retired_at: None,
            value: spec.payload.clone(),
        }]);
    staged.relation_arena.created_at.push(version_id);
    staged.relation_arena.retired_at.push(None);
    staged
        .relation_arena
        .endpoints
        .push(Some(RelationEndpoints {
            source: spec.source,
            target: spec.target,
        }));
    staged
        .relation_arena
        .diagnostics_enrichment
        .push(BTreeMap::new());
    staged.relation_arena.snapshot_pins.push(0);
    let relation_id = RelationId::new(slot, 1);
    staged.adjacency[spec.source.slot.0 as usize].insert(relation_id);
    relation_id
}
