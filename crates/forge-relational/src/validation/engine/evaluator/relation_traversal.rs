use std::collections::{BTreeMap, BTreeSet};

use crate::diagnostics::data::DiagnosticCode;
use crate::schema::data::{LoweredAcyclicityContract, LoweredConnectivityMinimumContract};
use crate::transactions::data::{CreatedEntityRef, EntityReference};
use crate::validation::data::{InvariantClass, InvariantViolation, InvariantViolationFields};

use super::super::context::InvariantExecutionContext;
use super::common::{contract_candidate_kind_matches, entity_reference_kind};

pub(super) fn evaluate_acyclicity_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredAcyclicityContract,
) -> Option<InvariantViolation> {
    let Some(scope) = context.relation_integrity_scope(contract.relation_kind_id) else {
        return None;
    };
    if scope.planned_edges.is_empty() {
        return None;
    }

    context.metrics().count_relation_contracts_evaluated(1);
    let planned_successors = planned_successor_map(&scope.planned_edges);
    for edge in &scope.planned_edges {
        context.metrics().count_relation_slot_scans(1);
        let reaches_cycle = if edge.source == edge.target {
            Ok(true)
        } else {
            relation_kind_reaches(
                context,
                class,
                &contract.contract_id,
                contract.relation_kind_id,
                edge.target.clone(),
                edge.source.clone(),
                &planned_successors,
            )
        };
        match reaches_cycle {
            Ok(true) => {
                return Some(InvariantViolation {
                    class,
                    code: DiagnosticCode::InvariantViolation,
                    detail: format!(
                        "acyclicity contract '{}' detected a cycle for relation kind {:?}",
                        contract.contract_id, contract.relation_kind_id
                    ),
                    fields: InvariantViolationFields::Acyclicity {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        source: edge.source.clone(),
                        target: edge.target.clone(),
                    },
                });
            }
            Ok(false) => {}
            Err(violation) => return Some(violation),
        }
    }
    None
}

pub(super) fn evaluate_connectivity_minimum_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredConnectivityMinimumContract,
) -> Option<InvariantViolation> {
    context.metrics().count_relation_contracts_evaluated(1);
    let source_entities = visible_entities_of_kinds(context, &contract.source_kind_ids);
    if source_entities.is_empty() {
        return None;
    }

    let planned_successors = context
        .relation_integrity_scope(contract.relation_kind_id)
        .map(|scope| planned_successor_map(&scope.planned_edges))
        .unwrap_or_default();
    for source in source_entities {
        let reachable_target_count = match reachable_target_count_for_connectivity(
            context,
            class,
            &contract.contract_id,
            contract.relation_kind_id,
            source.clone(),
            &contract.target_kind_ids,
            &planned_successors,
        ) {
            Ok(count) => count,
            Err(violation) => return Some(violation),
        };
        if reachable_target_count < contract.minimum_reachable_targets as usize {
            return Some(InvariantViolation {
                class,
                code: DiagnosticCode::InvariantViolation,
                detail: format!(
                    "connectivity minimum contract '{}' requires at least {} reachable target(s) for {:?}",
                    contract.contract_id,
                    contract.minimum_reachable_targets,
                    source
                ),
                fields: InvariantViolationFields::ConnectivityMinimum {
                    contract_id: contract.contract_id.clone(),
                    relation_kind_id: contract.relation_kind_id,
                    source,
                    reachable_target_count,
                    minimum_reachable_targets: contract.minimum_reachable_targets,
                },
            });
        }
    }
    None
}

