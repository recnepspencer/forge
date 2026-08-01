use std::cell::Cell;
use std::time::{Duration, Instant};

use worth_query_admission::facade::authenticated_principal::{
    WorthQueryCancellationSource, WorthQueryRequestScope,
};

use super::super::application_attempt::{authenticated_principal, resolved_account};
use super::super::fixture::{
    installed_authorization_world_on_branch, installed_capability_authorization_world,
    installed_capability_authorization_world_on_branch, live_scope, TouchAccountOperation,
};
use super::capability_progression::{admitted_capability_operation, time};
use crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind;
use worth_relational::facade::history::BranchId;

#[test]
fn commit_authorization_rechecks_cancellation_when_governed() {
    let mut world = installed_capability_authorization_world();
    world
        .application
        .script_authorization_time([time(100), time(100), time(100)]);
    let cancellation = WorthQueryCancellationSource::new();
    let request = WorthQueryRequestScope::new(
        Instant::now() + Duration::from_secs(60),
        cancellation.token(),
    );
    let principal = authenticated_principal(&world, &request);
    let mut admission = admitted_capability_operation(&world, &principal, &request);
    let (_, commit_basis) = admission
        .take_authorization_dependencies(world.application.authorization.bridge())
        .unwrap();
    let serialization = world
        .application
        .primary_provider
        .serialize_application_commit();
    let proof = world
        .application
        .authorize_application_commit(
            &admission,
            admission.graph_work_session(),
            &commit_basis,
            &serialization,
        )
        .unwrap();

    cancellation.cancel();

    let governed_action_ran = Cell::new(false);
    assert!(proof
        .govern((), |()| governed_action_ran.set(true))
        .is_err());
    assert!(!governed_action_ran.get());
}

#[test]
fn commit_basis_cannot_be_paired_with_a_different_admitted_operation() {
    let mut world = installed_capability_authorization_world();
    world
        .application
        .script_authorization_time([time(100), time(100), time(100), time(100)]);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let mut source = admitted_capability_operation(&world, &principal, &request);
    let target = admitted_capability_operation(&world, &principal, &request);
    let (_, source_basis) = source
        .take_authorization_dependencies(world.application.authorization.bridge())
        .unwrap();
    let serialization = world
        .application
        .primary_provider
        .serialize_application_commit();

    let Err(denial) = world.application.authorize_application_commit(
        &target,
        target.graph_work_session(),
        &source_basis,
        &serialization,
    ) else {
        panic!("a commit basis must remain bound to its originating admission");
    };
    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::InconsistentDecision
    );
}

#[test]
fn ordinary_commit_revalidation_uses_the_managed_session_branch() {
    let world = installed_authorization_world_on_branch(true, "tenant-blue");
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let operation = world
        .application
        .installed_schema()
        .installed_operation(TouchAccountOperation::reference())
        .unwrap();
    let mut admission = world
        .application
        .authorize_operation(
            &principal,
            &account,
            &operation,
            Default::default(),
            &request,
        )
        .unwrap();
    assert_eq!(
        admission
            .graph_work_session()
            .branch_affinity()
            .relational_branch(),
        &BranchId("tenant-blue".to_owned())
    );
    let (_, commit_basis) = admission
        .take_authorization_dependencies(world.application.authorization.bridge())
        .unwrap();
    let serialization = world
        .application
        .primary_provider
        .serialize_application_commit();

    world
        .application
        .authorize_application_commit(
            &admission,
            admission.graph_work_session(),
            &commit_basis,
            &serialization,
        )
        .expect("ordinary commit revalidation must stay on the session branch");
}

#[test]
fn capability_commit_revalidation_uses_the_managed_session_branch() {
    let mut world = installed_capability_authorization_world_on_branch("tenant-green");
    world
        .application
        .script_authorization_time([time(100), time(100), time(100)]);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let mut admission = admitted_capability_operation(&world, &principal, &request);
    assert_eq!(
        admission
            .graph_work_session()
            .branch_affinity()
            .relational_branch(),
        &BranchId("tenant-green".to_owned())
    );
    let (_, commit_basis) = admission
        .take_authorization_dependencies(world.application.authorization.bridge())
        .unwrap();
    let serialization = world
        .application
        .primary_provider
        .serialize_application_commit();

    world
        .application
        .authorize_application_commit(
            &admission,
            admission.graph_work_session(),
            &commit_basis,
            &serialization,
        )
        .expect("capability commit revalidation must stay on the session branch");
}

#[test]
fn commit_basis_cannot_cross_a_managed_session_or_branch() {
    let mut source_world = installed_capability_authorization_world_on_branch("tenant-source");
    let mut foreign_world = installed_capability_authorization_world_on_branch("tenant-foreign");
    source_world
        .application
        .script_authorization_time([time(100), time(100)]);
    foreign_world
        .application
        .script_authorization_time([time(100)]);
    let source_request = live_scope();
    let source_principal = authenticated_principal(&source_world, &source_request);
    let mut source =
        admitted_capability_operation(&source_world, &source_principal, &source_request);
    let foreign_request = live_scope();
    let foreign_principal = authenticated_principal(&foreign_world, &foreign_request);
    let foreign =
        admitted_capability_operation(&foreign_world, &foreign_principal, &foreign_request);
    let (_, source_basis) = source
        .take_authorization_dependencies(source_world.application.authorization.bridge())
        .unwrap();
    let serialization = source_world
        .application
        .primary_provider
        .serialize_application_commit();

    let Err(denial) = source_world.application.authorize_application_commit(
        &source,
        foreign.graph_work_session(),
        &source_basis,
        &serialization,
    ) else {
        panic!("commit authority cannot cross a managed session or typed branch");
    };
    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::InconsistentDecision
    );
}
