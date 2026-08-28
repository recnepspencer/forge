use std::collections::BTreeMap;

use worth_query_declaration::facade::authentication::WorthQueryPrincipalMappingStatus;
use worth_query_installation::facade::TypedApplicationValue;
use worth_relational::facade::identity::EntityId;
use worth_relational::facade::transactions::{
    AspectFieldPatch, DeleteRelationIntent, EntityMutationIntent, MutationIntent,
    RelationMutationIntent, UpdateEntityFieldsIntent, WorkerIntentBatch,
};

use super::super::fixture::{AccountOwner, AuthorizationWorld};

pub(super) fn disable_mapping(world: &AuthorizationWorld, mapping_id: EntityId) {
    let graph = world
        .application
        .runtime
        .primary_graph()
        .expect("test world publishes a primary graph");
    let layout = graph
        .layout
        .principal_binding(world.binding.binding())
        .expect("test binding is installed")
        .clone();
    graph.integration_handle().with_runtime_mut(|runtime| {
        let fields = AspectFieldPatch::from(BTreeMap::from([(
            layout.status_locator,
            WorthQueryPrincipalMappingStatus::Disabled.into_foundational_value(),
        )]));
        let main_identity = runtime.main_branch_identity();
        let basis = runtime
            .admit_branch_basis(&main_identity)
            .expect("main branch binding");
        let mut transaction = runtime
            .begin_branch_transaction(
                &basis,
                worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
            )
            .expect("owner-admitted transaction context");
        transaction
            .push_batch(WorkerIntentBatch::new("revoke-after-query-admission").push(
                MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                    UpdateEntityFieldsIntent {
                        entity_id: mapping_id,
                        fields,
                    },
                )),
            ))
            .expect("test staging stays within configured resource budgets");
        let committed = transaction.commit(runtime).unwrap();
        super::super::fixture::release_test_commit_snapshot(runtime, &committed);
    });
}

pub(super) fn revoke_account_ownership(world: &AuthorizationWorld, account: EntityId) {
    let graph = world
        .application
        .runtime
        .primary_graph()
        .expect("test world publishes a primary graph");
    let relation_kind = graph
        .layout
        .relation(AccountOwner::reference().name())
        .expect("account ownership is installed")
        .kind;
    let handle = graph.integration_handle();
    handle.with_runtime_mut(|runtime| {
        let snapshot = crate::domain_computation::primary_graph::exact_basis_access::open_current_main_snapshot(runtime)
            .expect("primary branch has a current snapshot");
        let relation = runtime
            .read_truth()
            .visible_relations_of_kind(relation_kind, snapshot.version_id())
            .into_iter()
            .find(|record| record.target == account)
            .expect("the admitted account has one ownership edge")
            .relation_id;
        crate::relational_snapshot_release::release_query_snapshot(runtime, &snapshot);
        let main_identity = runtime.main_branch_identity();
        let basis = runtime
            .admit_branch_basis(&main_identity)
            .expect("main branch binding");
        let mut transaction = runtime
            .begin_branch_transaction(
                &basis,
                worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
            )
            .expect("owner-admitted transaction context");
        transaction
            .push_batch(WorkerIntentBatch::new("revoke-query-account-owner").push(
                MutationIntent::Relation(RelationMutationIntent::Delete(DeleteRelationIntent {
                    relation_id: relation,
                })),
            ))
            .expect("test staging stays within configured resource budgets");
        let committed = transaction.commit(runtime).unwrap();
        super::super::fixture::release_test_commit_snapshot(runtime, &committed);
        handle.ensure_primary_indexes_current(runtime).unwrap();
    });
}
