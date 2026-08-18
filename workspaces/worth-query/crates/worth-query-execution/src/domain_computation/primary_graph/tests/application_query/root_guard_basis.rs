use std::collections::BTreeMap;
use std::num::NonZeroUsize;
use std::time::Duration;

use worth_query_declaration::facade::application_query::ApplicationQueryParameterSet;
use worth_query_installation::facade::TypedApplicationValue;
use worth_relational::facade::transactions::{
    AspectFieldPatch, EntityMutationIntent, MutationIntent, TransactionOptions,
    UpdateEntityFieldsIntent, WorkerIntentBatch,
};

use super::current_controls;
use crate::domain_computation::primary_graph::{
    tests::fixture::{
        installed_authorization_world, AccountIdentity, AccountStatus, CrossRootQuery,
    },
    WorthQueryApplicationQueryAccessContext, WorthQueryApplicationQueryBasisPosture,
    WorthQueryApplicationQueryControls, WorthQueryPrincipalResolutionMode,
};

#[test]
fn root_path_guard_reads_its_pinned_truth_version() {
    let world = installed_authorization_world(true);
    let request = super::super::fixture::live_scope();
    let external = world.authenticate("alice", Duration::from_secs(60), &request);
    let principal = world
        .application
        .resolve_authenticated_principal(
            &world.binding,
            external,
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let account = world
        .application
        .resolve_entity(
            AccountIdentity::reference(),
            "account-1".to_string(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let query = world
        .application
        .installed_schema()
        .application_query(CrossRootQuery::reference())
        .unwrap();
    let pinned = world
        .application
        .pin_current_application_query_basis(&request)
        .unwrap();
    change_account_status(&world, account.entity_id(), "closed");
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let pinned_plan = world
        .application
        .admit_application_query(
            &query,
            &access,
            ApplicationQueryParameterSet::new(),
            WorthQueryApplicationQueryControls::pinned_one_shot(
                pinned,
                NonZeroUsize::new(10).unwrap(),
                NonZeroUsize::new(10_000).unwrap(),
                &request,
            ),
        )
        .unwrap();
    let pinned_result = world
        .application
        .execute_application_query_one_shot(pinned_plan)
        .unwrap();
    let current_plan = world
        .application
        .admit_application_query(
            &query,
            &access,
            ApplicationQueryParameterSet::new(),
            current_controls(&request),
        )
        .unwrap();
    let current_result = world
        .application
        .execute_application_query_one_shot(current_plan)
        .unwrap();

    assert_eq!(pinned_result.rows().len(), 2);
    assert!(current_result.rows().is_empty());
    assert_eq!(
        pinned_result.receipt().basis_posture(),
        WorthQueryApplicationQueryBasisPosture::Pinned
    );
}

fn change_account_status(
    world: &super::super::fixture::AuthorizationWorld,
    account: worth_relational::facade::identity::EntityId,
    status: &str,
) {
    let graph = world.application.runtime.primary_graph().unwrap();
    let field = AccountStatus::reference();
    let locator = graph
        .layout
        .field_locator(field.entity(), field.aspect(), field.field())
        .unwrap()
        .clone();
    let handle = graph.integration_handle();
    handle.with_runtime_mut(|runtime| {
        let fields = AspectFieldPatch::from(BTreeMap::from([(
            locator,
            status.to_string().into_foundational_value(),
        )]));
        let mut transaction = runtime.begin_transaction(
            runtime
                .transaction_options_for_main()
                .expect("main branch binding"),
        );
        transaction.push_batch(
            WorkerIntentBatch::new("change-status-after-basis-pin").push(MutationIntent::Entity(
                EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                    entity_id: account,
                    fields,
                }),
            )),
        );
        transaction.commit().unwrap();
        handle.ensure_primary_indexes_current(runtime).unwrap();
    });
}
