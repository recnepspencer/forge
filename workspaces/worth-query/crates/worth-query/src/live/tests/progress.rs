use crate::live::*;
#[test]
fn live_progress_basis_advances_monotonically() {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

    let next = live
        .progress_basis()
        .advance(
            live.progress_basis().change_sequence_id(),
            LiveChangeOrdinal(1),
            preflight.basis().clone(),
        )
        .expect("monotonic advance should succeed");

    assert_eq!(next.last_ordinal().value(), 1);
    assert_ne!(
        next.replay_digest().as_str(),
        live.progress_basis().replay_digest().as_str()
    );
}

#[test]
fn live_progress_basis_rejects_non_monotonic_ordinal() {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

    let error = live
        .progress_basis()
        .advance(
            live.progress_basis().change_sequence_id(),
            LiveChangeOrdinal(2),
            preflight.basis().clone(),
        )
        .expect_err("ordinal gap should fail");

    assert_eq!(
        error,
        LiveProgressError::ChangeSequenceGap {
            expected: 1,
            received: 2,
        }
    );
}
