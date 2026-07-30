use std::sync::OnceLock;
use std::thread;
use std::time::{Duration, Instant};

use uiautomation::patterns::UIWindowPattern;
use uiautomation::types::Handle;
use uiautomation::UIAutomation;
use winsafe::{self as win, co, HwndPlace, HWND, POINT, SIZE};
use xcap::{Monitor, Window};

use crate::external_observation::{
    NativeClientAreaBounds, NativeClientPixelCapture, NativeInputDeliveryObservation,
    NativeInputProbeKind, NativeWindowIdentity, NormalNativeCloseRequestObservation,
    ProcessBoundNativeClientAreaObservation,
};

use super::contract::sealed::Sealed;
use super::{NativePlatformContract, NativePlatformFailure};

mod client_capture;
mod input_delivery;

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

struct WindowsCaptureExposure<'bound> {
    bound: &'bound WindowsProcessBoundNativeClientArea,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct NativeMonitorCaptureRegion {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct NativeMonitorPhysicalBounds {
    left: i32,
    top: i32,
    width: u32,
    height: u32,
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
                if let Ok(client) = client_bounds(&window) {
                    if let Some(bounds) = NativeClientAreaBounds::new(
                        client.left(),
                        client.top(),
                        client.right(),
                        client.bottom(),
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

    fn expose_bound_client_area<'bound>(
        &self,
        bound: &'bound WindowsProcessBoundNativeClientArea,
    ) -> Result<WindowsCaptureExposure<'bound>, NativePlatformFailure> {
        self.observe_bound_client_area(bound)?;
        bound
            .window
            .SetWindowPos(
                HwndPlace::Place(co::HWND_PLACE::TOP),
                POINT::default(),
                SIZE::default(),
                co::SWP::NOMOVE | co::SWP::NOSIZE | co::SWP::NOACTIVATE | co::SWP::SHOWWINDOW,
            )
            .map_err(|error| NativePlatformFailure::ClientExposure(error.to_string()))?;
        win::DwmFlush()
            .map_err(|error| NativePlatformFailure::ClientExposure(error.to_string()))?;
        self.observe_bound_client_area(bound)?;
        Ok(WindowsCaptureExposure { bound })
    }

    fn capture_exposed_client_area(
        exposure: WindowsCaptureExposure<'_>,
    ) -> Result<NativeClientPixelCapture, NativePlatformFailure> {
        let bound = exposure.bound;
        let client = bound.observation.bounds();
        let rgba = client_capture::capture_client(&bound.capture_window, client)?;
        NativeClientPixelCapture::new(
            bound.observation.process_id(),
            client.width(),
            client.height(),
            rgba,
        )
        .ok_or(NativePlatformFailure::InvalidClientCapture {
            image_width: client.width(),
            image_height: client.height(),
            outer: client,
            client,
        })
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
                    let capture_monitor = monitor_for_client(candidate.bounds)?;
                    let _capture_region =
                        monitor_capture_region(&capture_monitor, candidate.bounds)?;
                    let capture_window =
                        client_capture::exact_window(process_id, candidate.window.ptr() as u32)?;
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
        if client_bounds(&bound.window)? != bound.observation.bounds() {
            return Err(NativePlatformFailure::BoundClientAreaChanged);
        }
        Ok(bound.observation)
    }

    fn capture_client_area(
        &self,
        bound: &Self::BoundClientArea,
    ) -> Result<NativeClientPixelCapture, NativePlatformFailure> {
        let exposure = self.expose_bound_client_area(bound)?;
        Self::capture_exposed_client_area(exposure)
    }

    fn deliver_input_reachability_probe(
        &self,
        bound: &Self::BoundClientArea,
        kind: NativeInputProbeKind,
    ) -> Result<NativeInputDeliveryObservation, NativePlatformFailure> {
        let observed = self.observe_bound_client_area(bound)?;
        input_delivery::deliver(&bound.window, observed, kind)
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

fn client_bounds(window: &HWND) -> Result<NativeClientAreaBounds, NativePlatformFailure> {
    let client = window
        .GetClientRect()
        .and_then(|rect| window.ClientToScreenRc(rect))
        .map_err(|error| NativePlatformFailure::WindowEnumeration(error.to_string()))?;
    NativeClientAreaBounds::new(client.left, client.top, client.right, client.bottom)
        .ok_or(NativePlatformFailure::InvalidCaptureWindowBounds)
}

fn monitor_for_client(client: NativeClientAreaBounds) -> Result<Monitor, NativePlatformFailure> {
    let center_x = client
        .left()
        .checked_add_unsigned(client.width() / 2)
        .ok_or(NativePlatformFailure::InvalidCaptureWindowBounds)?;
    let center_y = client
        .top()
        .checked_add_unsigned(client.height() / 2)
        .ok_or(NativePlatformFailure::InvalidCaptureWindowBounds)?;
    Monitor::from_point(center_x, center_y)
        .map_err(|error| NativePlatformFailure::ClientCapture(error.to_string()))
}

fn monitor_capture_region(
    monitor: &Monitor,
    client: NativeClientAreaBounds,
) -> Result<NativeMonitorCaptureRegion, NativePlatformFailure> {
    let left = monitor
        .x()
        .map_err(|error| NativePlatformFailure::ClientCapture(error.to_string()))?;
    let top = monitor
        .y()
        .map_err(|error| NativePlatformFailure::ClientCapture(error.to_string()))?;
    let width = monitor
        .width()
        .map_err(|error| NativePlatformFailure::ClientCapture(error.to_string()))?;
    let height = monitor
        .height()
        .map_err(|error| NativePlatformFailure::ClientCapture(error.to_string()))?;
    monitor_capture_region_from_bounds(
        NativeMonitorPhysicalBounds {
            left,
            top,
            width,
            height,
        },
        client,
    )
}

fn monitor_capture_region_from_bounds(
    monitor: NativeMonitorPhysicalBounds,
    client: NativeClientAreaBounds,
) -> Result<NativeMonitorCaptureRegion, NativePlatformFailure> {
    let monitor_right = monitor
        .left
        .checked_add_unsigned(monitor.width)
        .ok_or(NativePlatformFailure::ClientOutsideCaptureMonitor)?;
    let monitor_bottom = monitor
        .top
        .checked_add_unsigned(monitor.height)
        .ok_or(NativePlatformFailure::ClientOutsideCaptureMonitor)?;
    if client.left() < monitor.left
        || client.top() < monitor.top
        || client.right() > monitor_right
        || client.bottom() > monitor_bottom
    {
        return Err(NativePlatformFailure::ClientOutsideCaptureMonitor);
    }
    Ok(NativeMonitorCaptureRegion {
        x: u32::try_from(client.left() - monitor.left)
            .map_err(|_| NativePlatformFailure::ClientOutsideCaptureMonitor)?,
        y: u32::try_from(client.top() - monitor.top)
            .map_err(|_| NativePlatformFailure::ClientOutsideCaptureMonitor)?,
        width: client.width(),
        height: client.height(),
    })
}

#[cfg(test)]
mod tests {
    use super::{
        monitor_capture_region_from_bounds, NativeClientAreaBounds, NativeMonitorCaptureRegion,
        NativeMonitorPhysicalBounds, NativePlatformFailure,
    };

    #[test]
    fn fractional_dpi_client_extent_is_captured_without_resampling() {
        let client = NativeClientAreaBounds::new(87, 121, 327, 265).unwrap();
        assert_eq!(
            monitor_capture_region_from_bounds(
                NativeMonitorPhysicalBounds {
                    left: 0,
                    top: 0,
                    width: 3_840,
                    height: 2_160,
                },
                client,
            )
            .unwrap(),
            NativeMonitorCaptureRegion {
                x: 87,
                y: 121,
                width: 240,
                height: 144,
            }
        );
    }

    #[test]
    fn negative_monitor_origin_projects_to_monitor_local_physical_pixels() {
        let client = NativeClientAreaBounds::new(-1_700, 140, -1_460, 284).unwrap();
        assert_eq!(
            monitor_capture_region_from_bounds(
                NativeMonitorPhysicalBounds {
                    left: -1_920,
                    top: 0,
                    width: 1_920,
                    height: 1_080,
                },
                client,
            )
            .unwrap(),
            NativeMonitorCaptureRegion {
                x: 220,
                y: 140,
                width: 240,
                height: 144,
            }
        );
    }

    #[test]
    fn client_crossing_a_monitor_edge_is_rejected_without_partial_capture() {
        let client = NativeClientAreaBounds::new(1_800, 140, 2_040, 284).unwrap();
        assert!(matches!(
            monitor_capture_region_from_bounds(
                NativeMonitorPhysicalBounds {
                    left: 0,
                    top: 0,
                    width: 1_920,
                    height: 1_080,
                },
                client,
            ),
            Err(NativePlatformFailure::ClientOutsideCaptureMonitor)
        ));
    }
}
