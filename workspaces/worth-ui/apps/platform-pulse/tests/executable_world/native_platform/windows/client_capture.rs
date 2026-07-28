use xcap::Window;

use crate::external_observation::NativeClientAreaBounds;

use super::super::NativePlatformFailure;

pub(super) fn exact_window(
    process_id: u32,
    window_id: u32,
) -> Result<Window, NativePlatformFailure> {
    let mut matches = Window::all()
        .map_err(capture_failure)?
        .into_iter()
        .filter(|window| {
            window.pid().ok() == Some(process_id) && window.id().ok() == Some(window_id)
        });
    let window = matches.next().ok_or_else(|| {
        NativePlatformFailure::ClientCapture("bound HWND is not capturable".into())
    })?;
    if matches.next().is_some() {
        return Err(NativePlatformFailure::ClientCapture(
            "bound HWND resolves to multiple capture windows".into(),
        ));
    }
    Ok(window)
}

pub(super) fn capture_client(
    window: &Window,
    client: NativeClientAreaBounds,
) -> Result<Vec<u8>, NativePlatformFailure> {
    let window_left = window.x().map_err(capture_failure)?;
    let window_top = window.y().map_err(capture_failure)?;
    let screenshot = window.capture_image().map_err(capture_failure)?;
    crop_client(screenshot, window_left, window_top, client)
}

fn crop_client(
    screenshot: xcap::image::RgbaImage,
    window_left: i32,
    window_top: i32,
    client: NativeClientAreaBounds,
) -> Result<Vec<u8>, NativePlatformFailure> {
    let left = u32::try_from(client.left() - window_left)
        .map_err(|_| NativePlatformFailure::InvalidCaptureWindowBounds)?;
    let top = u32::try_from(client.top() - window_top)
        .map_err(|_| NativePlatformFailure::InvalidCaptureWindowBounds)?;
    let right = left
        .checked_add(client.width())
        .ok_or(NativePlatformFailure::InvalidCaptureWindowBounds)?;
    let bottom = top
        .checked_add(client.height())
        .ok_or(NativePlatformFailure::InvalidCaptureWindowBounds)?;
    if right > screenshot.width() || bottom > screenshot.height() {
        return Err(NativePlatformFailure::InvalidClientCapture {
            image_width: screenshot.width(),
            image_height: screenshot.height(),
            outer: client,
            client,
        });
    }
    let source_width = screenshot.width() as usize;
    let client_width = client.width() as usize;
    let raw = screenshot.into_raw();
    let mut cropped = Vec::with_capacity(client_width * client.height() as usize * 4);
    for y in top as usize..bottom as usize {
        let start = (y * source_width + left as usize) * 4;
        cropped.extend_from_slice(&raw[start..start + client_width * 4]);
    }
    Ok(cropped)
}

fn capture_failure(error: xcap::XCapError) -> NativePlatformFailure {
    NativePlatformFailure::ClientCapture(error.to_string())
}
