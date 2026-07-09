use std::collections::{BTreeMap, BTreeSet};

use crate::config::data::RelationIntegrityScopeBudget;
use crate::identity::data::{EntityId, KindId, RelationId};
use crate::storage::overlay::PartitionAccess;
use crate::transactions::data::{EntityReference, MergedCommitPlan};
use crate::validation::data::{
    InvariantExecutionPoint, InvariantViolation, InvariantViolationFields,
};

use super::{PlannedRelationEdge, PreparedRelationIntegrityScope, PreparedRelationIntegrityScopes};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RelationIntegrityScopeBudgetSnapshot {
    relation_kind_count: usize,
    touched_entity_count: usize,
    deleted_entity_count: usize,
    scanned_relation_count: usize,
    planned_edge_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreparedRelationIntegrityScopeBudgetExceeded {
    limit_name: &'static str,
    limit: usize,
    observed: usize,
    snapshot: RelationIntegrityScopeBudgetSnapshot,
}

impl PreparedRelationIntegrityScopeBudgetExceeded {
    pub(super) fn into_violation(
        self,
        execution_point: InvariantExecutionPoint,
    ) -> InvariantViolation {
        InvariantViolation {
            class: execution_point.class(),
            code: crate::diagnostics::data::DiagnosticCode::PreparationFailure,
            detail: format!(
                "relation integrity scope preparation exceeded '{}' budget: {} > {}",
                self.limit_name, self.observed, self.limit
            ),
            fields: InvariantViolationFields::RelationIntegrityScopeBudgetExceeded {
                limit_name: self.limit_name.to_string(),
                limit: self.limit,
                observed: self.observed,
                relation_kind_count: self.snapshot.relation_kind_count,
                touched_entity_count: self.snapshot.touched_entity_count,
                deleted_entity_count: self.snapshot.deleted_entity_count,
                scanned_relation_count: self.snapshot.scanned_relation_count,
                planned_edge_count: self.snapshot.planned_edge_count,
            },
        }
    }
}

pub(crate) fn prepare_relation_integrity_scopes(
    merged_plan: Option<&MergedCommitPlan>,
    partitions: &dyn PartitionAccess,
    performance: &crate::performance::logic::PerformanceAccess<'_>,
    budget: &RelationIntegrityScopeBudget,
) -> Result<Option<PreparedRelationIntegrityScopes>, PreparedRelationIntegrityScopeBudgetExceeded> {
    let Some(merged_plan) = merged_plan else {
        return Ok(None);
    };
    let mut scopes = BTreeMap::<KindId, PreparedRelationIntegrityScope>::new();
    let mut touched_entities = BTreeSet::<EntityId>::new();
    let mut touched_relation_sources = BTreeSet::<EntityId>::new();
    let mut touched_relation_targets = BTreeSet::<EntityId>::new();
    let mut deleted_entities = BTreeSet::new();
    let mut deleted_relations = BTreeSet::new();
    let empty_scanned_relations = BTreeSet::new();
    let mut planned_edge_count = 0usize;

    for intent in &merged_plan.merged_intents {
        match intent {
            crate::transactions::data::MutationIntent::Create(
                crate::transactions::data::CreateIntent::Relation(spec),
            ) => {
                scopes
                    .entry(spec.kind_id)
                    .or_default()
                    .planned_edges
                    .push(PlannedRelationEdge {
                        source: spec.source.clone(),
                        target: spec.target.clone(),
                    });
                planned_edge_count += 1;
                include_existing_entity_reference(&mut touched_entities, &spec.source);
                include_existing_entity_reference(&mut touched_entities, &spec.target);
                include_existing_entity_reference(&mut touched_relation_sources, &spec.source);
                include_existing_entity_reference(&mut touched_relation_targets, &spec.target);
                ensure_relation_integrity_scope_budget(
                    budget,
                    scope_budget_snapshot(
                        &scopes,
                        &touched_entities,
                        &deleted_entities,
                        &empty_scanned_relations,
                        planned_edge_count,
                    ),
                )?;
            }
            crate::transactions::data::MutationIntent::Create(
                crate::transactions::data::CreateIntent::BulkRelations(spec),
            ) => {
                for (source, target) in &spec.endpoints {
                    scopes.entry(spec.kind_id).or_default().planned_edges.push(
                        PlannedRelationEdge {
                            source: source.clone(),
                            target: target.clone(),
                        },
                    );
                    planned_edge_count += 1;
                    include_existing_entity_reference(&mut touched_entities, source);
                    include_existing_entity_reference(&mut touched_entities, target);
                    include_existing_entity_reference(&mut touched_relation_sources, source);
                    include_existing_entity_reference(&mut touched_relation_targets, target);
                    ensure_relation_integrity_scope_budget(
                        budget,
                        scope_budget_snapshot(
                            &scopes,
                            &touched_entities,
                            &deleted_entities,
                            &empty_scanned_relations,
                            planned_edge_count,
                        ),
                    )?;
                }
            }
            crate::transactions::data::MutationIntent::Relation(
                crate::transactions::data::RelationMutationIntent::Delete(spec),
            ) => {
                deleted_relations.insert(spec.relation_id);
                if let Some((kind_id, source, target)) =
                    relation_scope_details_for_id(partitions, spec.relation_id)
                {
                    scopes.entry(kind_id).or_default().deleted_relation_count += 1;
                    touched_entities.insert(source);
                    touched_entities.insert(target);
                    touched_relation_sources.insert(source);
                    touched_relation_targets.insert(target);
                }
            }
            crate::transactions::data::MutationIntent::Entity(
                crate::transactions::data::EntityMutationIntent::Delete(spec),
            ) => {
                touched_entities.insert(spec.entity_id);
                touched_relation_sources.insert(spec.entity_id);
                touched_relation_targets.insert(spec.entity_id);
                deleted_entities.insert(spec.entity_id);
                ensure_relation_integrity_scope_budget(
                    budget,
                    scope_budget_snapshot(
                        &scopes,
                        &touched_entities,
                        &deleted_entities,
                        &empty_scanned_relations,
                        planned_edge_count,
                    ),
                )?;
            }
            crate::transactions::data::MutationIntent::Entity(
                crate::transactions::data::EntityMutationIntent::Replace(spec),
            ) => {
                touched_entities.insert(spec.entity_id);
                touched_relation_sources.insert(spec.entity_id);
                touched_relation_targets.insert(spec.entity_id);
                deleted_entities.insert(spec.entity_id);
                ensure_relation_integrity_scope_budget(
                    budget,
                    scope_budget_snapshot(
                        &scopes,
                        &touched_entities,
                        &deleted_entities,
                        &empty_scanned_relations,
                        planned_edge_count,
                    ),
                )?;
            }
            _ => {}
        }
    }

    let mut scanned_relations = BTreeSet::new();
    let relation_scan_entities = touched_relation_sources
        .union(&touched_relation_targets)
        .cloned()
        .collect::<BTreeSet<_>>();
    for entity_id in relation_scan_entities {
        let Some(partition) = partitions.get_partition(entity_id.partition_id) else {
            continue;
        };
        let slot = entity_id.slot_index();
        let outgoing = partition.adjacency.get(slot).map(|set| set.as_slice());
        scan_relation_integrity_ids(
            outgoing.into_iter().flatten().copied(),
            partitions,
            performance,
            budget,
            &touched_entities,
            &deleted_entities,
            &deleted_relations,
            planned_edge_count,
            &mut scanned_relations,
            &mut scopes,
        )?;
        let incoming = partition
            .reverse_adjacency
            .get(slot)
            .map(|set| set.as_slice());
        scan_relation_integrity_ids(
            incoming.into_iter().flatten().copied(),
            partitions,
            performance,
            budget,
            &touched_entities,
            &deleted_entities,
            &deleted_relations,
            planned_edge_count,
            &mut scanned_relations,
            &mut scopes,
        )?;
    }

    for scope in scopes.values_mut() {
        let planned_edges = std::mem::take(&mut scope.planned_edges);
        for edge in planned_edges {
            scope.increment_counts(edge.source.clone(), edge.target.clone());
            if let EntityReference::Existing(source) = &edge.source {
                if deleted_entities.contains(source) {
                    scope.deleted_entities.insert(*source);
                }
            }
            if let EntityReference::Existing(target) = &edge.target {
                if deleted_entities.contains(target) {
                    scope.deleted_entities.insert(*target);
                }
            }
            scope.planned_edges.push(edge);
        }
    }

    scopes.retain(|_, scope| !scope.is_empty());
    Ok((!scopes.is_empty()).then(|| PreparedRelationIntegrityScopes::new(scopes)))
}

fn include_existing_entity_reference(
    touched_entities: &mut BTreeSet<EntityId>,
    entity_reference: &EntityReference,
) {
    if let EntityReference::Existing(entity_id) = entity_reference {
        touched_entities.insert(*entity_id);
    }
}

fn scan_relation_integrity_ids(
    relation_ids: impl IntoIterator<Item = RelationId>,
    partitions: &dyn PartitionAccess,
    performance: &crate::performance::logic::PerformanceAccess<'_>,
    budget: &RelationIntegrityScopeBudget,
    touched_entities: &BTreeSet<EntityId>,
    deleted_entities: &BTreeSet<EntityId>,
    deleted_relations: &BTreeSet<RelationId>,
    planned_edge_count: usize,
    scanned_relations: &mut BTreeSet<RelationId>,
    scopes: &mut BTreeMap<KindId, PreparedRelationIntegrityScope>,
) -> Result<(), PreparedRelationIntegrityScopeBudgetExceeded> {
    for relation_id in relation_ids {
        if !scanned_relations.insert(relation_id) || deleted_relations.contains(&relation_id) {
            continue;
        }
        ensure_relation_integrity_scope_budget(
            budget,
            scope_budget_snapshot(
                scopes,
                touched_entities,
                deleted_entities,
                scanned_relations,
                planned_edge_count,
            ),
        )?;
        let Some((kind_id, source, target)) =
            relation_scope_details_for_id(partitions, relation_id)
        else {
            continue;
        };
        let scope = scopes.entry(kind_id).or_default();
        scope.increment_counts(
            EntityReference::Existing(source),
            EntityReference::Existing(target),
        );
        performance.count_relation_uniqueness_candidates(1);
        if deleted_entities.contains(&source) {
            scope.deleted_entities.insert(source);
        }
        if deleted_entities.contains(&target) {
            scope.deleted_entities.insert(target);
        }
    }
    Ok(())
}

fn scope_budget_snapshot(
    scopes: &BTreeMap<KindId, PreparedRelationIntegrityScope>,
    touched_entities: &BTreeSet<EntityId>,
    deleted_entities: &BTreeSet<EntityId>,
    scanned_relations: &BTreeSet<RelationId>,
    planned_edge_count: usize,
) -> RelationIntegrityScopeBudgetSnapshot {
    RelationIntegrityScopeBudgetSnapshot {
        relation_kind_count: scopes.len(),
        touched_entity_count: touched_entities.len(),
        deleted_entity_count: deleted_entities.len(),
        scanned_relation_count: scanned_relations.len(),
        planned_edge_count,
    }
}

fn ensure_relation_integrity_scope_budget(
    budget: &RelationIntegrityScopeBudget,
    snapshot: RelationIntegrityScopeBudgetSnapshot,
) -> Result<(), PreparedRelationIntegrityScopeBudgetExceeded> {
    let checks = [
        (
            "max_relation_kinds",
            budget.max_relation_kinds,
            snapshot.relation_kind_count,
        ),
        (
            "max_touched_entities",
            budget.max_touched_entities,
            snapshot.touched_entity_count,
        ),
        (
            "max_deleted_entities",
            budget.max_deleted_entities,
            snapshot.deleted_entity_count,
        ),
        (
            "max_scanned_relations",
            budget.max_scanned_relations,
            snapshot.scanned_relation_count,
        ),
        (
            "max_planned_edges",
            budget.max_planned_edges,
            snapshot.planned_edge_count,
        ),
    ];
    for (limit_name, limit, observed) in checks {
        if observed > limit {
            return Err(PreparedRelationIntegrityScopeBudgetExceeded {
                limit_name,
                limit,
                observed,
                snapshot,
            });
        }
    }
    Ok(())
}

fn relation_scope_details_for_id(
    partitions: &dyn PartitionAccess,
    relation_id: RelationId,
) -> Option<(KindId, EntityId, EntityId)> {
    let relation_partition = partitions.get_partition(relation_id.partition_id)?;
    let slot = relation_partition.relation_arena.get(&relation_id)?;
    let kind_id = slot.kind_id()?;
    let endpoints = slot.extra().endpoints.as_ref()?;
    Some((kind_id, endpoints.source, endpoints.target))
}
