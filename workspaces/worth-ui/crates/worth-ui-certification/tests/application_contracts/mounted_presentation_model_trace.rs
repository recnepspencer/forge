use worth_ui::facade::mounted::{
    UiHostSurfaceCancellationOutcome, UiMountedFrameOutcome, UiPresentationDeadline,
};
use worth_ui_test_support::WorthUiMountedPublicationCertificationExt;

use super::mounted_application_lifecycle::in_flight_presentation_world::{
    mounted_session, prepared,
};
use super::mounted_host_protocol::scripted_host::{
    ScriptedPresentationHost, ScriptedSurfaceCompletion as UiHostSurfaceInFlightCompletion,
};
use super::mounted_protocol_model::{
    ModelCancellation, ModelCompletion, ModelFrameState, ModelPresentation, ModelSurfaceStart,
};

#[test]
fn in_flight_terminal_classes_match_the_independent_publication_model() {
    let host = ScriptedPresentationHost::default();
    let (mut session, _) = mounted_session(host.clone(), "presentation-terminal-model", 1);

    host.push_in_flight(
        vec![UiHostSurfaceInFlightCompletion::RejectedBeforeEffects(
            worth_ui_host_contract::UiHostSurfacePresentationDenial::AdapterDeclined,
        )],
        UiHostSurfaceCancellationOutcome::CancelledBeforeEffects,
    );
    let mut rejection_model = ModelPresentation::start(&[ModelSurfaceStart::InFlight]);
    let rejection_frame = prepared(&mut session);
    let rejection_start = session.present_prepared_mounted_frame(
        rejection_frame,
        UiPresentationDeadline::at_tick(10),
        0,
    );
    assert_model_outcome(&rejection_model, &rejection_start);
    let rejection_pending = expect_in_flight(rejection_start);
    rejection_model.complete(0, ModelCompletion::RejectedBeforeEffects);
    let rejection = session.complete_mounted_presentation(rejection_pending, 1);
    assert_model_outcome(&rejection_model, &rejection);

    host.push_in_flight(
        vec![UiHostSurfaceInFlightCompletion::PresentationIndeterminate],
        UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun,
    );
    let mut lost_model = ModelPresentation::start(&[ModelSurfaceStart::InFlight]);
    let lost_frame = prepared(&mut session);
    let lost_start =
        session.present_prepared_mounted_frame(lost_frame, UiPresentationDeadline::at_tick(10), 0);
    assert_model_outcome(&lost_model, &lost_start);
    let lost_pending = expect_in_flight(lost_start);
    lost_model.complete(0, ModelCompletion::EffectStateUnknown);
    let lost = session.complete_mounted_presentation(lost_pending, 1);
    assert_model_outcome(&lost_model, &lost);

    assert_effectful_cancellation_matches_model();
}

fn assert_effectful_cancellation_matches_model() {
    let host = ScriptedPresentationHost::default();
    let (mut session, _) = mounted_session(host.clone(), "presentation-cancellation-model", 1);
    host.push_in_flight(
        vec![UiHostSurfaceInFlightCompletion::Pending],
        UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun,
    );
    let mut model = ModelPresentation::start(&[ModelSurfaceStart::InFlight]);
    let frame = prepared(&mut session);
    let start =
        session.present_prepared_mounted_frame(frame, UiPresentationDeadline::at_tick(2), 0);
    assert_model_outcome(&model, &start);
    let pending = expect_in_flight(start);
    model.cancel(0, ModelCancellation::EffectsMayHaveBegun);
    let cancellation = session.complete_mounted_presentation(pending, 2);
    assert_model_outcome(&model, &cancellation);
}

pub(crate) fn assert_model_outcome(model: &ModelPresentation, outcome: &UiMountedFrameOutcome) {
    match model.frame_state() {
        ModelFrameState::InFlight { pending_surfaces } => {
            let UiMountedFrameOutcome::InFlight(in_flight) = outcome else {
                panic!("independent model requires an in-flight production outcome");
            };
            assert_eq!(in_flight.pending_bindings().count(), pending_surfaces);
        }
        ModelFrameState::Presented => {
            assert!(matches!(outcome, UiMountedFrameOutcome::Published(_)));
        }
        ModelFrameState::RejectedBeforeEffects => {
            assert!(matches!(
                outcome,
                UiMountedFrameOutcome::RejectedBeforeEffects(_)
            ));
        }
        ModelFrameState::Indeterminate => {
            assert!(matches!(
                outcome,
                UiMountedFrameOutcome::PresentationIndeterminate(_)
            ));
        }
    }
    assert_eq!(
        model.publication_eligible(),
        matches!(outcome, UiMountedFrameOutcome::Published(_))
    );
}

fn expect_in_flight(
    outcome: UiMountedFrameOutcome,
) -> worth_ui::facade::mounted::UiMountedPresentationInFlight {
    match outcome {
        UiMountedFrameOutcome::InFlight(value) => value,
        _ => panic!("scripted pending presentation remains in flight"),
    }
}
