use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::diagnostics::data::DiagnosticCode;
use crate::schema::data::{
    LoweredAcyclicityContract, LoweredConnectivityMinimumContract,
    EndpointDeletionIntegrityMode, LoweredCardinalityMaximumContract,
    LoweredCardinalityMinimumContract, PairMinimumSemantics,
    LoweredEndpointDeletionIntegrityContract, LoweredEndpointKindContract,
    LoweredPartitionIsolationContract, LoweredPayloadSchemaContract,
    LoweredSymmetryContract, LoweredUniquenessContract, PayloadContractRecordKind,
    PayloadFieldConstraint, PayloadSchemaValueType, SymmetryMode, UniquenessScope,
};
use crate::storage::data::RecordLifecycleState;
use crate::storage::logic::state::HistoricalMetadata;
use crate::storage::logic::state::{EntityRecordKind, RecordKind, RelationRecordKind, SlotView};
use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::{
    InvariantClass, InvariantRule, InvariantViolation, InvariantViolationFields,
    RecordKindTag, RelationCardinalityBoundary, RelationEndpointBoundary,
};
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
                    fields: InvariantViolationFields::MergedIntentLimit {
                        merged_intent_count: merged_len,
                        limit: *limit,
                    },
                })
            } else {
                None
            }
        }
        InvariantRule::RelationIntegrityScopeBudget(_) => None,
        InvariantRule::MaxSnapshotEntities(limit) => {
            evaluate_max_snapshot_entities(context, class, *limit)
        }
        InvariantRule::UniqueEntityPayloadField(field) => {
            evaluate_unique_entity_payload_field(context, class, field)
        }
        InvariantRule::EndpointKindContract(contract) => {
            evaluate_endpoint_kind_contract(context, class, contract)
        }
        InvariantRule::CardinalityMaximumContract(contract) => {
            evaluate_cardinality_maximum_contract(context, class, contract)
        }
        InvariantRule::CardinalityMinimumContract(contract) => {
            evaluate_cardinality_minimum_contract(context, class, contract)
        }
        InvariantRule::UniquenessContract(contract) => {
            evaluate_uniqueness_contract(context, class, contract)
        }
        InvariantRule::SymmetryContract(contract) => evaluate_symmetry_contract(context, class, contract),
        InvariantRule::EndpointDeletionIntegrityContract(contract) => {
            evaluate_endpoint_deletion_integrity_contract(context, class, contract)
        }
        InvariantRule::PayloadSchemaContract(contract) => {
            evaluate_payload_schema_contract(context, class, contract)
        }
        InvariantRule::PartitionIsolationContract(contract) => {
            evaluate_partition_isolation_contract(context, class, contract)
        }
        InvariantRule::AcyclicityContract(_contract) => None,
        InvariantRule::ConnectivityMinimumContract(_contract) => None,
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
        let Some(partition) = context.partition_access().get_partition(partition_id) else {
            return Some(storage_inconsistency_violation(
                class,
                format!("partition {:?} missing during invariant sidecar scan", partition_id),
                json!({
                    "partition_id": partition_id.0,
                    "scan": "live_record_sidecar",
                }),
            ));
        };
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
            fields: InvariantViolationFields::SidecarConsistency {
                partition_id: partition.partition_id,
                slot,
                missing_label: missing_label.to_string(),
            },
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
            let Some(partition) = state_view.state().get_partition(partition_id) else {
                return Some(storage_inconsistency_violation(
                    class,
                    format!("partition {:?} missing during snapshot entity count", partition_id),
                    json!({
                        "partition_id": partition_id.0,
                        "scan": "max_snapshot_entities",
                    }),
                ));
            };
            visible_entities += partition.entity_arena.live_bitset.count_ones();
        }
    } else {
        for partition_id in state_view.state().partition_ids() {
            let Some(partition) = state_view.state().get_partition(partition_id) else {
                return Some(storage_inconsistency_violation(
                    class,
                    format!("partition {:?} missing during historical entity scan", partition_id),
                    json!({
                        "partition_id": partition_id.0,
                        "scan": "historical_max_snapshot_entities",
                    }),
                ));
            };
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
            fields: InvariantViolationFields::SnapshotEntityLimit {
                version_id: state_view.version_id(),
                visible_entities,
                limit,
            },
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
            let Some(partition) = state_view.state().get_partition(partition_id) else {
                return Some(storage_inconsistency_violation(
                    class,
                    format!("partition {:?} missing during historical uniqueness scan", partition_id),
                    json!({
                        "partition_id": partition_id.0,
                        "scan": "historical_unique_entity_payload_field",
                        "field": field,
                    }),
                ));
            };
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
        fields: InvariantViolationFields::UniqueEntityPayloadField {
            field: field.to_string(),
            value: value.to_string(),
        },
    }
}

