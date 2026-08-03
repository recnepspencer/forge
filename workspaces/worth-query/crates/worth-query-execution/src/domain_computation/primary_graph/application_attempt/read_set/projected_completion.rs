use worth_query_installation::facade::{
    ApplicationOperationDecisionReadTarget, ApplicationRelationRef, OperationReads,
};
use worth_relational::facade::runtime::ProjectionAspectScope;
use worth_relational::facade::storage::RecordLifecycleState;

use super::super::fact::{
    observe_adjacency, WorthQueryApplicationFactKey, WorthQueryApplicationObservedFact,
};
use super::super::read_phase::WorthQueryProjectedApplicationMutation;
use super::{denial, WorthQueryApplicationReadAttempt, WorthQueryCompleteApplicationReadSet};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationAttemptDenial, WorthQueryApplicationAttemptDenialKind,
    WorthQueryApplicationEntityIdentity,
};

impl<Schema, Operation, Input, Scope>
    WorthQueryApplicationReadAttempt<
        Schema,
        Operation,
        Input,
        Scope,
        WorthQueryProjectedApplicationMutation,
    >
{
    /// Re-observes exactly the decision dependencies carried by the sealed
    /// operation projection. No caller can add, remove, or replace a fact key.
    pub fn complete_projected_dependencies(
        mut self,
    ) -> Result<
        WorthQueryCompleteApplicationReadSet<
            Schema,
            Operation,
            Input,
            Scope,
            WorthQueryProjectedApplicationMutation,
        >,
        WorthQueryApplicationAttemptDenial,
    > {
        let expected = self.expected_facts.clone().ok_or_else(|| {
            denial(
                WorthQueryApplicationAttemptDenialKind::ProjectionAdmissionMismatch,
                self.admission.operation(),
            )
        })?;
        for key in expected {
            let fact = self.observe_projected_fact(&key)?;
            self.facts.insert(key, fact);
        }
        self.complete()
    }

    fn observe_projected_fact(
        &self,
        key: &WorthQueryApplicationFactKey,
    ) -> Result<WorthQueryApplicationObservedFact, WorthQueryApplicationAttemptDenial> {
        if !key_entities_are_in_scope(key, &self.read_scope) {
            return Err(denial(
                WorthQueryApplicationAttemptDenialKind::OutsideRealizedReadScope,
                self.admission.operation(),
            ));
        }
        let target = self.projected_target(key)?;
        match key {
            WorthQueryApplicationFactKey::Entity { entity, entity_id } => {
                let kind = self.layout.entity_kind(entity).ok_or_else(|| {
                    denial(
                        WorthQueryApplicationAttemptDenialKind::UndeclaredDecisionRead,
                        entity,
                    )
                })?;
                let exists = self.lease.handle().with_runtime(|runtime| {
                    runtime
                        .read_truth()
                        .project_snapshot(self.lease.snapshot())
                        .and_then(|view| {
                            view.entity_record_with_projection_scope(
                                *entity_id,
                                ProjectionAspectScope::empty(),
                                |record| {
                                    Some(
                                        record.kind_id() == kind
                                            && record.lifecycle() == RecordLifecycleState::Live,
                                    )
                                },
                            )
                        })
                        .unwrap_or(false)
                });
                if !exists {
                    return Err(denial(
                        WorthQueryApplicationAttemptDenialKind::MissingAuthoritativeFact,
                        entity,
                    ));
                }
                Ok(WorthQueryApplicationObservedFact::Entity {
                    target,
                    entity_id: *entity_id,
                    kind,
                })
            }
            WorthQueryApplicationFactKey::Field {
                entity,
                entity_id,
                locator,
            } => {
                let kind = self.layout.entity_kind(entity).ok_or_else(|| {
                    denial(
                        WorthQueryApplicationAttemptDenialKind::UndeclaredDecisionRead,
                        entity,
                    )
                })?;
                let value = self
                    .lease
                    .handle()
                    .with_runtime(|runtime| {
                        super::super::observation::observe_field_value(
                            runtime,
                            self.lease.snapshot(),
                            *entity_id,
                            kind,
                            locator,
                        )
                    })
                    .ok_or_else(|| {
                        denial(
                            WorthQueryApplicationAttemptDenialKind::MissingAuthoritativeFact,
                            entity,
                        )
                    })?;
                Ok(WorthQueryApplicationObservedFact::Field {
                    target,
                    entity_id: *entity_id,
                    kind,
                    locator: locator.clone(),
                    value,
                })
            }
            WorthQueryApplicationFactKey::Relation { relation, from, to } => {
                let layout = self.layout.relation(relation).ok_or_else(|| {
                    denial(
                        WorthQueryApplicationAttemptDenialKind::UndeclaredDecisionRead,
                        relation,
                    )
                })?;
                let matching_relations = self.lease.handle().with_runtime(|runtime| {
                    super::super::observation::exact_relations(
                        runtime,
                        self.lease.snapshot(),
                        layout.kind,
                        *from,
                        *to,
                    )
                })?;
                Ok(WorthQueryApplicationObservedFact::Relation {
                    target,
                    relation_kind: layout.kind,
                    from: *from,
                    to: *to,
                    matching_relations,
                })
            }
            WorthQueryApplicationFactKey::Adjacency {
                relation,
                anchor,
                direction,
                maximum_work_units,
            } => {
                let layout = self.layout.relation(relation).ok_or_else(|| {
                    denial(
                        WorthQueryApplicationAttemptDenialKind::UndeclaredDecisionRead,
                        relation,
                    )
                })?;
                let relations = self.lease.handle().with_runtime(|runtime| {
                    observe_adjacency(
                        runtime,
                        self.lease.snapshot(),
                        layout.kind,
                        *anchor,
                        *direction,
                        *maximum_work_units,
                    )
                });
                let relations = relations.ok_or_else(|| {
                    denial(
                        WorthQueryApplicationAttemptDenialKind::DecisionFactBudgetExceeded,
                        relation,
                    )
                })?;
                Ok(WorthQueryApplicationObservedFact::Adjacency {
                    target,
                    relation_kind: layout.kind,
                    anchor: *anchor,
                    direction: *direction,
                    maximum_work_units: *maximum_work_units,
                    relations,
                })
            }
        }
    }

    fn projected_target(
        &self,
        key: &WorthQueryApplicationFactKey,
    ) -> Result<ApplicationOperationDecisionReadTarget, WorthQueryApplicationAttemptDenial> {
        self.admission
            .allowed_graph_contract()
            .decision_reads()
            .iter()
            .find(|target| target_matches_key(target, key))
            .cloned()
            .ok_or_else(|| {
                denial(
                    WorthQueryApplicationAttemptDenialKind::UndeclaredDecisionRead,
                    self.admission.operation(),
                )
            })
    }
}

