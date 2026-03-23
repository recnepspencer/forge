use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::{
    InvariantCostClass, InvariantExecutionPoint, InvariantGroupSet, InvariantPlanContract,
    InvariantRegistration, InvariantViolation, InvariantViolationFields,
};
use crate::{
    config::data::RelationIntegrityScopeBudget,
    identity::data::{EntityId, KindId, RelationId},
    storage::overlay::PartitionAccess,
    validation::data::InvariantRule,
};

use super::observation::InvariantObservation;
use super::policy::{cost_allowed, RelationalInvariantRuntime};
use super::profile::InvariantRequestProfile;

pub(crate) struct InvariantExecutionRequest<'runtime> {
    observation: InvariantObservation<'runtime>,
    version_id: crate::identity::data::VersionId,
    current_version_id: crate::identity::data::VersionId,
    checkpoint: InvariantExecutionPoint,
    runtime_policy: RelationalInvariantRuntime,
    consumed_groups: InvariantGroupSet,
    applicable_groups: InvariantGroupSet,
    plan_contract: Option<InvariantPlanContract>,
    merged_plan: Option<&'runtime MergedCommitPlan>,
    relation_integrity_scopes: Option<PreparedRelationIntegrityScopes>,
    preparation_violation: Option<InvariantViolation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct PlannedRelationEdge {
    pub(crate) source: EntityId,
    pub(crate) target: EntityId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct PreparedRelationPairKey {
    pub(crate) source: EntityId,
    pub(crate) target: EntityId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct PreparedRelationEndpointKey {
    pub(crate) entity_id: EntityId,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct PreparedRelationIntegrityScope {
    pub(crate) planned_edges: Vec<PlannedRelationEdge>,
    pub(crate) source_counts: BTreeMap<PreparedRelationEndpointKey, usize>,
    pub(crate) target_counts: BTreeMap<PreparedRelationEndpointKey, usize>,
    pub(crate) directed_pair_counts: BTreeMap<PreparedRelationPairKey, usize>,
    pub(crate) normalized_pair_counts: BTreeMap<PreparedRelationPairKey, usize>,
    pub(crate) deleted_entities: BTreeSet<EntityId>,
}

#[derive(Debug, Clone)]
pub(crate) struct PreparedRelationIntegrityScopes(
    Arc<BTreeMap<KindId, PreparedRelationIntegrityScope>>,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RelationIntegrityScopeBudgetSnapshot {
    relation_kind_count: usize,
    touched_entity_count: usize,
    deleted_entity_count: usize,
    scanned_relation_count: usize,
    planned_edge_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PreparedRelationIntegrityScopeBudgetExceeded {
    limit_name: &'static str,
    limit: usize,
    observed: usize,
    snapshot: RelationIntegrityScopeBudgetSnapshot,
}

impl PreparedRelationIntegrityScopeBudgetExceeded {
    fn into_violation(
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

impl PreparedRelationIntegrityScopes {
    pub(crate) fn new(scopes: BTreeMap<KindId, PreparedRelationIntegrityScope>) -> Self {
        Self(Arc::new(scopes))
    }

    pub(crate) fn scope_for(
        &self,
        relation_kind_id: KindId,
    ) -> Option<&PreparedRelationIntegrityScope> {
        self.0.get(&relation_kind_id)
    }

    pub(crate) fn contains_relation_kind(&self, relation_kind_id: KindId) -> bool {
        self.0.contains_key(&relation_kind_id)
    }
}

impl PreparedRelationIntegrityScope {
    pub(crate) fn is_empty(&self) -> bool {
        self.planned_edges.is_empty()
            && self.source_counts.is_empty()
            && self.target_counts.is_empty()
            && self.directed_pair_counts.is_empty()
            && self.deleted_entities.is_empty()
    }

    fn increment_counts(&mut self, source: EntityId, target: EntityId) {
        *self
            .source_counts
            .entry(PreparedRelationEndpointKey { entity_id: source })
            .or_insert(0) += 1;
        *self
            .target_counts
            .entry(PreparedRelationEndpointKey { entity_id: target })
            .or_insert(0) += 1;
        *self
            .directed_pair_counts
            .entry(PreparedRelationPairKey { source, target })
            .or_insert(0) += 1;
        let (left, right) = if target < source {
            (target, source)
        } else {
            (source, target)
        };
        *self
            .normalized_pair_counts
            .entry(PreparedRelationPairKey {
                source: left,
                target: right,
            })
            .or_insert(0) += 1;
    }
}

impl<'runtime> InvariantExecutionRequest<'runtime> {
    pub(crate) fn from_profile_with_contract(
        profile: InvariantRequestProfile,
        runtime: &'runtime crate::logic::runtime::RelationalRuntime,
        observation: InvariantObservation<'runtime>,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'runtime MergedCommitPlan>,
        plan_contract: Option<InvariantPlanContract>,
    ) -> Self {
        debug_assert!(
            profile.supports_observation(observation.kind()),
            "invariant profile {:?} does not support {:?} observation",
            profile,
            observation.kind(),
        );
        let runtime_policy = RelationalInvariantRuntime::resolve(
            profile,
            super::policy::derive_invariant_context(runtime),
        );
        let consumed_groups = profile.consumed_groups();
        let applicable_groups = plan_contract
            .map(|contract| {
                contract
                    .may_invalidate_groups()
                    .intersection(consumed_groups)
            })
            .unwrap_or(consumed_groups);
        let (relation_integrity_scopes, preparation_violation) =
            match prepare_relation_integrity_scopes(
            merged_plan,
            observation.partition_access(),
            runtime.performance_access(),
            &runtime.config.execution.relation_integrity_scope_budget,
        ) {
                Ok(scopes) => (scopes, None),
                Err(exceeded) => (None, Some(exceeded.into_violation(profile.execution_point()))),
            };
        Self {
            observation,
            version_id,
            current_version_id: runtime.current_version_id(),
            checkpoint: profile.execution_point(),
            runtime_policy,
            consumed_groups,
            applicable_groups,
            plan_contract,
            merged_plan,
            relation_integrity_scopes,
            preparation_violation,
        }
    }

    pub(crate) fn observation(&self) -> &InvariantObservation<'runtime> {
        &self.observation
    }

    pub(crate) fn version_id(&self) -> crate::identity::data::VersionId {
        self.version_id
    }

    pub(crate) fn execution_point(&self) -> InvariantExecutionPoint {
        self.checkpoint
    }

    pub(crate) fn current_version_id(&self) -> crate::identity::data::VersionId {
        self.current_version_id
    }

    pub(crate) fn merged_plan(&self) -> Option<&'runtime MergedCommitPlan> {
        self.merged_plan
    }

    pub(crate) fn consumed_groups(&self) -> InvariantGroupSet {
        self.consumed_groups
    }

    pub(crate) fn applicable_groups(&self) -> InvariantGroupSet {
        self.applicable_groups
    }

    pub(crate) fn plan_contract(&self) -> Option<InvariantPlanContract> {
        self.plan_contract
    }

    pub(crate) fn max_cost(&self) -> InvariantCostClass {
        self.runtime_policy.max_cost_at(self.checkpoint)
    }

    pub(crate) fn relation_integrity_scopes(&self) -> Option<&PreparedRelationIntegrityScopes> {
        self.relation_integrity_scopes.as_ref()
    }

    pub(crate) fn preparation_violation(&self) -> Option<&InvariantViolation> {
        self.preparation_violation.as_ref()
    }

    pub(crate) fn should_execute_anything(&self) -> bool {
        self.merged_plan.is_none() || !self.applicable_groups.is_empty()
    }

    pub(crate) fn includes_registration(&self, registration: &InvariantRegistration) -> bool {
        let rule_groups = registration.rule.groups();
        self.runtime_policy.should_run(rule_groups, self.checkpoint)
            && (self.applicable_groups.is_empty() || self.applicable_groups.intersects(rule_groups))
            && self
                .plan_contract
                .is_none_or(|contract| contract.applies_to_rule(&registration.rule))
            && self.rule_matches_plan_scope(&registration.rule)
            && cost_allowed(
                self.runtime_policy.max_cost_at(self.checkpoint),
                registration.cost(),
            )
    }

    fn rule_matches_plan_scope(&self, rule: &InvariantRule) -> bool {
        let Some(_merged_plan) = self.merged_plan else {
            return true;
        };
        let Some(relation_kind_id) = relation_kind_scope(rule) else {
            return true;
        };
        self.relation_integrity_scopes
            .as_ref()
            .is_some_and(|scopes| scopes.contains_relation_kind(relation_kind_id))
    }

    #[cfg(test)]
    pub(crate) fn with_applicable_groups(mut self, applicable_groups: InvariantGroupSet) -> Self {
        self.applicable_groups = applicable_groups;
        self
    }
}

fn relation_kind_scope(rule: &InvariantRule) -> Option<KindId> {
    match rule {
        InvariantRule::EndpointKindContract(contract) => Some(contract.relation_kind_id),
        InvariantRule::CardinalityMaximumContract(contract) => Some(contract.relation_kind_id),
        InvariantRule::CardinalityMinimumContract(contract) => Some(contract.relation_kind_id),
        InvariantRule::UniquenessContract(contract) => Some(contract.relation_kind_id),
        InvariantRule::SymmetryContract(contract) => Some(contract.relation_kind_id),
        InvariantRule::EndpointDeletionIntegrityContract(contract) => Some(contract.relation_kind_id),
        _ => None,
    }
}

fn prepare_relation_integrity_scopes(
    merged_plan: Option<&MergedCommitPlan>,
    partitions: &dyn PartitionAccess,
    performance: crate::performance::logic::PerformanceAccess<'_>,
    budget: &RelationIntegrityScopeBudget,
) -> Result<Option<PreparedRelationIntegrityScopes>, PreparedRelationIntegrityScopeBudgetExceeded> {
    let Some(merged_plan) = merged_plan else {
        return Ok(None);
    };
    let mut scopes = BTreeMap::<KindId, PreparedRelationIntegrityScope>::new();
    let mut touched_entities = BTreeSet::new();
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
                        source: spec.source,
                        target: spec.target,
                    });
                planned_edge_count += 1;
                touched_entities.insert(spec.source);
                touched_entities.insert(spec.target);
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
                    scopes.entry(spec.kind_id).or_default().planned_edges.push(PlannedRelationEdge {
                        source: *source,
                        target: *target,
                    });
                    planned_edge_count += 1;
                    touched_entities.insert(*source);
                    touched_entities.insert(*target);
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
                if let Some(kind_id) = relation_kind_for_id(partitions, spec.relation_id) {
                    scopes.entry(kind_id).or_default();
                }
            }
            crate::transactions::data::MutationIntent::Entity(
                crate::transactions::data::EntityMutationIntent::Delete(spec),
            ) => {
                touched_entities.insert(spec.entity_id);
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
    for &entity_id in &touched_entities {
        let Some(partition) = partitions.get_partition(entity_id.partition_id) else {
            continue;
        };
        let slot = entity_id.local_slot.0 as usize;
        let outgoing = partition
            .adjacency
            .get(slot)
            .map(|set| set.as_slice())
            .into_iter()
            .flatten();
        let incoming = partition
            .reverse_adjacency
            .get(slot)
            .map(|set| set.as_slice())
            .into_iter()
            .flatten();
        for relation_id in outgoing.chain(incoming).copied() {
            if !scanned_relations.insert(relation_id) || deleted_relations.contains(&relation_id) {
                continue;
            }
            ensure_relation_integrity_scope_budget(
                budget,
                scope_budget_snapshot(
                    &scopes,
                    &touched_entities,
                    &deleted_entities,
                    &scanned_relations,
                    planned_edge_count,
                ),
            )?;
            let Some(relation_partition) = partitions.get_partition(relation_id.partition_id) else {
                continue;
            };
            let Some(slot) = relation_partition.relation_arena.get(&relation_id) else {
                continue;
            };
            let Some(kind_id) = slot.kind_id() else {
                continue;
            };
            let Some(endpoints) = slot.extra().as_ref() else {
                continue;
            };
            if slot.lifecycle()
                != crate::storage::data::RecordLifecycleState::Live
            {
                continue;
            }
            let scope = scopes.entry(kind_id).or_default();
            scope.increment_counts(endpoints.source, endpoints.target);
            performance.count_relation_uniqueness_candidates(1);
            if deleted_entities.contains(&endpoints.source) {
                scope.deleted_entities.insert(endpoints.source);
            }
            if deleted_entities.contains(&endpoints.target) {
                scope.deleted_entities.insert(endpoints.target);
            }
        }
    }

    for scope in scopes.values_mut() {
        let planned_edges = std::mem::take(&mut scope.planned_edges);
        for edge in planned_edges {
            scope.increment_counts(edge.source, edge.target);
            if deleted_entities.contains(&edge.source) {
                scope.deleted_entities.insert(edge.source);
            }
            if deleted_entities.contains(&edge.target) {
                scope.deleted_entities.insert(edge.target);
            }
            scope.planned_edges.push(edge);
        }
    }

    scopes.retain(|_, scope| !scope.is_empty());
    Ok((!scopes.is_empty()).then(|| PreparedRelationIntegrityScopes::new(scopes)))
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

fn relation_kind_for_id(
    partitions: &dyn PartitionAccess,
    relation_id: RelationId,
) -> Option<KindId> {
    partitions
        .get_partition(relation_id.partition_id)?
        .relation_arena
        .get(&relation_id)
        .and_then(|slot| slot.kind_id())
}

#[cfg(test)]
mod tests {
    use super::InvariantExecutionRequest;
    use crate::config::data::{CascadeDeletePolicy, CrossContextPolicy};
    use crate::facade::{
        runtime::RelationalRuntimeApi,
        schema::{
            EntityKindRegistration, KindAspectDeclarations, RelationKindRegistration,
            RelationalSchemaRegistry, SchemaId, SchemaVersionId,
        },
    };
    use crate::identity::data::KindId;
    use crate::schema::data::{
        EndpointKindContractDeclaration, RelationIntegrityDeclarations, RelationPayloadClass,
    };
    use crate::identity::data::PartitionId;
    use crate::payloads::data::RecordPayload;
    use crate::transactions::data::{
        CreateIntent, DeleteEntityIntent, EntityMutationIntent, EntitySpec, MergedCommitPlan,
        MutationIntent, RelationSpec, ReplaceEntityIntent, TransactionId, TransactionOptions,
        WorkerIntentBatch,
    };
    use crate::validation::data::InvariantPlanContract;
    use crate::validation::engine::{InvariantObservation, InvariantRequestProfile};
    use serde_json::json;

    fn relation_integrity_runtime() -> crate::logic::runtime::RelationalRuntime {
        let registry = RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: KindId(1),
                kind_name: "test.entity".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                aspect_declarations: KindAspectDeclarations::default(),
            })
            .and_then(|registry| {
                registry.register_relation_kind(RelationKindRegistration {
                    kind_id: KindId(2),
                    kind_name: "test.edge.a".to_string(),
                    schema_id: SchemaId("test".to_string()),
                    schema_version_id: SchemaVersionId(1),
                    payload_class: RelationPayloadClass::PayloadBearingRelation,
                    cross_context_policy: CrossContextPolicy::AllowExplicit,
                    cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                    aspect_declarations: KindAspectDeclarations::default(),
                    relation_integrity: RelationIntegrityDeclarations::new(
                        vec![EndpointKindContractDeclaration {
                            contract_id: "kind2".into(),
                            allowed_source_kinds: vec![KindId(1)],
                            allowed_target_kinds: vec![KindId(1)],
                            self_edges_allowed: false,
                            cross_context_policy: CrossContextPolicy::AllowExplicit,
                        }],
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                    ),
                })
            })
            .and_then(|registry| {
                registry.register_relation_kind(RelationKindRegistration {
                    kind_id: KindId(3),
                    kind_name: "test.edge.b".to_string(),
                    schema_id: SchemaId("test".to_string()),
                    schema_version_id: SchemaVersionId(1),
                    payload_class: RelationPayloadClass::PayloadBearingRelation,
                    cross_context_policy: CrossContextPolicy::AllowExplicit,
                    cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                    aspect_declarations: KindAspectDeclarations::default(),
                    relation_integrity: RelationIntegrityDeclarations::new(
                        vec![EndpointKindContractDeclaration {
                            contract_id: "kind3".into(),
                            allowed_source_kinds: vec![KindId(1)],
                            allowed_target_kinds: vec![KindId(1)],
                            self_edges_allowed: false,
                            cross_context_policy: CrossContextPolicy::AllowExplicit,
                        }],
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                    ),
                })
            })
            .unwrap();
        RelationalRuntimeApi::builder()
            .schema_registry(registry)
            .build()
    }

    fn create_relation_of_kind(
        runtime: &mut crate::logic::runtime::RelationalRuntime,
        kind_id: KindId,
        source: crate::identity::data::EntityId,
        target: crate::identity::data::EntityId,
        client_key: &str,
    ) {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new(format!("relation-{client_key}")).push(MutationIntent::Create(
                CreateIntent::Relation(RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id,
                    client_key: crate::symbols::data::InternedString::Raw(client_key.to_string()),
                    source,
                    target,
                    payload: Some(RecordPayload::StructuredJson(json!({"label":client_key}))),
                }),
            )),
        );
        txn.commit().unwrap();
    }

    fn create_entity(
        runtime: &mut crate::logic::runtime::RelationalRuntime,
        name: &str,
    ) -> crate::identity::data::EntityId {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new(format!("entity-{name}")).push(MutationIntent::Create(
                CreateIntent::Entity(EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_key: crate::symbols::data::InternedString::Raw(name.to_string()),
                    payload: RecordPayload::StructuredJson(json!({"name": name})),
                }),
            )),
        );
        let outcome = txn.commit().unwrap();
        outcome
            .changed_records
            .iter()
            .find_map(|record| match record {
                crate::facade::transactions::RecordRef::Entity(entity_id) => Some(*entity_id),
                crate::facade::transactions::RecordRef::Relation(_) => None,
            })
            .expect("created entity")
    }

    fn request_for_plan<'runtime>(
        runtime: &'runtime crate::logic::runtime::RelationalRuntime,
        plan: &'runtime MergedCommitPlan,
    ) -> InvariantExecutionRequest<'runtime> {
        InvariantExecutionRequest::from_profile_with_contract(
            InvariantRequestProfile::CommitBoundary,
            runtime,
            InvariantObservation::committed(runtime.storage_access().current_state()),
            runtime.current_version_id(),
            Some(plan),
            Some(InvariantPlanContract::from_merged_plan(plan)),
        )
    }

    fn relation_rule_kind(rule: &crate::validation::data::InvariantRule) -> Option<KindId> {
        match rule {
            crate::validation::data::InvariantRule::EndpointKindContract(contract) => {
                Some(contract.relation_kind_id)
            }
            crate::validation::data::InvariantRule::CardinalityMaximumContract(contract) => {
                Some(contract.relation_kind_id)
            }
            crate::validation::data::InvariantRule::CardinalityMinimumContract(contract) => {
                Some(contract.relation_kind_id)
            }
            crate::validation::data::InvariantRule::UniquenessContract(contract) => {
                Some(contract.relation_kind_id)
            }
            crate::validation::data::InvariantRule::SymmetryContract(contract) => {
                Some(contract.relation_kind_id)
            }
            crate::validation::data::InvariantRule::EndpointDeletionIntegrityContract(contract) => {
                Some(contract.relation_kind_id)
            }
            _ => None,
        }
    }

    #[test]
    fn request_excludes_unrelated_relation_kind_registrations_for_relation_create() {
        let mut runtime = relation_integrity_runtime();
        let source = create_entity(&mut runtime, "source");
        let target = create_entity(&mut runtime, "target");
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(11),
            merged_intents: vec![MutationIntent::Create(CreateIntent::Relation(RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: crate::symbols::data::InternedString::Raw("planned".to_string()),
                source,
                target,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"planned"}))),
            }))],
        };

        let request = request_for_plan(&runtime, &plan);
        let included_relation_kinds = runtime
            .aspect_semantics
            .relation_integrity_registrations
            .iter()
            .filter(|registration| request.includes_registration(registration))
            .filter_map(|registration| relation_rule_kind(&registration.rule))
            .collect::<Vec<_>>();

        assert_eq!(included_relation_kinds, vec![KindId(2)]);
    }

    #[test]
    fn request_excludes_unrelated_relation_kind_registrations_for_entity_delete() {
        let mut runtime = relation_integrity_runtime();
        let anchor = create_entity(&mut runtime, "anchor");
        let target = create_entity(&mut runtime, "target");
        let isolated_a = create_entity(&mut runtime, "isolated-a");
        let isolated_b = create_entity(&mut runtime, "isolated-b");
        create_relation_of_kind(&mut runtime, KindId(2), anchor, target, "adjacent-kind2");
        create_relation_of_kind(&mut runtime, KindId(3), isolated_a, isolated_b, "remote-kind3");

        let plan = MergedCommitPlan {
            transaction_id: TransactionId(12),
            merged_intents: vec![MutationIntent::Entity(EntityMutationIntent::Delete(
                DeleteEntityIntent { entity_id: anchor },
            ))],
        };

        let request = request_for_plan(&runtime, &plan);
        let included_relation_kinds = runtime
            .aspect_semantics
            .relation_integrity_registrations
            .iter()
            .filter(|registration| request.includes_registration(registration))
            .filter_map(|registration| relation_rule_kind(&registration.rule))
            .collect::<Vec<_>>();

        assert_eq!(included_relation_kinds, vec![KindId(2)]);
    }

    #[test]
    fn request_excludes_unrelated_relation_kind_registrations_for_entity_replace() {
        let mut runtime = relation_integrity_runtime();
        let anchor = create_entity(&mut runtime, "anchor");
        let target = create_entity(&mut runtime, "target");
        let isolated_a = create_entity(&mut runtime, "isolated-a");
        let isolated_b = create_entity(&mut runtime, "isolated-b");
        create_relation_of_kind(&mut runtime, KindId(2), anchor, target, "adjacent-kind2");
        create_relation_of_kind(&mut runtime, KindId(3), isolated_a, isolated_b, "remote-kind3");

        let plan = MergedCommitPlan {
            transaction_id: TransactionId(13),
            merged_intents: vec![MutationIntent::Entity(EntityMutationIntent::Replace(
                ReplaceEntityIntent {
                    entity_id: anchor,
                    replacement: EntitySpec {
                        partition_id: PartitionId::main(),
                        kind_id: KindId(1),
                        client_key: crate::symbols::data::InternedString::Raw(
                            "replacement".to_string(),
                        ),
                        payload: RecordPayload::StructuredJson(json!({"name":"replacement"})),
                    },
                },
            ))],
        };

        let request = request_for_plan(&runtime, &plan);
        let included_relation_kinds = runtime
            .aspect_semantics
            .relation_integrity_registrations
            .iter()
            .filter(|registration| request.includes_registration(registration))
            .filter_map(|registration| relation_rule_kind(&registration.rule))
            .collect::<Vec<_>>();

        assert_eq!(included_relation_kinds, vec![KindId(2)]);
    }
}
