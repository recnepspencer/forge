use std::collections::{BTreeMap, BTreeSet};

use serde_json::json;

use crate::config::data::PatchSurfacePolicy;
use crate::diagnostics::data::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::identity::data::{EntityId, KindId, PartitionId, RelationId};
use crate::payloads::data::RecordPayload;
use crate::publication::data::diff::{PatchDetail, PatchRecord, PatchRecordKind};
use crate::schema::data::RelationalSchemaRegistry;
use crate::storage::data::RecordLifecycleState;
use crate::storage::logic::state::{
    AdjacencySet, PartitionAccess, PartitionState, RelationEndpoints, VersionedValue, WorkingState,
};
use crate::transactions::data::{
    AuthoritativeApplyPlan, RecordRef, RelationSpec, TransactionIntent,
};

pub(crate) fn apply_plan_to_staged_state(
    staged: &mut WorkingState,
    apply_plan: &AuthoritativeApplyPlan,
    patch_surface_policy: PatchSurfacePolicy,
    schema_registry: &RelationalSchemaRegistry,
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
                    changed_records.push(RecordRef::Entity(entity_id));
                    patch_records.push(PatchRecord {
                        kind: PatchRecordKind::EntityCreated,
                        entity_id: Some(entity_id),
                        relation_id: None,
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
                partition.entity_arena.payloads[slot] = Some(payload.clone());
                if let Some(current) = partition.entity_arena.payload_history[slot].last_mut() {
                    current.retired_at = Some(apply_plan.version_id);
                }
                partition.entity_arena.payload_history[slot].push(VersionedValue {
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
                    detail: patch_detail_for_relation(
                        patch_surface_policy,
                        PatchRecordKind::RelationCreated,
                        relation_id,
                        spec.source,
                        spec.target,
                        spec.payload.as_ref(),
                    ),
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
                        detail: patch_detail_for_relation(
                            patch_surface_policy,
                            PatchRecordKind::RelationCreated,
                            relation_id,
                            spec.source,
                            spec.target,
                            spec.payload.as_ref(),
                        ),
                    });
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

fn allocate_entity(
    staged: &mut WorkingState,
    version_id: crate::identity::data::VersionId,
    partition_id: PartitionId,
    kind_id: KindId,
    payload: RecordPayload,
) -> EntityId {
    let payload = payload.canonicalized();
    let partition = ensure_partition_state(staged, partition_id);
    if let Some(slot) = partition.entity_arena.free_list.pop() {
        let idx = slot as usize;
        partition.entity_arena.partition_ids[idx] = partition_id;
        partition.entity_arena.lifecycle[idx] = RecordLifecycleState::Live;
        partition.entity_arena.live_bitset.set(idx, true);
        partition.entity_arena.reclaimable_bitset.set(idx, false);
        partition.entity_arena.kind_ids[idx] = Some(kind_id);
        partition.entity_arena.payloads[idx] = Some(payload.clone());
        partition.entity_arena.payload_history[idx] = vec![VersionedValue {
            effective_at: version_id,
            retired_at: None,
            value: payload,
        }];
        partition.entity_arena.created_at[idx] = version_id;
        partition.entity_arena.retired_at[idx] = None;
        partition.entity_arena.generations[idx] += 1;
        partition.entity_arena.aspect_versions[idx].clear();
        partition.entity_arena.structural_fingerprints[idx] = None;
        partition.entity_arena.lineage_ids[idx] = None;
        partition.entity_arena.diagnostics_enrichment[idx].clear();
        partition.adjacency[idx].clear();
        partition.reverse_adjacency[idx].clear();
        return EntityId::new(partition_id, slot, partition.entity_arena.generations[idx]);
    }
    let slot = partition.entity_arena.generations.len() as u64;
    partition.entity_arena.partition_ids.push(partition_id);
    partition.entity_arena.generations.push(1);
    partition
        .entity_arena
        .lifecycle
        .push(RecordLifecycleState::Live);
    partition.entity_arena.live_bitset.set(slot as usize, true);
    partition
        .entity_arena
        .reclaimable_bitset
        .set(slot as usize, false);
    partition.entity_arena.kind_ids.push(Some(kind_id));
    partition.entity_arena.payloads.push(Some(payload.clone()));
    partition
        .entity_arena
        .payload_history
        .push(vec![VersionedValue {
            effective_at: version_id,
            retired_at: None,
            value: payload,
        }]);
    partition.entity_arena.created_at.push(version_id);
    partition.entity_arena.retired_at.push(None);
    partition.entity_arena.aspect_versions.push(BTreeMap::new());
    partition.entity_arena.structural_fingerprints.push(None);
    partition.entity_arena.lineage_ids.push(None);
    partition
        .entity_arena
        .diagnostics_enrichment
        .push(BTreeMap::new());
    partition.entity_arena.branch_pins.push(0);
    partition.entity_arena.replay_pins.push(0);
    partition.entity_arena.snapshot_pins.push(0);
    partition
        .adjacency
        .push(AdjacencySet::new(&partition.adjacency_policy));
    partition
        .reverse_adjacency
        .push(AdjacencySet::new(&partition.adjacency_policy));
    EntityId::new(partition_id, slot, 1)
}

fn allocate_relation(
    staged: &mut WorkingState,
    version_id: crate::identity::data::VersionId,
    spec: &RelationSpec,
) -> RelationId {
    let relation_id = {
        let partition = ensure_partition_state(staged, spec.partition_id);
        if let Some(slot) = partition.relation_arena.free_list.pop() {
            let idx = slot as usize;
            partition.relation_arena.partition_ids[idx] = spec.partition_id;
            partition.relation_arena.lifecycle[idx] = RecordLifecycleState::Live;
            partition.relation_arena.live_bitset.set(idx, true);
            partition.relation_arena.reclaimable_bitset.set(idx, false);
            partition.relation_arena.kind_ids[idx] = Some(spec.kind_id);
            let canonical_payload = spec.payload.clone().map(|payload| payload.canonicalized());
            partition.relation_arena.payloads[idx] = canonical_payload.clone();
            if let Some(payload) = canonical_payload {
                partition.relation_arena.payload_history.insert(
                    idx,
                    vec![VersionedValue {
                        effective_at: version_id,
                        retired_at: None,
                        value: payload,
                    }],
                );
            } else {
                partition.relation_arena.payload_history.remove(&idx);
            }
            partition.relation_arena.created_at[idx] = version_id;
            partition.relation_arena.retired_at[idx] = None;
            partition.relation_arena.endpoints[idx] = Some(RelationEndpoints {
                source: spec.source,
                target: spec.target,
            });
            partition.relation_arena.diagnostics_enrichment[idx].clear();
            partition.relation_arena.generations[idx] += 1;
            RelationId::new(
                spec.partition_id,
                slot,
                partition.relation_arena.generations[idx],
            )
        } else {
            let slot = partition.relation_arena.generations.len() as u64;
            partition
                .relation_arena
                .partition_ids
                .push(spec.partition_id);
            partition.relation_arena.generations.push(1);
            partition
                .relation_arena
                .lifecycle
                .push(RecordLifecycleState::Live);
            partition
                .relation_arena
                .live_bitset
                .set(slot as usize, true);
            partition
                .relation_arena
                .reclaimable_bitset
                .set(slot as usize, false);
            partition.relation_arena.kind_ids.push(Some(spec.kind_id));
            let canonical_payload = spec.payload.clone().map(|payload| payload.canonicalized());
            partition
                .relation_arena
                .payloads
                .push(canonical_payload.clone());
            if let Some(payload) = canonical_payload {
                partition.relation_arena.payload_history.insert(
                    slot as usize,
                    vec![VersionedValue {
                        effective_at: version_id,
                        retired_at: None,
                        value: payload,
                    }],
                );
            }
            partition.relation_arena.created_at.push(version_id);
            partition.relation_arena.retired_at.push(None);
            partition
                .relation_arena
                .endpoints
                .push(Some(RelationEndpoints {
                    source: spec.source,
                    target: spec.target,
                }));
            partition
                .relation_arena
                .diagnostics_enrichment
                .push(BTreeMap::new());
            partition.relation_arena.snapshot_pins.push(0);
            RelationId::new(spec.partition_id, slot, 1)
        }
    };

    let source_partition = ensure_partition_state(staged, spec.source.partition_id);
    ensure_entity_adjacency_capacity(source_partition, spec.source.local_slot.0 as usize);
    source_partition.adjacency[spec.source.local_slot.0 as usize].insert(relation_id);

    let target_partition = ensure_partition_state(staged, spec.target.partition_id);
    ensure_entity_adjacency_capacity(target_partition, spec.target.local_slot.0 as usize);
    target_partition.reverse_adjacency[spec.target.local_slot.0 as usize].insert(relation_id);

    relation_id
}

fn delete_entity_with_cascade(
    staged: &mut WorkingState,
    version_id: crate::identity::data::VersionId,
    entity_id: EntityId,
    patch_surface_policy: PatchSurfacePolicy,
    schema_registry: &RelationalSchemaRegistry,
    default_cascade_delete_policy: crate::config::data::CascadeDeletePolicy,
    changed_records: &mut Vec<RecordRef>,
    patch_records: &mut Vec<PatchRecord>,
) {
    let slot = entity_id.local_slot.0 as usize;
    staged.mark_entity_slot_touched(entity_id.partition_id, slot);
    let partition = staged.get_partition_mut(entity_id.partition_id);
    partition.entity_arena.retired_at[slot] = Some(version_id);
    partition.entity_arena.lifecycle[slot] = RecordLifecycleState::DeletedRetained;
    partition.entity_arena.live_bitset.set(slot, false);
    partition.entity_arena.reclaimable_bitset.set(slot, true);
    if let Some(current) = partition.entity_arena.payload_history[slot].last_mut() {
        current.retired_at = Some(version_id);
    }
    changed_records.push(RecordRef::Entity(entity_id));
    patch_records.push(PatchRecord {
        kind: PatchRecordKind::EntityDeleted,
        entity_id: Some(entity_id),
        relation_id: None,
        detail: patch_detail_for_entity(
            patch_surface_policy,
            PatchRecordKind::EntityDeleted,
            entity_id,
            None,
        ),
    });

    let mut attached = BTreeSet::new();
    partition.adjacency[slot].extend_into(&mut attached);
    partition.reverse_adjacency[slot].extend_into(&mut attached);
    for relation_id in attached {
        let cascade_policy = staged
            .get_partition(relation_id.partition_id)
            .and_then(|partition| {
                partition
                    .relation_arena
                    .kind_ids
                    .get(relation_id.local_slot.0 as usize)
            })
            .and_then(|kind_id| kind_id.as_ref().copied())
            .and_then(|kind_id| {
                schema_registry
                    .relation_registration(kind_id)
                    .ok()
                    .map(|registration| registration.cascade_delete_policy)
            })
            .unwrap_or(default_cascade_delete_policy);
        match cascade_policy {
            crate::config::data::CascadeDeletePolicy::CascadeDeleteRelations
            | crate::config::data::CascadeDeletePolicy::RetainDanglingForAudit => {
                delete_relation(
                    staged,
                    version_id,
                    relation_id,
                    patch_surface_policy,
                    changed_records,
                    patch_records,
                );
            }
        }
    }
}

fn delete_relation(
    staged: &mut WorkingState,
    version_id: crate::identity::data::VersionId,
    relation_id: RelationId,
    patch_surface_policy: PatchSurfacePolicy,
    changed_records: &mut Vec<RecordRef>,
    patch_records: &mut Vec<PatchRecord>,
) {
    let slot = relation_id.local_slot.0 as usize;
    let relation_is_live =
        staged
            .get_partition(relation_id.partition_id)
            .is_some_and(|partition| {
                partition.relation_arena.lifecycle[slot] == RecordLifecycleState::Live
            });
    if !relation_is_live {
        return;
    }
    staged.mark_relation_slot_touched(relation_id.partition_id, slot);
    let partition = staged.get_partition_mut(relation_id.partition_id);
    let endpoints = partition.relation_arena.endpoints[slot].clone();
    partition.relation_arena.retired_at[slot] = Some(version_id);
    partition.relation_arena.lifecycle[slot] = RecordLifecycleState::DeletedRetained;
    partition.relation_arena.live_bitset.set(slot, false);
    partition.relation_arena.reclaimable_bitset.set(slot, true);
    if let Some(current) = partition
        .relation_arena
        .payload_history
        .get_mut(&slot)
        .and_then(|history| history.last_mut())
    {
        current.retired_at = Some(version_id);
    }
    let fallback_source = endpoints
        .as_ref()
        .map(|value| value.source)
        .unwrap_or(EntityId::new(relation_id.partition_id, 0, 0));
    let fallback_target = endpoints
        .as_ref()
        .map(|value| value.target)
        .unwrap_or(EntityId::new(relation_id.partition_id, 0, 0));
    if let Some(endpoints) = endpoints {
        staged.mark_adjacency_slot_touched(
            endpoints.source.partition_id,
            endpoints.source.local_slot.0 as usize,
        );
        staged.mark_reverse_adjacency_slot_touched(
            endpoints.target.partition_id,
            endpoints.target.local_slot.0 as usize,
        );
        let source_partition = staged.get_partition_mut(endpoints.source.partition_id);
        if let Some(relations) = source_partition
            .adjacency
            .get_mut(endpoints.source.local_slot.0 as usize)
        {
            relations.remove(&relation_id);
        }
        let target_partition = staged.get_partition_mut(endpoints.target.partition_id);
        if let Some(relations) = target_partition
            .reverse_adjacency
            .get_mut(endpoints.target.local_slot.0 as usize)
        {
            relations.remove(&relation_id);
        }
    }
    changed_records.push(RecordRef::Relation(relation_id));
    patch_records.push(PatchRecord {
        kind: PatchRecordKind::RelationDeleted,
        entity_id: None,
        relation_id: Some(relation_id),
        detail: patch_detail_for_relation(
            patch_surface_policy,
            PatchRecordKind::RelationDeleted,
            relation_id,
            fallback_source,
            fallback_target,
            None,
        ),
    });
}

fn patch_detail_for_entity(
    patch_surface_policy: PatchSurfacePolicy,
    kind: PatchRecordKind,
    entity_id: EntityId,
    payload: Option<&RecordPayload>,
) -> PatchDetail {
    match patch_surface_policy {
        PatchSurfacePolicy::StructuredPatchSurface => match payload {
            Some(payload) => PatchDetail::Payload(payload.clone()),
            None => PatchDetail::StructuredJson(json!({})),
        },
        PatchSurfacePolicy::DensePatchSurface => PatchDetail::DenseBitset(vec![
            patch_kind_code(kind),
            entity_id.partition_id.0 as u64,
            entity_id.local_slot.0,
            entity_id.generation.0 as u64,
            payload.map(payload_class_code).unwrap_or(0),
        ]),
    }
}

fn patch_detail_for_relation(
    patch_surface_policy: PatchSurfacePolicy,
    kind: PatchRecordKind,
    relation_id: RelationId,
    source: EntityId,
    target: EntityId,
    payload: Option<&RecordPayload>,
) -> PatchDetail {
    match patch_surface_policy {
        PatchSurfacePolicy::StructuredPatchSurface => match payload {
            Some(payload) => PatchDetail::Payload(payload.clone()),
            None => PatchDetail::StructuredJson(json!({"payload_class":"topology_only"})),
        },
        PatchSurfacePolicy::DensePatchSurface => PatchDetail::DenseBitset(vec![
            patch_kind_code(kind),
            relation_id.partition_id.0 as u64,
            relation_id.local_slot.0,
            relation_id.generation.0 as u64,
            source.partition_id.0 as u64,
            source.local_slot.0,
            target.partition_id.0 as u64,
            target.local_slot.0,
            payload.map(payload_class_code).unwrap_or(0),
        ]),
    }
}

fn patch_kind_code(kind: PatchRecordKind) -> u64 {
    match kind {
        PatchRecordKind::EntityCreated => 1,
        PatchRecordKind::EntityUpdated => 2,
        PatchRecordKind::EntityDeleted => 3,
        PatchRecordKind::RelationCreated => 4,
        PatchRecordKind::RelationDeleted => 5,
    }
}

fn payload_class_code(payload: &RecordPayload) -> u64 {
    match payload {
        RecordPayload::StructuredJson(_) => 1,
        RecordPayload::OpaqueBytes(_) => 2,
    }
}

fn ensure_partition_state(
    staged: &mut WorkingState,
    partition_id: PartitionId,
) -> &mut PartitionState {
    staged.get_partition_mut(partition_id)
}

fn ensure_entity_adjacency_capacity(partition: &mut PartitionState, slot: usize) {
    while partition.adjacency.len() <= slot {
        partition
            .adjacency
            .push(AdjacencySet::new(&partition.adjacency_policy));
    }
    while partition.reverse_adjacency.len() <= slot {
        partition
            .reverse_adjacency
            .push(AdjacencySet::new(&partition.adjacency_policy));
    }
}
