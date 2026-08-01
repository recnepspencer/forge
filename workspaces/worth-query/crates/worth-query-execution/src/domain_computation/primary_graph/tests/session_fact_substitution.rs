use super::application_attempt::{authenticated_principal, resolved_account};
use super::fixture::{
    installed_authorization_world, live_scope, AuthorizationWorld, TouchAccountOperation,
};
use crate::domain_computation::authorization::bind_operation_execution_authority;
use crate::domain_computation::authorization::WorthQueryRetainedAuthorizationDecisionFacts;
use crate::domain_computation::primary_graph::application_attempt::application_resource_request;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationSnapshotLease, WorthQueryOperationGraphReadCompletion,
};
use crate::domain_computation::provider_session::{
    record_ability_authorization_completion, record_operation_graph_read_completion,
    start_mutation_graph_work_session, WorthQueryGraphOwnerCompletionDenial,
    WorthQueryGraphWorkAccessContextAffinity, WorthQueryGraphWorkBasisAffinity,
    WorthQueryGraphWorkSessionAffinity, WorthQueryManagedGraphWorkSession,
    WorthQueryMutationGraphWorkLane,
};
use worth_query_admission::facade::graph_obligation::WorthQueryGraphWorkIntent;
use worth_query_admission::integration::{
    admit_application_operation_graph_work, require_selected_graph_work,
    select_installed_graph_obligations, WorthQueryAdmittedGraphWorkPlan,
};
use worth_query_declaration::facade::domain_computation::WorthQueryExecutionResourceRequest;

type MutationSession = WorthQueryManagedGraphWorkSession<
    WorthQueryMutationGraphWorkLane,
    WorthQueryApplicationSnapshotLease,
>;

#[test]
fn completion_from_another_session_is_rejected_and_both_sessions_release_exactly_once() {
    let world = installed_authorization_world(true);
    let (source_session, source_reservations) = touch_session(&world);
    let (mut target_session, target_reservations) = touch_session(&world);

    assert_ne!(source_session.identity(), target_session.identity());
    let foreign = WorthQueryOperationGraphReadCompletion::mint(
        *source_session.identity(),
        world
            .application
            .branch_affinity()
            .relational_branch()
            .clone(),
    );
    assert_eq!(
        record_operation_graph_read_completion(&mut target_session, foreign),
        Err(WorthQueryGraphOwnerCompletionDenial::ForeignSession)
    );

    assert_exact_release(source_session.abort_mutation(), source_reservations);
    assert_exact_release(target_session.abort_mutation(), target_reservations);
}

#[test]
fn ability_decisions_from_another_session_cannot_complete_authorization() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let operation = world
        .application
        .installed_schema()
        .installed_operation(TouchAccountOperation::reference())
        .unwrap();
    let admission = world
        .application
        .authorize_operation(
            &principal,
            &account,
            &operation,
            Default::default(),
            &request,
        )
        .unwrap();
    let WorthQueryRetainedAuthorizationDecisionFacts::Abilities { decisions, .. } =
        admission.authorization().unwrap()
    else {
        panic!("the touch operation fixture must retain ability decisions")
    };
    let (mut other_session, other_reservations) = touch_session(&world);

    assert_eq!(
        record_ability_authorization_completion(&mut other_session, decisions),
        Err(WorthQueryGraphOwnerCompletionDenial::ForeignSession)
    );

    assert_exact_release(other_session.abort_mutation(), other_reservations);
    drop(admission);
}

fn touch_session(world: &AuthorizationWorld) -> (MutationSession, usize) {
    let operation = world
        .application
        .installed_schema()
        .installed_operation(TouchAccountOperation::reference())
        .unwrap();
    open_session(
        world,
        admitted_plan(
            world,
            operation.contracts().obligations(),
            application_resource_request(operation.contracts()).unwrap(),
        ),
        operation.contracts().obligations().identity(),
        operation.authority_identity(),
    )
}

fn admitted_plan(
    world: &AuthorizationWorld,
    obligations: &worth_query_installation::facade::WorthQueryInstalledGraphObligationSet,
    request: WorthQueryExecutionResourceRequest,
) -> WorthQueryAdmittedGraphWorkPlan {
    let selected = select_installed_graph_obligations(
        obligations,
        WorthQueryGraphWorkIntent::application_operation_mutation(),
    )
    .unwrap();
    let required =
        require_selected_graph_work(selected, world.application.graph_admission_authority())
            .unwrap();
    admit_application_operation_graph_work(
        required,
        "session-substitution-operation",
        &request,
        world
            .application
            .primary_provider
            .application_resource_support(),
    )
    .unwrap()
}

fn open_session(
    world: &AuthorizationWorld,
    plan: WorthQueryAdmittedGraphWorkPlan,
    obligation_identity: &worth_query_installation::facade::WorthQueryInstalledGraphObligationSetIdentity,
    subject_authority: &str,
) -> (MutationSession, usize) {
    let reservations = plan.reservation_count();
    let request = live_scope();
    let principal = authenticated_principal(world, &request);
    let account = resolved_account(world, "open", &request);
    let graph = world.application.runtime.primary_graph().unwrap();
    let lease = WorthQueryApplicationSnapshotLease::acquire(
        graph.integration_handle(),
        graph.retain_layout(),
        world.application.branch_affinity().relational_branch(),
    )
    .unwrap();
    let branch = world.application.branch_affinity().clone();
    let basis = WorthQueryGraphWorkBasisAffinity::mutation(lease.snapshot(), &branch).unwrap();
    let affinity = WorthQueryGraphWorkSessionAffinity::new(
        &plan,
        world.application.runtime.authority_identity(),
        obligation_identity,
        subject_authority,
        principal.principal_entity_id(),
        WorthQueryGraphWorkAccessContextAffinity::entity(account.entity_id()),
        branch,
        basis,
        world.application.graph_work_provider_authority(),
    )
    .unwrap();
    let operation = world
        .application
        .installed_schema()
        .installed_operation(TouchAccountOperation::reference())
        .unwrap();
    let authority = bind_operation_execution_authority(
        &world.application,
        &operation,
        "session-substitution-operation",
        &lease,
        world.application.branch_affinity(),
    )
    .unwrap();
    (
        start_mutation_graph_work_session(
            plan,
            lease,
            affinity,
            &world.application.runtime,
            &authority,
        )
        .unwrap(),
        reservations,
    )
}

fn assert_exact_release(
    release: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionReleaseReceipt,
    reservations: usize,
) {
    assert!(release.basis_released());
    assert_eq!(
        release.capacity().released_reservation_count(),
        reservations
    );
}
