use std::collections::BTreeMap;

use worth_query_installation::facade::TypedApplicationValue;
use worth_relational::facade::identity::{EntityId, PartitionId};
use worth_relational::facade::symbols::ClientKey;
use worth_relational::facade::transactions::{
    AspectFieldPatch, CreateIntent, DeleteRelationIntent, EntityMutationIntent, EntityReference,
    MutationIntent, RelationMutationIntent, RelationSpec, UpdateEntityFieldsIntent,
    WorkerIntentBatch,
};

use super::super::super::fixture::capability::{CapabilityCustodian, CapabilityGrantor};
use super::super::super::fixture::{
    live_scope, CapabilityIdentity, CapabilityReviewIdentity, CapabilityReviewStatus,
    CapabilityReviewStatusField, CapabilityReviewer,
};
use crate::domain_computation::primary_graph::WorthQueryPrincipalResolutionMode;

pub(super) fn complete_review_out_of_band(
    world: &super::super::super::fixture::AuthorizationWorld,
    reviewer: EntityId,
) {
    let review = world
        .application
        .resolve_entity(
            CapabilityReviewIdentity::reference(),
            "review-2".to_owned(),
            &live_scope(),
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let field = CapabilityReviewStatusField::reference();
    let graph = world.application.runtime.primary_graph().unwrap();
    let locator = graph
        .layout()
        .field_locator(field.entity(), field.aspect(), field.field())
        .unwrap()
        .clone();
    let relation_kind = graph
        .layout()
        .relation(CapabilityReviewer::reference().name())
        .unwrap()
        .kind;
    let handle = graph.integration_handle();
    handle.with_runtime_mut(|runtime| {
        let fields = AspectFieldPatch::from(BTreeMap::from([(
            locator,
            CapabilityReviewStatus::Completed.into_foundational_value(),
        )]));
        let basis = runtime
            .admit_main_branch_basis()
            .expect("main branch binding");
        let mut transaction = runtime
            .begin_branch_transaction(
                &basis,
                worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
            )
            .expect("owner-admitted transaction context");
        transaction
            .push_batch(
                WorkerIntentBatch::new("complete-review-out-of-band")
                    .push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                        UpdateEntityFieldsIntent {
                            entity_id: review.entity_id(),
                            fields,
                        },
                    )))
                    .push(MutationIntent::Create(CreateIntent::Relation(
                        RelationSpec {
                            partition_id: PartitionId::main(),
                            kind_id: relation_kind,
                            client_key: ClientKey::raw("out-of-band-reviewer"),
                            source: EntityReference::Existing(reviewer),
                            target: EntityReference::Existing(review.entity_id()),
                            fields: AspectFieldPatch::default(),
                        },
                    ))),
            )
            .expect("test staging stays within configured resource budgets");
        let committed = transaction.commit(runtime).unwrap();
        super::super::super::fixture::release_test_commit_snapshot(runtime, &committed);
        handle.ensure_primary_indexes_current(runtime).unwrap();
    });
}

pub(super) fn replace_support_grantor_with_custodian(
    world: &super::super::super::fixture::AuthorizationWorld,
    principal: EntityId,
) {
    let grant = world
        .application
        .resolve_entity(
            CapabilityIdentity::reference(),
            "capability-1".to_owned(),
            &live_scope(),
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let graph = world.application.runtime.primary_graph().unwrap();
    let grantor_kind = graph
        .layout()
        .relation(CapabilityGrantor::reference().name())
        .unwrap()
        .kind;
    let custodian_kind = graph
        .layout()
        .relation(CapabilityCustodian::reference().name())
        .unwrap()
        .kind;
    let handle = graph.integration_handle();
    handle.with_runtime_mut(|runtime| {
        let snapshot = crate::domain_computation::primary_graph::exact_basis_access::open_current_main_snapshot(runtime)
            .expect("primary branch has a current snapshot");
        let grantor = runtime
            .read_truth()
            .visible_relations_of_kind(grantor_kind, snapshot.version_id())
            .into_iter()
            .find(|record| record.source == principal && record.target == grant.entity_id())
            .expect("the request support has one current grantor path")
            .relation_id;
        crate::relational_snapshot_release::release_query_snapshot(runtime, &snapshot);
        let basis = runtime
            .admit_main_branch_basis()
            .expect("main branch binding");
        let mut transaction = runtime
            .begin_branch_transaction(
                &basis,
                worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
            )
            .expect("owner-admitted transaction context");
        transaction
            .push_batch(
                WorkerIntentBatch::new("replace-elevation-support-policy-path")
                    .push(MutationIntent::Relation(RelationMutationIntent::Delete(
                        DeleteRelationIntent {
                            relation_id: grantor,
                        },
                    )))
                    .push(MutationIntent::Create(CreateIntent::Relation(
                        RelationSpec {
                            partition_id: PartitionId::main(),
                            kind_id: custodian_kind,
                            client_key: ClientKey::raw("capability-1-custodian"),
                            source: EntityReference::Existing(principal),
                            target: EntityReference::Existing(grant.entity_id()),
                            fields: AspectFieldPatch::default(),
                        },
                    ))),
            )
            .expect("test staging stays within configured resource budgets");
        let committed = transaction.commit(runtime).unwrap();
        super::super::super::fixture::release_test_commit_snapshot(runtime, &committed);
        handle.ensure_primary_indexes_current(runtime).unwrap();
    });
}