fn evaluate_endpoint_kind_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredEndpointKindContract,
) -> Option<InvariantViolation> {
    let Some(scope) = context.relation_integrity_scope(contract.relation_kind_id) else {
        return None;
    };
    if scope.planned_edges.is_empty() {
        return None;
    }

    context.metrics().count_relation_contracts_evaluated(1);
    for edge in &scope.planned_edges {
        context.metrics().count_relation_endpoint_kind_checks(1);
        let source_kind = match entity_kind_in_state(context, class, edge.source) {
            Ok(Some(kind_id)) => kind_id,
            Ok(None) => continue,
            Err(violation) => return Some(violation),
        };
        let target_kind = match entity_kind_in_state(context, class, edge.target) {
            Ok(Some(kind_id)) => kind_id,
            Ok(None) => continue,
            Err(violation) => return Some(violation),
        };
        if !contract.allows_source_kind(source_kind) {
            return Some(relation_violation(
                class,
                DiagnosticCode::RelationEndpointKindViolation,
                format!(
                    "relation contract '{}' rejected source kind {:?} for relation kind {:?}",
                    contract.contract_id, source_kind, contract.relation_kind_id
                ),
                InvariantViolationFields::RelationEndpointKindMismatch {
                    contract_id: contract.contract_id.clone(),
                    relation_kind_id: contract.relation_kind_id,
                    source: edge.source,
                    target: edge.target,
                    source_kind_id: source_kind,
                    target_kind_id: target_kind,
                    boundary: RelationEndpointBoundary::Source,
                },
            ));
        }
        if !contract.allows_target_kind(target_kind) {
            return Some(relation_violation(
                class,
                DiagnosticCode::RelationEndpointKindViolation,
                format!(
                    "relation contract '{}' rejected target kind {:?} for relation kind {:?}",
                    contract.contract_id, target_kind, contract.relation_kind_id
                ),
                InvariantViolationFields::RelationEndpointKindMismatch {
                    contract_id: contract.contract_id.clone(),
                    relation_kind_id: contract.relation_kind_id,
                    source: edge.source,
                    target: edge.target,
                    source_kind_id: source_kind,
                    target_kind_id: target_kind,
                    boundary: RelationEndpointBoundary::Target,
                },
            ));
        }
        if !contract.self_edges_allowed && edge.source == edge.target {
            return Some(relation_violation(
                class,
                DiagnosticCode::RelationEndpointKindViolation,
                format!(
                    "relation contract '{}' forbids self edges for relation kind {:?}",
                    contract.contract_id, contract.relation_kind_id
                ),
                InvariantViolationFields::RelationEndpointKindSelfEdge {
                    contract_id: contract.contract_id.clone(),
                    relation_kind_id: contract.relation_kind_id,
                    source: edge.source,
                    target: edge.target,
                    self_edge: true,
                },
            ));
        }
        if edge.source.partition_id != edge.target.partition_id
            && contract.cross_context_policy != crate::config::data::CrossContextPolicy::AllowExplicit
        {
            return Some(relation_violation(
                class,
                DiagnosticCode::InvalidRelationEndpoint,
                format!(
                    "relation contract '{}' forbids cross-context endpoints for relation kind {:?}",
                    contract.contract_id, contract.relation_kind_id
                ),
                InvariantViolationFields::RelationEndpointKindCrossContext {
                    contract_id: contract.contract_id.clone(),
                    relation_kind_id: contract.relation_kind_id,
                    source_partition_id: edge.source.partition_id,
                    target_partition_id: edge.target.partition_id,
                },
            ));
        }
    }
    None
}

