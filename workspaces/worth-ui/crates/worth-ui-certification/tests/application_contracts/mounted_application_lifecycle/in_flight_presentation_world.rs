use worth_ui::facade::mounted::{UiHostSurfacePresentationMode, UiMountedFrameRequest};
use worth_ui_test_support::{
    WorthUiMountedFrameExecutionCertificationExt, WorthUiMountedPublicationCertificationExt,
};

use super::known_empty_surface_world::{first_node, mounted_application_with_host, profile};
use crate::mounted_host_protocol::scripted_host::ScriptedPresentationHost;

pub(crate) struct InFlightPresentationWorld {
    pub session: worth_ui::facade::app::WorthUiActiveApplicationSession,
    pub handle: worth_ui::facade::mounted::UiMountedPresentationInFlight,
}

impl InFlightPresentationWorld {
    pub(crate) fn accepted(label: &str) -> Self {
        let host = ScriptedPresentationHost::default();
        let (mut session, _) = mounted_session(host.clone(), label, 1);
        let frame = prepared(&mut session);
        host.push_in_flight(
            vec![crate::mounted_host_protocol::scripted_host::presented_completion()],
            worth_ui::facade::mounted::UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun,
        );
        let outcome = session.present_prepared_mounted_frame(
            frame,
            worth_ui::facade::mounted::UiPresentationDeadline::at_tick(20),
            0,
        );
        let handle = match outcome {
            worth_ui::facade::mounted::UiMountedFrameOutcome::InFlight(handle) => handle,
            _ => panic!("canonical in-flight world requires host acceptance"),
        };
        Self { session, handle }
    }
}

pub(crate) fn mounted_session(
    host: ScriptedPresentationHost,
    label: &str,
    surface_count: usize,
) -> (
    worth_ui::facade::app::WorthUiActiveApplicationSession,
    Vec<worth_ui::facade::mounted::UiSurfaceBindingGeneration>,
) {
    let mut session = mounted_application_with_host(label, host).launch().unwrap();
    let node = first_node(&session);
    let mut bindings = Vec::new();
    for epoch in 0..surface_count {
        let surface = session.create_semantic_surface().unwrap();
        let binding = session
            .register_host_surface(
                surface,
                UiHostSurfacePresentationMode::RecordOnly,
                profile(u64::try_from(epoch + 1).unwrap()),
            )
            .unwrap()
            .binding_generation();
        session.mount_instance(node, surface).unwrap();
        bindings.push(binding);
    }
    bindings.sort();
    (session, bindings)
}

pub(crate) fn prepared(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> worth_ui::facade::mounted::UiPreparedMountedFrame {
    session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("empty source turn permits presentation preparation"))
        .prepare_mounted_frame(UiMountedFrameRequest::all_bound_surfaces())
        .unwrap()
}