fn visible_entities_of_kinds(
    context: &InvariantExecutionContext<'_>,
    kind_ids: &[crate::identity::data::KindId],
) -> Vec<EntityReference> {
    let state_view = context.state_view();
    let mut entities = Vec::new();
    for partition_id in state_view.state().partition_ids() {
        let Some(partition) = state_view.state().get_partition(partition_id) else {
            continue;
        };
        if state_view.version_id() == context.current_version_id() {
            for slot in partition.entity_arena.live_bitset.iter_set_slots() {
                context.metrics().count_entity_slot_scans(1);
                let Some(slot_view) = partition.entity_arena.get_slot(slot) else {
                    continue;
                };
                let Some(kind_id) = slot_view.kind_id() else {
                    continue;
                };
                if contract_candidate_kind_matches(kind_id, kind_ids) {
                    entities.push(EntityReference::Existing(
                        crate::identity::data::EntityId::new(
                            partition_id,
                            slot as u64,
                            slot_view.generation(),
                        ),
                    ));
                }
            }
        } else {
            for slot in 0..partition.entity_arena.slot_count() {
                let Some(metadata) =
                    state_view.entity_metadata_at(&partition.entity_arena, partition_id, slot)
                else {
                    continue;
                };
                context.metrics().count_entity_slot_scans(1);
                if contract_candidate_kind_matches(metadata.kind_id, kind_ids) {
                    entities.push(EntityReference::Existing(metadata.entity_id));
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
                    if contract_candidate_kind_matches(spec.kind_id, kind_ids) {
                        entities.push(EntityReference::Created(CreatedEntityRef {
                            partition_id: spec.partition_id,
                            kind_id: spec.kind_id,
                            client_key: spec.client_key.clone(),
                        }));
                    }
                }
                crate::transactions::data::MutationIntent::Create(
                    crate::transactions::data::CreateIntent::BulkEntities(spec),
                ) => {
                    if contract_candidate_kind_matches(spec.kind_id, kind_ids) {
                        entities.extend(spec.client_keys.iter().cloned().map(|client_key| {
                            EntityReference::Created(CreatedEntityRef {
                                partition_id: spec.partition_id,
                                kind_id: spec.kind_id,
                                client_key,
                            })
                        }));
                    }
                }
                _ => {}
            }
        }
    }
    entities
}

fn reachable_target_count_for_connectivity(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract_id: &crate::schema::data::ContractId,
    relation_kind_id: crate::identity::data::KindId,
    source: EntityReference,
    target_kind_ids: &[crate::identity::data::KindId],
    planned_successors: &BTreeMap<EntityReference, Vec<EntityReference>>,
) -> Result<usize, InvariantViolation> {
    let mut visited = BTreeSet::new();
    let mut frontier = vec![source.clone()];
    let mut reachable_targets = BTreeSet::new();
    let planned_edge_count = planned_successor_count(planned_successors);
    let mut traversal_budget = RelationTraversalBudget::new(
        context.relation_integrity_scope_budget(),
        planned_edge_count,
    );
    visited.insert(source);
    if traversal_budget.record_entity_visit().is_err() {
        return Err(traversal_budget_exceeded_violation(
            class,
            contract_id,
            relation_kind_id,
            traversal_budget,
            planned_edge_count,
        ));
    }

    while let Some(entity_id) = frontier.pop() {
        for next in relation_kind_successors(
            context,
            class,
            contract_id,
            relation_kind_id,
            &entity_id,
            planned_successors,
            &mut traversal_budget,
        )? {
            if !visited.insert(next.clone()) {
                continue;
            }
            if traversal_budget.record_entity_visit().is_err() {
                return Err(traversal_budget_exceeded_violation(
                    class,
                    contract_id,
                    relation_kind_id,
                    traversal_budget,
                    planned_edge_count,
                ));
            }
            if let Some(kind_id) = entity_reference_kind(context, class, &next)? {
                if contract_candidate_kind_matches(kind_id, target_kind_ids) {
                    reachable_targets.insert(next.clone());
                }
            }
            frontier.push(next);
        }
    }

    Ok(reachable_targets.len())
}

fn relation_kind_reaches(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract_id: &crate::schema::data::ContractId,
    relation_kind_id: crate::identity::data::KindId,
    start: EntityReference,
    target: EntityReference,
    planned_successors: &BTreeMap<EntityReference, Vec<EntityReference>>,
) -> Result<bool, InvariantViolation> {
    let mut visited = BTreeSet::new();
    let mut frontier = vec![start.clone()];
    let planned_edge_count = planned_successor_count(planned_successors);
    let mut traversal_budget = RelationTraversalBudget::new(
        context.relation_integrity_scope_budget(),
        planned_edge_count,
    );
    visited.insert(start);
    if traversal_budget.record_entity_visit().is_err() {
        return Err(traversal_budget_exceeded_violation(
            class,
            contract_id,
            relation_kind_id,
            traversal_budget,
            planned_edge_count,
        ));
    }

    while let Some(entity_id) = frontier.pop() {
        for next in relation_kind_successors(
            context,
            class,
            contract_id,
            relation_kind_id,
            &entity_id,
            planned_successors,
            &mut traversal_budget,
        )? {
            if next == target {
                return Ok(true);
            }
            if visited.insert(next.clone()) {
                if traversal_budget.record_entity_visit().is_err() {
                    return Err(traversal_budget_exceeded_violation(
                        class,
                        contract_id,
                        relation_kind_id,
                        traversal_budget,
                        planned_edge_count,
                    ));
                }
                frontier.push(next);
            }
        }
    }

    Ok(false)
}