fn evaluate_cardinality_maximum_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredCardinalityMaximumContract,
) -> Option<InvariantViolation> {
    let Some(scope) = context.relation_integrity_scope(contract.relation_kind_id) else {
        return None;
    };
    if scope.is_empty() {
        return None;
    }
    context.metrics().count_relation_contracts_evaluated(1);
    for (key, count) in &scope.source_counts {
        if let Some(limit) = contract.source_max {
            context.metrics().count_relation_cardinality_checks(1);
            if (*count as u64) > limit {
                return Some(relation_violation(
                    class,
                    DiagnosticCode::RelationCardinalityViolation,
                    format!(
                        "relation contract '{}' overflowed source cardinality for entity {:?}: {} > {}",
                        contract.contract_id, key.entity_id, count, limit
                    ),
                    InvariantViolationFields::RelationCardinalityEndpoint {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        entity_id: key.entity_id,
                        boundary: RelationCardinalityBoundary::Source,
                        count: *count,
                        limit,
                    },
                ));
            }
        }
    }
    for (key, count) in &scope.target_counts {
        if let Some(limit) = contract.target_max {
            context.metrics().count_relation_cardinality_checks(1);
            if (*count as u64) > limit {
                return Some(relation_violation(
                    class,
                    DiagnosticCode::RelationCardinalityViolation,
                    format!(
                        "relation contract '{}' overflowed target cardinality for entity {:?}: {} > {}",
                        contract.contract_id, key.entity_id, count, limit
                    ),
                    InvariantViolationFields::RelationCardinalityEndpoint {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        entity_id: key.entity_id,
                        boundary: RelationCardinalityBoundary::Target,
                        count: *count,
                        limit,
                    },
                ));
            }
        }
    }
    for (key, count) in &scope.directed_pair_counts {
        if let Some(limit) = contract.pair_max {
            context.metrics().count_relation_cardinality_checks(1);
            if (*count as u64) > limit {
                return Some(relation_violation(
                    class,
                    DiagnosticCode::RelationCardinalityViolation,
                    format!(
                        "relation contract '{}' overflowed pair cardinality for {:?}->{:?}: {} > {}",
                        contract.contract_id, key.source, key.target, count, limit
                    ),
                    InvariantViolationFields::RelationCardinalityPair {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        source: key.source,
                        target: key.target,
                        count: *count,
                        limit,
                    },
                ));
            }
        }
    }
    None
}

