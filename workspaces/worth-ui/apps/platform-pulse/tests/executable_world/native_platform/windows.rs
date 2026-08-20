use std::sync::OnceLock;
use std::thread;
use std::time::{Duration, Instant};

use uiautomation::patterns::UIWindowPattern;
use uiautomation::types::Handle;
use uiautomation::UIAutomation;
use winsafe::{self as win, co, HwndPlace, HWND, POINT, SIZE};
use xcap::Window;

use crate::external_observation::{
    NativeClientPixelCapture, NativeClientPixelPoint, NativeInputDeliveryObservation,
    NativeInputProbeKind, NativeWindowIdentity, NormalNativeCloseRequestObservation,
    ProcessBoundNativeClientAreaObservation,
};

use super::contract::sealed::Sealed;
use super::{NativePlatformContract, NativePlatformFailure};

mod capture_region;
mod client_capture;
mod environment;
mod gdi_capture;
mod input_delivery;
mod observation_readiness;
mod process_windows;

use capture_region::{
    capture_bound_client_area, client_bounds, monitor_capture_region, monitor_for_client,
};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct WindowsNativePlatform {
    _private: (),
}

pub(crate) struct WindowsProcessBoundNativeClientArea {
    window: HWND,
    capture_window: Window,
    observation: ProcessBoundNativeClientAreaObservation,
}

struct WindowsCaptureExposure<'bound> {
    bound: &'bound WindowsProcessBoundNativeClientArea,
}

impl Drop for WindowsCaptureExposure<'_> {
    fn drop(&mut self) {
        let restoration = self.bound.window.SetWindowPos(
            HwndPlace::Place(co::HWND_PLACE::NOTOPMOST),
            POINT::default(),
            SIZE::default(),
            co::SWP::NOMOVE | co::SWP::NOSIZE | co::SWP::NOACTIVATE,
        );
        if let Err(error) = restoration {
            assert!(
                !self.bound.window.IsWindow(),
                "capture exposure must restore ordinary z-order for a live window: {error}"
            );
        }
    }
}

impl WindowsNativePlatform {
    pub(crate) fn certified() -> Result<Self, NativePlatformFailure> {
        if std::env::consts::ARCH != "x86_64" {
            return Err(NativePlatformFailure::EnvironmentQualification(
                std::env::consts::ARCH.to_owned(),
            ));
        }
        static DPI_AWARENESS: OnceLock<Result<(), String>> = OnceLock::new();
        match DPI_AWARENESS
            .get_or_init(|| win::SetProcessDPIAware().map_err(|error| error.to_string()))
        {
            Ok(()) => Ok(Self { _private: () }),
            Err(error) => Err(NativePlatformFailure::DpiAwareness(error.clone())),
        }
    }

    pub(crate) fn observed_os_version(&self) -> Result<String, NativePlatformFailure> {
        let output = std::process::Command::new("cmd")
            .args(["/c", "ver"])
            .output()
            .map_err(|error| NativePlatformFailure::EnvironmentQualification(error.to_string()))?;
        let version = String::from_utf8(output.stdout)
            .map_err(|error| NativePlatformFailure::EnvironmentQualification(error.to_string()))?
            .trim()
            .to_owned();
        if output.status.success() && environment::qualified_windows_11_version(&version) {
            Ok(version)
        } else {
            Err(NativePlatformFailure::EnvironmentQualification(version))
        }
    }

    fn expose_bound_client_area<'bound>(
        &self,
        bound: &'bound WindowsProcessBoundNativeClientArea,
    ) -> Result<WindowsCaptureExposure<'bound>, NativePlatformFailure> {
        self.observe_bound_client_area(bound)?;
        bound
            .window
            .SetWindowPos(
                HwndPlace::Place(co::HWND_PLACE::TOPMOST),
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
        let monitor = monitor_for_client(client)?;
        let region = monitor_capture_region(&monitor, client)?;
        let monitor_capture =
            capture_bound_client_area(&monitor, region, bound.observation.process_id())?;
        let window_capture = client_capture::capture_client_area(
            &bound.capture_window,
            client,
            bound.observation.process_id(),
        )?;
        require_matching_capture_sources(&monitor_capture, &window_capture)?;
        let gdi_capture = gdi_capture::capture_client_area(client, bound.observation.process_id())?;
        require_matching_composited_sources(&monitor_capture, &gdi_capture)?;
        Ok(monitor_capture)
    }
}

fn require_matching_capture_sources(
    monitor: &NativeClientPixelCapture,
    window: &NativeClientPixelCapture,
) -> Result<(), NativePlatformFailure> {
    if monitor.process_id() != window.process_id()
        || monitor.width() != window.width()
        || monitor.height() != window.height()
    {
        return Err(capture_mismatch(monitor, window));
    }
    for (x, y) in capture_control_points(monitor) {
        let observed = capture_pixel(monitor, x, y);
        let source = capture_pixel(window, x, y);
        if source[3] != 255 || observed != source {
            return Err(capture_mismatch(monitor, window));
        }
    }
    Ok(())
}

fn capture_mismatch(
    monitor: &NativeClientPixelCapture,
    window: &NativeClientPixelCapture,
) -> NativePlatformFailure {
    NativePlatformFailure::ClientCapture(format!(
        "independent capture mismatch: monitor={:?}; window={:?}",
        capture_signature(monitor),
        capture_signature(window),
    ))
}

