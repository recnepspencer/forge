use crate::application::ForgeQueryDeclarationFamilyMarker;
use crate::continuation_pipeline::{
    prepare_continuation_from_signal_checked_on_handle, ForgeQueryPreparedContinuationOutcome,
};
use crate::future_signal_test_support::{
    checked_signal_public_posture, future_signal_admitted_handle, future_signal_bridge_request,
    AsyncFutureSignalFamily, FutureSignalInput, OrdinaryFutureSignalFamily,
    TemporalFutureSignalFamily,
};

#[test]
fn ordinary_prepared_continuation_keeps_future_projection_visible() {
    let handle = future_signal_admitted_handle("future");
    let checked = checked_signal_public_posture(
        &handle,
        FutureSignalInput::<OrdinaryFutureSignalFamily>::new("face-a"),
    );
    let outcome = prepare_continuation_from_signal_checked_on_handle(
        &handle,
        checked,
        future_signal_bridge_request(),
        OrdinaryFutureSignalFamily::aspect_contract(),
    )
    .into_checked()
    .into_outcome();

    match outcome {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => {
            assert_eq!(prepared.future_projection().class().as_str(), "ordinary");
            assert!(!prepared.basis_lifecycle_support_digest().is_empty());
            assert!(prepared.signal_compatibility_digest().is_some());
        }
        _ => panic!("ordinary compatible signal posture should prepare"),
    }
}

#[test]
fn temporal_and_async_signal_debt_stop_before_prepared_continuation_exists() {
    let handle = future_signal_admitted_handle("future-public");

    let temporal = prepare_continuation_from_signal_checked_on_handle(
        &handle,
        checked_signal_public_posture(
            &handle,
            FutureSignalInput::<TemporalFutureSignalFamily>::new("face-a"),
        ),
        future_signal_bridge_request(),
        TemporalFutureSignalFamily::aspect_contract(),
    )
    .into_checked()
    .into_outcome();
    let async_signal = prepare_continuation_from_signal_checked_on_handle(
        &handle,
        checked_signal_public_posture(
            &handle,
            FutureSignalInput::<AsyncFutureSignalFamily>::new("face-a"),
        ),
        future_signal_bridge_request(),
        AsyncFutureSignalFamily::aspect_contract(),
    )
    .into_checked()
    .into_outcome();

    assert!(matches!(
        temporal,
        ForgeQueryPreparedContinuationOutcome::Deferred(_)
    ));
    assert!(matches!(
        async_signal,
        ForgeQueryPreparedContinuationOutcome::Deferred(_)
    ));
}
