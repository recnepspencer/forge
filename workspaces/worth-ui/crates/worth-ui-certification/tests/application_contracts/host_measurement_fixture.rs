use worth_ui::facade::measurement_exchange::WorthUiHostMeasurementSessionExt;
use worth_ui::facade::measurement_exchange::{
    UiFontMeasurementKey, UiHostMeasurementDeadline, UiHostMeasurementIntent,
    UiHostMeasurementObservation, UiHostMeasurementObservationValue, UiHostMeasurementOutcome,
    UiHostMeasurementRequestIntent, UiPortalAnchorRectObservation, UiPortalAnchorRectRequest,
    UiRequestedHostMeasurement, UiTextIntrinsicSizeObservation, UiTextIntrinsicSizeRequest,
    UiViewportExtentObservation, UiViewportExtentRequest,
};
use worth_ui_runtime::facade::host::{WorthUiHostCapability, WorthUiHostCapabilityReport};

use super::mounted_application_lifecycle::in_flight_presentation_world::mounted_session;
use super::mounted_host_protocol::scripted_host::ScriptedPresentationHost;

pub(super) fn mounted_measurement_session(
    label: &str,
    surface_count: usize,
) -> (
    ScriptedPresentationHost,
    worth_ui::facade::app::WorthUiActiveApplicationSession,
    Vec<worth_ui_runtime::facade::mounted::UiSurfaceBindingGeneration>,
) {
    let host = measurement_host();
    let (session, bindings) = mounted_session(host.clone(), label, surface_count);
    (host, session, bindings)
}

pub(super) fn measurement_host() -> ScriptedPresentationHost {
    let host = ScriptedPresentationHost::default();
    host.set_capabilities(WorthUiHostCapabilityReport::available(vec![
        WorthUiHostCapability::MountedFrameRecording,
        WorthUiHostCapability::ViewportObservation,
        WorthUiHostCapability::TextIntrinsicMeasurement,
        WorthUiHostCapability::PortalAnchorObservation,
    ]));
    host
}

pub(super) fn begin_viewport(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    binding: Option<worth_ui_runtime::facade::mounted::UiSurfaceBindingGeneration>,
    deadline: u64,
    now: u64,
) -> UiRequestedHostMeasurement {
    admitted(session.begin_host_measurement(
        UiHostMeasurementIntent::new(
            binding,
            UiHostMeasurementRequestIntent::viewport_extent(UiViewportExtentRequest),
            UiHostMeasurementDeadline::at_tick(deadline),
        ),
        now,
    ))
}

pub(super) fn begin_text(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    binding: Option<worth_ui_runtime::facade::mounted::UiSurfaceBindingGeneration>,
    text: impl Into<Box<str>>,
    deadline: u64,
    now: u64,
) -> UiRequestedHostMeasurement {
    admitted(session.begin_host_measurement(
        UiHostMeasurementIntent::new(
            binding,
            UiHostMeasurementRequestIntent::text_intrinsic_size(
                UiTextIntrinsicSizeRequest::single_line(text, UiFontMeasurementKey::new("body")),
            ),
            UiHostMeasurementDeadline::at_tick(deadline),
        ),
        now,
    ))
}

pub(super) fn begin_portal(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    binding: worth_ui_runtime::facade::mounted::UiSurfaceBindingGeneration,
    deadline: u64,
    now: u64,
) -> UiRequestedHostMeasurement {
    admitted(session.begin_host_measurement(
        UiHostMeasurementIntent::new(
            Some(binding),
            UiHostMeasurementRequestIntent::portal_anchor_rect(UiPortalAnchorRectRequest::new(1)),
            UiHostMeasurementDeadline::at_tick(deadline),
        ),
        now,
    ))
}

pub(super) fn viewport_observation(
    request: &UiRequestedHostMeasurement,
    width: f32,
    height: f32,
) -> UiHostMeasurementObservation {
    UiHostMeasurementObservation::from_request(
        request.request(),
        UiHostMeasurementObservationValue::ViewportExtent(UiViewportExtentObservation {
            width,
            height,
        }),
    )
    .unwrap()
}

pub(super) fn text_observation(
    request: &UiRequestedHostMeasurement,
    width: f32,
    height: f32,
) -> UiHostMeasurementObservation {
    UiHostMeasurementObservation::from_request(
        request.request(),
        UiHostMeasurementObservationValue::TextIntrinsicSize(UiTextIntrinsicSizeObservation {
            width,
            height,
        }),
    )
    .unwrap()
}

pub(super) fn portal_observation(
    request: &UiRequestedHostMeasurement,
) -> UiHostMeasurementObservation {
    UiHostMeasurementObservation::from_request(
        request.request(),
        UiHostMeasurementObservationValue::PortalAnchorRect(UiPortalAnchorRectObservation {
            x: 10.0,
            y: 20.0,
            width: 30.0,
            height: 40.0,
        }),
    )
    .unwrap()
}

fn admitted(outcome: UiHostMeasurementOutcome) -> UiRequestedHostMeasurement {
    match outcome {
        UiHostMeasurementOutcome::Admitted(requested) => requested,
        other => panic!("measurement intent should admit, got {other:?}"),
    }
}
