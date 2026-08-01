use super::application_attempt::{authenticated_principal, resolved_account};
use super::fixture::{
    installed_authorization_world, live_scope, AuthorizationWorld, GovernedAccountSummaryQuery,
    MultiTouchOperation, TouchAccountOperation,
};
use crate::domain_computation::authorization::bind_operation_execution_authority;
use crate::domain_computation::primary_graph::application_attempt::application_resource_request;
use crate::domain_computation::primary_graph::WorthQueryApplicationSnapshotLease;
use crate::domain_computation::provider_session::{
    start_mutation_graph_work_session, start_read_graph_work_session,
    WorthQueryGraphWorkAccessContextAffinity, WorthQueryGraphWorkBasisAffinity,
    WorthQueryGraphWorkSessionAffinity, WorthQueryGraphWorkSessionStartDenial,
};
use worth_query_admission::facade::graph_obligation::WorthQueryGraphWorkIntent;
use worth_query_admission::integration::{
    admit_application_operation_graph_work, require_selected_graph_work,
    select_installed_graph_obligations,
};
use worth_query_installation::facade::{
    WorthQueryInstalledGraphAuthorizationRequirement, WorthQueryInstalledGraphObligationKind,
    WorthQueryInstalledGraphObligationResourcePosture,
};

#[test]
fn governed_query_installs_its_disclosure_capability_obligation() {
    let world = installed_authorization_world(true);
    let query = world
        .application
        .installed_schema()
        .application_query(GovernedAccountSummaryQuery::reference())
        .unwrap();
    let authorization = query
        .obligations()
        .rows()
        .iter()
        .filter(|row| {
            row.kind() == WorthQueryInstalledGraphObligationKind::AuthorizationObservation
        })
        .collect::<Vec<_>>();

    assert_eq!(authorization.len(), 1);
    assert!(authorization.iter().any(|row| matches!(
        row.authorization_requirement(),
        Some(WorthQueryInstalledGraphAuthorizationRequirement::Capabilities(requirements))
            if requirements.len() == 1
    )));
    assert!(query.obligations().rows().iter().all(|row| matches!(
        row.resource_posture(),
        WorthQueryInstalledGraphObligationResourcePosture::ApplicationQuery {
            maximum_authorization_facts: 2,
            ..
        }
    )));
}

#[test]
fn installed_mutation_uses_the_common_capacity_reserved_graph_work_plan() {
    let world = installed_authorization_world(true);
    let operation = world
        .application
        .installed_schema()
        .installed_operation(TouchAccountOperation::reference())
        .unwrap();
    let selected = select_installed_graph_obligations(
        operation.contracts().obligations(),
        WorthQueryGraphWorkIntent::application_operation_mutation(),
    )
    .unwrap();
    let required =
        require_selected_graph_work(selected, world.application.graph_admission_authority())
            .unwrap();
    let request = application_resource_request(operation.contracts()).unwrap();
    let plan = admit_application_operation_graph_work(
        required,
        "installed-mutation-graph-work",
        &request,
        world
            .application
            .primary_provider
            .application_resource_support(),
    )
    .unwrap();

    assert_eq!(
        plan.obligation_identity(),
        operation.contracts().obligations().identity()
    );
    assert!(plan.execution_resources().is_some());
    let reservation_count = plan.reservation_count();
    assert!(reservation_count >= 1);
    assert_eq!(plan.canonical_work().digest_derivations(), 1);
    let receipt = plan.release();
    assert_eq!(receipt.released_reservation_count(), reservation_count);
}

#[test]
fn affinity_for_one_installation_cannot_open_a_foreign_admitted_plan() {
    let owner = installed_authorization_world(true);
    let foreign = installed_authorization_world(true);
    let owner_plan = admitted_touch_plan(&owner);
    let foreign_plan = admitted_touch_plan(&foreign);
    let (lease, affinity) = mutation_affinity(&owner, &owner_plan, None);
    let authority = operation_execution_authority(&owner, &lease, "admitted-touch-plan");

    let denial = match start_mutation_graph_work_session(
        foreign_plan,
        lease,
        affinity,
        &owner.application.runtime,
        &authority,
    ) {
        Ok(_) => panic!("a foreign admitted plan must not enter the bound session affinity"),
        Err(denial) => denial,
    };
    assert_eq!(
        denial,
        WorthQueryGraphWorkSessionStartDenial::BindingMismatch
    );
}