impl<Schema, Operation, Input, Scope>
    WorthQueryCompleteApplicationReadSet<
        Schema,
        Operation,
        Input,
        Scope,
        WorthQueryProjectedApplicationMutation,
    >
{
    pub fn projected_relation<Relation, From, To>(
        &self,
        relation: ApplicationRelationRef<Schema, Relation, From, To>,
        from: &WorthQueryApplicationEntityIdentity<Schema, From>,
        to: &WorthQueryApplicationEntityIdentity<Schema, To>,
    ) -> Result<
        super::WorthQueryObservedApplicationRelation<Schema, Relation, From, To>,
        WorthQueryApplicationAttemptDenial,
    >
    where
        Relation: OperationReads<Operation>,
    {
        let layout = self.lease.layout.relation(relation.name()).ok_or_else(|| {
            denial(
                WorthQueryApplicationAttemptDenialKind::UndeclaredDecisionRead,
                relation.name(),
            )
        })?;
        if from.runtime_authority() != self.admission.runtime_authority()
            || to.runtime_authority() != self.admission.runtime_authority()
            || from.binding_identity() != self.admission.binding_identity()
            || to.binding_identity() != self.admission.binding_identity()
        {
            return Err(denial(
                WorthQueryApplicationAttemptDenialKind::ForeignEffectTarget,
                relation.name(),
            ));
        }
        let matching_relations = self
            .facts
            .iter()
            .find_map(|fact| match fact {
                WorthQueryApplicationObservedFact::Relation {
                    relation_kind,
                    from: observed_from,
                    to: observed_to,
                    matching_relations,
                    ..
                } if *relation_kind == layout.kind
                    && *observed_from == from.entity_id()
                    && *observed_to == to.entity_id() =>
                {
                    Some(matching_relations.clone())
                }
                WorthQueryApplicationObservedFact::Adjacency {
                    relation_kind,
                    relations,
                    ..
                } if *relation_kind == layout.kind => {
                    let matches = relations
                        .iter()
                        .filter(|observed| {
                            observed.from == from.entity_id() && observed.to == to.entity_id()
                        })
                        .map(|observed| observed.relation_id)
                        .collect::<Vec<_>>();
                    (!matches.is_empty()).then_some(matches)
                }
                _ => None,
            })
            .ok_or_else(|| {
                denial(
                    WorthQueryApplicationAttemptDenialKind::MissingAuthoritativeFact,
                    relation.name(),
                )
            })?;
        Ok(super::WorthQueryObservedApplicationRelation {
            count: matching_relations.len(),
            matching_relations,
            _marker: std::marker::PhantomData,
        })
    }
}

fn key_entities_are_in_scope(
    key: &WorthQueryApplicationFactKey,
    scope: &super::super::read_scope::WorthQueryApplicationReadScope,
) -> bool {
    match key {
        WorthQueryApplicationFactKey::Entity { entity_id, .. }
        | WorthQueryApplicationFactKey::Field { entity_id, .. } => scope.admits(*entity_id),
        WorthQueryApplicationFactKey::Relation { from, to, .. } => {
            scope.admits(*from) && scope.admits(*to)
        }
        WorthQueryApplicationFactKey::Adjacency { anchor, .. } => scope.admits(*anchor),
    }
}

fn target_matches_key(
    target: &ApplicationOperationDecisionReadTarget,
    key: &WorthQueryApplicationFactKey,
) -> bool {
    match (target, key) {
        (
            ApplicationOperationDecisionReadTarget::Entity { entity: declared },
            WorthQueryApplicationFactKey::Entity { entity, .. },
        ) => declared == entity,
        (
            ApplicationOperationDecisionReadTarget::Field {
                entity: declared_entity,
                aspect: declared_aspect,
                field: declared_field,
            },
            WorthQueryApplicationFactKey::Field {
                entity, locator, ..
            },
        ) => {
            declared_entity == entity
                && declared_aspect == locator.aspect().aspect_key().as_str()
                && locator
                    .field_path()
                    .fields()
                    .first()
                    .is_some_and(|field| declared_field == field.as_str())
        }
        (
            ApplicationOperationDecisionReadTarget::Relation {
                relation: declared, ..
            },
            WorthQueryApplicationFactKey::Relation { relation, .. }
            | WorthQueryApplicationFactKey::Adjacency { relation, .. },
        ) => declared == relation,
        _ => false,
    }
}
