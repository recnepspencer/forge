use std::collections::BTreeSet;

use worth_foundational::facade::{AspectFieldLocator, AspectValue};

use crate::capabilities::AspectPlanSource;
use crate::identity::data::{EntityId, KindId, RelationId};
use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::RecordLifecycleState;
use crate::visibility::materialization::read_records::{
    ProjectionAspectRequirement, ProjectionAspectScope, VisibilityProjectionView,
};

use super::evidence::RelationalAuthorizationPathDependencies;
use super::identity::observation_evidence_identity;
use super::{
    RelationalAuthorizationAdjacencyDependency, RelationalAuthorizationDecision,
    RelationalAuthorizationObservationCounters, RelationalAuthorizationObservationDenial,
    RelationalAuthorizationObservationEvidence, RelationalAuthorizationObservationPlan,
    RelationalAuthorizationPathEffect, RelationalAuthorizationPathObservation,
    RelationalAuthorizationPathPlan, RelationalAuthorizationTraversalDirection,
};

impl RelationalRuntime {
    pub fn observe_authorization(
        &self,
        plan: RelationalAuthorizationObservationPlan,
    ) -> Result<RelationalAuthorizationObservationEvidence, RelationalAuthorizationObservationDenial>
    {
        let evaluation = self.evaluate_authorization_plan(&plan)?;
        let observation_identity =
            observation_evidence_identity(plan.identity(), evaluation.decision, &evaluation.paths);
        Ok(RelationalAuthorizationObservationEvidence::mint(
            plan,
            observation_identity,
            evaluation.decision,
            evaluation.paths,
            evaluation.counters,
        ))
    }

    pub(super) fn evaluate_authorization_plan(
        &self,
        plan: &RelationalAuthorizationObservationPlan,
    ) -> Result<RelationalAuthorizationEvaluation, RelationalAuthorizationObservationDenial> {
        if plan.snapshot().runtime_instance_id != self.runtime_instance_id() {
            return Err(RelationalAuthorizationObservationDenial::ForeignRuntime {
                expected_runtime_instance_id: self.runtime_instance_id(),
                actual_runtime_instance_id: plan.snapshot().runtime_instance_id,
            });
        }
        let mut counters = RelationalAuthorizationObservationCounters::default();
        let view = self
            .read_truth()
            .project_snapshot(plan.snapshot())
            .ok_or(RelationalAuthorizationObservationDenial::SnapshotUnavailable)?;
        if !entity_is_live_kind(
            &view,
            plan.principal(),
            plan.principal_kind(),
            &mut counters,
        ) {
            return Err(RelationalAuthorizationObservationDenial::PrincipalUnavailableOrWrongKind);
        }
        if plan.scope() != plan.principal()
            && !entity_is_live_kind(&view, plan.scope(), plan.scope_kind(), &mut counters)
        {
            return Err(RelationalAuthorizationObservationDenial::ScopeUnavailableOrWrongKind);
        }
        let mut observations = Vec::with_capacity(plan.paths().len());
        for path in plan.paths() {
            observations.push(evaluate_path(self, &view, &plan, path, &mut counters));
        }
        let denied = observations
            .iter()
            .any(|path| path.effect() == RelationalAuthorizationPathEffect::Deny && path.matched());
        let allowed = observations.iter().any(|path| {
            path.effect() == RelationalAuthorizationPathEffect::Allow && path.matched()
        });
        let decision = if allowed && !denied {
            RelationalAuthorizationDecision::Allowed
        } else {
            RelationalAuthorizationDecision::Denied
        };
        Ok(RelationalAuthorizationEvaluation {
            decision,
            paths: observations,
            counters,
        })
    }
}

pub(super) struct RelationalAuthorizationEvaluation {
    pub(super) decision: RelationalAuthorizationDecision,
    pub(super) paths: Vec<RelationalAuthorizationPathObservation>,
    pub(super) counters: RelationalAuthorizationObservationCounters,
}

