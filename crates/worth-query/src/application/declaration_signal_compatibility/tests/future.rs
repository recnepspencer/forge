use crate::application::WorthQueryDeclarationSignalCompatibilityChecked;

use super::support::{
    domain::{handle, AsyncRuntimeFamily, Input, RuntimeFamily, TemporalRuntimeFamily},
    proof::{
        checked_from_future_public_runtime_signal_posture, compatibility_from_envelope_input,
        compatibility_from_future_supported_runtime_test_posture,
    },
};

#[test]
fn ordinary_temporal_and_async_signal_subjects_keep_distinct_future_projection_truth() {
    let handle = handle("future");

    let ordinary =
        compatibility_from_envelope_input(&handle, Input::<RuntimeFamily>::new("edge:42"));
    let temporal = compatibility_from_future_supported_runtime_test_posture(
        &handle,
        Input::<TemporalRuntimeFamily>::new("edge:42"),
    );
    let async_signal = compatibility_from_future_supported_runtime_test_posture(
        &handle,
        Input::<AsyncRuntimeFamily>::new("edge:42"),
    );

    assert_eq!(ordinary.future_projection().class().as_str(), "ordinary");
    assert_eq!(temporal.future_projection().class().as_str(), "temporal");
    assert_eq!(
        async_signal.future_projection().class().as_str(),
        "async_resource"
    );
    assert_ne!(
        temporal.signal_compatibility_digest(),
        async_signal.signal_compatibility_digest()
    );
}

#[test]
fn future_signal_subjects_stay_typed_under_public_runtime_posture() {
    let handle = handle("future-public");

    match checked_from_future_public_runtime_signal_posture(
        &handle,
        Input::<TemporalRuntimeFamily>::new("edge:42"),
    ) {
        WorthQueryDeclarationSignalCompatibilityChecked::Denied(denied) => {
            assert_eq!(
                denied.cause(),
                crate::application::WorthQueryDeclarationSignalCompatibilityDenialCause::SignalBasisMismatch
            );
            assert_eq!(
                denied
                    .envelope()
                    .route_plan()
                    .expect("deferred temporal signal retains route truth")
                    .future_projection()
                    .class()
                    .as_str(),
                "temporal"
            );
        }
        _ => panic!("temporal signal compatibility should stay typed and denied"),
    }

    match checked_from_future_public_runtime_signal_posture(
        &handle,
        Input::<AsyncRuntimeFamily>::new("edge:42"),
    ) {
        WorthQueryDeclarationSignalCompatibilityChecked::Denied(denied) => {
            assert_eq!(
                denied.cause(),
                crate::application::WorthQueryDeclarationSignalCompatibilityDenialCause::SignalBasisMismatch
            );
            assert_eq!(
                denied
                    .envelope()
                    .route_plan()
                    .expect("deferred async signal retains route truth")
                    .future_projection()
                    .class()
                    .as_str(),
                "async_resource"
            );
        }
        _ => panic!("async signal compatibility should stay typed and denied"),
    }
}