fn evaluate_cardinality_minimum_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredCardinalityMinimumContract,
) -> Option<InvariantViolation> {
    let snapshot = visible_relation_counts(context, contract);
    context.metrics().count_relation_contracts_evaluated(1);
    context
        .metrics()
        .count_relation_cardinality_minimum_certification(
            1,
            snapshot.entity_slot_scans,
            snapshot.relation_slot_scans,
        );

    if let Some(minimum) = contract.source_min {
        for entity_id in snapshot
            .candidate_source_entities
            .iter()
            .copied()
            .collect::<Vec<_>>()
        {
            let count = snapshot.source_counts.get(&entity_id).copied().unwrap_or_default();
            context.metrics().count_relation_cardinality_checks(1);
            if (count as u64) < minimum {
                return Some(relation_violation(
                    class,
                    DiagnosticCode::RelationCardinalityViolation,
                    format!(
                        "relation contract '{}' underflowed source cardinality for entity {:?}: {} < {}",
                        contract.contract_id, entity_id, count, minimum
                    ),
                    InvariantViolationFields::RelationCardinalityEndpoint {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        entity_id,
                        boundary: RelationCardinalityBoundary::Source,
                        count,
                        limit: minimum,
                    },
                ));
            }
        }
    }

    if let Some(minimum) = contract.target_min {
        for entity_id in snapshot
            .candidate_target_entities
            .iter()
            .copied()
            .collect::<Vec<_>>()
        {
            let count = snapshot.target_counts.get(&entity_id).copied().unwrap_or_default();
            context.metrics().count_relation_cardinality_checks(1);
            if (count as u64) < minimum {
                return Some(relation_violation(
                    class,
                    DiagnosticCode::RelationCardinalityViolation,
                    format!(
                        "relation contract '{}' underflowed target cardinality for entity {:?}: {} < {}",
                        contract.contract_id, entity_id, count, minimum
                    ),
                    InvariantViolationFields::RelationCardinalityEndpoint {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        entity_id,
                        boundary: RelationCardinalityBoundary::Target,
                        count,
                        limit: minimum,
                    },
                ));
            }
        }
    }

    if let Some(minimum) = contract.pair_min {
        match contract.pair_min_semantics {
            PairMinimumSemantics::ObservedDirectedPairs => {
                for ((source, target), count) in snapshot.directed_pair_counts {
                    context.metrics().count_relation_cardinality_checks(1);
                    if (count as u64) < minimum {
                        return Some(relation_violation(
                            class,
                            DiagnosticCode::RelationCardinalityViolation,
                            format!(
                                "relation contract '{}' underflowed observed directed pair cardinality for {:?}->{:?}: {} < {}",
                                contract.contract_id, source, target, count, minimum
                            ),
                            InvariantViolationFields::RelationCardinalityPair {
                                contract_id: contract.contract_id.clone(),
                                relation_kind_id: contract.relation_kind_id,
                                source,
                                target,
                                count,
                                limit: minimum,
                            },
                        ));
                    }
                }
            }
        }
    }

    None
}

#[derive(Default)]
struct VisibleRelationCountSnapshot {
    source_counts: BTreeMap<crate::identity::data::EntityId, usize>,
    target_counts: BTreeMap<crate::identity::data::EntityId, usize>,
    directed_pair_counts:
        BTreeMap<(crate::identity::data::EntityId, crate::identity::data::EntityId), usize>,
    candidate_source_entities: BTreeSet<crate::identity::data::EntityId>,
    candidate_target_entities: BTreeSet<crate::identity::data::EntityId>,
    relation_slot_scans: usize,
    entity_slot_scans: usize,
}

fn visible_relation_counts(
    context: &InvariantExecutionContext<'_>,
    contract: &LoweredCardinalityMinimumContract,
) -> VisibleRelationCountSnapshot {
    let state_view = context.state_view();
    let mut snapshot = VisibleRelationCountSnapshot::default();

    for partition_id in state_view.state().partition_ids() {
        let Some(partition) = state_view.state().get_partition(partition_id) else {
            continue;
        };
        for slot in 0..partition.relation_arena.slot_count() {
            context.metrics().count_relation_slot_scans(1);
            snapshot.relation_slot_scans += 1;
            let Some(metadata) =
                visible_relation_metadata(&state_view, &partition.relation_arena, slot)
            else {
                continue;
            };
            if metadata.kind_id != contract.relation_kind_id {
                continue;
            }
            *snapshot
                .source_counts
                .entry(metadata.endpoints.source)
                .or_insert(0) += 1;
            *snapshot
                .target_counts
                .entry(metadata.endpoints.target)
                .or_insert(0) += 1;
            *snapshot
                .directed_pair_counts
                .entry((metadata.endpoints.source, metadata.endpoints.target))
                .or_insert(0) += 1;
        }

        for slot in 0..partition.entity_arena.slot_count() {
            context.metrics().count_entity_slot_scans(1);
            snapshot.entity_slot_scans += 1;
            let Some(metadata) = state_view
                .entity_metadata_at(&partition.entity_arena, partition_id, slot)
            else {
                continue;
            };
            if contract_candidate_kind_matches(metadata.kind_id, &contract.candidate_source_kinds) {
                snapshot.candidate_source_entities.insert(metadata.entity_id);
            }
            if contract_candidate_kind_matches(metadata.kind_id, &contract.candidate_target_kinds) {
                snapshot.candidate_target_entities.insert(metadata.entity_id);
            }
        }
    }

    snapshot
}

