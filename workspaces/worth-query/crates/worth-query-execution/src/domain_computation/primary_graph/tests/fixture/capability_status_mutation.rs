use std::collections::BTreeMap;

use worth_foundational::facade::AspectFieldLocator;
use worth_query_installation::facade::TypedApplicationValue;
use worth_relational::facade::transactions::{
    AspectFieldPatch, EntityMutationIntent, MutationIntent, UpdateEntityFieldsIntent,
    WorkerIntentBatch,
};

use super::world_installation::AuthorizationWorld;
use super::{CapabilityIdentity, CapabilityStatus, CapabilityStatusField};
use crate::domain_computation::primary_graph::tests::fixture::live_scope;
use crate::domain_computation::primary_graph::WorthQueryPrincipalResolutionMode;

pub(in crate::domain_computation::primary_graph) fn revoke_current_capability(
    world: &AuthorizationWorld,
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
    let field = CapabilityStatusField::reference();
    let locator = installed_field(world, field.entity(), field.aspect(), field.field());
    let graph = world.application.runtime.primary_graph().unwrap();
    let handle = graph.integration_handle();
    handle.with_runtime_mut(|runtime| {
        let fields = AspectFieldPatch::from(BTreeMap::from([(
            locator,
            CapabilityStatus::Revoked.into_foundational_value(),
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
            .push_batch(WorkerIntentBatch::new("revoke-live-capability").push(
                MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                    UpdateEntityFieldsIntent {
                        entity_id: grant.entity_id(),
                        fields,
                    },
                )),
            ))
            .expect("test staging stays within configured resource budgets");
        let committed = transaction.commit(runtime).unwrap();
        super::release_test_commit_snapshot(runtime, &committed);
        handle.ensure_primary_indexes_current(runtime).unwrap();
    });
}

fn installed_field(
    world: &AuthorizationWorld,
    entity: &str,
    aspect: &str,
    field: &str,
) -> AspectFieldLocator {
    world
        .application
        .runtime
        .primary_graph()
        .unwrap()
        .layout()
        .field_locator(entity, aspect, field)
        .unwrap()
        .clone()
}
