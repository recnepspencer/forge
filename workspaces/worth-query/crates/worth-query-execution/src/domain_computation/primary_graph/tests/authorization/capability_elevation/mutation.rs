use std::collections::BTreeMap;

use worth_query_installation::facade::TypedApplicationValue;
use worth_relational::facade::identity::{EntityId, PartitionId};
use worth_relational::facade::symbols::ClientKey;
use worth_relational::facade::transactions::{
    AspectFieldPatch, CreateIntent, DeleteRelationIntent, EntityMutationIntent, EntityReference,
    MutationIntent, RelationMutationIntent, RelationSpec, UpdateEntityFieldsIntent,
    WorkerIntentBatch,
};

use super::super::super::fixture::{
    live_scope, AccountIdentity, CapabilityElevationApprover, CapabilityElevationIdentity,
    CapabilityElevationResource, CapabilityElevationStatus, CapabilityElevationStatusField,
};
use crate::domain_computation::primary_graph::WorthQueryPrincipalResolutionMode;

pub(super) use super::review_support_mutation::{
    complete_review_out_of_band, replace_support_grantor_with_custodian,
};

pub(super) fn set_status(
    world: &super::super::super::fixture::AuthorizationWorld,
    elevation_identity: &str,
    status: CapabilityElevationStatus,
) {
    let elevation = world
        .application
        .resolve_entity(
            CapabilityElevationIdentity::reference(),
            elevation_identity.to_owned(),
            &live_scope(),
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let field = CapabilityElevationStatusField::reference();
    let graph = world.application.runtime.primary_graph().unwrap();
    let locator = graph
        .layout()
        .field_locator(field.entity(), field.aspect(), field.field())
        .unwrap()
        .clone();
    let handle = graph.integration_handle();
    handle.with_runtime_mut(|runtime| {
        let fields = AspectFieldPatch::from(BTreeMap::from([(
            locator,
            status.into_foundational_value(),
        )]));
        let mut transaction = {
            let transaction_validation_input = runtime
                .admit_branch_basis(&runtime.main_branch_identity())
                .expect("main branch binding");
            runtime
                .begin_branch_transaction(
                    &transaction_validation_input,
                    worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
                )
                .expect("owner-admitted transaction context")
        };
        transaction
            .push_batch(WorkerIntentBatch::new("set-elevation-status").push(
                MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                    UpdateEntityFieldsIntent {
                        entity_id: elevation.entity_id(),
                        fields,
                    },
                )),
            ))
            .expect("test staging stays within configured resource budgets");
        let committed = transaction.commit(runtime).unwrap();
        crate::domain_computation::primary_graph::tests::fixture::release_test_commit_snapshot(
            runtime, &committed,
        );
        handle.ensure_primary_indexes_current(runtime).unwrap();
    });
}