#[test]
fn affinity_for_one_plan_cannot_open_another_plan_in_the_same_binding() {
    let world = installed_authorization_world(true);
    let owner_plan = admitted_touch_plan(&world);
    let other = world
        .application
        .installed_schema()
        .installed_operation(MultiTouchOperation::reference())
        .unwrap();
    let selected = select_installed_graph_obligations(
        other.contracts().obligations(),
        WorthQueryGraphWorkIntent::application_operation_mutation(),
    )
    .unwrap();
    let required =
        require_selected_graph_work(selected, world.application.graph_admission_authority())
            .unwrap();
    let other_plan = admit_application_operation_graph_work(
        required,
        "foreign-plan-same-binding",
        &application_resource_request(other.contracts()).unwrap(),
        world
            .application
            .primary_provider
            .application_resource_support(),
    )
    .unwrap();
    let (lease, affinity) = mutation_affinity(&world, &owner_plan, None);
    let authority = operation_execution_authority(&world, &lease, "admitted-touch-plan");

    let denial = match start_mutation_graph_work_session(
        other_plan,
        lease,
        affinity,
        &world.application.runtime,
        &authority,
    ) {
        Ok(_) => panic!("another admitted plan must not enter the bound session affinity"),
        Err(denial) => denial,
    };
    assert_eq!(denial, WorthQueryGraphWorkSessionStartDenial::PlanMismatch);
}

#[test]
fn another_installed_operation_cannot_satisfy_plan_subject_affinity() {
    let world = installed_authorization_world(true);
    let plan = admitted_touch_plan(&world);
    let other = world
        .application
        .installed_schema()
        .installed_operation(MultiTouchOperation::reference())
        .unwrap();
    let (principal, scope) = installed_subject_ids(&world);
    let (lease, branch, basis) = mutation_basis(&world);
    let denial = match WorthQueryGraphWorkSessionAffinity::new(
        &plan,
        world.application.runtime.authority_identity(),
        other.contracts().obligations().identity(),
        other.authority_identity(),
        principal,
        WorthQueryGraphWorkAccessContextAffinity::entity(scope),
        branch,
        basis,
        world.application.graph_work_provider_authority(),
    ) {
        Ok(_) => panic!("another operation must not satisfy the plan's obligation affinity"),
        Err(denial) => denial,
    };
    drop(lease);
    assert_eq!(
        denial,
        WorthQueryGraphWorkSessionStartDenial::ObligationMismatch
    );
}

#[test]
fn foreign_provider_authority_cannot_enter_plan_affinity() {
    let owner = installed_authorization_world(true);
    let foreign = installed_authorization_world(true);
    let plan = admitted_touch_plan(&owner);
    let operation = touch_operation(&owner);
    let (principal, scope) = installed_subject_ids(&owner);
    let (lease, branch, basis) = mutation_basis(&owner);
    let denial = match WorthQueryGraphWorkSessionAffinity::new(
        &plan,
        owner.application.runtime.authority_identity(),
        operation.contracts().obligations().identity(),
        operation.authority_identity(),
        principal,
        WorthQueryGraphWorkAccessContextAffinity::entity(scope),
        branch,
        basis,
        foreign.application.graph_work_provider_authority(),
    ) {
        Ok(_) => panic!("a foreign provider must not satisfy the plan's provider affinity"),
        Err(denial) => denial,
    };
    drop(lease);
    assert_eq!(
        denial,
        WorthQueryGraphWorkSessionStartDenial::ProviderMismatch
    );
}

