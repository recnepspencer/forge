use std::collections::{BTreeMap, BTreeSet};

use serde_json::json;

use crate::data::diagnostics::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::data::diff::{PatchDetail, PatchRecord, PatchRecordKind};
use crate::data::identity::{EntityId, KindId, PartitionId, RelationId};
use crate::data::payload::RecordPayload;
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
                let entity_id =
                    allocate_entity(staged, apply_plan.version_id, spec.partition_id, spec.kind_id, spec.payload.clone());
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
                    detail: PatchDetail::Payload(spec.payload),
                });
            }
            TransactionIntent::BulkCreateEntities {
                partition_id,
                kind_id,
                client_keys: _,
                payloads,
            } => {
                for payload in payloads {
                    let entity_id =
                        allocate_entity(staged, apply_plan.version_id, partition_id, kind_id, payload.clone());
                    changed_records.push(RecordRef::Entity(entity_id));
                    patch_records.push(PatchRecord {
                        kind: PatchRecordKind::EntityCreated,
                        entity_id: Some(entity_id),
                        relation_id: None,
                        detail: PatchDetail::Payload(payload),
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
                let slot = entity_id.local_slot.0 as usize;
                staged.entity_arena.payloads[slot] = Some(payload.clone());
                if let Some(current) = staged.entity_arena.payload_history[slot].last_mut() {
                    current.retired_at = Some(apply_plan.version_id);
                }
                staged.entity_arena.payload_history[slot].push(VersionedValue {
                    effective_at: apply_plan.version_id,
                    retired_at: None,
                    value: payload.clone(),
                });
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
                    detail: PatchDetail::Payload(payload),
                });
            }
            TransactionIntent::DeleteEntity { entity_id } => {
                delete_entity_with_cascade(staged, apply_plan.version_id, entity_id, &mut changed_records, &mut patch_records);
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
                    detail: match spec.payload {
                        Some(payload) => PatchDetail::Payload(payload),
                        None => PatchDetail::StructuredJson(json!({"payload_class":"topology_only"})),
                    },
                });
            }
            TransactionIntent::BulkCreateRelations {
                partition_id,
                kind_id,
                client_keys: _,
                endpoints,
                payloads,
            } => {
                for (index, (source, target)) in endpoints.into_iter().enumerate() {
                    let spec = RelationSpec {
                        partition_id,
                        kind_id,
                        client_key: crate::data::symbols::InternedString::from("bulk"),
                        source,
                        target,
                        payload: payloads.get(index).cloned().unwrap_or(None),
                    };
                    let relation_id = allocate_relation(staged, apply_plan.version_id, &spec);
                    changed_records.push(RecordRef::Relation(relation_id));
                    patch_records.push(PatchRecord {
                        kind: PatchRecordKind::RelationCreated,
                        entity_id: None,
                        relation_id: Some(relation_id),
                        detail: match spec.payload {
                            Some(payload) => PatchDetail::Payload(payload),
                            None => PatchDetail::StructuredJson(json!({"payload_class":"topology_only"})),
                        },
                    });
                }
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::RelationCreated,
                    message: "bulk relations created".to_string(),
                    fields: json!({"partition_id": partition_id.0, "kind_id": kind_id.0}),
                });
            }
            TransactionIntent::DeleteRelation { relation_id } => {
                delete_relation(staged, apply_plan.version_id, relation_id, &mut changed_records, &mut patch_records);
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

fn allocate_entity(
    staged: &mut WorkingState,
    version_id: crate::data::identity::VersionId,
    partition_id: PartitionId,
    kind_id: KindId,
    payload: RecordPayload,
) -> EntityId {
    if let Some(slot) = staged.entity_arena.free_list.pop() {
        let idx = slot as usize;
        staged.entity_arena.partition_ids[idx] = partition_id;
        staged.entity_arena.lifecycle[idx] = RecordLifecycleState::Live;
        staged.entity_arena.live_bitset.set(idx, true);
        staged.entity_arena.reclaimable_bitset.set(idx, false);
        staged.entity_arena.kind_ids[idx] = Some(kind_id);
        staged.entity_arena.payloads[idx] = Some(payload.clone());
        staged.entity_arena.payload_history[idx] = vec![VersionedValue {
            effective_at: version_id,
            retired_at: None,
            value: payload,
        }];
        staged.entity_arena.created_at[idx] = version_id;
        staged.entity_arena.retired_at[idx] = None;
        staged.entity_arena.generations[idx] += 1;
        staged.entity_arena.aspect_versions[idx].clear();
        staged.entity_arena.structural_fingerprints[idx] = None;
        staged.entity_arena.lineage_ids[idx] = None;
        staged.entity_arena.diagnostics_enrichment[idx].clear();
        staged.adjacency[idx].clear();
        staged.reverse_adjacency[idx].clear();
        return EntityId::new(partition_id, slot, staged.entity_arena.generations[idx]);
    }
    let slot = staged.entity_arena.generations.len() as u64;
    staged.entity_arena.partition_ids.push(partition_id);
    staged.entity_arena.generations.push(1);
    staged.entity_arena.lifecycle.push(RecordLifecycleState::Live);
    staged.entity_arena.live_bitset.set(slot as usize, true);
    staged.entity_arena.reclaimable_bitset.set(slot as usize, false);
    staged.entity_arena.kind_ids.push(Some(kind_id));
    staged.entity_arena.payloads.push(Some(payload.clone()));
    staged.entity_arena.payload_history.push(vec![VersionedValue {
        effective_at: version_id,
        retired_at: None,
        value: payload,
    }]);
    staged.entity_arena.created_at.push(version_id);
    staged.entity_arena.retired_at.push(None);
    staged.entity_arena.aspect_versions.push(BTreeMap::new());
    staged.entity_arena.structural_fingerprints.push(None);
    staged.entity_arena.lineage_ids.push(None);
    staged.entity_arena.diagnostics_enrichment.push(BTreeMap::new());
    staged.entity_arena.branch_pins.push(0);
    staged.entity_arena.replay_pins.push(0);
    staged.entity_arena.snapshot_pins.push(0);
    staged.adjacency.push(BTreeSet::new());
    staged.reverse_adjacency.push(BTreeSet::new());
    EntityId::new(partition_id, slot, 1)
}

fn allocate_relation(
    staged: &mut WorkingState,
    version_id: crate::data::identity::VersionId,
    spec: &RelationSpec,
) -> RelationId {
    if let Some(slot) = staged.relation_arena.free_list.pop() {
        let idx = slot as usize;
        staged.relation_arena.partition_ids[idx] = spec.partition_id;
        staged.relation_arena.lifecycle[idx] = RecordLifecycleState::Live;
        staged.relation_arena.live_bitset.set(idx, true);
        staged.relation_arena.reclaimable_bitset.set(idx, false);
        staged.relation_arena.kind_ids[idx] = Some(spec.kind_id);
        staged.relation_arena.payloads[idx] = spec.payload.clone();
        if let Some(payload) = spec.payload.clone() {
            staged.relation_arena.payload_history.insert(
                idx,
                vec![VersionedValue {
                    effective_at: version_id,
                    retired_at: None,
                    value: payload,
                }],
            );
        } else {
            staged.relation_arena.payload_history.remove(&idx);
        }
        staged.relation_arena.created_at[idx] = version_id;
        staged.relation_arena.retired_at[idx] = None;
        staged.relation_arena.endpoints[idx] = Some(RelationEndpoints {
            source: spec.source,
            target: spec.target,
        });
        staged.relation_arena.diagnostics_enrichment[idx].clear();
        staged.relation_arena.generations[idx] += 1;
        let relation_id = RelationId::new(spec.partition_id, slot, staged.relation_arena.generations[idx]);
        staged.adjacency[spec.source.local_slot.0 as usize].insert(relation_id);
        staged.reverse_adjacency[spec.target.local_slot.0 as usize].insert(relation_id);
        return relation_id;
    }
    let slot = staged.relation_arena.generations.len() as u64;
    staged.relation_arena.partition_ids.push(spec.partition_id);
    staged.relation_arena.generations.push(1);
    staged.relation_arena.lifecycle.push(RecordLifecycleState::Live);
    staged.relation_arena.live_bitset.set(slot as usize, true);
    staged.relation_arena.reclaimable_bitset.set(slot as usize, false);
    staged.relation_arena.kind_ids.push(Some(spec.kind_id));
    staged.relation_arena.payloads.push(spec.payload.clone());
    if let Some(payload) = spec.payload.clone() {
        staged.relation_arena.payload_history.insert(
            slot as usize,
            vec![VersionedValue {
                effective_at: version_id,
                retired_at: None,
                value: payload,
            }],
        );
    }
    staged.relation_arena.created_at.push(version_id);
    staged.relation_arena.retired_at.push(None);
    staged.relation_arena.endpoints.push(Some(RelationEndpoints {
        source: spec.source,
        target: spec.target,
    }));
    staged.relation_arena.diagnostics_enrichment.push(BTreeMap::new());
    staged.relation_arena.snapshot_pins.push(0);
    let relation_id = RelationId::new(spec.partition_id, slot, 1);
    staged.adjacency[spec.source.local_slot.0 as usize].insert(relation_id);
    staged.reverse_adjacency[spec.target.local_slot.0 as usize].insert(relation_id);
    relation_id
}

fn delete_entity_with_cascade(
    staged: &mut WorkingState,
    version_id: crate::data::identity::VersionId,
    entity_id: EntityId,
    changed_records: &mut Vec<RecordRef>,
    patch_records: &mut Vec<PatchRecord>,
) {
    let slot = entity_id.local_slot.0 as usize;
    staged.entity_arena.retired_at[slot] = Some(version_id);
    staged.entity_arena.lifecycle[slot] = RecordLifecycleState::DeletedRetained;
    staged.entity_arena.live_bitset.set(slot, false);
    staged.entity_arena.reclaimable_bitset.set(slot, true);
    if let Some(current) = staged.entity_arena.payload_history[slot].last_mut() {
        current.retired_at = Some(version_id);
    }
    changed_records.push(RecordRef::Entity(entity_id));
    patch_records.push(PatchRecord {
        kind: PatchRecordKind::EntityDeleted,
        entity_id: Some(entity_id),
        relation_id: None,
        detail: PatchDetail::StructuredJson(json!({})),
    });

    let mut attached = staged.adjacency[slot].clone();
    attached.extend(staged.reverse_adjacency[slot].iter().copied());
    for relation_id in attached {
        delete_relation(staged, version_id, relation_id, changed_records, patch_records);
    }
}

fn delete_relation(
    staged: &mut WorkingState,
    version_id: crate::data::identity::VersionId,
    relation_id: RelationId,
    changed_records: &mut Vec<RecordRef>,
    patch_records: &mut Vec<PatchRecord>,
) {
    let slot = relation_id.local_slot.0 as usize;
    if staged.relation_arena.lifecycle[slot] != RecordLifecycleState::Live {
        return;
    }
    staged.relation_arena.retired_at[slot] = Some(version_id);
    staged.relation_arena.lifecycle[slot] = RecordLifecycleState::DeletedRetained;
    staged.relation_arena.live_bitset.set(slot, false);
    staged.relation_arena.reclaimable_bitset.set(slot, true);
    if let Some(current) = staged
        .relation_arena
        .payload_history
        .get_mut(&slot)
        .and_then(|history| history.last_mut())
    {
        current.retired_at = Some(version_id);
    }
    if let Some(endpoints) = staged.relation_arena.endpoints[slot].as_ref() {
        staged.adjacency[endpoints.source.local_slot.0 as usize].remove(&relation_id);
        staged.reverse_adjacency[endpoints.target.local_slot.0 as usize].remove(&relation_id);
    }
    changed_records.push(RecordRef::Relation(relation_id));
    patch_records.push(PatchRecord {
        kind: PatchRecordKind::RelationDeleted,
        entity_id: None,
        relation_id: Some(relation_id),
        detail: PatchDetail::StructuredJson(json!({})),
    });
}
