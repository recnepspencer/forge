use worth_relational::facade::identity::{EntityId, PartitionId, RelationId};
use worth_relational::facade::symbols::ClientKey;
use worth_relational::facade::transactions::{
    AspectFieldPatch, CreateIntent, DeleteRelationIntent, EntityReference, MutationIntent,
    RelationMutationIntent, RelationSpec, WorkerIntentBatch,
};

use super::super::fixture::capability::{CapabilityActionRecord, CapabilityActionRecordIdentity};
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

pub(super) fn add_action_policy_relation<Relation>(
    world: &AuthorizationWorld,
    relation: worth_query_declaration::facade::application_schema::ApplicationRelationRef<
        IdentityExecutionSchema,
        Relation,
        Principal,
        CapabilityActionRecord,
    >,
    key: &str,
    record: &str,
) {
    let source = actor(world);
    let target = resolve_action_record(world, record);
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
    let principal = actor(world);
    let scope = live_scope();
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

fn actor(world: &AuthorizationWorld) -> EntityId {
    world
        .application
        .resolve_entity(
            PrincipalIdentityField::reference(),
            1_u64,
            &live_scope(),
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap()
        .entity_id()
}

fn resolve_action_record(world: &AuthorizationWorld, record: &str) -> EntityId {
    world
        .application
        .resolve_entity(
            CapabilityActionRecordIdentity::reference(),
            record.to_owned(),
            &live_scope(),
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap()
        .entity_id()
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
        let snapshot = crate::domain_computation::primary_graph::exact_basis_access::open_current_main_snapshot(runtime)
            .expect("primary branch has a current snapshot");
        let relation = runtime
            .read_truth()
            .visible_relations_of_kind(kind, snapshot.version_id())
            .into_iter()
            .find(|record| record.source == source && record.target == target)
            .unwrap()
            .relation_id;
        crate::relational_snapshot_release::release_query_snapshot(runtime, &snapshot);
        relation
    })
}

fn mutate(world: &AuthorizationWorld, build: impl FnOnce(WorkerIntentBatch) -> WorkerIntentBatch) {
    let graph = world.application.runtime.primary_graph().unwrap();
    let handle = graph.integration_handle();
    handle.with_runtime_mut(|runtime| {
        let mut transaction = {
            let transaction_validation_input = runtime
                .admit_main_branch_basis()
                .expect("main branch binding");
            runtime
                .begin_branch_transaction(
                    &transaction_validation_input,
                    worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
                )
                .expect("owner-admitted transaction context")
        };
        transaction
            .push_batch(build(WorkerIntentBatch::new(
                "capability-composition-hostility",
            )))
            .expect("test staging stays within configured resource budgets");
        let committed = transaction.commit(runtime).unwrap();
        crate::domain_computation::primary_graph::tests::fixture::release_test_commit_snapshot(
            runtime, &committed,
        );
        handle.ensure_primary_indexes_current(runtime).unwrap();
    });
}