pub(super) fn add_self_approver(
    world: &super::super::super::fixture::AuthorizationWorld,
    elevation_identity: &str,
    requester: EntityId,
) {
    let elevation = world
        .application
        .resolve_entity(
            CapabilityElevationIdentity::reference(),
            elevation_identity.to_owned(),
            &live_scope(),
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let graph = world.application.runtime.primary_graph().unwrap();
    let relation_kind = graph
        .layout()
        .relation(CapabilityElevationApprover::reference().name())
        .unwrap()
        .kind;
    let handle = graph.integration_handle();
    handle.with_runtime_mut(|runtime| {
        let mut transaction = {
            let transaction_validation_input = runtime
                .admit_branch_basis(&runtime.main_branch_identity())
                .expect("main branch binding");
            runtime
                .begin_branch_transaction(
                    &transaction_validation_input,
                    worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
                )
                .expect("owner-admitted transaction context")
        };
        transaction
            .push_batch(
                WorkerIntentBatch::new("add-self-approver").push(MutationIntent::Create(
                    CreateIntent::Relation(RelationSpec {
                        partition_id: PartitionId::main(),
                        kind_id: relation_kind,
                        client_key: ClientKey::raw("elevation-self-approver"),
                        source: EntityReference::Existing(requester),
                        target: EntityReference::Existing(elevation.entity_id()),
                        fields: AspectFieldPatch::default(),
                    }),
                )),
            )
            .expect("test staging stays within configured resource budgets");
        let committed = transaction.commit(runtime).unwrap();
        crate::domain_computation::primary_graph::tests::fixture::release_test_commit_snapshot(
            runtime, &committed,
        );
        handle.ensure_primary_indexes_current(runtime).unwrap();
    });
}

pub(super) fn replace_elevation_resource(
    world: &super::super::super::fixture::AuthorizationWorld,
    elevation_identity: &str,
    replacement_account: Option<&str>,
) {
    let elevation = world
        .application
        .resolve_entity(
            CapabilityElevationIdentity::reference(),
            elevation_identity.to_owned(),
            &live_scope(),
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let replacement = replacement_account.map(|identity| {
        world
            .application
            .resolve_entity(
                AccountIdentity::reference(),
                identity.to_owned(),
                &live_scope(),
                WorthQueryPrincipalResolutionMode::Ordinary,
            )
            .unwrap()
            .entity_id()
    });
    let graph = world.application.runtime.primary_graph().unwrap();
    let relation_kind = graph
        .layout()
        .relation(CapabilityElevationResource::reference().name())
        .unwrap()
        .kind;
    let handle = graph.integration_handle();
    handle.with_runtime_mut(|runtime| {
        let snapshot = crate::domain_computation::primary_graph::exact_basis_access::open_current_main_snapshot(runtime)
            .expect("primary branch has a current snapshot");
        let relation = runtime
            .read_truth()
            .visible_relations_of_kind(relation_kind, snapshot.version_id())
            .into_iter()
            .find(|record| record.source == elevation.entity_id())
            .expect("the elevation has one current direct resource")
            .relation_id;
        crate::relational_snapshot_release::release_query_snapshot(runtime, &snapshot);
        let mut batch = WorkerIntentBatch::new("replace-elevation-resource").push(
            MutationIntent::Relation(RelationMutationIntent::Delete(DeleteRelationIntent {
                relation_id: relation,
            })),
        );
        if let Some(account) = replacement {
            batch = batch.push(MutationIntent::Create(CreateIntent::Relation(
                RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id: relation_kind,
                    client_key: ClientKey::raw("replacement-elevation-resource"),
                    source: EntityReference::Existing(elevation.entity_id()),
                    target: EntityReference::Existing(account),
                    fields: AspectFieldPatch::default(),
                },
            )));
        }
        let mut transaction = {
    let transaction_validation_input = runtime
                .admit_branch_basis(&runtime.main_branch_identity())
                .expect("main branch binding");
    runtime
        .begin_branch_transaction(
            &transaction_validation_input,
            worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
        )
        .expect("owner-admitted transaction context")
};
        transaction.push_batch(batch).expect("test staging stays within configured resource budgets");
        let committed = transaction.commit(runtime).unwrap();
        crate::domain_computation::primary_graph::tests::fixture::release_test_commit_snapshot(
            runtime, &committed,
        );
        handle.ensure_primary_indexes_current(runtime).unwrap();
    });
}

pub(super) fn add_elevation_resource(
    world: &super::super::super::fixture::AuthorizationWorld,
    elevation_identity: &str,
    account_identity: &str,
) {
    let elevation = world
        .application
        .resolve_entity(
            CapabilityElevationIdentity::reference(),
            elevation_identity.to_owned(),
            &live_scope(),
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let account = world
        .application
        .resolve_entity(
            AccountIdentity::reference(),
            account_identity.to_owned(),
            &live_scope(),
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let graph = world.application.runtime.primary_graph().unwrap();
    let relation_kind = graph
        .layout()
        .relation(CapabilityElevationResource::reference().name())
        .unwrap()
        .kind;
    let handle = graph.integration_handle();
    handle.with_runtime_mut(|runtime| {
        let mut transaction = {
            let transaction_validation_input = runtime
                .admit_branch_basis(&runtime.main_branch_identity())
                .expect("main branch binding");
            runtime
                .begin_branch_transaction(
                    &transaction_validation_input,
                    worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
                )
                .expect("owner-admitted transaction context")
        };
        transaction
            .push_batch(WorkerIntentBatch::new("add-elevation-resource").push(
                MutationIntent::Create(CreateIntent::Relation(RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id: relation_kind,
                    client_key: ClientKey::raw("additional-elevation-resource"),
                    source: EntityReference::Existing(elevation.entity_id()),
                    target: EntityReference::Existing(account.entity_id()),
                    fields: AspectFieldPatch::default(),
                })),
            ))
            .expect("test staging stays within configured resource budgets");
        let committed = transaction.commit(runtime).unwrap();
        crate::domain_computation::primary_graph::tests::fixture::release_test_commit_snapshot(
            runtime, &committed,
        );
        handle.ensure_primary_indexes_current(runtime).unwrap();
    });
}
