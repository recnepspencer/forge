use std::collections::BTreeMap;
use std::num::NonZeroUsize;
use std::time::{Duration, Instant};

use worth_query_admission::facade::authenticated_principal::{
    WorthQueryCancellationSource, WorthQueryRequestScope,
};
use worth_query_declaration::facade::application_query::ApplicationQueryParameterSet;
use worth_query_installation::facade::TypedApplicationValue;
use worth_relational::facade::transactions::{
    AspectFieldPatch, EntityMutationIntent, MutationIntent, UpdateEntityFieldsIntent,
    WorkerIntentBatch,
};

use super::{current_controls, installed_query};
use crate::domain_computation::primary_graph::{
    tests::fixture::{
        installed_authorization_world, status_parameter, AccountLabel, AccountStatus,
    },
    WorthQueryApplicationPinnedBasisDenialKind, WorthQueryApplicationQueryAccessContext,
    WorthQueryApplicationQueryAdmissionDenialKind, WorthQueryApplicationQueryBasisPosture,
    WorthQueryApplicationQueryControls, WorthQueryOperationAuthorizationDenialKind,
    WorthQueryPrincipalResolutionMode,
};

#[test]
fn pinned_basis_reads_old_truth_through_its_exact_index_generation() {
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
    let pinned = world
        .application
        .pin_current_application_query_basis(&request)
        .unwrap();
    change_account_label(&world, account.entity_id(), "changed");
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let pinned_plan = world
        .application
        .admit_application_query(
            &query,
            &access,
            ApplicationQueryParameterSet::new().bind(status_parameter(), "open".to_string()),
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
            ApplicationQueryParameterSet::new().bind(status_parameter(), "open".to_string()),
            current_controls(&request),
        )
        .unwrap();
    let current_result = world
        .application
        .execute_application_query_one_shot(current_plan)
        .unwrap();

    assert_eq!(pinned_result.rows()[0].label(), "primary");
    assert_eq!(current_result.rows()[0].label(), "changed");
    assert_eq!(
        pinned_result.receipt().basis_posture(),
        WorthQueryApplicationQueryBasisPosture::Pinned
    );
    assert_eq!(
        current_result.receipt().basis_posture(),
        WorthQueryApplicationQueryBasisPosture::Current
    );
    let pinned_generation = pinned_result
        .receipt()
        .predicate_index_generation()
        .expect("pinned query uses its exact retained index generation");
    let current_generation = current_result
        .receipt()
        .predicate_index_generation()
        .expect("current query uses its exact current index generation");
    assert_ne!(pinned_generation, current_generation);
    assert_eq!(pinned_result.receipt().fallback_count(), 0);
    assert_eq!(current_result.receipt().fallback_count(), 0);
    assert!(pinned_result.receipt().basis_released());
    assert!(current_result.receipt().basis_released());
}

#[test]
fn explicit_release_and_drop_both_release_the_pinned_lease() {
    let world = installed_authorization_world(true);
    let request = super::super::fixture::live_scope();
    let explicitly_released = world
        .application
        .pin_current_application_query_basis(&request)
        .unwrap();
    let explicit_identity = explicitly_released.identity().clone();

    let receipt = explicitly_released.release();
    assert!(receipt.released());
    assert!(!super::lifecycle::basis_is_live(&world, &explicit_identity));

    let dropped = world
        .application
        .pin_current_application_query_basis(&request)
        .unwrap();
    let dropped_identity = dropped.identity().clone();
    drop(dropped);
    assert!(!super::lifecycle::basis_is_live(&world, &dropped_identity));
}

#[test]
fn cancelled_request_cannot_mint_a_pinned_basis() {
    let world = installed_authorization_world(true);
    let cancellation = WorthQueryCancellationSource::new();
    cancellation.cancel();
    let request = WorthQueryRequestScope::new(
        Instant::now() + Duration::from_secs(60),
        cancellation.token(),
    );

    let denial = world
        .application
        .pin_current_application_query_basis(&request)
        .err()
        .expect("cancelled request cannot mint snapshot authority");
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationPinnedBasisDenialKind::Cancelled
    );
}

