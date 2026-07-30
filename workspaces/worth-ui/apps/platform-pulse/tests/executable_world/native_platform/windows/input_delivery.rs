use uiautomation::inputs::Mouse;
use uiautomation::types::{Handle, Point};
use uiautomation::UIAutomation;
use winsafe::HWND;

use crate::external_observation::{
    NativeInputDeliveryObservation, NativeInputProbeKind, ProcessBoundNativeClientAreaObservation,
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
    let automation = UIAutomation::new().map_err(input_failure)?;
    let element = automation
        .element_from_handle(Handle::from(window.ptr() as isize))
        .map_err(input_failure)?;
    element.set_focus().map_err(|error| {
        NativePlatformFailure::InputDelivery(format!(
            "focus process-bound automation element: {error}"
        ))
    })?;
    match kind {
        NativeInputProbeKind::Pointer => Mouse::default().click(&Point::new(screen_x, screen_y)),
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
    if HWND::GetForegroundWindow().as_ref() != Some(window) {
        return Err(NativePlatformFailure::InputDelivery(
            "process-bound child window was not the foreground input target".to_owned(),
        ));
    }
    let delivered_point = Mouse::get_cursor_pos().map_err(input_failure)?;
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

fn input_failure(error: uiautomation::Error) -> NativePlatformFailure {
    NativePlatformFailure::InputDelivery(error.to_string())
}
