use super::super::super::application_attempt::idempotency;
use super::super::super::fixture::{CapabilityElevationStatus, CapabilityReviewStatus};
use super::super::capability_progression::time;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationCommitDenialKind, WorthQueryApplicationCommitDenialStage,
    WorthQueryElevationCloseOutcome, WorthQueryElevationClosureKind,
};

#[test]
fn provider_time_rejects_stale_revoked_classification_then_commits_exact_expired_close() {
    let (mut world, request, approved) = super::approval_transition::exact_approved_world();
    world
        .application
        .script_authorization_time(std::iter::repeat_n(time(100), 16));
    let program = super::terminal_lifecycle_support::materialize_close(&world, &request, approved);

    world
        .application
        .script_authorization_time(std::iter::repeat_n(time(106), 16));
    let WorthQueryElevationCloseOutcome::Denied(denial, approved) = world
        .application
        .compare_and_commit_elevation_close(program, idempotency(177, 177))
    else {
        panic!("provider time must reject a Revoked close after the exact window expires");
    };
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationCommitDenialKind::ProviderRejected
    );
    assert_eq!(
        denial.stage(),
        WorthQueryApplicationCommitDenialStage::DecisionReadSet
    );
    assert_eq!(
        super::terminal_state::elevation_status(&world),
        CapabilityElevationStatus::Approved
    );
    assert_eq!(
        super::terminal_state::review_status(&world),
        CapabilityReviewStatus::Required
    );

    world
        .application
        .script_authorization_time(std::iter::repeat_n(time(106), 16));
    let program = super::terminal_lifecycle_support::materialize_close(&world, &request, approved);
    let WorthQueryElevationCloseOutcome::Closed(mandatory) = world
        .application
        .compare_and_commit_elevation_close(program, idempotency(178, 178))
    else {
        panic!("a freshly classified Expired close must commit");
    };
    assert_eq!(
        mandatory.closure_kind(),
        WorthQueryElevationClosureKind::Expired
    );
    assert_eq!(mandatory.closed_at(), &time_value(106));
    assert_eq!(
        super::terminal_state::elevation_status(&world),
        CapabilityElevationStatus::Expired
    );
}

fn time_value(seconds: u64) -> worth_foundational::facade::AspectValue {
    worth_foundational::facade::AspectValue::UInt64(seconds)
}