fn require_matching_composited_sources(
    monitor: &NativeClientPixelCapture,
    gdi: &NativeClientPixelCapture,
) -> Result<(), NativePlatformFailure> {
    let same_identity = monitor.process_id() == gdi.process_id()
        && monitor.width() == gdi.width()
        && monitor.height() == gdi.height();
    let same_rgb = capture_control_points(monitor)
        .into_iter()
        .all(|(x, y)| capture_pixel(monitor, x, y)[..3] == capture_pixel(gdi, x, y)[..3]);
    if same_identity && same_rgb {
        Ok(())
    } else {
        Err(capture_mismatch(monitor, gdi))
    }
}

fn capture_control_points(capture: &NativeClientPixelCapture) -> [(u32, u32); 3] {
    [
        (capture.width() / 4, capture.height() / 4),
        (capture.width() / 2, capture.height() / 2),
        (capture.width() * 3 / 4, capture.height() * 3 / 4),
    ]
}

fn capture_pixel(capture: &NativeClientPixelCapture, x: u32, y: u32) -> [u8; 4] {
    let index = ((y * capture.width() + x) * 4) as usize;
    capture.rgba()[index..index + 4].try_into().unwrap()
}

fn capture_signature(capture: &NativeClientPixelCapture) -> ([u32; 2], [[u8; 4]; 3]) {
    let points = [
        (0, 0),
        (capture.width() / 2, capture.height() / 2),
        (capture.width() - 1, capture.height() - 1),
    ];
    let pixels = points.map(|(x, y)| {
        let index = ((y * capture.width() + x) * 4) as usize;
        capture.rgba()[index..index + 4].try_into().unwrap()
    });
    ([capture.width(), capture.height()], pixels)
}

#[test]
fn independent_window_capture_rejects_monitor_pixel_substitution() {
    let monitor = NativeClientPixelCapture::new(
        7,
        4,
        1,
        vec![
            1, 2, 3, 4, 47, 129, 247, 255, 47, 129, 247, 255, 47, 129, 247, 255,
        ],
    )
    .unwrap();
    let exact = NativeClientPixelCapture::new(7, 4, 1, monitor.rgba().to_vec()).unwrap();
    assert!(require_matching_capture_sources(&monitor, &exact).is_ok());
    let gdi = NativeClientPixelCapture::new(
        7,
        4,
        1,
        vec![
            9, 9, 9, 0, 47, 129, 247, 0, 47, 129, 247, 0, 47, 129, 247, 0,
        ],
    )
    .unwrap();
    assert!(require_matching_composited_sources(&monitor, &gdi).is_ok());
    let substituted = NativeClientPixelCapture::new(
        7,
        4,
        1,
        vec![
            1, 2, 3, 4, 47, 129, 247, 255, 1, 2, 3, 255, 47, 129, 247, 255,
        ],
    )
    .unwrap();
    assert!(matches!(
        require_matching_capture_sources(&substituted, &exact),
        Err(NativePlatformFailure::ClientCapture(_))
    ));
    assert!(matches!(
        require_matching_composited_sources(&substituted, &gdi),
        Err(NativePlatformFailure::ClientCapture(_))
    ));
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
        let mut prior_candidate = None;
        let mut stable_observations = 0_u8;
        loop {
            lookup_count = lookup_count.saturating_add(1);
            let mut candidates = process_windows::enumerate(process_id)?;
            match candidates.len() {
                0 if Instant::now() < deadline => {
                    thread::sleep(Duration::from_millis(20));
                }
                0 => return Err(NativePlatformFailure::WindowLookupDeadline),
                1 => {
                    let candidate = candidates.pop().expect("one process window");
                    let identity = candidate.window.ptr() as usize;
                    let posture = (identity, candidate.bounds);
                    if prior_candidate == Some(posture) {
                        stable_observations = stable_observations.saturating_add(1);
                    } else {
                        prior_candidate = Some(posture);
                        stable_observations = 1;
                    }
                    if stable_observations < 3 && Instant::now() < deadline {
                        thread::sleep(Duration::from_millis(20));
                        continue;
                    }
                    let window_identity = NativeWindowIdentity::from_native_value(identity)
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
        if bound.capture_window.pid().ok() != Some(owner_process_id)
            || bound.capture_window.id().ok() != Some(bound.window.ptr() as u32)
        {
            return Err(NativePlatformFailure::BoundWindowOwnerChanged);
        }
        if client_bounds(&bound.window)? != bound.observation.bounds() {
            return Err(NativePlatformFailure::BoundClientAreaChanged);
        }
        Ok(bound.observation)
    }

    fn await_external_observation_ready(
        &self,
        bound: &Self::BoundClientArea,
        deadline: Instant,
    ) -> Result<ProcessBoundNativeClientAreaObservation, NativePlatformFailure> {
        observation_readiness::await_ready(self, bound, deadline)
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

    fn deliver_pointer_activation(
        &self,
        bound: &Self::BoundClientArea,
        point: NativeClientPixelPoint,
    ) -> Result<NativeInputDeliveryObservation, NativePlatformFailure> {
        let observed = self.observe_bound_client_area(bound)?;
        input_delivery::deliver_pointer(&bound.window, observed, point)
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
        let windows = process_windows::enumerate(process_id)?;
        if windows.is_empty() {
            Ok(())
        } else {
            Err(NativePlatformFailure::ProcessWindowResidue(windows.len()))
        }
    }
}
