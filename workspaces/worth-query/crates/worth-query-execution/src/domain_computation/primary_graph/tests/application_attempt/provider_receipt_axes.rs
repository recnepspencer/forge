//! Provider wire fields may select, but cannot reconstruct, committed truth.

use super::{admitted_program, authenticated_principal, idempotency, live_scope, resolved_account};
use crate::domain_computation::primary_graph::application_attempt::parse_provider_receipt;
use crate::domain_computation::primary_graph::tests::fixture::installed_authorization_world;
use crate::domain_computation::primary_graph::{
    primary_relational_branch_id, WorthQueryApplicationCommitOutcome,
};

#[test]
fn provider_receipt_cannot_substitute_any_owner_sealed_axis() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let program = admitted_program(&world, &principal, &account, &request, "receipt-axes");
    let WorthQueryApplicationCommitOutcome::Committed(receipt) = world
        .application
        .compare_and_commit_application(program, idempotency(93, 94))
    else {
        panic!("receipt-axis fixture commits");
    };
    let branch = primary_relational_branch_id();
    let outcome = receipt.outcome_identity().unwrap().get();
    let exact = encoded(
        receipt.provider_runtime_instance_id(),
        receipt.commit_id().0,
        receipt.changed_record_count(),
        receipt.emitted_effect_count(),
        outcome,
    );
    assert!(parse_provider_receipt(&exact, &world.application.primary_provider, &branch).is_some());

    let substitutions = [
        encoded(
            receipt.provider_runtime_instance_id(),
            receipt.commit_id().0 + 1,
            receipt.changed_record_count(),
            receipt.emitted_effect_count(),
            outcome,
        ),
        encoded(
            receipt.provider_runtime_instance_id() + 1,
            receipt.commit_id().0,
            receipt.changed_record_count(),
            receipt.emitted_effect_count(),
            outcome,
        ),
        encoded(
            receipt.provider_runtime_instance_id(),
            receipt.commit_id().0,
            receipt.changed_record_count() + 1,
            receipt.emitted_effect_count(),
            outcome,
        ),
        encoded(
            receipt.provider_runtime_instance_id(),
            receipt.commit_id().0,
            receipt.changed_record_count(),
            receipt.emitted_effect_count() + 1,
            outcome,
        ),
        encoded(
            receipt.provider_runtime_instance_id(),
            receipt.commit_id().0,
            receipt.changed_record_count(),
            receipt.emitted_effect_count(),
            outcome + 1,
        ),
    ];
    for substituted in substitutions {
        assert!(
            parse_provider_receipt(&substituted, &world.application.primary_provider, &branch,)
                .is_none(),
            "encoded axes cannot override the owner-sealed application"
        );
    }
}

fn encoded(runtime: u64, commit: u64, changed: usize, emitted: usize, outcome: u64) -> String {
    format!("primary-application-commit:{runtime}:{commit}:{changed}:{emitted}:{outcome}")
}
