use worth_query_installation::facade::ApplicationSchema;

use super::{terminal_for, WorthQueryProgressedApplicationCommit};
use crate::domain_computation::primary_graph::application_attempt::provider_execution::phase::{
    finish_application_commit, prepare_application_commit, progress_application_commit,
    start_managed_application_commit, WorthQueryApplicationCommitPreparation,
    WorthQueryApplicationCommitPreparationRequest, WorthQueryRunningApplicationCommit,
};
use crate::domain_computation::primary_graph::tests::application_attempt::preimage_evidence::{
    retained_status_program, RetentionMutationBreadth,
};
use crate::domain_computation::primary_graph::tests::application_attempt::{
    authenticated_principal, idempotency, resolved_account,
};
use crate::domain_computation::primary_graph::tests::fixture::{
    installed_authorization_world, live_scope, Account, AuthorizationWorld,
    ExactStatusRetentionInput, ExactStatusRetentionOperation, IdentityExecutionSchema,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationCommitOutcome, WorthQueryApplicationCommitTerminalKind,
    WorthQueryApplicationEffectProgram, WorthQueryApplicationIdempotencyBinding,
    WorthQueryPrimaryGraphApplicationRuntime,
};
use crate::domain_computation::provider_session::WorthQueryMutationGraphWorkCompletion;

type RetainedProgram = WorthQueryApplicationEffectProgram<
    IdentityExecutionSchema,
    ExactStatusRetentionOperation,
    ExactStatusRetentionInput,
    Account,
>;

#[test]
fn genuine_same_branch_sessions_reject_crossed_cleanup_completions() {
    let crossed = installed_authorization_world(true);
    let (left, right) = equivalent_programs(&crossed, "crossed-equivalent");
    let binding = idempotency(131, 132);
    let left = start(&crossed.application, left, binding);
    let right = start(&crossed.application, right, binding);
    let left = progress_application_commit(&crossed.application, left);
    let right = progress_application_commit(&crossed.application, right);
    let (left_outcome, left_completion) = complete_cleanup(&crossed.application, left);
    let (right_outcome, right_completion) = complete_cleanup(&crossed.application, right);

    assert!(left_outcome.finish(right_completion).is_none());
    assert!(right_outcome.finish(left_completion).is_none());

    assert_rightful_same_branch_peers();
}

#[test]
fn genuinely_interleaved_equivalent_sessions_validate_retry_cleanup_separately() {
    let world = installed_authorization_world(true);
    let (left, right) = equivalent_programs(&world, "racing-equivalent");
    let binding = idempotency(139, 140);
    let left = start(&world.application, left, binding);
    let right = start(&world.application, right, binding);
    let left = progress_application_commit(&world.application, left);
    let right = progress_application_commit(&world.application, right);
    let left = finish_application_commit(&world.application, left);
    let right = finish_application_commit(&world.application, right);

    let (executed, recovered) = match (left, right) {
        (
            WorthQueryApplicationCommitOutcome::Committed(executed),
            WorthQueryApplicationCommitOutcome::AlreadyCommitted(recovered),
        )
        | (
            WorthQueryApplicationCommitOutcome::AlreadyCommitted(recovered),
            WorthQueryApplicationCommitOutcome::Committed(executed),
        ) => (executed, recovered),
        unexpected => panic!("expected one executed and one recovered commit: {unexpected:?}"),
    };
    assert!(executed.is_same_authoritative_commit(&recovered));
    assert_eq!(executed.retained_preimage(), recovered.retained_preimage());
    assert_eq!(executed.dispatch_outbox(), recovered.dispatch_outbox());
    assert_eq!(executed.terminal().attempt_resources_released(), Some(true));
    assert_eq!(
        recovered.terminal().attempt_resources_released(),
        Some(true)
    );
    assert_eq!(
        executed.terminal().kind(),
        WorthQueryApplicationCommitTerminalKind::Executed
    );
    assert_eq!(
        recovered.terminal().kind(),
        WorthQueryApplicationCommitTerminalKind::Recovered
    );
}