#[test]
fn mutation_plan_cannot_start_the_read_terminal_lane() {
    let world = installed_authorization_world(true);
    let plan = admitted_touch_plan(&world);
    let (lease, affinity) = mutation_affinity(&world, &plan, None);

    let denial = match start_read_graph_work_session(plan, lease, affinity) {
        Ok(_) => panic!("mutation authority must not enter the read lane"),
        Err(denial) => denial,
    };
    assert_eq!(denial, WorthQueryGraphWorkSessionStartDenial::WrongLane);
}

fn admitted_touch_plan(
    world: &AuthorizationWorld,
) -> worth_query_admission::integration::WorthQueryAdmittedGraphWorkPlan {
    let operation = touch_operation(world);
    let selected = select_installed_graph_obligations(
        operation.contracts().obligations(),
        WorthQueryGraphWorkIntent::application_operation_mutation(),
    )
    .unwrap();
    let required =
        require_selected_graph_work(selected, world.application.graph_admission_authority())
            .unwrap();
    admit_application_operation_graph_work(
        required,
        "admitted-touch-plan",
        &application_resource_request(operation.contracts()).unwrap(),
        world
            .application
            .primary_provider
            .application_resource_support(),
    )
    .unwrap()
}

fn touch_operation(
    world: &AuthorizationWorld,
) -> worth_query_installation::facade::WorthQueryInstalledApplicationOperation<
    super::fixture::IdentityExecutionSchema,
    TouchAccountOperation,
    super::fixture::TouchAccountInput,
> {
    world
        .application
        .installed_schema()
        .installed_operation(TouchAccountOperation::reference())
        .unwrap()
}

fn mutation_affinity(
    world: &AuthorizationWorld,
    plan: &worth_query_admission::integration::WorthQueryAdmittedGraphWorkPlan,
    provider: Option<
        &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
    >,
) -> (
    WorthQueryApplicationSnapshotLease,
    WorthQueryGraphWorkSessionAffinity,
) {
    let operation = touch_operation(world);
    let (principal, scope) = installed_subject_ids(world);
    let (lease, branch, basis) = mutation_basis(world);
    let affinity = WorthQueryGraphWorkSessionAffinity::new(
        plan,
        world.application.runtime.authority_identity(),
        operation.contracts().obligations().identity(),
        operation.authority_identity(),
        principal,
        WorthQueryGraphWorkAccessContextAffinity::entity(scope),
        branch,
        basis,
        provider.unwrap_or_else(|| world.application.graph_work_provider_authority()),
    )
    .unwrap();
    (lease, affinity)
}

fn installed_subject_ids(
    world: &AuthorizationWorld,
) -> (
    worth_relational::facade::identity::EntityId,
    worth_relational::facade::identity::EntityId,
) {
    let request = live_scope();
    let principal = authenticated_principal(world, &request);
    let account = resolved_account(world, "open", &request);
    (principal.principal_entity_id(), account.entity_id())
}

fn mutation_basis(
    world: &AuthorizationWorld,
) -> (
    WorthQueryApplicationSnapshotLease,
    crate::domain_computation::provider_session::WorthQueryGraphWorkBranchAffinity,
    WorthQueryGraphWorkBasisAffinity,
) {
    let graph = world.application.runtime.primary_graph().unwrap();
    let lease = WorthQueryApplicationSnapshotLease::acquire(
        graph.integration_handle(),
        graph.retain_layout(),
        world.application.branch_affinity().relational_branch(),
    )
    .unwrap();
    let branch = world.application.branch_affinity().clone();
    let basis = WorthQueryGraphWorkBasisAffinity::mutation(lease.snapshot(), &branch).unwrap();
    (lease, branch, basis)
}

fn operation_execution_authority(
    world: &AuthorizationWorld,
    lease: &WorthQueryApplicationSnapshotLease,
    resource_binding_identity: &str,
) -> crate::domain_computation::WorthQueryExecutionBoundOperationAuthority {
    bind_operation_execution_authority(
        &world.application,
        &touch_operation(world),
        resource_binding_identity,
        lease,
        world.application.branch_affinity(),
    )
    .unwrap()
}
