use std::num::NonZeroUsize;
use std::time::{Duration, Instant};

use worth_query_admission::facade::authenticated_principal::{
    WorthQueryCancellationSource, WorthQueryRequestScope,
};
use worth_runtime_bridge::facade::{TruthBranchIdentity, TruthCommitIdentity};

use crate::domain_computation::primary_graph::WorthQueryApplicationPreviewSessionDenialKind;

use super::lane_parity::{branch_head, parameters, principal_and_account};
use super::lifecycle::{disable_mapping, revoke_account_ownership};
use super::*;

#[test]
fn foreign_preview_session_mints_no_query_basis() {
    let owner = installed_authorization_world(true);
    let foreign = installed_authorization_world(true);
    let request = live_scope();
    let (_principal, _account) = principal_and_account(&owner, &request);
    let session = owner
        .application
        .open_application_preview_session(&request)
        .unwrap();
    let observer = foreign.application.application_query_basis_observer();
    let before = observer.observe();

    let denial = foreign
        .application
        .admit_application_preview_basis(&session, &request)
        .err()
        .expect("foreign preview session must not mint Query authority");

    assert_eq!(
        denial.kind(),
        WorthQueryApplicationQueryAdmissionDenialKind::ForeignPreviewSession
    );
    assert_eq!(observer.observe().active(), before.active());
    assert!(session.discard().unwrap().discarded());
}

#[test]
fn cancelled_request_opens_no_preview_session() {
    let world = installed_authorization_world(true);
    let cancellation = WorthQueryCancellationSource::new();
    cancellation.cancel();
    let request = WorthQueryRequestScope::new(
        Instant::now() + Duration::from_secs(60),
        cancellation.token(),
    );

    let denial = world
        .application
        .open_application_preview_session(&request)
        .err()
        .expect("a cancelled request must open no preview session");

    assert_eq!(
        denial.kind(),
        WorthQueryApplicationPreviewSessionDenialKind::Cancelled
    );
}

#[test]
fn cancelled_historical_request_mints_no_query_basis() {
    let world = installed_authorization_world(true);
    let cancellation = WorthQueryCancellationSource::new();
    cancellation.cancel();
    let request = WorthQueryRequestScope::new(
        Instant::now() + Duration::from_secs(60),
        cancellation.token(),
    );
    let head = branch_head(&world, "main");
    let observer = world.application.application_query_basis_observer();
    let before = observer.observe();

    let denial = world
        .application
        .admit_application_historical_basis(
            crate::domain_computation::primary_graph::WorthQueryApplicationHistoricalRead::at_commit(
                TruthBranchIdentity::from_relational_branch_id("main"),
                TruthCommitIdentity::from_relational_commit_id(head.commit_id.0),
            ),
            &request,
        )
        .err()
        .expect("cancelled historical request must not mint Query authority");

    assert_eq!(
        denial.kind(),
        WorthQueryApplicationQueryAdmissionDenialKind::Cancelled
    );
    assert_eq!(observer.observe().active(), before.active());
}

#[test]
fn unsupported_historical_lane_releases_its_prepared_basis() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let (principal, account) = principal_and_account(&world, &request);
    let observer = world.application.application_query_basis_observer();
    let before = observer.observe();
    let basis = historical_basis(&world, &request);
    assert_eq!(observer.observe().active(), before.active() + 1);
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let query = installed_governed_query(&world);
    let governed_parameters =
        ApplicationQueryParameterSet::new().bind(status_parameter(), "open".to_string());

    let denial = world
        .application
        .admit_application_query(
            &query,
            &access,
            governed_parameters,
            WorthQueryApplicationQueryControls::historical(
                basis,
                NonZeroUsize::new(10).unwrap(),
                NonZeroUsize::new(10_000).unwrap(),
                &request,
            ),
        )
        .err()
        .expect("one-shot-only query must deny the historical lane");

    assert_eq!(
        denial.kind(),
        WorthQueryApplicationQueryAdmissionDenialKind::LaneUnsupported
    );
    assert_eq!(observer.observe().active(), before.active());
}

#[test]
fn abandoned_historical_plan_releases_its_basis() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let (principal, account) = principal_and_account(&world, &request);
    let observer = world.application.application_query_basis_observer();
    let before = observer.observe();
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let query = installed_query(&world);
    let plan = world
        .application
        .admit_application_query(
            &query,
            &access,
            parameters(),
            WorthQueryApplicationQueryControls::historical(
                historical_basis(&world, &request),
                NonZeroUsize::new(10).unwrap(),
                NonZeroUsize::new(10_000).unwrap(),
                &request,
            ),
        )
        .unwrap();
    assert_eq!(observer.observe().active(), before.active() + 1);

    drop(plan);

    assert_eq!(observer.observe().active(), before.active());
}

