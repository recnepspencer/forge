use std::collections::{HashMap, HashSet};

use crate::diagnostics::data::DiagnosticCode;
use crate::config::data::CascadeDeletePolicy;
use crate::schema::data::{
    EndpointDeletionIntegrityMode, LoweredCardinalityContract,
    LoweredEndpointDeletionIntegrityContract, LoweredEndpointKindContract,
    LoweredSymmetryContract, LoweredUniquenessContract, SymmetryMode, UniquenessScope,
};
use crate::storage::data::RecordLifecycleState;
use crate::storage::logic::state::{EntityRecordKind, RecordKind, RelationRecordKind, SlotView};
use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::{InvariantClass, InvariantRule, InvariantViolation, RecordKindTag};
use serde_json::json;

use super::context::InvariantExecutionContext;
use super::state_view::InvariantStateView;

pub(crate) fn evaluate_rule(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    rule: &InvariantRule,
) -> Option<InvariantViolation> {
    match rule {
        InvariantRule::LiveRecordRequiresSidecar(kind) => {
            evaluate_live_record_sidecar_rule(context, class, kind)
        }
        InvariantRule::MaxMergedIntents(limit) => {
            let merged_len = context
                .merged_plan()
                .map(|plan| plan.merged_intents.len())
                .unwrap_or(0);
            if merged_len > *limit {
                Some(InvariantViolation {
                    class,
                    code: DiagnosticCode::InvariantViolation,
                    detail: format!(
                        "merged commit plan has {} intents, limit is {}",
                        merged_len, limit
                    ),
                    fields: json!({
                        "merged_intent_count": merged_len,
                        "limit": limit,
                    }),
                })
            } else {
                None
            }
        }
        InvariantRule::MaxSnapshotEntities(limit) => {
            evaluate_max_snapshot_entities(context, class, *limit)
        }
        InvariantRule::UniqueEntityPayloadField(field) => {
            evaluate_unique_entity_payload_field(context, class, field)
        }
        InvariantRule::EndpointKindContract(contract) => {
            evaluate_endpoint_kind_contract(context, class, contract)
        }
        InvariantRule::CardinalityContract(contract) => {
            evaluate_cardinality_contract(context, class, contract)
        }
        InvariantRule::UniquenessContract(contract) => {
            evaluate_uniqueness_contract(context, class, contract)
        }
        InvariantRule::SymmetryContract(contract) => evaluate_symmetry_contract(context, class, contract),
        InvariantRule::EndpointDeletionIntegrityContract(contract) => {
            evaluate_endpoint_deletion_integrity_contract(context, class, contract)
        }
    }
}

fn evaluate_live_record_sidecar_rule(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    kind: &RecordKindTag,
) -> Option<InvariantViolation> {
    match kind {
        RecordKindTag::Entity => evaluate_live_record_sidecar::<EntityRecordKind>(
            context,
            class,
            |state, partition_id| state.touched_entity_slots(partition_id),
            |slot_view| slot_view.kind_id().is_some(),
            "kind id",
            |context, slots| {
                context.metrics().count_entity_slot_scans(slots);
            },
        ),
        RecordKindTag::Relation => evaluate_live_record_sidecar::<RelationRecordKind>(
            context,
            class,
            |state, partition_id| state.touched_relation_slots(partition_id),
            |slot_view| slot_view.extra().is_some(),
            "endpoints",
            |context, slots| {
                context.metrics().count_relation_slot_scans(slots);
            },
        ),
    }
}