#[test]
fn foreign_runtime_rejects_and_releases_a_pinned_basis() {
    let source = installed_authorization_world(true);
    let foreign = installed_authorization_world(true);
    let request = super::super::fixture::live_scope();
    let pinned = source
        .application
        .pin_current_application_query_basis(&request)
        .unwrap();
    let pinned_identity = pinned.identity().clone();
    let external = foreign.authenticate("alice", Duration::from_secs(60), &request);
    let principal = foreign
        .application
        .resolve_authenticated_principal(
            &foreign.binding,
            external,
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let account = foreign
        .application
        .resolve_entity(
            AccountStatus::reference(),
            "open".to_string(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let query = installed_query(&foreign);
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);

    let denial = foreign
        .application
        .admit_application_query(
            &query,
            &access,
            ApplicationQueryParameterSet::new().bind(status_parameter(), "open".to_string()),
            WorthQueryApplicationQueryControls::pinned_one_shot(
                pinned,
                NonZeroUsize::new(10).unwrap(),
                NonZeroUsize::new(10_000).unwrap(),
                &request,
            ),
        )
        .err()
        .expect("foreign runtime cannot admit source snapshot authority");
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationQueryAdmissionDenialKind::ForeignBasis
    );
    assert!(!super::lifecycle::basis_is_live(&source, &pinned_identity));
}

#[test]
fn expired_pinned_basis_is_rejected_and_released() {
    let world = installed_authorization_world(true);
    // Q8.12. Capture the settled deadline *before* pinning, then pin under a
    // scope that cannot expire. The old form pinned under a 5ms scope and slept
    // 10ms, so a loaded machine failed the `.unwrap()` below on its own fixture
    // deadline instead of reaching the expiry under test. Nothing here waits,
    // and nothing here gets less true as the machine gets slower.
    let settled = Instant::now();
    let pinned = world
        .application
        .pin_current_application_query_basis(&super::super::fixture::live_scope())
        .unwrap()
        .with_deadline_settled_at(settled);
    let pinned_identity = pinned.identity().clone();
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

    let denial = world
        .application
        .admit_application_query(
            &query,
            &access,
            ApplicationQueryParameterSet::new().bind(status_parameter(), "open".to_string()),
            WorthQueryApplicationQueryControls::pinned_one_shot(
                pinned,
                NonZeroUsize::new(10).unwrap(),
                NonZeroUsize::new(10_000).unwrap(),
                &request,
            ),
        )
        .err()
        .expect("expired snapshot authority cannot be admitted");
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationQueryAdmissionDenialKind::ExpiredBasis
    );
    assert!(!super::lifecycle::basis_is_live(&world, &pinned_identity));
}

#[test]
fn pinned_data_basis_does_not_pin_authorization_truth() {
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
    let pinned = world
        .application
        .pin_current_application_query_basis(&request)
        .unwrap();
    let pinned_identity = pinned.identity().clone();
    super::lifecycle::revoke_account_ownership(&world, account.entity_id());
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);

    let denial = world
        .application
        .admit_application_query(
            &query,
            &access,
            ApplicationQueryParameterSet::new().bind(status_parameter(), "open".to_string()),
            WorthQueryApplicationQueryControls::pinned_one_shot(
                pinned,
                NonZeroUsize::new(10).unwrap(),
                NonZeroUsize::new(10_000).unwrap(),
                &request,
            ),
        )
        .err()
        .expect("pinning data must not retain revoked authorization");
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationQueryAdmissionDenialKind::Authorization(
            WorthQueryOperationAuthorizationDenialKind::PermissionDenied
        )
    );
    assert!(!super::lifecycle::basis_is_live(&world, &pinned_identity));
}

fn change_account_label(
    world: &super::super::fixture::AuthorizationWorld,
    account: worth_relational::facade::identity::EntityId,
    new_label: &str,
) {
    let graph = world
        .application
        .runtime
        .primary_graph()
        .expect("test world publishes a primary graph");
    let label_field = AccountLabel::reference();
    let locator = graph
        .layout
        .field_locator(
            label_field.entity(),
            label_field.aspect(),
            label_field.field(),
        )
        .expect("account label is installed")
        .clone();
    let handle = graph.integration_handle();
    handle.with_runtime_mut(|runtime| {
        let fields = AspectFieldPatch::from(BTreeMap::from([(
            locator,
            new_label.to_string().into_foundational_value(),
        )]));
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
            .push_batch(WorkerIntentBatch::new("change-label-after-basis-pin").push(
                MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                    UpdateEntityFieldsIntent {
                        entity_id: account,
                        fields,
                    },
                )),
            ))
            .expect("test staging stays within configured resource budgets");
        let committed = transaction.commit(runtime).unwrap();
        super::super::fixture::release_test_commit_snapshot(runtime, &committed);
        handle.ensure_primary_indexes_current(runtime).unwrap();
    });
}