fn relation_kind_successors(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract_id: &crate::schema::data::ContractId,
    relation_kind_id: crate::identity::data::KindId,
    entity_id: &EntityReference,
    planned_successors: &BTreeMap<EntityReference, Vec<EntityReference>>,
    traversal_budget: &mut RelationTraversalBudget,
) -> Result<Vec<EntityReference>, InvariantViolation> {
    let mut successors = BTreeSet::new();
    let planned_edge_count = planned_successor_count(planned_successors);
    if let Some(edges) = planned_successors.get(entity_id) {
        for target in edges {
            if traversal_budget.record_relation_scan().is_err() {
                return Err(traversal_budget_exceeded_violation(
                    class,
                    contract_id,
                    relation_kind_id,
                    *traversal_budget,
                    planned_edge_count,
                ));
            }
            successors.insert(target.clone());
        }
    }
    let EntityReference::Existing(entity_id) = entity_id else {
        return Ok(successors.into_iter().collect());
    };
    let Some(partition) = context
        .partition_access()
        .get_partition(entity_id.partition_id)
    else {
        return Ok(successors.into_iter().collect());
    };
    let slot = entity_id.slot_index();
    let outgoing = partition
        .adjacency
        .get(slot)
        .map(|set| set.as_slice())
        .into_iter()
        .flatten();
    for relation_id in outgoing.copied() {
        context.metrics().count_relation_slot_scans(1);
        if traversal_budget.record_relation_scan().is_err() {
            return Err(traversal_budget_exceeded_violation(
                class,
                contract_id,
                relation_kind_id,
                *traversal_budget,
                planned_edge_count,
            ));
        }
        let Some(metadata) = context.state_view().relation_metadata(relation_id) else {
            continue;
        };
        if metadata.kind_id == relation_kind_id {
            successors.insert(EntityReference::Existing(metadata.target));
        }
    }
    Ok(successors.into_iter().collect())
}

#[derive(Debug, Clone, Copy)]
struct RelationTraversalBudget {
    max_relation_scans: usize,
    max_visited_entities: usize,
    relation_scans: usize,
    visited_entities: usize,
}

impl RelationTraversalBudget {
    fn new(
        budget: &crate::config::data::RelationIntegrityScopeBudget,
        planned_edge_count: usize,
    ) -> Self {
        Self {
            max_relation_scans: budget
                .max_scanned_relations
                .saturating_add(planned_edge_count),
            max_visited_entities: budget
                .max_scanned_relations
                .saturating_add(planned_edge_count)
                .saturating_add(1),
            relation_scans: 0,
            visited_entities: 0,
        }
    }

    fn record_relation_scan(&mut self) -> Result<(), ()> {
        self.relation_scans = self.relation_scans.saturating_add(1);
        if self.relation_scans > self.max_relation_scans {
            return Err(());
        }
        Ok(())
    }

    fn record_entity_visit(&mut self) -> Result<(), ()> {
        self.visited_entities = self.visited_entities.saturating_add(1);
        if self.visited_entities > self.max_visited_entities {
            return Err(());
        }
        Ok(())
    }
}

fn planned_successor_map(
    planned_edges: &[super::super::request::PlannedRelationEdge],
) -> BTreeMap<EntityReference, Vec<EntityReference>> {
    let mut successors = BTreeMap::new();
    for edge in planned_edges {
        successors
            .entry(edge.source.clone())
            .or_insert_with(Vec::new)
            .push(edge.target.clone());
    }
    successors
}

fn planned_successor_count(
    planned_successors: &BTreeMap<EntityReference, Vec<EntityReference>>,
) -> usize {
    planned_successors.values().map(Vec::len).sum()
}

fn traversal_budget_exceeded_violation(
    class: InvariantClass,
    contract_id: &crate::schema::data::ContractId,
    relation_kind_id: crate::identity::data::KindId,
    traversal_budget: RelationTraversalBudget,
    planned_edge_count: usize,
) -> InvariantViolation {
    InvariantViolation {
        class,
        code: DiagnosticCode::InvariantViolation,
        detail: format!(
            "relation contract '{}' exceeded evaluator traversal budget for relation kind {:?}",
            contract_id, relation_kind_id
        ),
        fields: InvariantViolationFields::RelationIntegrityScopeBudgetExceeded {
            limit_name: "max_scanned_relations".to_string(),
            limit: traversal_budget.max_relation_scans,
            observed: traversal_budget.relation_scans,
            relation_kind_count: 1,
            touched_entity_count: traversal_budget.visited_entities,
            deleted_entity_count: 0,
            scanned_relation_count: traversal_budget.relation_scans,
            planned_edge_count,
        },
    }
}