fn evaluate_live_record_sidecar<K: RecordKind>(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    touched_slots: impl Fn(
        &dyn crate::logic::runtime::PartitionAccess,
        crate::identity::data::PartitionId,
    ) -> Option<Vec<usize>>,
    has_required_sidecar: impl Fn(&SlotView<'_, K>) -> bool,
    missing_label: &str,
    count_scans: impl Fn(&InvariantExecutionContext<'_>, usize),
) -> Option<InvariantViolation> {
    for partition_id in context.partition_access().partition_ids() {
        let partition = context
            .partition_access()
            .get_partition(partition_id)
            .expect("partition for invariant scan");
        if let Some(slots) = touched_slots(context.partition_access(), partition_id) {
            count_scans(context, slots.len());
            for slot in slots {
                if let Some(violation) = sidecar_violation_for_slot(
                    class,
                    partition,
                    slot,
                    &has_required_sidecar,
                    missing_label,
                ) {
                    return Some(violation);
                }
            }
        } else {
            let arena = K::arena(partition);
            count_scans(context, arena.slot_count());
            for slot in 0..arena.slot_count() {
                if let Some(violation) = sidecar_violation_for_slot(
                    class,
                    partition,
                    slot,
                    &has_required_sidecar,
                    missing_label,
                ) {
                    return Some(violation);
                }
            }
        }
    }
    None
}

fn sidecar_violation_for_slot<K: RecordKind>(
    class: InvariantClass,
    partition: &crate::storage::logic::state::PartitionState,
    slot: usize,
    has_required_sidecar: &impl Fn(&SlotView<'_, K>) -> bool,
    missing_label: &str,
) -> Option<InvariantViolation> {
    let slot_view = K::arena(partition).get_slot(slot)?;
    if slot_view.lifecycle() == RecordLifecycleState::Live && !has_required_sidecar(&slot_view) {
        return Some(InvariantViolation {
            class,
            code: DiagnosticCode::SidecarConsistencyFailure,
            detail: format!(
                "live slot {} in partition {} missing {}",
                slot, partition.partition_id.0, missing_label
            ),
            fields: json!({
                "partition_id": partition.partition_id.0,
                "slot": slot,
                "missing_label": missing_label,
            }),
        });
    }
    None
}

fn evaluate_max_snapshot_entities(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    limit: usize,
) -> Option<InvariantViolation> {
    let state_view = context.state_view();
    let mut visible_entities = 0;
    if state_view.version_id() == context.current_version_id() {
        for partition_id in state_view.state().partition_ids() {
            let partition = state_view
                .state()
                .get_partition(partition_id)
                .expect("partition for invariant scan");
            visible_entities += partition.entity_arena.live_bitset.count_ones();
        }
    } else {
        for partition_id in state_view.state().partition_ids() {
            let partition = state_view
                .state()
                .get_partition(partition_id)
                .expect("partition for invariant scan");
            context
                .metrics()
                .count_entity_slot_scans(partition.entity_arena.slot_count());
            visible_entities += (0..partition.entity_arena.slot_count())
                .filter(|slot| state_view.entity_visible_at_version(&partition.entity_arena, *slot))
                .count();
        }
    }
    if visible_entities > limit {
        return Some(InvariantViolation {
            class,
            code: DiagnosticCode::InvariantViolation,
            detail: format!(
                "snapshot at version {} has {} entities, limit is {}",
                state_view.version_id().0,
                visible_entities,
                limit
            ),
            fields: json!({
                "version_id": state_view.version_id().0,
                "visible_entities": visible_entities,
                "limit": limit,
            }),
        });
    }
    None
}

fn evaluate_unique_entity_payload_field(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    field: &str,
) -> Option<InvariantViolation> {
    let state_view = context.state_view();
    if let Some(violation) = planned_unique_entity_payload_violation(context, class, field) {
        return Some(violation);
    } else if let Some(touched_entity_ids) = state_view.touched_visible_entity_ids() {
        let mut touched_value_to_entity = HashMap::with_capacity(touched_entity_ids.len());
        let touched_set = InvariantStateView::touched_entity_set(&touched_entity_ids);
        for entity_id in touched_entity_ids {
            context.metrics().count_entity_slot_scans(1);
            let Some(payload) = state_view.entity_payload(entity_id) else {
                continue;
            };
            let Some(value) = payload
                .as_json()
                .and_then(|value| value.get(field))
                .and_then(|value| value.as_str())
            else {
                continue;
            };
            if touched_value_to_entity
                .insert(value.to_owned(), entity_id)
                .is_some()
            {
                return Some(duplicate_field_violation(class, field, value));
            }
            if context
                .indexes()
                .conflicts_with_entity_value_outside(field, value, &touched_set)
            {
                return Some(duplicate_field_violation(class, field, value));
            }
        }
    } else if state_view.version_id() == context.current_version_id() {
        if let Some(value) = context.indexes().first_duplicate_entity_value(field) {
            return Some(duplicate_field_violation(class, field, value));
        }
    } else {
        let mut seen = HashSet::new();
        for partition_id in state_view.state().partition_ids() {
            let partition = state_view
                .state()
                .get_partition(partition_id)
                .expect("partition for invariant scan");
            for slot in 0..partition.entity_arena.slot_count() {
                context.metrics().count_entity_slot_scans(1);
                let Some(payload) = partition
                    .entity_arena
                    .payload_history_at(slot)
                    .and_then(|history| state_view.visible_payload(history))
                else {
                    continue;
                };
                let Some(value) = payload
                    .as_json()
                    .and_then(|value| value.get(field))
                    .and_then(|value| value.as_str())
                else {
                    continue;
                };
                if !seen.insert(value.to_string()) {
                    return Some(duplicate_field_violation(class, field, value));
                }
            }
        }
    }
    None
}

fn planned_entity_field_values(
    merged_plan: Option<&MergedCommitPlan>,
    field: &str,
) -> Option<Vec<(Option<crate::identity::data::EntityId>, String)>> {
    let plan = merged_plan?;
    let mut values = Vec::new();
    for intent in &plan.merged_intents {
        match intent {
            crate::transactions::data::MutationIntent::Create(
                crate::transactions::data::CreateIntent::Entity(create),
            ) => {
                let Some(value) = payload_field_value(&create.payload, field) else {
                    continue;
                };
                values.push((None, value));
            }
            crate::transactions::data::MutationIntent::Create(
                crate::transactions::data::CreateIntent::BulkEntities(bulk),
            ) => {
                for payload in &bulk.payloads {
                    let Some(value) = payload_field_value(payload, field) else {
                        continue;
                    };
                    values.push((None, value));
                }
            }
            crate::transactions::data::MutationIntent::Entity(
                crate::transactions::data::EntityMutationIntent::Update(update),
            ) => {
                let Some(value) = payload_field_value(&update.payload, field) else {
                    continue;
                };
                values.push((Some(update.entity_id), value));
            }
            crate::transactions::data::MutationIntent::Entity(
                crate::transactions::data::EntityMutationIntent::Replace(replace),
            ) => {
                let Some(value) = payload_field_value(&replace.replacement.payload, field) else {
                    continue;
                };
                values.push((Some(replace.entity_id), value));
            }
            _ => {}
        }
    }
    Some(values)
}

fn planned_unique_entity_payload_violation(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    field: &str,
) -> Option<InvariantViolation> {
    let planned_values = planned_entity_field_values(context.merged_plan(), field)?;
    let mut planned_value_to_entity = HashMap::with_capacity(planned_values.len());
    for (entity_id, value) in planned_values {
        context.metrics().count_entity_slot_scans(1);
        if let Some(existing_entity_id) = planned_value_to_entity.insert(value.clone(), entity_id) {
            if existing_entity_id != entity_id || entity_id.is_none() {
                return Some(duplicate_field_violation(class, field, &value));
            }
        }
        if context
            .indexes()
            .conflicts_with_entity_value(field, &value, entity_id)
        {
            return Some(duplicate_field_violation(class, field, &value));
        }
    }
    None
}

fn payload_field_value(
    payload: &crate::payloads::data::RecordPayload,
    field: &str,
) -> Option<String> {
    payload
        .as_json()?
        .get(field)
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned)
}

fn duplicate_field_violation(
    class: InvariantClass,
    field: &str,
    value: &str,
) -> InvariantViolation {
    InvariantViolation {
        class,
        code: DiagnosticCode::InvariantViolation,
        detail: format!(
            "entity payload field '{}' must be unique, duplicate value '{}'",
            field, value
        ),
        fields: json!({
            "field": field,
            "value": value,
        }),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RelationPairKey {
    kind_id: crate::identity::data::KindId,
    source: crate::identity::data::EntityId,
    target: crate::identity::data::EntityId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RelationEndpointKey {
    kind_id: crate::identity::data::KindId,
    entity_id: crate::identity::data::EntityId,
}

fn evaluate_endpoint_kind_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredEndpointKindContract,
) -> Option<InvariantViolation> {
    let creates = planned_relation_specs(context.merged_plan(), contract.relation_kind_id);
    if creates.is_empty() {
        return None;
    }

    context.metrics().count_relation_contracts_evaluated(1);
    for spec in creates {
        context.metrics().count_relation_endpoint_kind_checks(1);
        let source_kind = entity_kind_in_state(context, spec.source)?;
        let target_kind = entity_kind_in_state(context, spec.target)?;
        if !contract.allowed_source_kinds.contains(&source_kind) {
            return Some(relation_violation(
                class,
                DiagnosticCode::RelationEndpointKindViolation,
                format!(
                    "relation contract '{}' rejected source kind {:?} for relation kind {:?}",
                    contract.contract_id, source_kind, contract.relation_kind_id
                ),
                json!({
                    "contract_id": contract.contract_id,
                    "relation_kind_id": contract.relation_kind_id.0,
                    "source": spec.source,
                    "target": spec.target,
                    "source_kind_id": source_kind.0,
                    "target_kind_id": target_kind.0,
                }),
            ));
        }
        if !contract.allowed_target_kinds.contains(&target_kind) {
            return Some(relation_violation(
                class,
                DiagnosticCode::RelationEndpointKindViolation,
                format!(
                    "relation contract '{}' rejected target kind {:?} for relation kind {:?}",
                    contract.contract_id, target_kind, contract.relation_kind_id
                ),
                json!({
                    "contract_id": contract.contract_id,
                    "relation_kind_id": contract.relation_kind_id.0,
                    "source": spec.source,
                    "target": spec.target,
                    "source_kind_id": source_kind.0,
                    "target_kind_id": target_kind.0,
                }),
            ));
        }
        if !contract.self_edges_allowed && spec.source == spec.target {
            return Some(relation_violation(
                class,
                DiagnosticCode::RelationEndpointKindViolation,
                format!(
                    "relation contract '{}' forbids self edges for relation kind {:?}",
                    contract.contract_id, contract.relation_kind_id
                ),
                json!({
                    "contract_id": contract.contract_id,
                    "relation_kind_id": contract.relation_kind_id.0,
                    "source": spec.source,
                    "target": spec.target,
                    "self_edge": true,
                }),
            ));
        }
        if spec.source.partition_id != spec.target.partition_id
            && contract.cross_context_policy != crate::config::data::CrossContextPolicy::AllowExplicit
        {
            return Some(relation_violation(
                class,
                DiagnosticCode::InvalidRelationEndpoint,
                format!(
                    "relation contract '{}' forbids cross-context endpoints for relation kind {:?}",
                    contract.contract_id, contract.relation_kind_id
                ),
                json!({
                    "contract_id": contract.contract_id,
                    "relation_kind_id": contract.relation_kind_id.0,
                    "source_partition_id": spec.source.partition_id.0,
                    "target_partition_id": spec.target.partition_id.0,
                }),
            ));
        }
    }
    None
}

fn evaluate_cardinality_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredCardinalityContract,
) -> Option<InvariantViolation> {
    let touched_counts = touched_relation_counts(context, contract.relation_kind_id);
    if touched_counts.is_empty() {
        return None;
    }
    context.metrics().count_relation_contracts_evaluated(1);
    for (key, count) in &touched_counts.source_counts {
        if let Some(limit) = contract.source_max {
            context.metrics().count_relation_cardinality_checks(1);
            if *count > limit {
                return Some(relation_violation(
                    class,
                    DiagnosticCode::RelationCardinalityViolation,
                    format!(
                        "relation contract '{}' overflowed source cardinality for entity {:?}: {} > {}",
                        contract.contract_id, key.entity_id, count, limit
                    ),
                    json!({
                        "contract_id": contract.contract_id,
                        "relation_kind_id": key.kind_id.0,
                        "entity_id": key.entity_id,
                        "boundary": "source",
                        "count": count,
                        "limit": limit,
                    }),
                ));
            }
        }
    }
    for (key, count) in &touched_counts.target_counts {
        if let Some(limit) = contract.target_max {
            context.metrics().count_relation_cardinality_checks(1);
            if *count > limit {
                return Some(relation_violation(
                    class,
                    DiagnosticCode::RelationCardinalityViolation,
                    format!(
                        "relation contract '{}' overflowed target cardinality for entity {:?}: {} > {}",
                        contract.contract_id, key.entity_id, count, limit
                    ),
                    json!({
                        "contract_id": contract.contract_id,
                        "relation_kind_id": key.kind_id.0,
                        "entity_id": key.entity_id,
                        "boundary": "target",
                        "count": count,
                        "limit": limit,
                    }),
                ));
            }
        }
    }
    for (key, count) in &touched_counts.directed_pair_counts {
        if let Some(limit) = contract.pair_max {
            context.metrics().count_relation_cardinality_checks(1);
            if *count > limit {
                return Some(relation_violation(
                    class,
                    DiagnosticCode::RelationCardinalityViolation,
                    format!(
                        "relation contract '{}' overflowed pair cardinality for {:?}->{:?}: {} > {}",
                        contract.contract_id, key.source, key.target, count, limit
                    ),
                    json!({
                        "contract_id": contract.contract_id,
                        "relation_kind_id": key.kind_id.0,
                        "source": key.source,
                        "target": key.target,
                        "count": count,
                        "limit": limit,
                    }),
                ));
            }
        }
    }
    None
}

fn evaluate_uniqueness_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredUniquenessContract,
) -> Option<InvariantViolation> {
    let counts = touched_relation_counts(context, contract.relation_kind_id);
    if counts.is_empty() {
        return None;
    }
    context.metrics().count_relation_contracts_evaluated(1);
    match contract.scope {
        UniquenessScope::DirectedSemanticEdge => {
            for (key, count) in &counts.directed_pair_counts {
                context.metrics().count_relation_uniqueness_checks(1);
                if *count > 1 {
                    return Some(relation_violation(
                        class,
                        DiagnosticCode::RelationUniquenessViolation,
                        format!(
                            "relation contract '{}' forbids duplicate directed edge {:?}->{:?}",
                            contract.contract_id, key.source, key.target
                        ),
                        json!({
                            "contract_id": contract.contract_id,
                            "relation_kind_id": key.kind_id.0,
                            "scope": "directed",
                            "source": key.source,
                            "target": key.target,
                            "count": count,
                        }),
                    ));
                }
            }
        }
        UniquenessScope::NormalizedSymmetricEdge => {
            for (key, count) in &counts.normalized_pair_counts {
                context.metrics().count_relation_uniqueness_checks(1);
                if *count > 1 {
                    return Some(relation_violation(
                        class,
                        DiagnosticCode::RelationUniquenessViolation,
                        format!(
                            "relation contract '{}' forbids duplicate normalized edge {:?}<->{:?}",
                            contract.contract_id, key.source, key.target
                        ),
                        json!({
                            "contract_id": contract.contract_id,
                            "relation_kind_id": key.kind_id.0,
                            "scope": "normalized",
                            "source": key.source,
                            "target": key.target,
                            "count": count,
                        }),
                    ));
                }
            }
        }
    }
    None
}

fn evaluate_symmetry_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredSymmetryContract,
) -> Option<InvariantViolation> {
    let counts = touched_relation_counts(context, contract.relation_kind_id);
    let creates = planned_relation_specs(context.merged_plan(), contract.relation_kind_id);
    if creates.is_empty() {
        return None;
    }
    context.metrics().count_relation_contracts_evaluated(1);
    for spec in creates {
        context.metrics().count_relation_symmetry_checks(1);
        match contract.mode {
            SymmetryMode::CanonicalUndirected => {
                if spec.target < spec.source {
                    return Some(relation_violation(
                        class,
                        DiagnosticCode::RelationSymmetryViolation,
                        format!(
                            "relation contract '{}' requires canonical undirected ordering",
                            contract.contract_id
                        ),
                        json!({
                            "contract_id": contract.contract_id,
                            "relation_kind_id": contract.relation_kind_id.0,
                            "source": spec.source,
                            "target": spec.target,
                            "mode": "canonical_undirected",
                        }),
                    ));
                }
            }
            SymmetryMode::PairedInverseRequired | SymmetryMode::PairedTwinRequired => {
                let inverse = RelationPairKey {
                    kind_id: contract.relation_kind_id,
                    source: spec.target,
                    target: spec.source,
                };
                if counts.directed_pair_counts.get(&inverse).copied().unwrap_or_default() == 0 {
                    return Some(relation_violation(
                        class,
                        DiagnosticCode::RelationSymmetryViolation,
                        format!(
                            "relation contract '{}' requires an inverse/twin edge for {:?}->{:?}",
                            contract.contract_id, spec.source, spec.target
                        ),
                        json!({
                            "contract_id": contract.contract_id,
                            "relation_kind_id": contract.relation_kind_id.0,
                            "source": spec.source,
                            "target": spec.target,
                            "mode": "paired",
                        }),
                    ));
                }
            }
            SymmetryMode::InverseProhibited => {
                let inverse = RelationPairKey {
                    kind_id: contract.relation_kind_id,
                    source: spec.target,
                    target: spec.source,
                };
                if counts.directed_pair_counts.get(&inverse).copied().unwrap_or_default() > 0 {
                    return Some(relation_violation(
                        class,
                        DiagnosticCode::RelationSymmetryViolation,
                        format!(
                            "relation contract '{}' prohibits inverse duplication for {:?}->{:?}",
                            contract.contract_id, spec.source, spec.target
                        ),
                        json!({
                            "contract_id": contract.contract_id,
                            "relation_kind_id": contract.relation_kind_id.0,
                            "source": spec.source,
                            "target": spec.target,
                            "mode": "inverse_prohibited",
                        }),
                    ));
                }
            }
        }
    }
    None
}

fn evaluate_endpoint_deletion_integrity_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredEndpointDeletionIntegrityContract,
) -> Option<InvariantViolation> {
    let counts = touched_relation_counts(context, contract.relation_kind_id);
    if counts.deleted_entities.is_empty() {
        return None;
    }
    context.metrics().count_relation_contracts_evaluated(1);
    let cascade_policy = context
        .runtime()
        .config
        .schema
        .registry
        .relation_registration(contract.relation_kind_id)
        .ok()
        .map(|registration| registration.cascade_delete_policy);
    for entity_id in &counts.deleted_entities {
        let endpoint_key = RelationEndpointKey {
            kind_id: contract.relation_kind_id,
            entity_id: *entity_id,
        };
        let live_relations = counts.source_counts.get(&endpoint_key).copied().unwrap_or_default()
            + counts.target_counts.get(&endpoint_key).copied().unwrap_or_default();
        context.metrics().count_relation_endpoint_deletion_checks(1);
        if live_relations > 0 {
            match contract.mode {
                EndpointDeletionIntegrityMode::RejectDeleteWithLiveRelations => {
                    return Some(relation_violation(
                        class,
                        DiagnosticCode::RelationEndpointDeletionIntegrityViolation,
                        format!(
                            "relation contract '{}' forbids deleting endpoint {:?} while {} relation endpoints remain live",
                            contract.contract_id, entity_id, live_relations
                        ),
                        json!({
                            "contract_id": contract.contract_id,
                            "relation_kind_id": contract.relation_kind_id.0,
                            "entity_id": entity_id,
                            "remaining_relation_endpoint_count": live_relations,
                            "mode": "reject_delete_with_live_relations",
                        }),
                    ));
                }
                EndpointDeletionIntegrityMode::RequireRelationDeletionInSameCommit => {
                    if cascade_policy != Some(CascadeDeletePolicy::CascadeDeleteRelations) {
                        return Some(relation_violation(
                            class,
                            DiagnosticCode::RelationEndpointDeletionIntegrityViolation,
                            format!(
                                "relation contract '{}' requires deleting dependent relations in the same commit before deleting endpoint {:?}",
                                contract.contract_id, entity_id
                            ),
                            json!({
                                "contract_id": contract.contract_id,
                                "relation_kind_id": contract.relation_kind_id.0,
                                "entity_id": entity_id,
                                "remaining_relation_endpoint_count": live_relations,
                                "mode": "require_relation_deletion_in_same_commit",
                                "cascade_delete_policy": cascade_policy.map(endpoint_deletion_policy_label),
                            }),
                        ));
                    }
                }
                EndpointDeletionIntegrityMode::RequireRelationRetirement => {
                    if cascade_policy != Some(CascadeDeletePolicy::RetainDanglingForAudit) {
                        return Some(relation_violation(
                            class,
                            DiagnosticCode::RelationEndpointDeletionIntegrityViolation,
                            format!(
                                "relation contract '{}' requires audit-retained relation retirement before deleting endpoint {:?}",
                                contract.contract_id, entity_id
                            ),
                            json!({
                                "contract_id": contract.contract_id,
                                "relation_kind_id": contract.relation_kind_id.0,
                                "entity_id": entity_id,
                                "remaining_relation_endpoint_count": live_relations,
                                "mode": "require_relation_retirement",
                                "cascade_delete_policy": cascade_policy.map(endpoint_deletion_policy_label),
                            }),
                        ));
                    }
                }
            }
        }
    }
    None
}