fn contract_candidate_kind_matches(
    kind_id: crate::identity::data::KindId,
    candidate_kinds: &[crate::identity::data::KindId],
) -> bool {
    candidate_kinds.binary_search(&kind_id).is_ok()
}

fn visible_relation_metadata<'state>(
    state_view: &InvariantStateView<'state>,
    arena: &'state crate::storage::logic::state::RelationArena,
    slot: usize,
) -> Option<&'state crate::storage::logic::state::VersionedRelationMetadata> {
    let history = arena.metadata_history_at(slot)?;
    let end = history.partition_point(|entry| entry.effective_at() <= state_view.version_id());
    history[..end].iter().rev().find(|entry| {
        entry.effective_at() <= state_view.version_id()
            && entry
                .retired_at()
                .is_none_or(|retired| state_view.version_id() < retired)
    })
}

fn evaluate_uniqueness_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredUniquenessContract,
) -> Option<InvariantViolation> {
    let Some(scope) = context.relation_integrity_scope(contract.relation_kind_id) else {
        return None;
    };
    if scope.is_empty() {
        return None;
    }
    context.metrics().count_relation_contracts_evaluated(1);
    match contract.scope {
        UniquenessScope::DirectedSemanticEdge => {
            for (key, count) in &scope.directed_pair_counts {
                context.metrics().count_relation_uniqueness_checks(1);
                if *count > 1 {
                    return Some(relation_violation(
                        class,
                        DiagnosticCode::RelationUniquenessViolation,
                        format!(
                            "relation contract '{}' forbids duplicate directed edge {:?}->{:?}",
                            contract.contract_id, key.source, key.target
                        ),
                        InvariantViolationFields::RelationUniqueness {
                            contract_id: contract.contract_id.clone(),
                            relation_kind_id: contract.relation_kind_id,
                            scope: contract.scope,
                            source: key.source,
                            target: key.target,
                            count: *count,
                        },
                    ));
                }
            }
        }
        UniquenessScope::NormalizedSymmetricEdge => {
            for (key, count) in &scope.normalized_pair_counts {
                context.metrics().count_relation_uniqueness_checks(1);
                if *count > 1 {
                    return Some(relation_violation(
                        class,
                        DiagnosticCode::RelationUniquenessViolation,
                        format!(
                            "relation contract '{}' forbids duplicate normalized edge {:?}<->{:?}",
                            contract.contract_id, key.source, key.target
                        ),
                        InvariantViolationFields::RelationUniqueness {
                            contract_id: contract.contract_id.clone(),
                            relation_kind_id: contract.relation_kind_id,
                            scope: contract.scope,
                            source: key.source,
                            target: key.target,
                            count: *count,
                        },
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
    let Some(scope) = context.relation_integrity_scope(contract.relation_kind_id) else {
        return None;
    };
    if scope.planned_edges.is_empty() {
        return None;
    }
    context.metrics().count_relation_contracts_evaluated(1);
    for edge in &scope.planned_edges {
        context.metrics().count_relation_symmetry_checks(1);
        match contract.mode {
            SymmetryMode::CanonicalUndirected => {
                if edge.target < edge.source {
                    return Some(relation_violation(
                        class,
                        DiagnosticCode::RelationSymmetryViolation,
                        format!(
                            "relation contract '{}' requires canonical undirected ordering",
                            contract.contract_id
                        ),
                        InvariantViolationFields::RelationSymmetry {
                            contract_id: contract.contract_id.clone(),
                            relation_kind_id: contract.relation_kind_id,
                            source: edge.source,
                            target: edge.target,
                            mode: contract.mode,
                        },
                    ));
                }
            }
            SymmetryMode::PairedInverseRequired | SymmetryMode::PairedTwinRequired => {
                let inverse = super::request::PreparedRelationPairKey {
                    source: edge.target,
                    target: edge.source,
                };
                if scope
                    .directed_pair_counts
                    .get(&inverse)
                    .copied()
                    .unwrap_or_default()
                    == 0
                {
                    return Some(relation_violation(
                        class,
                        DiagnosticCode::RelationSymmetryViolation,
                        format!(
                            "relation contract '{}' requires an inverse/twin edge for {:?}->{:?}",
                            contract.contract_id, edge.source, edge.target
                        ),
                        InvariantViolationFields::RelationSymmetry {
                            contract_id: contract.contract_id.clone(),
                            relation_kind_id: contract.relation_kind_id,
                            source: edge.source,
                            target: edge.target,
                            mode: contract.mode,
                        },
                    ));
                }
            }
            SymmetryMode::InverseProhibited => {
                let inverse = super::request::PreparedRelationPairKey {
                    source: edge.target,
                    target: edge.source,
                };
                if scope
                    .directed_pair_counts
                    .get(&inverse)
                    .copied()
                    .unwrap_or_default()
                    > 0
                {
                    return Some(relation_violation(
                        class,
                        DiagnosticCode::RelationSymmetryViolation,
                        format!(
                            "relation contract '{}' prohibits inverse duplication for {:?}->{:?}",
                            contract.contract_id, edge.source, edge.target
                        ),
                        InvariantViolationFields::RelationSymmetry {
                            contract_id: contract.contract_id.clone(),
                            relation_kind_id: contract.relation_kind_id,
                            source: edge.source,
                            target: edge.target,
                            mode: contract.mode,
                        },
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
    let Some(scope) = context.relation_integrity_scope(contract.relation_kind_id) else {
        return None;
    };
    if scope.deleted_entities.is_empty() {
        return None;
    }
    context.metrics().count_relation_contracts_evaluated(1);
    for entity_id in &scope.deleted_entities {
        let endpoint_key = super::request::PreparedRelationEndpointKey {
            entity_id: *entity_id,
        };
        let live_relations = scope.source_counts.get(&endpoint_key).copied().unwrap_or_default()
            + scope.target_counts.get(&endpoint_key).copied().unwrap_or_default();
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
                        InvariantViolationFields::RelationEndpointDeletionIntegrity {
                            contract_id: contract.contract_id.clone(),
                            relation_kind_id: contract.relation_kind_id,
                            entity_id: *entity_id,
                            remaining_relation_endpoint_count: live_relations,
                            mode: contract.mode,
                            cascade_delete_policy: None,
                        },
                    ));
                }
                EndpointDeletionIntegrityMode::RequireRelationDeletionInSameCommit => {
                    if contract.cascade_delete_policy
                        != crate::config::data::CascadeDeletePolicy::CascadeDeleteRelations
                    {
                        return Some(relation_violation(
                            class,
                            DiagnosticCode::RelationEndpointDeletionIntegrityViolation,
                            format!(
                                "relation contract '{}' requires deleting dependent relations in the same commit before deleting endpoint {:?}",
                                contract.contract_id, entity_id
                            ),
                            InvariantViolationFields::RelationEndpointDeletionIntegrity {
                                contract_id: contract.contract_id.clone(),
                                relation_kind_id: contract.relation_kind_id,
                                entity_id: *entity_id,
                                remaining_relation_endpoint_count: live_relations,
                                mode: contract.mode,
                                cascade_delete_policy: Some(contract.cascade_delete_policy),
                            },
                        ));
                    }
                }
                EndpointDeletionIntegrityMode::RequireRelationRetirement => {
                    if contract.cascade_delete_policy
                        != crate::config::data::CascadeDeletePolicy::RetainDanglingForAudit
                    {
                        return Some(relation_violation(
                            class,
                            DiagnosticCode::RelationEndpointDeletionIntegrityViolation,
                            format!(
                                "relation contract '{}' requires audit-retained relation retirement before deleting endpoint {:?}",
                                contract.contract_id, entity_id
                            ),
                            InvariantViolationFields::RelationEndpointDeletionIntegrity {
                                contract_id: contract.contract_id.clone(),
                                relation_kind_id: contract.relation_kind_id,
                                entity_id: *entity_id,
                                remaining_relation_endpoint_count: live_relations,
                                mode: contract.mode,
                                cascade_delete_policy: Some(contract.cascade_delete_policy),
                            },
                        ));
                    }
                }
            }
        }
    }
    None
}

fn entity_kind_in_state(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    entity_id: crate::identity::data::EntityId,
) -> Result<Option<crate::identity::data::KindId>, InvariantViolation> {
    let Some(partition) = context.partition_access().get_partition(entity_id.partition_id) else {
        return Err(storage_inconsistency_violation(
            class,
            format!(
                "entity endpoint {:?} references missing partition {:?}",
                entity_id, entity_id.partition_id
            ),
            json!({
                "entity_id": entity_id,
                "partition_id": entity_id.partition_id.0,
                "lookup": "entity_kind_in_state",
            }),
        ));
    };
    let Some(slot) = partition.entity_arena.get(&entity_id) else {
        return Err(storage_inconsistency_violation(
            class,
            format!("entity endpoint {:?} references missing entity slot", entity_id),
            json!({
                "entity_id": entity_id,
                "partition_id": entity_id.partition_id.0,
                "lookup": "entity_kind_in_state",
                "failure": "missing_slot",
            }),
        ));
    };
    if slot.lifecycle() != RecordLifecycleState::Live {
        return Ok(None);
    }
    slot.kind_id().ok_or_else(|| {
        storage_inconsistency_violation(
            class,
            format!("entity endpoint {:?} is live but missing kind id", entity_id),
            json!({
                "entity_id": entity_id,
                "partition_id": entity_id.partition_id.0,
                "lookup": "entity_kind_in_state",
                "failure": "missing_kind_id",
            }),
        )
    }).map(Some)
}

fn relation_violation(
    class: InvariantClass,
    code: DiagnosticCode,
    detail: String,
    fields: InvariantViolationFields,
) -> InvariantViolation {
    InvariantViolation {
        class,
        code,
        detail,
        fields,
    }
}

fn storage_inconsistency_violation(
    class: InvariantClass,
    detail: String,
    fields: serde_json::Value,
) -> InvariantViolation {
    let mut entity_id = None;
    let mut partition_id = None;
    let mut slot = None;
    let mut missing_label = None;
    let mut scan = None;
    let mut lookup = None;
    let mut failure = None;
    if let Some(object) = fields.as_object() {
        entity_id = object
            .get("entity_id")
            .cloned()
            .and_then(|value| serde_json::from_value(value).ok());
        partition_id = object
            .get("partition_id")
            .and_then(|value| value.as_u64())
            .map(|value| crate::identity::data::PartitionId(value as u32));
        slot = object
            .get("slot")
            .and_then(|value| value.as_u64())
            .map(|value| value as usize);
        missing_label = object
            .get("missing_label")
            .and_then(|value| value.as_str())
            .map(ToOwned::to_owned);
        scan = object
            .get("scan")
            .and_then(|value| value.as_str())
            .map(ToOwned::to_owned);
        lookup = object
            .get("lookup")
            .and_then(|value| value.as_str())
            .map(ToOwned::to_owned);
        failure = object
            .get("failure")
            .and_then(|value| value.as_str())
            .map(ToOwned::to_owned);
    }
    InvariantViolation {
        class,
        code: DiagnosticCode::StorageInconsistencyDetected,
        detail,
        fields: InvariantViolationFields::StorageInconsistency {
            entity_id,
            partition_id,
            slot,
            missing_label,
            scan,
            lookup,
            failure,
        },
    }
}
