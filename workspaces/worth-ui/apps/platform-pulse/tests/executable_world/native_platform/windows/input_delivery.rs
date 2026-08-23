use uiautomation::inputs::{Mouse, MouseButton};
use uiautomation::types::{Handle, Point};
use uiautomation::UIAutomation;
use winsafe::{co, HWND, POINT};

use crate::external_observation::{
    NativeClientPixelPoint, NativeInputDeliveryObservation, NativeInputProbeKind,
    ProcessBoundNativeClientAreaObservation,
};

use super::NativePlatformFailure;

pub(super) fn deliver(
    window: &HWND,
    observed: ProcessBoundNativeClientAreaObservation,
    kind: NativeInputProbeKind,
) -> Result<NativeInputDeliveryObservation, NativePlatformFailure> {
    let bounds = observed.bounds();
    let screen_x = bounds
        .left()
        .checked_add_unsigned(bounds.width() / 2)
        .ok_or(NativePlatformFailure::InvalidCaptureWindowBounds)?;
    let screen_y = bounds
        .top()
        .checked_add_unsigned(bounds.height() / 2)
        .ok_or(NativePlatformFailure::InvalidCaptureWindowBounds)?;
    deliver_at(window, observed, kind, (screen_x, screen_y), None)
}

pub(super) fn deliver_pointer(
    window: &HWND,
    observed: ProcessBoundNativeClientAreaObservation,
    point: NativeClientPixelPoint,
) -> Result<NativeInputDeliveryObservation, NativePlatformFailure> {
    let bounds = observed.bounds();
    if point.capture_extent() != (bounds.width(), bounds.height()) {
        return Err(NativePlatformFailure::InputDelivery(
            "pointer point was adjudicated from a different client capture extent".to_owned(),
        ));
    }
    let (client_x, client_y) = point.coordinates();
    let screen_x = bounds
        .left()
        .checked_add_unsigned(client_x)
        .ok_or(NativePlatformFailure::InvalidCaptureWindowBounds)?;
    let screen_y = bounds
        .top()
        .checked_add_unsigned(client_y)
        .ok_or(NativePlatformFailure::InvalidCaptureWindowBounds)?;
    deliver_at(
        window,
        observed,
        NativeInputProbeKind::Pointer,
        (screen_x, screen_y),
        Some(point.landing_tolerance()),
    )
}

fn deliver_at(
    window: &HWND,
    observed: ProcessBoundNativeClientAreaObservation,
    kind: NativeInputProbeKind,
    screen_point: (i32, i32),
    pointer_tolerance: Option<u32>,
) -> Result<NativeInputDeliveryObservation, NativePlatformFailure> {
    let bounds = observed.bounds();
    let (screen_x, screen_y) = screen_point;
    let automation = UIAutomation::new().map_err(input_failure)?;
    let element = automation
        .element_from_handle(Handle::from(window.ptr() as isize))
        .map_err(input_failure)?;
    element.set_focus().map_err(|error| {
        NativePlatformFailure::InputDelivery(format!(
            "focus process-bound automation element: {error}"
        ))
    })?;
    if kind == NativeInputProbeKind::Pointer {
        super::input_environment::qualify_pointer_world(
            window,
            observed.process_id(),
            (screen_x, screen_y),
        )
        .map_err(NativePlatformFailure::InputEnvironment)?;
    }
    match kind {
        NativeInputProbeKind::Pointer => {
            move_pointer_to((screen_x, screen_y))?;
            Mouse::default().click_button(MouseButton::LEFT)
        }
        NativeInputProbeKind::Keyboard => {
            let result = element.send_keys("A", 0);
            if !element.has_keyboard_focus().map_err(input_failure)? {
                return Err(NativePlatformFailure::InputDelivery(
                    "process-bound automation element did not retain keyboard focus".to_owned(),
                ));
            }
            result
        }
    }
    .map_err(input_failure)?;
    require_point_hit_target(window, (screen_x, screen_y))?;
    let delivered_point = Mouse::get_cursor_pos().map_err(input_failure)?;
    if pointer_tolerance.is_some_and(|tolerance| {
        delivered_point.get_x().abs_diff(screen_x) > tolerance
            || delivered_point.get_y().abs_diff(screen_y) > tolerance
    }) {
        return Err(NativePlatformFailure::InputDelivery(format!(
            "native pointer landed at ({}, {}) instead of ({screen_x}, {screen_y})",
            delivered_point.get_x(),
            delivered_point.get_y()
        )));
    }
    if delivered_point.get_x() < bounds.left()
        || delivered_point.get_x() >= bounds.right()
        || delivered_point.get_y() < bounds.top()
        || delivered_point.get_y() >= bounds.bottom()
    {
        return Err(NativePlatformFailure::InputDelivery(
            "native input cursor was outside the process-bound client area".to_owned(),
        ));
    }
    Ok(NativeInputDeliveryObservation::for_client(
        kind,
        observed,
        (delivered_point.get_x(), delivered_point.get_y()),
        2,
    ))
}

fn move_pointer_to(screen_point: (i32, i32)) -> Result<(), NativePlatformFailure> {
    Mouse::set_cursor_pos(&Point::new(screen_point.0, screen_point.1)).map_err(input_failure)
}

fn require_point_hit_target(
    window: &HWND,
    screen_point: (i32, i32),
) -> Result<(), NativePlatformFailure> {
    let hit = HWND::WindowFromPoint(POINT {
        x: screen_point.0,
        y: screen_point.1,
    });
    let root = hit
        .as_ref()
        .and_then(|child| child.GetAncestor(co::GA::ROOT));
    if root.as_ref() == Some(window) {
        return Ok(());
    }
    Err(NativePlatformFailure::InputDelivery(format!(
        "native pointer hit window {} instead of bound window {}",
        hit.map_or(0, |handle| handle.ptr() as usize),
        window.ptr() as usize,
    )))
}

fn input_failure(error: uiautomation::Error) -> NativePlatformFailure {
    NativePlatformFailure::InputDelivery(error.to_string())
}