fn endpoint_deletion_policy_label(policy: CascadeDeletePolicy) -> &'static str {
    match policy {
        CascadeDeletePolicy::CascadeDeleteRelations => "cascade_delete_relations",
        CascadeDeletePolicy::RetainDanglingForAudit => "retain_dangling_for_audit",
    }
}

struct TouchedRelationCounts {
    source_counts: HashMap<RelationEndpointKey, usize>,
    target_counts: HashMap<RelationEndpointKey, usize>,
    directed_pair_counts: HashMap<RelationPairKey, usize>,
    normalized_pair_counts: HashMap<RelationPairKey, usize>,
    deleted_entities: HashSet<crate::identity::data::EntityId>,
}

impl TouchedRelationCounts {
    fn empty() -> Self {
        Self {
            source_counts: HashMap::new(),
            target_counts: HashMap::new(),
            directed_pair_counts: HashMap::new(),
            normalized_pair_counts: HashMap::new(),
            deleted_entities: HashSet::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.source_counts.is_empty()
            && self.target_counts.is_empty()
            && self.directed_pair_counts.is_empty()
            && self.deleted_entities.is_empty()
    }
}

fn touched_relation_counts(
    context: &InvariantExecutionContext<'_>,
    relation_kind_id: crate::identity::data::KindId,
) -> TouchedRelationCounts {
    let Some(plan) = context.merged_plan() else {
        return TouchedRelationCounts::empty();
    };
    let mut touched_entities = HashSet::new();
    let mut deleted_relations = HashSet::new();
    let mut counts = TouchedRelationCounts::empty();

    for intent in &plan.merged_intents {
        match intent {
            crate::transactions::data::MutationIntent::Create(
                crate::transactions::data::CreateIntent::Relation(spec),
            ) if spec.kind_id == relation_kind_id => {
                touched_entities.insert(spec.source);
                touched_entities.insert(spec.target);
            }
            crate::transactions::data::MutationIntent::Create(
                crate::transactions::data::CreateIntent::BulkRelations(spec),
            ) if spec.kind_id == relation_kind_id => {
                for (source, target) in &spec.endpoints {
                    touched_entities.insert(*source);
                    touched_entities.insert(*target);
                }
            }
            crate::transactions::data::MutationIntent::Relation(
                crate::transactions::data::RelationMutationIntent::Delete(spec),
            ) => {
                deleted_relations.insert(spec.relation_id);
            }
            crate::transactions::data::MutationIntent::Entity(
                crate::transactions::data::EntityMutationIntent::Delete(spec),
            ) => {
                counts.deleted_entities.insert(spec.entity_id);
                touched_entities.insert(spec.entity_id);
            }
            crate::transactions::data::MutationIntent::Entity(
                crate::transactions::data::EntityMutationIntent::Replace(spec),
            ) => {
                counts.deleted_entities.insert(spec.entity_id);
                touched_entities.insert(spec.entity_id);
            }
            _ => {}
        }
    }

    for entity_id in touched_entities {
        scan_current_relations_for_entity(
            context,
            relation_kind_id,
            entity_id,
            &deleted_relations,
            &mut counts,
        );
    }

    for spec in planned_relation_specs(Some(plan), relation_kind_id) {
        increment_counts(&mut counts, relation_kind_id, spec.source, spec.target);
    }

    counts
}

fn scan_current_relations_for_entity(
    context: &InvariantExecutionContext<'_>,
    relation_kind_id: crate::identity::data::KindId,
    entity_id: crate::identity::data::EntityId,
    deleted_relations: &HashSet<crate::identity::data::RelationId>,
    counts: &mut TouchedRelationCounts,
) {
    let Some(partition) = context.partition_access().get_partition(entity_id.partition_id) else {
        return;
    };
    let Some(outgoing) = partition.adjacency.get(entity_id.local_slot.0 as usize) else {
        return;
    };
    for relation_id in outgoing.as_slice().iter().copied() {
        if deleted_relations.contains(&relation_id) {
            continue;
        }
        let Some(relation_partition) = context.partition_access().get_partition(relation_id.partition_id) else {
            continue;
        };
        let Some(slot) = relation_partition.relation_arena.get(&relation_id) else {
            continue;
        };
        if slot.kind_id() != Some(relation_kind_id) || slot.lifecycle() != RecordLifecycleState::Live {
            continue;
        }
        let Some(endpoints) = slot.extra().as_ref() else {
            continue;
        };
        context.metrics().count_relation_uniqueness_candidates(1);
        increment_counts(counts, relation_kind_id, endpoints.source, endpoints.target);
    }
}

fn increment_counts(
    counts: &mut TouchedRelationCounts,
    relation_kind_id: crate::identity::data::KindId,
    source: crate::identity::data::EntityId,
    target: crate::identity::data::EntityId,
) {
    *counts
        .source_counts
        .entry(RelationEndpointKey {
            kind_id: relation_kind_id,
            entity_id: source,
        })
        .or_insert(0) += 1;
    *counts
        .target_counts
        .entry(RelationEndpointKey {
            kind_id: relation_kind_id,
            entity_id: target,
        })
        .or_insert(0) += 1;
    *counts
        .directed_pair_counts
        .entry(RelationPairKey {
            kind_id: relation_kind_id,
            source,
            target,
        })
        .or_insert(0) += 1;
    let (left, right) = if target < source {
        (target, source)
    } else {
        (source, target)
    };
    *counts
        .normalized_pair_counts
        .entry(RelationPairKey {
            kind_id: relation_kind_id,
            source: left,
            target: right,
        })
        .or_insert(0) += 1;
}

fn planned_relation_specs(
    merged_plan: Option<&MergedCommitPlan>,
    relation_kind_id: crate::identity::data::KindId,
) -> Vec<crate::transactions::data::RelationSpec> {
    let Some(plan) = merged_plan else {
        return Vec::new();
    };
    let mut specs = Vec::new();
    for intent in &plan.merged_intents {
        match intent {
            crate::transactions::data::MutationIntent::Create(
                crate::transactions::data::CreateIntent::Relation(spec),
            ) if spec.kind_id == relation_kind_id => specs.push(spec.clone()),
            crate::transactions::data::MutationIntent::Create(
                crate::transactions::data::CreateIntent::BulkRelations(spec),
            ) if spec.kind_id == relation_kind_id => {
                for ((source, target), payload) in spec.endpoints.iter().zip(spec.payloads.iter()) {
                    specs.push(crate::transactions::data::RelationSpec {
                        partition_id: spec.partition_id,
                        kind_id: spec.kind_id,
                        client_key: crate::symbols::data::InternedString::Raw("bulk".to_string()),
                        source: *source,
                        target: *target,
                        payload: payload.clone(),
                    });
                }
            }
            _ => {}
        }
    }
    specs
}

fn entity_kind_in_state(
    context: &InvariantExecutionContext<'_>,
    entity_id: crate::identity::data::EntityId,
) -> Option<crate::identity::data::KindId> {
    let partition = context.partition_access().get_partition(entity_id.partition_id)?;
    partition
        .entity_arena
        .get(&entity_id)
        .and_then(|slot| (slot.lifecycle() == RecordLifecycleState::Live).then(|| slot.kind_id()))
        .flatten()
}

fn relation_violation(
    class: InvariantClass,
    code: DiagnosticCode,
    detail: String,
    fields: serde_json::Value,
) -> InvariantViolation {
    InvariantViolation {
        class,
        code,
        detail,
        fields,
    }
}
