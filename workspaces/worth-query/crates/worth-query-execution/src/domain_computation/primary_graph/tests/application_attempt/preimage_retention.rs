//! Production-path evidence that exact-field retention denial precedes commit.

use worth_query_declaration::facade::application_schema::TypedMutationPreconditions;
use worth_relational::facade::history::CommitReference;

use super::{authenticated_principal, idempotency, live_scope, resolved_account};
use crate::domain_computation::primary_graph::tests::fixture::{
    installed_authorization_world, AccountLabel, AccountStatus, AuthorizationWorld,
    WrongFieldRetentionOperation,
};
use crate::domain_computation::primary_graph::{
    primary_relational_branch_id, WorthQueryApplicationCommitOutcome,
};

fn relational_head(world: &AuthorizationWorld) -> CommitReference {
    world
        .application
        .relational_branch_head(&primary_relational_branch_id())
        .expect("fixture has a Relational head")
}

fn assert_retention_denied(outcome: &WorthQueryApplicationCommitOutcome) {
    assert!(
        matches!(outcome, WorthQueryApplicationCommitOutcome::Aborted),
        "unexpected retention-denial outcome: {outcome:?}"
    );
}

#[test]
fn right_record_wrong_field_retention_denial_commits_nothing() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let before = relational_head(&world);
    let operation = world
        .application
        .installed_schema()
        .installed_operation(WrongFieldRetentionOperation::reference())
        .unwrap();
    let admission = world
        .application
        .authorize_operation(
            &principal,
            &account,
            &operation,
            TypedMutationPreconditions::new(),
            &request,
        )
        .unwrap();
    let (_, projection, _) = world
        .invariant
        .project_admitted_operation(&admission, |reader, projected| {
            reader
                .require_decision_field(projected, AccountStatus::reference())
                .unwrap();
            reader
                .require_decision_field(projected, AccountLabel::reference())
                .unwrap();
        })
        .unwrap()
        .into_parts();
    let reads = world
        .application
        .begin_projected_application_read_attempt(admission, projection)
        .unwrap();
    let mut effects = reads
        .complete_projected_dependencies()
        .unwrap()
        .begin_effect_program();
    let account = effects.existing_entity(&account).unwrap();
    effects
        .write_field(
            &account,
            AccountLabel::reference(),
            "must-not-land".to_owned(),
        )
        .unwrap();
    let outcome = world
        .application
        .compare_and_commit_application(effects.finish().unwrap(), idempotency(71, 72));

    assert_retention_denied(&outcome);
    let after = relational_head(&world);
    assert_eq!(
        after, before,
        "retention denial must precede Relational commit"
    );
}
