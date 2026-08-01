use std::time::{Duration, Instant};

use worth_query_admission::facade::authenticated_principal::{
    WorthQueryCancellationSource, WorthQueryRequestScope,
};
use worth_query_declaration::facade::application_query::ApplicationQueryParameterSet;

use super::{current_controls, installed_query};
use crate::domain_computation::primary_graph::{
    provider::PRIMARY_GRAPH_CONCURRENT_ATTEMPT_LIMIT,
    tests::fixture::{installed_authorization_world, status_parameter, AccountStatus},
    WorthQueryApplicationOneShotDenialKind, WorthQueryApplicationQueryAccessContext,
    WorthQueryApplicationQueryAdmissionDenialKind, WorthQueryPrincipalResolutionMode,
};

#[test]
fn cancellation_restores_the_exact_graph_capacity_baseline() {
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
        .expect("the fixture principal is current");
    let account = world
        .application
        .resolve_entity(
            AccountStatus::reference(),
            "open".to_owned(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .expect("the fixture scope is current");
    let query = installed_query(&world);
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let plan = world
        .application
        .admit_application_query(
            &query,
            &access,
            ApplicationQueryParameterSet::new().bind(status_parameter(), "open".to_owned()),
            current_controls(&request),
        )
        .expect("capacity below the fixed ceiling must admit");
    let basis = plan.basis_identity().clone();
    let result_buffer = world.application.result_buffer_observer();

    cancellation.cancel();
    let denial = world
        .application
        .execute_application_query_one_shot(plan)
        .err()
        .expect("cancelled query cannot enter provider execution");

    assert_eq!(
        denial.kind(),
        WorthQueryApplicationOneShotDenialKind::Cancelled
    );
    assert!(!super::lifecycle::basis_is_live(&world, &basis));
    assert_eq!(result_buffer.observe().active_buffers(), 0);
    assert_eq!(result_buffer.observe().retained_bytes(), 0);
    assert_full_capacity_available(&world);
}

#[test]
fn foreign_plan_denial_restores_the_exact_graph_capacity_baseline() {
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
        .expect("the fixture principal is current");
    let account = world
        .application
        .resolve_entity(
            AccountStatus::reference(),
            "open".to_owned(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .expect("the fixture scope is current");
    let query = installed_query(&world);
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let plan = world
        .application
        .admit_application_query(
            &query,
            &access,
            ApplicationQueryParameterSet::new().bind(status_parameter(), "open".to_owned()),
            current_controls(&request),
        )
        .expect("capacity below the fixed ceiling must admit");
    let basis = plan.basis_identity().clone();

    let denial = foreign
        .application
        .execute_application_query_one_shot(plan)
        .err()
        .expect("foreign runtime cannot consume a sealed plan");

    assert_eq!(
        denial.kind(),
        WorthQueryApplicationOneShotDenialKind::ForeignPlan
    );
    assert!(!super::lifecycle::basis_is_live(&world, &basis));
    assert_full_capacity_available(&world);
}

fn assert_full_capacity_available(world: &super::super::fixture::AuthorizationWorld) {
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
        .expect("the fixture principal is current");
    let account = world
        .application
        .resolve_entity(
            AccountStatus::reference(),
            "open".to_owned(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .expect("the fixture scope is current");
    let query = installed_query(world);
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let admit = || {
        world.application.admit_application_query(
            &query,
            &access,
            ApplicationQueryParameterSet::new().bind(status_parameter(), "open".to_owned()),
            current_controls(&request),
        )
    };
    let mut admitted = Vec::with_capacity(PRIMARY_GRAPH_CONCURRENT_ATTEMPT_LIMIT);
    for _ in 0..PRIMARY_GRAPH_CONCURRENT_ATTEMPT_LIMIT {
        admitted.push(admit().expect("capacity below the fixed ceiling must admit"));
    }

    let denial = admit()
        .err()
        .expect("the first request beyond the fixed ceiling must saturate");
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationQueryAdmissionDenialKind::RuntimeSupportUnavailable
    );

    cancellation.cancel();
    for plan in admitted {
        let denial = world
            .application
            .execute_application_query_one_shot(plan)
            .err()
            .expect("probe plans are cancelled to release every reservation");
        assert_eq!(
            denial.kind(),
            WorthQueryApplicationOneShotDenialKind::Cancelled
        );
    }
}