fn evaluate_path(
    runtime: &RelationalRuntime,
    view: &VisibilityProjectionView<'_>,
    plan: &RelationalAuthorizationObservationPlan,
    path: &RelationalAuthorizationPathPlan,
    counters: &mut RelationalAuthorizationObservationCounters,
) -> RelationalAuthorizationPathObservation {
    counters.paths_evaluated += 1;
    let mut frontier = BTreeSet::from([plan.principal()]);
    let mut touched_entities = BTreeSet::from([plan.principal()]);
    let mut touched_relations = BTreeSet::new();
    let mut touched_adjacencies = BTreeSet::new();
    let mut touched_fields = BTreeSet::new();
    counters.maximum_frontier_width = counters.maximum_frontier_width.max(frontier.len());
    apply_predicates(
        runtime,
        view,
        path,
        0,
        &mut frontier,
        &mut touched_fields,
        counters,
    );
    apply_entity_anchors(path, 0, &mut frontier);
    apply_related_entities(
        runtime,
        view,
        plan,
        path,
        0,
        &mut frontier,
        &mut touched_entities,
        &mut touched_relations,
        &mut touched_adjacencies,
        counters,
    );
    for (index, traversal) in path.traversals().iter().enumerate() {
        let mut next = BTreeSet::new();
        for entity in frontier.iter().copied() {
            let relation_ids = relation_ids_for_step(
                runtime,
                view,
                plan,
                entity,
                traversal,
                &mut touched_relations,
                &mut touched_adjacencies,
                counters,
            );
            for relation_id in relation_ids {
                counters.relation_records_inspected += 1;
                let Some(candidate) =
                    traverse_relation(view, relation_id, entity, traversal, &mut touched_relations)
                else {
                    continue;
                };
                if entity_is_live_kind(view, candidate.0, candidate.1, counters) {
                    touched_entities.insert(candidate.0);
                    next.insert(candidate.0);
                }
            }
        }
        frontier = next;
        apply_predicates(
            runtime,
            view,
            path,
            index + 1,
            &mut frontier,
            &mut touched_fields,
            counters,
        );
        apply_entity_anchors(path, index + 1, &mut frontier);
        apply_related_entities(
            runtime,
            view,
            plan,
            path,
            index + 1,
            &mut frontier,
            &mut touched_entities,
            &mut touched_relations,
            &mut touched_adjacencies,
            counters,
        );
        counters.maximum_frontier_width = counters.maximum_frontier_width.max(frontier.len());
        if frontier.is_empty() {
            break;
        }
    }
    RelationalAuthorizationPathObservation::new(
        path.effect(),
        frontier.contains(&plan.scope()),
        RelationalAuthorizationPathDependencies {
            entities: touched_entities.into_iter().collect(),
            relations: touched_relations.into_iter().collect(),
            adjacency_lists: touched_adjacencies.into_iter().collect(),
            fields: touched_fields.into_iter().collect(),
        },
        true,
    )
}

fn relation_ids_for_step(
    runtime: &RelationalRuntime,
    view: &VisibilityProjectionView<'_>,
    plan: &RelationalAuthorizationObservationPlan,
    entity: EntityId,
    traversal: &super::RelationalAuthorizationTraversal,
    touched_relations: &mut BTreeSet<RelationId>,
    touched_adjacencies: &mut BTreeSet<RelationalAuthorizationAdjacencyDependency>,
    counters: &mut RelationalAuthorizationObservationCounters,
) -> Vec<RelationId> {
    touched_adjacencies.insert(RelationalAuthorizationAdjacencyDependency::new(
        entity,
        traversal.relation_kind(),
        traversal.direction(),
    ));
    if plan.snapshot().version_id == runtime.current_version_id() {
        counters.adjacency_lists_read += 1;
        let relation_ids = match traversal.direction() {
            RelationalAuthorizationTraversalDirection::Forward => {
                crate::storage::partition::adjacency_queries::outgoing_relations_for_entity_kind(
                    runtime,
                    entity,
                    traversal.relation_kind(),
                    plan.snapshot().version_id,
                )
            }
            RelationalAuthorizationTraversalDirection::Reverse => {
                crate::storage::partition::adjacency_queries::incoming_relations_for_entity_kind(
                    runtime,
                    entity,
                    traversal.relation_kind(),
                    plan.snapshot().version_id,
                )
            }
        };
        counters.adjacency_edges_inspected += relation_ids.len();
        return relation_ids;
    }
    counters.reconstructive_graph_scans += 1;
    let records = view.all_authoritative_relation_records();
    counters.reconstructive_relation_records_scanned += records.len();
    touched_relations.extend(records.iter().map(|record| record.relation_id));
    records
        .into_iter()
        .filter(|record| {
            record.kind.kind_id == traversal.relation_kind()
                && match traversal.direction() {
                    RelationalAuthorizationTraversalDirection::Forward => record.source == entity,
                    RelationalAuthorizationTraversalDirection::Reverse => record.target == entity,
                }
        })
        .map(|record| record.relation_id)
        .collect()
}

fn traverse_relation(
    view: &VisibilityProjectionView<'_>,
    relation_id: RelationId,
    current: EntityId,
    traversal: &super::RelationalAuthorizationTraversal,
    touched: &mut BTreeSet<RelationId>,
) -> Option<(EntityId, KindId)> {
    view.relation_record_with_projection_scope(
        relation_id,
        ProjectionAspectScope::empty(),
        |record| {
            touched.insert(record.relation_id());
            if record.lifecycle() != RecordLifecycleState::Live
                || record.kind_id() != traversal.relation_kind()
            {
                return None;
            }
            match traversal.direction() {
                RelationalAuthorizationTraversalDirection::Forward
                    if record.source() == current =>
                {
                    Some((record.target(), traversal.to_kind()))
                }
                RelationalAuthorizationTraversalDirection::Reverse
                    if record.target() == current =>
                {
                    Some((record.source(), traversal.from_kind()))
                }
                _ => None,
            }
        },
    )
}

