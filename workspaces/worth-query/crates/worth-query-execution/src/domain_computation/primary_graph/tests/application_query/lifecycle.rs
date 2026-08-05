use std::collections::BTreeMap;
use std::num::NonZeroUsize;
use std::time::{Duration, Instant};

use worth_query_admission::facade::authenticated_principal::{
    WorthQueryCancellationSource, WorthQueryRequestScope,
};
use worth_query_declaration::facade::application_query::ApplicationQueryParameterSet;
use worth_query_declaration::facade::authentication::WorthQueryPrincipalMappingStatus;
use worth_query_installation::facade::TypedApplicationValue;
use worth_relational::facade::transactions::{
    AspectFieldPatch, DeleteRelationIntent, EntityMutationIntent, MutationIntent,
    RelationMutationIntent, TransactionOptions, UpdateEntityFieldsIntent, WorkerIntentBatch,
};

use super::{current_controls, installed_query};
use crate::domain_computation::primary_graph::{
    tests::fixture::{
        installed_authorization_world, live_account_parameters, status_parameter, AccountIdentity,
        AccountOwner, AccountStatus,
    },
    WorthQueryApplicationOneShotDenialKind, WorthQueryApplicationQueryAccessContext,
    WorthQueryPrincipalResolutionMode,
};

#[test]
fn retained_continuation_holds_identity_but_no_relational_basis() {
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
            "account-1".to_owned(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let query = super::installed_live_query(&world);
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let plan = world
        .application
        .admit_application_query(
            &query,
            &access,
            live_account_parameters("account-1"),
            crate::domain_computation::primary_graph::WorthQueryApplicationQueryControls::
                current_continuation_page(
                    NonZeroUsize::new(1).unwrap(),
                    NonZeroUsize::new(10_000).unwrap(),
                    &request,
                ),
        )
        .unwrap();
    let basis = plan.basis_identity().clone();
    let session = plan.graph_work_session_identity();
    assert!(basis_is_live(&world, &basis));

    let page = world
        .application
        .execute_application_query_continuation_page(plan)
        .unwrap();
    let (_, continuation, receipt) = page.into_parts();
    let continuation = continuation.expect("one of two activities must retain a next page");

    assert_eq!(continuation.page_ordinal(), 2);
    assert!(receipt.basis_released());
    assert_eq!(receipt.read_completion().session_identity(), session);
    assert_eq!(receipt.read_completion().basis_identity(), &basis);
    assert_eq!(
        receipt
            .read_completion()
            .release()
            .released_reservation_count(),
        1
    );
    assert!(!basis_is_live(&world, &basis));
    drop(continuation);
    assert!(!basis_is_live(&world, &basis));
}

#[test]
fn foreign_runtime_rejects_plan_and_releases_its_basis() {
    let world = installed_authorization_world(true);
    let foreign = installed_authorization_world(true);
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
            AccountStatus::reference(),
            "open".to_string(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let query = installed_query(&world);
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let plan = world
        .application
        .admit_application_query(
            &query,
            &access,
            ApplicationQueryParameterSet::new().bind(status_parameter(), "open".to_string()),
            current_controls(&request),
        )
        .unwrap();
    let basis = plan.basis_identity().clone();
    assert!(basis_is_live(&world, &basis));

    let denial = foreign
        .application
        .execute_application_query_one_shot(plan)
        .err()
        .expect("foreign runtime cannot consume a sealed plan");
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationOneShotDenialKind::ForeignPlan
    );
    assert!(!basis_is_live(&world, &basis));
}

#[test]
fn cancellation_after_admission_releases_basis_before_projection() {
    let world = installed_authorization_world(true);
    let cancellation = WorthQueryCancellationSource::new();
    let request = WorthQueryRequestScope::new(
        Instant::now() + Duration::from_secs(60),
        cancellation.token(),
    );
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
            AccountStatus::reference(),
            "open".to_string(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let query = installed_query(&world);
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let plan = world
        .application
        .admit_application_query(
            &query,
            &access,
            ApplicationQueryParameterSet::new().bind(status_parameter(), "open".to_string()),
            current_controls(&request),
        )
        .unwrap();
    let basis = plan.basis_identity().clone();
    let result_buffer = world.application.result_buffer_observer();
    assert!(basis_is_live(&world, &basis));
    cancellation.cancel();

    let denial = world
        .application
        .execute_application_query_one_shot(plan)
        .err()
        .expect("cancelled request cannot enter provider execution");
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationOneShotDenialKind::Cancelled
    );
    assert!(!basis_is_live(&world, &basis));
    assert_eq!(result_buffer.observe().active_buffers(), 0);
    assert_eq!(result_buffer.observe().retained_bytes(), 0);
}

#[test]
fn revocation_after_admission_denies_under_the_execution_lock() {
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
            AccountStatus::reference(),
            "open".to_string(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let query = installed_query(&world);
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let plan = world
        .application
        .admit_application_query(
            &query,
            &access,
            ApplicationQueryParameterSet::new().bind(status_parameter(), "open".to_string()),
            current_controls(&request),
        )
        .unwrap();
    let basis = plan.basis_identity().clone();
    disable_mapping(&world, principal.mapping_entity_id());

    let denial = world
        .application
        .execute_application_query_one_shot(plan)
        .err()
        .expect("revocation must deny before reading the admitted basis");
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationOneShotDenialKind::StalePrincipal
    );
    assert!(!basis_is_live(&world, &basis));
}

#[test]
fn scope_authorization_revocation_after_admission_denies_under_the_execution_lock() {
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
            AccountStatus::reference(),
            "open".to_string(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let query = installed_query(&world);
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let plan = world
        .application
        .admit_application_query(
            &query,
            &access,
            ApplicationQueryParameterSet::new().bind(status_parameter(), "open".to_string()),
            current_controls(&request),
        )
        .unwrap();
    let basis = plan.basis_identity().clone();
    revoke_account_ownership(&world, account.entity_id());

    let denial = world
        .application
        .execute_application_query_one_shot(plan)
        .err()
        .expect("revoked scope authorization must deny before projection");
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationOneShotDenialKind::Authorization(
            crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind::StaleAuthorization,
        )
    );
    assert!(!basis_is_live(&world, &basis));
}

pub(super) fn basis_is_live(
    world: &super::super::fixture::AuthorizationWorld,
    basis: &worth_relational::facade::runtime::RelationalExecutionBasisIdentity,
) -> bool {
    let graph = world
        .application
        .runtime
        .primary_graph()
        .expect("test world publishes a primary graph");
    graph
        .integration_handle()
        .with_runtime_mut(|runtime| runtime.snapshots().execution_basis_is_live(basis))
}

pub(super) fn disable_mapping(
    world: &super::super::fixture::AuthorizationWorld,
    mapping_id: worth_relational::facade::identity::EntityId,
) {
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
        let mut transaction = runtime.begin_transaction(TransactionOptions::default());
        transaction.push_batch(WorkerIntentBatch::new("revoke-after-query-admission").push(
            MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                UpdateEntityFieldsIntent {
                    entity_id: mapping_id,
                    fields,
                },
            )),
        ));
        transaction.commit().unwrap();
    });
}

pub(super) fn revoke_account_ownership(
    world: &super::super::fixture::AuthorizationWorld,
    account: worth_relational::facade::identity::EntityId,
) {
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
        let snapshot = runtime.snapshots().snapshot();
        let relation = runtime
            .read_truth()
            .visible_relations_of_kind(relation_kind, snapshot.version_id)
            .into_iter()
            .find(|record| record.target == account)
            .expect("the admitted account has one ownership edge")
            .relation_id;
        runtime.snapshots().release_snapshot(&snapshot);
        let mut transaction = runtime.begin_transaction(TransactionOptions::default());
        transaction.push_batch(WorkerIntentBatch::new("revoke-query-account-owner").push(
            MutationIntent::Relation(RelationMutationIntent::Delete(DeleteRelationIntent {
                relation_id: relation,
            })),
        ));
        transaction.commit().unwrap();
        handle.ensure_primary_indexes_current(runtime).unwrap();
    });
}
