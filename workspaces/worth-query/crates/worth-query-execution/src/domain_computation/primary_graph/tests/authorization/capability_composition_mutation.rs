use worth_relational::facade::identity::{EntityId, PartitionId, RelationId};
use worth_relational::facade::symbols::ClientKey;
use worth_relational::facade::transactions::{
    AspectFieldPatch, CreateIntent, DeleteRelationIntent, EntityReference, MutationIntent,
    RelationMutationIntent, RelationSpec, TransactionOptions, WorkerIntentBatch,
};

use super::super::fixture::{
    live_scope, Account, AccountIdentity, AuthorizationWorld, IdentityExecutionSchema, Principal,
    PrincipalIdentityField,
};
use crate::domain_computation::primary_graph::WorthQueryPrincipalResolutionMode;

pub(super) fn add_policy_relation<Relation>(
    world: &AuthorizationWorld,
    relation: worth_query_declaration::facade::application_schema::ApplicationRelationRef<
        IdentityExecutionSchema,
        Relation,
        Principal,
        Account,
    >,
    key: &str,
) {
    let (source, target) = actor_and_account(world);
    let kind = relation_kind(world, relation.name());
    mutate(world, |batch| {
        batch.push(MutationIntent::Create(CreateIntent::Relation(
            RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: kind,
                client_key: ClientKey::raw(key),
                source: EntityReference::Existing(source),
                target: EntityReference::Existing(target),
                fields: AspectFieldPatch::default(),
            },
        )))
    });
}

pub(super) fn remove_policy_relation<Relation>(
    world: &AuthorizationWorld,
    relation: worth_query_declaration::facade::application_schema::ApplicationRelationRef<
        IdentityExecutionSchema,
        Relation,
        Principal,
        Account,
    >,
) {
    let (source, target) = actor_and_account(world);
    let kind = relation_kind(world, relation.name());
    let relation = current_relation(world, kind, source, target);
    mutate(world, |batch| {
        batch.push(MutationIntent::Relation(RelationMutationIntent::Delete(
            DeleteRelationIntent {
                relation_id: relation,
            },
        )))
    });
}

fn actor_and_account(world: &AuthorizationWorld) -> (EntityId, EntityId) {
    let scope = live_scope();
    let principal = world
        .application
        .resolve_entity(
            PrincipalIdentityField::reference(),
            1_u64,
            &scope,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap()
        .entity_id();
    let account = world
        .application
        .resolve_entity(
            AccountIdentity::reference(),
            "account-1".to_owned(),
            &scope,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap()
        .entity_id();
    (principal, account)
}

fn relation_kind(
    world: &AuthorizationWorld,
    name: &str,
) -> worth_relational::facade::identity::KindId {
    world
        .application
        .runtime
        .primary_graph()
        .unwrap()
        .layout()
        .relation(name)
        .unwrap()
        .kind
}

fn current_relation(
    world: &AuthorizationWorld,
    kind: worth_relational::facade::identity::KindId,
    source: EntityId,
    target: EntityId,
) -> RelationId {
    let graph = world.application.runtime.primary_graph().unwrap();
    graph.integration_handle().with_runtime_mut(|runtime| {
        let snapshot = runtime.snapshots().snapshot();
        let relation = runtime
            .read_truth()
            .visible_relations_of_kind(kind, snapshot.version_id)
            .into_iter()
            .find(|record| record.source == source && record.target == target)
            .unwrap()
            .relation_id;
        runtime.snapshots().release_snapshot(&snapshot);
        relation
    })
}

fn mutate(world: &AuthorizationWorld, build: impl FnOnce(WorkerIntentBatch) -> WorkerIntentBatch) {
    let graph = world.application.runtime.primary_graph().unwrap();
    let handle = graph.integration_handle();
    handle.with_runtime_mut(|runtime| {
        let mut transaction = runtime.begin_transaction(TransactionOptions::default());
        transaction.push_batch(build(WorkerIntentBatch::new(
            "capability-composition-hostility",
        )));
        transaction.commit().unwrap();
        handle.ensure_primary_indexes_current(runtime).unwrap();
    });
}
