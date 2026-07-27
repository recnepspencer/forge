use std::sync::OnceLock;
use std::thread;
use std::time::{Duration, Instant};

use uiautomation::patterns::UIWindowPattern;
use uiautomation::types::Handle;
use uiautomation::UIAutomation;
use winsafe::{self as win, HWND};
use xcap::Window;

use crate::external_observation::{
    NativeClientAreaBounds, NativeClientPixelCapture, NativeWindowIdentity,
    NormalNativeCloseRequestObservation, ProcessBoundNativeClientAreaObservation,
};

use super::contract::sealed::Sealed;
use super::{NativePlatformContract, NativePlatformFailure};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct WindowsNativePlatform {
    _private: (),
}

pub(crate) struct WindowsProcessBoundNativeClientArea {
    window: HWND,
    capture_window: Window,
    observation: ProcessBoundNativeClientAreaObservation,
}

struct ProcessWindowCandidate {
    window: HWND,
    bounds: NativeClientAreaBounds,
}

impl WindowsNativePlatform {
    pub(crate) fn certified() -> Result<Self, NativePlatformFailure> {
        static DPI_AWARENESS: OnceLock<Result<(), String>> = OnceLock::new();
        match DPI_AWARENESS
            .get_or_init(|| win::SetProcessDPIAware().map_err(|error| error.to_string()))
        {
            Ok(()) => Ok(Self { _private: () }),
            Err(error) => Err(NativePlatformFailure::DpiAwareness(error.clone())),
        }
    }

    fn process_windows(
        process_id: u32,
    ) -> Result<Vec<ProcessWindowCandidate>, NativePlatformFailure> {
        let mut candidates = Vec::new();
        win::EnumWindows(|window| {
            let (_, owner_process_id) = window.GetWindowThreadProcessId();
            if owner_process_id == process_id && window.IsWindowVisible() && !window.IsIconic() {
                if let Ok(client) = window
                    .GetClientRect()
                    .and_then(|rect| window.ClientToScreenRc(rect))
                {
                    if let Some(bounds) = NativeClientAreaBounds::new(
                        client.left,
                        client.top,
                        client.right,
                        client.bottom,
                    ) {
                        candidates.push(ProcessWindowCandidate { window, bounds });
                    }
                }
            }
            true
        })
        .map_err(|error| NativePlatformFailure::WindowEnumeration(error.to_string()))?;
        Ok(candidates)
    }

    fn capture_window(process_id: u32) -> Result<Window, NativePlatformFailure> {
        let windows = Window::all()
            .map_err(|error| NativePlatformFailure::ClientCapture(error.to_string()))?;
        let mut matches = windows
            .into_iter()
            .filter(|window| window.pid().ok() == Some(process_id))
            .collect::<Vec<_>>();
        match matches.len() {
            0 => Err(NativePlatformFailure::CaptureWindowMissing),
            1 => Ok(matches.pop().expect("one process capture window")),
            count => Err(NativePlatformFailure::CaptureWindowAmbiguous(count)),
        }
    }
}

impl Sealed for WindowsNativePlatform {}

impl NativePlatformContract for WindowsNativePlatform {
    type BoundClientArea = WindowsProcessBoundNativeClientArea;

    fn bind_process_client_area(
        &self,
        process_id: u32,
        deadline: Instant,
    ) -> Result<Self::BoundClientArea, NativePlatformFailure> {
        let mut lookup_count = 0_u32;
        loop {
            lookup_count = lookup_count.saturating_add(1);
            let mut candidates = Self::process_windows(process_id)?;
            match candidates.len() {
                0 if Instant::now() < deadline => {
                    thread::sleep(Duration::from_millis(20));
                }
                0 => return Err(NativePlatformFailure::WindowLookupDeadline),
                1 => {
                    let candidate = candidates.pop().expect("one process window");
                    let window_identity =
                        NativeWindowIdentity::from_native_value(candidate.window.ptr() as usize)
                            .expect("enumerated HWND is non-null");
                    let capture_window = Self::capture_window(process_id)?;
                    return Ok(WindowsProcessBoundNativeClientArea {
                        window: candidate.window,
                        capture_window,
                        observation: ProcessBoundNativeClientAreaObservation::new(
                            process_id,
                            window_identity,
                            candidate.bounds,
                            lookup_count,
                        ),
                    });
                }
                count => {
                    return Err(NativePlatformFailure::AmbiguousProcessWindows(count));
                }
            }
        }
    }

    fn observe_bound_client_area(
        &self,
        bound: &Self::BoundClientArea,
    ) -> Result<ProcessBoundNativeClientAreaObservation, NativePlatformFailure> {
        if !bound.window.IsWindow() {
            return Err(NativePlatformFailure::BoundWindowMissing);
        }
        let (_, owner_process_id) = bound.window.GetWindowThreadProcessId();
        if owner_process_id != bound.observation.process_id() {
            return Err(NativePlatformFailure::BoundWindowOwnerChanged);
        }
        Ok(bound.observation)
    }

