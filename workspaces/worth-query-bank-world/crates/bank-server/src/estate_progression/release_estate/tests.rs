use bank_domain::estate::EstateAction;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationCommitOutcome, WorthQueryApplicationIdempotencyBinding,
};

use crate::estate_capability_admission::fixture::{
    release_world, request_scope, AUTHORITY, COMPLETED_REVIEW, ESTATE, EXECUTOR,
};

#[test]
fn estate_status_drift_after_materialization_stales_provider_commit() {
    let fixture = release_world("estate-release-provider-currentness");
    let specialist = fixture.authenticate();
    let action = EstateAction::ReleaseEstate {
        estate: ESTATE,
        executor: EXECUTOR,
        authority: AUTHORITY,
        review: COMPLETED_REVIEW,
    };
    let admission = fixture
        .runtime
        .admit_estate_release(&specialist, action, &request_scope())
        .expect("the exact ready release should admit");
    let program = fixture
        .runtime
        .materialize_estate_release(admission)
        .expect("the exact release program should materialize while facts are current");

    let committed = fixture
        .runtime
        .release_estate(&specialist, action, idempotency(201), &request_scope())
        .expect("a separate public release should change the retained estate status");
    assert!(matches!(
        committed,
        crate::BankMutationCommitOutcome::Committed(_)
    ));

    let outcome = fixture
        .runtime
        .application_runtime()
        .compare_and_commit_application(program, idempotency(203));
    assert!(matches!(
        outcome,
        WorthQueryApplicationCommitOutcome::Stale(_)
    ));
}

fn idempotency(seed: u8) -> WorthQueryApplicationIdempotencyBinding {
    WorthQueryApplicationIdempotencyBinding::new([seed; 32], [seed + 1; 32])
}
