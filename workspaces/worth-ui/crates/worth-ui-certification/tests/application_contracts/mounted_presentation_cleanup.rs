use worth_ui::facade::mounted::{
    UiHostSurfaceCancellationOutcome, UiMountedFrameOutcome, UiPresentationDeadline,
};

use super::mounted_application_lifecycle::in_flight_presentation_world::{
    mounted_session, prepared,
};
use super::mounted_host_protocol::scripted_host::{
    ScriptedPresentationHost, ScriptedSurfaceCompletion as UiHostSurfaceInFlightCompletion,
};

#[test]
fn start_time_indeterminacy_cancels_every_earlier_in_flight_surface() {
    let host = ScriptedPresentationHost::default();
    let (mut session, _) = mounted_session(host.clone(), "start-indeterminate-cleanup", 2);
    host.push_in_flight(
        vec![UiHostSurfaceInFlightCompletion::Pending],
        UiHostSurfaceCancellationOutcome::CancelledBeforeEffects,
    );
    host.push_presentation(
        worth_ui_host_contract::UiHostSurfacePresentationOutcome::PresentationIndeterminate,
    );
    let frame = prepared(&mut session);

    let outcome =
        session.present_prepared_mounted_frame(frame, UiPresentationDeadline::at_tick(10), 0);

    assert!(matches!(
        outcome,
        UiMountedFrameOutcome::PresentationIndeterminate(_)
    ));
    assert_eq!(
        host.cancellation_calls().len(),
        1,
        "terminal start-time indeterminacy must not abandon an earlier host token"
    );
    assert!(session.inspect_mounted_identity().current_frame().is_none());
}

#[test]
fn completion_time_indeterminacy_cancels_every_other_pending_surface() {
    let host = ScriptedPresentationHost::default();
    let (mut session, _) = mounted_session(host.clone(), "completion-indeterminate-cleanup", 2);
    host.push_in_flight(
        vec![UiHostSurfaceInFlightCompletion::Pending],
        UiHostSurfaceCancellationOutcome::CancelledBeforeEffects,
    );
    host.push_in_flight(
        vec![UiHostSurfaceInFlightCompletion::PresentationIndeterminate],
        UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun,
    );
    let frame = prepared(&mut session);
    let started =
        session.present_prepared_mounted_frame(frame, UiPresentationDeadline::at_tick(10), 0);
    let UiMountedFrameOutcome::InFlight(in_flight) = started else {
        panic!("both host surfaces accepted asynchronous work");
    };

    let outcome = session.complete_mounted_presentation(in_flight, 1);

    assert!(matches!(
        outcome,
        UiMountedFrameOutcome::PresentationIndeterminate(_)
    ));
    assert_eq!(
        host.cancellation_calls().len(),
        1,
        "terminal completion-time indeterminacy must drain every sibling token"
    );
    assert!(session.inspect_mounted_identity().current_frame().is_none());
}