    fn capture_client_area(
        &self,
        bound: &Self::BoundClientArea,
    ) -> Result<NativeClientPixelCapture, NativePlatformFailure> {
        let client = bound.observation.bounds();
        let outer = capture_bounds(&bound.capture_window)?;
        let screenshot = bound
            .capture_window
            .capture_image()
            .map_err(|error| NativePlatformFailure::ClientCapture(error.to_string()))?;
        let invalid_capture = || NativePlatformFailure::InvalidClientCapture {
            image_width: screenshot.width(),
            image_height: screenshot.height(),
            outer,
            client,
        };
        let pixels = crop_client_pixels(&screenshot, outer, client).ok_or_else(invalid_capture)?;
        NativeClientPixelCapture::new(
            bound.observation.process_id(),
            client.width(),
            client.height(),
            pixels,
        )
        .ok_or_else(invalid_capture)
    }

    fn request_normal_close(
        &self,
        bound: &Self::BoundClientArea,
    ) -> Result<NormalNativeCloseRequestObservation, NativePlatformFailure> {
        self.observe_bound_client_area(bound)?;
        let automation = UIAutomation::new()
            .map_err(|error| NativePlatformFailure::NormalClose(error.to_string()))?;
        automation
            .element_from_handle(Handle::from(bound.window.ptr() as isize))
            .and_then(|element| element.get_pattern::<UIWindowPattern>())
            .and_then(|pattern| pattern.close())
            .map_err(|error| NativePlatformFailure::NormalClose(error.to_string()))?;
        Ok(NormalNativeCloseRequestObservation::one(
            bound.observation.process_id(),
        ))
    }

    fn verify_process_window_released(&self, process_id: u32) -> Result<(), NativePlatformFailure> {
        let windows = Self::process_windows(process_id)?;
        if windows.is_empty() {
            Ok(())
        } else {
            Err(NativePlatformFailure::ProcessWindowResidue(windows.len()))
        }
    }
}

fn capture_bounds(window: &Window) -> Result<NativeClientAreaBounds, NativePlatformFailure> {
    let left = window
        .x()
        .map_err(|error| NativePlatformFailure::ClientCapture(error.to_string()))?;
    let top = window
        .y()
        .map_err(|error| NativePlatformFailure::ClientCapture(error.to_string()))?;
    let width = window
        .width()
        .map_err(|error| NativePlatformFailure::ClientCapture(error.to_string()))?;
    let height = window
        .height()
        .map_err(|error| NativePlatformFailure::ClientCapture(error.to_string()))?;
    let right = left.checked_add_unsigned(width);
    let bottom = top.checked_add_unsigned(height);
    right
        .zip(bottom)
        .and_then(|(right, bottom)| NativeClientAreaBounds::new(left, top, right, bottom))
        .ok_or(NativePlatformFailure::InvalidCaptureWindowBounds)
}

fn crop_client_pixels(
    image: &xcap::image::RgbaImage,
    outer: NativeClientAreaBounds,
    client: NativeClientAreaBounds,
) -> Option<Vec<u8>> {
    let offset_x = u32::try_from(client.left().checked_sub(outer.left())?).ok()?;
    let offset_y = u32::try_from(client.top().checked_sub(outer.top())?).ok()?;
    let outer_width = outer.width();
    let outer_height = outer.height();
    let client_right = offset_x.checked_add(client.width())?;
    let client_bottom = offset_y.checked_add(client.height())?;
    if client_right > outer_width || client_bottom > outer_height {
        return None;
    }
    let image_left = scale_floor(offset_x, image.width(), outer_width)?;
    let image_top = scale_floor(offset_y, image.height(), outer_height)?;
    let image_right = scale_ceil(client_right, image.width(), outer_width)?;
    let image_bottom = scale_ceil(client_bottom, image.height(), outer_height)?;
    let crop_width = image_right.checked_sub(image_left)?;
    let crop_height = image_bottom.checked_sub(image_top)?;
    if crop_width == 0
        || crop_height == 0
        || image_right > image.width()
        || image_bottom > image.height()
    {
        return None;
    }
    let crop =
        xcap::image::imageops::crop_imm(image, image_left, image_top, crop_width, crop_height)
            .to_image();
    Some(
        xcap::image::imageops::resize(
            &crop,
            client.width(),
            client.height(),
            xcap::image::imageops::FilterType::Nearest,
        )
        .into_raw(),
    )
}

fn scale_floor(value: u32, image_extent: u32, outer_extent: u32) -> Option<u32> {
    let scaled = u64::from(value).checked_mul(u64::from(image_extent))?;
    u32::try_from(scaled / u64::from(outer_extent)).ok()
}

fn scale_ceil(value: u32, image_extent: u32, outer_extent: u32) -> Option<u32> {
    let scaled = u64::from(value).checked_mul(u64::from(image_extent))?;
    let rounded =
        scaled.checked_add(u64::from(outer_extent).checked_sub(1)?)? / u64::from(outer_extent);
    u32::try_from(rounded).ok()
}
