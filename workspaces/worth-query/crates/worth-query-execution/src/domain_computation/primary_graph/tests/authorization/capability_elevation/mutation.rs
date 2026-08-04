use std::collections::BTreeMap;

use worth_query_installation::facade::TypedApplicationValue;
use worth_relational::facade::identity::{EntityId, PartitionId};
use worth_relational::facade::symbols::ClientKey;
use worth_relational::facade::transactions::{
    AspectFieldPatch, CreateIntent, EntityMutationIntent, EntityReference, MutationIntent,
    RelationSpec, TransactionOptions, UpdateEntityFieldsIntent, WorkerIntentBatch,
};

use super::super::super::fixture::{
    live_scope, CapabilityElevationApprover, CapabilityElevationIdentity,
    CapabilityElevationStatus, CapabilityElevationStatusField,
};
use crate::domain_computation::primary_graph::WorthQueryPrincipalResolutionMode;

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
        let mut transaction = runtime.begin_transaction(TransactionOptions::default());
        transaction.push_batch(WorkerIntentBatch::new("set-elevation-status").push(
            MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                UpdateEntityFieldsIntent {
                    entity_id: elevation.entity_id(),
                    fields,
                },
            )),
        ));
        transaction.commit().unwrap();
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
        let mut transaction = runtime.begin_transaction(TransactionOptions::default());
        transaction.push_batch(WorkerIntentBatch::new("add-self-approver").push(
            MutationIntent::Create(CreateIntent::Relation(RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: relation_kind,
                client_key: ClientKey::raw("elevation-self-approver"),
                source: EntityReference::Existing(requester),
                target: EntityReference::Existing(elevation.entity_id()),
                fields: AspectFieldPatch::default(),
            })),
        ));
        transaction.commit().unwrap();
        handle.ensure_primary_indexes_current(runtime).unwrap();
    });
}
