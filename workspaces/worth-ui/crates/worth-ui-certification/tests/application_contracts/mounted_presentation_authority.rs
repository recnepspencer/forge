use worth_ui_test_support::WorthUiMountedPublicationCertificationExt;

use super::mounted_application_lifecycle::in_flight_presentation_world::mounted_session;
use super::mounted_application_lifecycle::in_flight_presentation_world::prepared;
use super::mounted_host_protocol::scripted_host::ScriptedPresentationHost;

#[test]
fn prepared_frame_cannot_publish_after_mounted_authority_changes() {
    let host = ScriptedPresentationHost::default();
    let (mut session, _) = mounted_session(host.clone(), "presentation-authority-stale-frame", 1);
    let mounted_instance = session.inspect_mounted_identity().mounted_instances()[0].identity();
    let frame = prepared(&mut session);
    session.unmount_instance(mounted_instance).unwrap();

    let outcome = session.present_prepared_mounted_frame(
        frame,
        worth_ui::facade::mounted::UiPresentationDeadline::at_tick(10),
        0,
    );
    match outcome {
        worth_ui::facade::mounted::UiMountedFrameOutcome::AdmissionDenied(rejection) => {
            assert_eq!(
                rejection.denial(),
                worth_ui::facade::mounted::UiMountedPresentationAdmissionDenial::PreparedFrameBasisChanged
            );
        }
        _ => panic!("stale mounted authority must deny before presentation"),
    }
    assert_eq!(host.presentation_calls(), 0);
}

#[test]
fn runtime_session_authority_isolates_shared_adapter_resources() {
    let host = ScriptedPresentationHost::default();
    let (first, _) = mounted_session(host.clone(), "presentation-authority-first", 1);
    let (second, _) = mounted_session(host.clone(), "presentation-authority-second", 1);
    assert_eq!(host.native_registration_count(), 2);

    let first_shutdown = first.shutdown();
    assert_eq!(
        match first_shutdown
            .host_session_release()
            .expect("active application shutdown releases its host session")
        {
            worth_ui::facade::host::UiHostSessionReleaseOutcome::Released(receipt) => {
                receipt.released_surface_count()
            }
            worth_ui::facade::host::UiHostSessionReleaseOutcome::ReleaseIndeterminate(_) => {
                panic!("scripted host release is deterministic")
            }
        },
        1
    );
    assert_eq!(host.native_registration_count(), 1);

    let second_shutdown = second.shutdown();
    assert_eq!(
        match second_shutdown
            .host_session_release()
            .expect("active application shutdown releases its host session")
        {
            worth_ui::facade::host::UiHostSessionReleaseOutcome::Released(receipt) => {
                receipt.released_surface_count()
            }
            worth_ui::facade::host::UiHostSessionReleaseOutcome::ReleaseIndeterminate(_) => {
                panic!("scripted host release is deterministic")
            }
        },
        1
    );
    assert_eq!(host.native_registration_count(), 0);
}

#[test]
fn dropped_session_releases_in_flight_adapter_state() {
    let host = ScriptedPresentationHost::default();
    let (mut session, _) = mounted_session(host.clone(), "presentation-authority-in-flight", 1);
    host.push_in_flight(
        vec![super::mounted_host_protocol::scripted_host::ScriptedSurfaceCompletion::Pending],
        worth_ui::facade::mounted::UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun,
    );
    let frame = prepared(&mut session);
    assert!(matches!(
        session.present_prepared_mounted_frame(
            frame,
            worth_ui::facade::mounted::UiPresentationDeadline::at_tick(10),
            0,
        ),
        worth_ui::facade::mounted::UiMountedFrameOutcome::InFlight(_)
    ));
    assert_eq!(host.native_in_flight_count(), 1);

    drop(session);

    assert_eq!(host.native_in_flight_count(), 0);
    assert_eq!(host.native_registration_count(), 0);
}
