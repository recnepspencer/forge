use std::collections::{BTreeMap, BTreeSet};

use crate::diagnostics::data::DiagnosticCode;
use crate::schema::data::{LoweredCardinalityMaximumContract, LoweredCardinalityMinimumContract};
use crate::storage::logic::state::HistoricalMetadata;
use crate::transactions::data::{CreatedEntityRef, EntityReference};
use crate::validation::data::{
    InvariantClass, InvariantViolation, InvariantViolationFields, RelationCardinalityBoundary,
};

use super::super::context::InvariantExecutionContext;
use super::super::state_view::InvariantStateView;
use super::common::{canonicalize_violations, contract_candidate_kind_matches, relation_violation};

pub(super) fn evaluate_cardinality_maximum_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredCardinalityMaximumContract,
) -> Vec<InvariantViolation> {
    let Some(scope) = context.relation_integrity_scope(contract.relation_kind_id) else {
        return Vec::new();
    };
    if scope.is_empty() {
        return Vec::new();
    }
    context.metrics().count_relation_contracts_evaluated(1);
    let mut violations = Vec::new();
    for (key, count) in &scope.source_counts {
        if let Some(limit) = contract.source_max {
            context.metrics().count_relation_cardinality_checks(1);
            if (*count as u64) > limit {
                violations.push(relation_violation(
                    class,
                    DiagnosticCode::RelationCardinalityViolation,
                    format!(
                        "relation contract '{}' overflowed source cardinality for entity {:?}: {} > {}",
                        contract.contract_id, key.entity_id, count, limit
                    ),
                    InvariantViolationFields::RelationCardinalityEndpoint {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        entity_id: key.entity_id.clone(),
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
                violations.push(relation_violation(
                    class,
                    DiagnosticCode::RelationCardinalityViolation,
                    format!(
                        "relation contract '{}' overflowed target cardinality for entity {:?}: {} > {}",
                        contract.contract_id, key.entity_id, count, limit
                    ),
                    InvariantViolationFields::RelationCardinalityEndpoint {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        entity_id: key.entity_id.clone(),
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
                violations.push(relation_violation(
                    class,
                    DiagnosticCode::RelationCardinalityViolation,
                    format!(
                        "relation contract '{}' overflowed pair cardinality for {:?}->{:?}: {} > {}",
                        contract.contract_id, key.source, key.target, count, limit
                    ),
                    InvariantViolationFields::RelationCardinalityPair {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        source: key.source.clone(),
                        target: key.target.clone(),
                        count: *count,
                        limit,
                    },
                ));
            }
        }
    }
    canonicalize_violations(violations)
}

pub(super) fn evaluate_cardinality_minimum_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredCardinalityMinimumContract,
) -> Vec<InvariantViolation> {
    let snapshot = visible_relation_counts(context, contract);
    context.metrics().count_relation_contracts_evaluated(1);
    context
        .metrics()
        .count_relation_cardinality_minimum_certification(
            1,
            snapshot.entity_slot_scans,
            snapshot.relation_slot_scans,
        );
    let mut violations = Vec::new();

    if let Some(minimum) = contract.source_min {
        for entity_id in snapshot
            .candidate_source_entities
            .iter()
            .cloned()
            .collect::<Vec<_>>()
        {
            let count = snapshot
                .source_counts
                .get(&entity_id)
                .copied()
                .unwrap_or_default();
            context.metrics().count_relation_cardinality_checks(1);
            if (count as u64) < minimum {
                violations.push(relation_violation(
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
            .cloned()
            .collect::<Vec<_>>()
        {
            let count = snapshot
                .target_counts
                .get(&entity_id)
                .copied()
                .unwrap_or_default();
            context.metrics().count_relation_cardinality_checks(1);
            if (count as u64) < minimum {
                violations.push(relation_violation(
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
        for ((source, target), count) in &snapshot.directed_pair_counts {
            context.metrics().count_relation_cardinality_checks(1);
            if (count.to_owned() as u64) < minimum {
                violations.push(relation_violation(
                    class,
                    DiagnosticCode::RelationCardinalityViolation,
                    format!(
                        "relation contract '{}' underflowed pair cardinality for {:?}->{:?}: {} < {}",
                        contract.contract_id, source, target, count, minimum
                    ),
                    InvariantViolationFields::RelationCardinalityPair {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        source: source.clone(),
                        target: target.clone(),
                        count: *count,
                        limit: minimum,
                    },
                ));
            }
        }
    }

    canonicalize_violations(violations)
}

#[derive(Default)]
struct VisibleRelationCountSnapshot {
    source_counts: BTreeMap<EntityReference, usize>,
    target_counts: BTreeMap<EntityReference, usize>,
    directed_pair_counts: BTreeMap<(EntityReference, EntityReference), usize>,
    candidate_source_entities: BTreeSet<EntityReference>,
    candidate_target_entities: BTreeSet<EntityReference>,
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
        if state_view.version_id() == context.current_version_id() {
            for slot in partition.relation_arena.live_bitset.iter_set_slots() {
                context.metrics().count_relation_slot_scans(1);
                snapshot.relation_slot_scans += 1;
                let Some(slot_view) = partition.relation_arena.get_slot(slot) else {
                    continue;
                };
                let Some(kind_id) = slot_view.kind_id() else {
                    continue;
                };
                if kind_id != contract.relation_kind_id {
                    continue;
                }
                let Some(endpoints) = slot_view.extra().endpoints.as_ref() else {
                    continue;
                };
                *snapshot
                    .source_counts
                    .entry(EntityReference::Existing(endpoints.source))
                    .or_insert(0) += 1;
                *snapshot
                    .target_counts
                    .entry(EntityReference::Existing(endpoints.target))
                    .or_insert(0) += 1;
                *snapshot
                    .directed_pair_counts
                    .entry((
                        EntityReference::Existing(endpoints.source),
                        EntityReference::Existing(endpoints.target),
                    ))
                    .or_insert(0) += 1;
            }

            for slot in partition.entity_arena.live_bitset.iter_set_slots() {
                context.metrics().count_entity_slot_scans(1);
                snapshot.entity_slot_scans += 1;
                let Some(slot_view) = partition.entity_arena.get_slot(slot) else {
                    continue;
                };
                let Some(kind_id) = slot_view.kind_id() else {
                    continue;
                };
                let entity_id = crate::identity::data::EntityId::new(
                    partition_id,
                    slot as u64,
                    slot_view.generation(),
                );
                if contract_candidate_kind_matches(kind_id, &contract.candidate_source_kinds) {
                    snapshot
                        .candidate_source_entities
                        .insert(EntityReference::Existing(entity_id));
                }
                if contract_candidate_kind_matches(kind_id, &contract.candidate_target_kinds) {
                    snapshot
                        .candidate_target_entities
                        .insert(EntityReference::Existing(entity_id));
                }
            }
        } else {
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
                    .entry(EntityReference::Existing(metadata.endpoints.source))
                    .or_insert(0) += 1;
                *snapshot
                    .target_counts
                    .entry(EntityReference::Existing(metadata.endpoints.target))
                    .or_insert(0) += 1;
                *snapshot
                    .directed_pair_counts
                    .entry((
                        EntityReference::Existing(metadata.endpoints.source),
                        EntityReference::Existing(metadata.endpoints.target),
                    ))
                    .or_insert(0) += 1;
            }

            for slot in 0..partition.entity_arena.slot_count() {
                context.metrics().count_entity_slot_scans(1);
                snapshot.entity_slot_scans += 1;
                let Some(metadata) =
                    state_view.entity_metadata_at(&partition.entity_arena, partition_id, slot)
                else {
                    continue;
                };
                if contract_candidate_kind_matches(
                    metadata.kind_id,
                    &contract.candidate_source_kinds,
                ) {
                    snapshot
                        .candidate_source_entities
                        .insert(EntityReference::Existing(metadata.entity_id));
                }
                if contract_candidate_kind_matches(
                    metadata.kind_id,
                    &contract.candidate_target_kinds,
                ) {
                    snapshot
                        .candidate_target_entities
                        .insert(EntityReference::Existing(metadata.entity_id));
                }
            }
        }
    }

    if let Some(merged_plan) = context.merged_plan() {
        for intent in &merged_plan.merged_intents {
            match intent {
                crate::transactions::data::MutationIntent::Create(
                    crate::transactions::data::CreateIntent::Entity(spec),
                ) => {
                    let entity = EntityReference::Created(CreatedEntityRef {
                        partition_id: spec.partition_id,
                        kind_id: spec.kind_id,
                        client_key: spec.client_key.clone(),
                    });
                    if contract_candidate_kind_matches(
                        spec.kind_id,
                        &contract.candidate_source_kinds,
                    ) {
                        snapshot.candidate_source_entities.insert(entity.clone());
                    }
                    if contract_candidate_kind_matches(
                        spec.kind_id,
                        &contract.candidate_target_kinds,
                    ) {
                        snapshot.candidate_target_entities.insert(entity);
                    }
                }
                crate::transactions::data::MutationIntent::Create(
                    crate::transactions::data::CreateIntent::BulkEntities(spec),
                ) => {
                    for client_key in &spec.client_keys {
                        let entity = EntityReference::Created(CreatedEntityRef {
                            partition_id: spec.partition_id,
                            kind_id: spec.kind_id,
                            client_key: client_key.clone(),
                        });
                        if contract_candidate_kind_matches(
                            spec.kind_id,
                            &contract.candidate_source_kinds,
                        ) {
                            snapshot.candidate_source_entities.insert(entity.clone());
                        }
                        if contract_candidate_kind_matches(
                            spec.kind_id,
                            &contract.candidate_target_kinds,
                        ) {
                            snapshot.candidate_target_entities.insert(entity);
                        }
                    }
                }
                crate::transactions::data::MutationIntent::Create(
                    crate::transactions::data::CreateIntent::Relation(spec),
                ) => {
                    if spec.kind_id == contract.relation_kind_id {
                        *snapshot
                            .source_counts
                            .entry(spec.source.clone())
                            .or_insert(0) += 1;
                        *snapshot
                            .target_counts
                            .entry(spec.target.clone())
                            .or_insert(0) += 1;
                        *snapshot
                            .directed_pair_counts
                            .entry((spec.source.clone(), spec.target.clone()))
                            .or_insert(0) += 1;
                    }
                }
                crate::transactions::data::MutationIntent::Create(
                    crate::transactions::data::CreateIntent::BulkRelations(spec),
                ) => {
                    if spec.kind_id == contract.relation_kind_id {
                        for (source, target) in &spec.endpoints {
                            *snapshot.source_counts.entry(source.clone()).or_insert(0) += 1;
                            *snapshot.target_counts.entry(target.clone()).or_insert(0) += 1;
                            *snapshot
                                .directed_pair_counts
                                .entry((source.clone(), target.clone()))
                                .or_insert(0) += 1;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    snapshot
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