fn apply_predicates(
    runtime: &RelationalRuntime,
    view: &VisibilityProjectionView<'_>,
    path: &RelationalAuthorizationPathPlan,
    ordinal: usize,
    frontier: &mut BTreeSet<EntityId>,
    touched: &mut BTreeSet<(EntityId, AspectFieldLocator)>,
    counters: &mut RelationalAuthorizationObservationCounters,
) {
    for predicate in path
        .predicates()
        .iter()
        .filter(|predicate| predicate.traversal_ordinal() == ordinal)
    {
        frontier.retain(|entity| {
            counters.predicate_fields_inspected += 1;
            counters.entity_records_inspected += 1;
            touched.insert((*entity, predicate.field().clone()));
            observed_field(
                runtime,
                view,
                *entity,
                predicate.entity_kind(),
                predicate.field(),
            )
            .is_some_and(|value| predicate.matches(&value))
        });
    }
}

fn apply_entity_anchors(
    path: &RelationalAuthorizationPathPlan,
    ordinal: usize,
    frontier: &mut BTreeSet<EntityId>,
) {
    for anchor in path
        .entity_anchors()
        .iter()
        .filter(|anchor| anchor.traversal_ordinal() == ordinal)
    {
        frontier.retain(|entity| *entity == anchor.entity());
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_related_entities(
    runtime: &RelationalRuntime,
    view: &VisibilityProjectionView<'_>,
    plan: &RelationalAuthorizationObservationPlan,
    path: &RelationalAuthorizationPathPlan,
    ordinal: usize,
    frontier: &mut BTreeSet<EntityId>,
    touched_entities: &mut BTreeSet<EntityId>,
    touched_relations: &mut BTreeSet<RelationId>,
    touched_adjacencies: &mut BTreeSet<RelationalAuthorizationAdjacencyDependency>,
    counters: &mut RelationalAuthorizationObservationCounters,
) {
    for constraint in path
        .related_entities()
        .iter()
        .filter(|constraint| constraint.traversal_ordinal() == ordinal)
    {
        frontier.retain(|source| {
            let relation_ids = relation_ids_for_step(
                runtime,
                view,
                plan,
                *source,
                constraint.traversal(),
                touched_relations,
                touched_adjacencies,
                counters,
            );
            relation_ids.into_iter().any(|relation_id| {
                counters.relation_records_inspected += 1;
                let Some((candidate, kind)) = traverse_relation(
                    view,
                    relation_id,
                    *source,
                    constraint.traversal(),
                    touched_relations,
                ) else {
                    return false;
                };
                if candidate != constraint.entity()
                    || !entity_is_live_kind(view, candidate, kind, counters)
                {
                    return false;
                }
                touched_entities.insert(candidate);
                true
            })
        });
    }
}

fn observed_field(
    runtime: &RelationalRuntime,
    view: &VisibilityProjectionView<'_>,
    entity: EntityId,
    kind: KindId,
    locator: &AspectFieldLocator,
) -> Option<AspectValue> {
    let field = locator.field_path().fields().first()?.clone();
    let plan = runtime.entity_aspect_plan(kind)?;
    let binding = plan
        .executable_bindings
        .iter()
        .find(|binding| binding.aspect_key() == locator.aspect().aspect_key())?;
    let scalar_aspect = binding.targets_entity_scalar_field(&field);
    let requirement = if scalar_aspect {
        ProjectionAspectRequirement::whole_aspect(locator.aspect().aspect_key().clone())
    } else if binding.targets_entity_struct_field(&field) {
        ProjectionAspectRequirement::fields(locator.aspect().aspect_key().clone(), [field.clone()])
    } else {
        return None;
    };
    let scope = ProjectionAspectScope::from_requirements([requirement]);
    view.entity_record_with_projection_scope(entity, scope, |record| {
        (record.kind_id() == kind && record.lifecycle() == RecordLifecycleState::Live)
            .then(|| {
                if scalar_aspect {
                    record.aspect_value(locator.aspect().aspect_key()).cloned()
                } else {
                    record
                        .aspect_field_value(locator.aspect().aspect_key(), &field)
                        .cloned()
                }
            })
            .flatten()
    })
}

fn entity_is_live_kind(
    view: &VisibilityProjectionView<'_>,
    entity: EntityId,
    kind: KindId,
    counters: &mut RelationalAuthorizationObservationCounters,
) -> bool {
    counters.entity_records_inspected += 1;
    view.entity_record_with_projection_scope(entity, ProjectionAspectScope::empty(), |record| {
        Some(record.kind_id() == kind && record.lifecycle() == RecordLifecycleState::Live)
    })
    .unwrap_or(false)
}