fn assert_rightful_same_branch_peers() {
    let world = installed_authorization_world(true);
    let (left, right) = independent_programs(&world, "left-rightful", "right-rightful");
    let WorthQueryApplicationCommitOutcome::Committed(left) = world
        .application
        .compare_and_commit_application(left, idempotency(135, 136))
    else {
        panic!("left owner must complete with its own provider session")
    };
    let WorthQueryApplicationCommitOutcome::Committed(right) = world
        .application
        .compare_and_commit_application(right, idempotency(137, 138))
    else {
        panic!("right owner must complete with its own provider session")
    };
    assert_ne!(left.commit_reference(), right.commit_reference());
    assert!(!left.same_provider_session_for_test(&right));
    assert_eq!(left.terminal().attempt_resources_released(), Some(true));
    assert_eq!(right.terminal().attempt_resources_released(), Some(true));
    assert!(left.dispatch_outbox().is_some());
    assert!(right.dispatch_outbox().is_some());
    assert!(left.retained_preimage().is_some());
    assert!(right.retained_preimage().is_some());
}

fn start(
    application: &WorthQueryPrimaryGraphApplicationRuntime<IdentityExecutionSchema>,
    program: RetainedProgram,
    idempotency: WorthQueryApplicationIdempotencyBinding,
) -> WorthQueryRunningApplicationCommit<
    IdentityExecutionSchema,
    ExactStatusRetentionOperation,
    ExactStatusRetentionInput,
    Account,
> {
    let prepared = prepare_application_commit(
        application,
        WorthQueryApplicationCommitPreparationRequest::new(program, idempotency, None, None),
    );
    let WorthQueryApplicationCommitPreparation::Ready(prepared) = prepared else {
        panic!("association fixture must reach ordinary prepared posture")
    };
    start_managed_application_commit(application, prepared)
        .unwrap_or_else(|outcome| panic!("association fixture must start: {outcome:?}"))
}

fn complete_cleanup<Schema>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    progressed: WorthQueryProgressedApplicationCommit,
) -> (
    super::super::super::outcome::WorthQueryProviderProgressionOutcome,
    WorthQueryMutationGraphWorkCompletion,
)
where
    Schema: ApplicationSchema,
{
    let WorthQueryProgressedApplicationCommit {
        outcome,
        lease,
        running,
        cleanup,
    } = progressed;
    let terminal = terminal_for(&outcome);
    let snapshot_released = lease.release();
    application
        .primary_provider
        .observe_managed_application_cleanup();
    let completion = cleanup
        .finish(running, terminal, snapshot_released)
        .expect("genuine owner cleanup must complete");
    (outcome, completion)
}

fn independent_programs(
    world: &AuthorizationWorld,
    left_replacement: &str,
    right_replacement: &str,
) -> (RetainedProgram, RetainedProgram) {
    let request = live_scope();
    let principal = authenticated_principal(world, &request);
    let left_account = resolved_account(world, "open", &request);
    let right_account = resolved_account(world, "unrelated", &request);
    (
        retained_status_program(
            world,
            &principal,
            &left_account,
            &request,
            left_replacement,
            RetentionMutationBreadth::Narrow,
        ),
        retained_status_program(
            world,
            &principal,
            &right_account,
            &request,
            right_replacement,
            RetentionMutationBreadth::Narrow,
        ),
    )
}

fn equivalent_programs(
    world: &AuthorizationWorld,
    replacement: &str,
) -> (RetainedProgram, RetainedProgram) {
    let request = live_scope();
    let principal = authenticated_principal(world, &request);
    let account = resolved_account(world, "open", &request);
    (
        retained_status_program(
            world,
            &principal,
            &account,
            &request,
            replacement,
            RetentionMutationBreadth::Narrow,
        ),
        retained_status_program(
            world,
            &principal,
            &account,
            &request,
            replacement,
            RetentionMutationBreadth::Narrow,
        ),
    )
}
