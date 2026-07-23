use tempfile::tempdir;
use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    PhysicalWorkCapacity, PhysicalWorkCapacityDimension, PhysicalWorkReadiness,
    PhysicalWorkSubmissionOutcome,
};

use super::fixture::{serving_from_initialization_with_work_profile, work_fixture};

#[test]
fn bounded_command_arena_defers_without_retaining_an_unadmitted_identity() {
    let root = tempdir().unwrap();
    let (profile, request, _) = work_fixture();
    let capacity = PhysicalWorkCapacity::new(4, 1, 1_024, 1024 * 1024, 4 * 1024 * 1024).unwrap();
    let serving =
        serving_from_initialization_with_work_profile(root.path(), profile.with_capacity(capacity));
    let submission = serving.physical_read_submission();
    for _ in 0..4 {
        success(submission.submit(request.clone()));
    }
    assert!(matches!(
        submission.submit(request).into_raw(),
        TransitionOutcome::Deferred(deferred)
            if deferred.capacity() == 4
                && deferred.dimension() == PhysicalWorkCapacityDimension::Commands
    ));
    let closed = serving.close();
    assert_eq!(closed.work().declared(), 4);
    assert_eq!(closed.work().terminal().len(), 4);
    assert_eq!(closed.work().residual(), 0);
}

#[test]
fn completed_pre_effect_work_releases_capacity_for_sustained_churn() {
    let root = tempdir().unwrap();
    let (profile, request, _) = work_fixture();
    let capacity = PhysicalWorkCapacity::new(1, 1, 1, 1024 * 1024, 1024 * 1024).unwrap();
    let serving =
        serving_from_initialization_with_work_profile(root.path(), profile.with_capacity(capacity));
    let submission = serving.physical_read_submission();

    for _ in 0..32 {
        let receipt = success(submission.submit(request.clone()));
        let admitted = serving.admit_physical_work(receipt).unwrap();
        match serving.request_physical_work(admitted).unwrap() {
            PhysicalWorkReadiness::Ready(ready) => drop(ready),
            PhysicalWorkReadiness::Blocked(blocked) => {
                panic!("clean dependency unexpectedly blocked: {:?}", blocked.condition())
            }
        }
    }

    let closed = serving.close();
    assert_eq!(closed.work().declared(), 32);
    assert!(closed.work().terminal().is_empty());
    assert_eq!(closed.work().residual(), 0);
    assert_eq!(closed.work().unaccounted_terminal(), 0);
}

#[test]
fn aggregate_scope_and_per_work_semantic_budgets_defer_before_retention() {
    let root = tempdir().unwrap();
    let (profile, request, _) = work_fixture();
    let scope_limited = PhysicalWorkCapacity::new(4, 1, 1, 1024 * 1024, 4 * 1024 * 1024).unwrap();
    let serving = serving_from_initialization_with_work_profile(
        root.path(),
        profile.clone().with_capacity(scope_limited),
    );
    let submission = serving.physical_read_submission();
    success(submission.submit(request.clone()));
    assert!(matches!(
        submission.submit(request.clone()).into_raw(),
        TransitionOutcome::Deferred(deferred)
            if deferred.dimension() == PhysicalWorkCapacityDimension::TotalScopeMembers
    ));
    serving.close();

    let semantic_limited = PhysicalWorkCapacity::new(4, 1, 1_024, 1, 4).unwrap();
    let reopened = super::fixture::serving_from_open_with_work_profile(
        root.path(),
        profile.with_capacity(semantic_limited),
    );
    assert!(matches!(
        reopened.physical_read_submission().submit(request).into_raw(),
        TransitionOutcome::Deferred(deferred)
            if deferred.dimension() == PhysicalWorkCapacityDimension::SemanticBytesPerWork
    ));
    assert_eq!(reopened.close().work().declared(), 0);
}

fn success(
    outcome: PhysicalWorkSubmissionOutcome,
) -> worth_store::physical_runtime::PhysicalWorkSubmissionReceipt {
    match outcome.into_raw() {
        TransitionOutcome::Success(receipt) => receipt,
        outcome => panic!("physical work should submit: {outcome:?}"),
    }
}