#[test]
fn historical_execution_revalidates_principal_after_plan_admission() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let (principal, account) = principal_and_account(&world, &request);
    let observer = world.application.application_query_basis_observer();
    let before = observer.observe();
    let query = installed_query(&world);
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let plan = world
        .application
        .admit_application_query(
            &query,
            &access,
            parameters(),
            WorthQueryApplicationQueryControls::historical(
                historical_basis(&world, &request),
                NonZeroUsize::new(10).unwrap(),
                NonZeroUsize::new(10_000).unwrap(),
                &request,
            ),
        )
        .unwrap();
    disable_mapping(&world, principal.mapping_entity_id());

    let denial = world
        .application
        .execute_application_query_historical(plan)
        .err()
        .expect("historical execution must revalidate its principal");

    assert_eq!(
        denial.kind(),
        crate::domain_computation::primary_graph::WorthQueryBoundedLaneDenialKind::StalePrincipal
    );
    assert_eq!(observer.observe().active(), before.active());
}

#[test]
fn preview_execution_revalidates_scope_ability_after_plan_admission() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let (principal, account) = principal_and_account(&world, &request);
    let session = world
        .application
        .open_application_preview_session(&request)
        .unwrap();
    let observer = world.application.application_query_basis_observer();
    let before = observer.observe();
    let basis = world
        .application
        .admit_application_preview_basis(&session, &request)
        .unwrap();
    let query = installed_query(&world);
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let plan = world
        .application
        .admit_application_query(
            &query,
            &access,
            parameters(),
            WorthQueryApplicationQueryControls::preview(
                basis,
                NonZeroUsize::new(10).unwrap(),
                NonZeroUsize::new(10_000).unwrap(),
                &request,
            ),
        )
        .unwrap();
    revoke_account_ownership(&world, account.entity_id());

    let denial = world
        .application
        .execute_application_query_preview(plan)
        .err()
        .expect("preview execution must revalidate its scope ability");

    assert_eq!(
        denial.kind(),
        crate::domain_computation::primary_graph::WorthQueryBoundedLaneDenialKind::Authorization(
            crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind::PermissionDenied,
        )
    );
    assert_eq!(observer.observe().active(), before.active());
    assert!(session.discard().unwrap().discarded());
}

#[test]
fn discarded_preview_session_invalidates_an_admitted_plan() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let (principal, account) = principal_and_account(&world, &request);
    let session = world
        .application
        .open_application_preview_session(&request)
        .unwrap();
    let observer = world.application.application_query_basis_observer();
    let before = observer.observe();
    let basis = world
        .application
        .admit_application_preview_basis(&session, &request)
        .unwrap();
    let query = installed_query(&world);
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let plan = world
        .application
        .admit_application_query(
            &query,
            &access,
            parameters(),
            WorthQueryApplicationQueryControls::preview(
                basis,
                NonZeroUsize::new(10).unwrap(),
                NonZeroUsize::new(10_000).unwrap(),
                &request,
            ),
        )
        .unwrap();

    session
        .discard()
        .expect("the active preview session should discard");
    let denial = world
        .application
        .execute_application_query_preview(plan)
        .err()
        .expect("a discarded preview session must invalidate its admitted plan");

    assert_eq!(
        denial.kind(),
        crate::domain_computation::primary_graph::WorthQueryBoundedLaneDenialKind::StalePreviewSession
    );
    assert_eq!(observer.observe().active(), before.active());
}

#[test]
fn cancellation_after_preview_admission_releases_basis_without_closing_session() {
    let world = installed_authorization_world(true);
    let cancellation = WorthQueryCancellationSource::new();
    let request = WorthQueryRequestScope::new(
        Instant::now() + Duration::from_secs(60),
        cancellation.token(),
    );
    let (principal, account) = principal_and_account(&world, &request);
    let session = world
        .application
        .open_application_preview_session(&request)
        .unwrap();
    let observer = world.application.application_query_basis_observer();
    let before = observer.observe();
    let basis = world
        .application
        .admit_application_preview_basis(&session, &request)
        .unwrap();
    let query = installed_query(&world);
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let plan = world
        .application
        .admit_application_query(
            &query,
            &access,
            parameters(),
            WorthQueryApplicationQueryControls::preview(
                basis,
                NonZeroUsize::new(10).unwrap(),
                NonZeroUsize::new(10_000).unwrap(),
                &request,
            ),
        )
        .unwrap();

    cancellation.cancel();
    let denial = world
        .application
        .execute_application_query_preview(plan)
        .err()
        .expect("cancelled preview execution must deny");

    assert_eq!(
        denial.kind(),
        crate::domain_computation::primary_graph::WorthQueryBoundedLaneDenialKind::Cancelled
    );
    assert_eq!(observer.observe().active(), before.active());
    assert!(session.discard().unwrap().discarded());
}

fn historical_basis(
    world: &super::super::fixture::AuthorizationWorld,
    request: &WorthQueryRequestScope,
) -> crate::domain_computation::primary_graph::WorthQueryApplicationHistoricalBasis<
    super::super::fixture::IdentityExecutionSchema,
> {
    let head = branch_head(world, "main");
    world
        .application
        .admit_application_historical_basis(
            crate::domain_computation::primary_graph::WorthQueryApplicationHistoricalRead::at_commit(
                TruthBranchIdentity::from_relational_branch_id("main"),
                TruthCommitIdentity::from_relational_commit_id(head.commit_id.0),
            ),
            request,
        )
        .unwrap()
}
