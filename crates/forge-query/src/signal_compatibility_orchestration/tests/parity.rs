use crate::application::ForgeQueryDeclarationFamilyMarker;
use crate::application::ForgeQueryDeclarationSignalCompatibilityInput;

use super::support::{
    admitted_handle, bridge_request, continuation_outcome_token, envelope, orchestration_input,
    orchestration_outcome_token, progressed_input, OutcomeDigestToken, SignalFamily,
};
use crate::signal_compatibility_orchestration::ForgeQuerySignalCompatibilityOrchestrationClass;

#[test]
fn compatible_only_orchestration_matches_retained_signal_truth() {
    let handle = admitted_handle("main");
    let expected = handle
        .signal_compatibility(ForgeQueryDeclarationSignalCompatibilityInput::enveloped(
            envelope(&handle, "face-a"),
        ))
        .unwrap_or_else(|_| panic!("expected retained signal compatibility"));

    let outcome = handle.orchestrate_signal_compatibility(orchestration_input(&handle, "face-a"));
    match outcome {
        crate::signal_compatibility_orchestration::ForgeQuerySignalCompatibilityOrchestrationOutcome::Bound(value) => {
            assert_eq!(
                value.class(),
                ForgeQuerySignalCompatibilityOrchestrationClass::Compatible
            );
            assert_eq!(
                value.signal_execution_family(),
                Some(expected.execution_family())
            );
            assert_eq!(value.basis_families(), expected.basis_families());
            assert_eq!(value.envelope_digest(), expected.envelope_digest());
        }
        _ => panic!("expected compatible orchestration outcome"),
    }
}

#[test]
fn progressed_input_can_lower_through_signal_orchestration() {
    let handle = admitted_handle("main");
    let outcome = handle.orchestrate_signal_compatibility(progressed_input(&handle, "face-a"));

    match outcome {
        crate::signal_compatibility_orchestration::ForgeQuerySignalCompatibilityOrchestrationOutcome::Bound(value) => {
            assert_eq!(
                value.class(),
                ForgeQuerySignalCompatibilityOrchestrationClass::Compatible
            );
        }
        _ => panic!("expected progressed input to lower into signal compatibility"),
    }
}

#[test]
fn bridge_request_path_matches_explicit_continuation_preparation_posture() {
    let handle = admitted_handle("main");
    let compatibility = handle.signal_compatibility_checked(
        ForgeQueryDeclarationSignalCompatibilityInput::enveloped(envelope(&handle, "face-a")),
    );
    let explicit =
        crate::continuation_pipeline::prepare_continuation_from_signal_checked_on_handle(
            &handle,
            compatibility,
            bridge_request(),
            SignalFamily::aspect_contract(),
        )
        .into_checked()
        .into_outcome();
    let orchestrated = handle.orchestrate_signal_compatibility(
        orchestration_input(&handle, "face-a").with_bridge_request(bridge_request()),
    );

    assert_eq!(
        continuation_outcome_token(&explicit),
        orchestration_outcome_token(&orchestrated)
    );
    assert!(matches!(
        orchestration_outcome_token(&orchestrated),
        OutcomeDigestToken::Prepared { .. } | OutcomeDigestToken::Status(_)
    ));
}
