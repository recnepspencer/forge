use std::fmt;
use std::time::Instant;

#[cfg(target_os = "windows")]
use crate::external_observation::{
    NativeClientPixelCapture, NativeClientPixelPoint, NativeInputDeliveryObservation,
    NativeInputProbeKind, NormalNativeCloseRequestObservation,
    ProcessBoundNativeClientAreaObservation,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NativePlatformPosture {
    CertifiedExecutable,
    #[cfg(not(target_os = "windows"))]
    CompileOnly,
}

#[derive(Debug)]
pub(crate) enum NativePlatformFailure {
    DpiAwareness(String),
    EnvironmentQualification(String),
    WindowEnumeration(String),
    WindowLookupDeadline,
    ExternalObservationDeadline,
    AmbiguousProcessWindows(usize),
    ClientCapture(String),
    ClientExposure(String),
    InvalidCaptureWindowBounds,
    BoundWindowMissing,
    BoundWindowOwnerChanged,
    BoundClientAreaChanged,
    ClientOutsideCaptureMonitor,
    InvalidClientCapture {
        image_width: u32,
        image_height: u32,
        outer: crate::external_observation::NativeClientAreaBounds,
        client: crate::external_observation::NativeClientAreaBounds,
    },
    NormalClose(String),
    InputDelivery(String),
    ProcessWindowResidue(usize),
}

impl fmt::Display for NativePlatformFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DpiAwareness(error) => {
                write!(formatter, "establish process DPI awareness: {error}")
            }
            Self::EnvironmentQualification(error) => {
                write!(formatter, "qualify native environment: {error}")
            }
            Self::WindowEnumeration(error) => {
                write!(formatter, "enumerate process windows: {error}")
            }
            Self::WindowLookupDeadline => {
                formatter.write_str("process-bound native window lookup deadline elapsed")
            }
            Self::ExternalObservationDeadline => {
                formatter.write_str("owner-issued external observation readiness deadline elapsed")
            }
            Self::AmbiguousProcessWindows(count) => {
                write!(formatter, "found {count} visible process windows")
            }
            Self::ClientCapture(error) => write!(formatter, "capture native client area: {error}"),
            Self::ClientExposure(error) => {
                write!(formatter, "expose native client area for capture: {error}")
            }
            Self::InvalidCaptureWindowBounds => {
                formatter.write_str("native capture window reported invalid bounds")
            }
            Self::BoundWindowMissing => {
                formatter.write_str("the process-bound native window no longer exists")
            }
            Self::BoundWindowOwnerChanged => {
                formatter.write_str("the bound native window no longer belongs to the child")
            }
            Self::BoundClientAreaChanged => {
                formatter.write_str("the bound native client area changed after observation")
            }
            Self::ClientOutsideCaptureMonitor => {
                formatter.write_str("the native client area is not contained by one monitor")
            }
            Self::InvalidClientCapture {
                image_width,
                image_height,
                outer,
                client,
            } => write!(
                formatter,
                "native client crop is invalid: image={image_width}x{image_height}, outer={outer:?}, client={client:?}"
            ),
            Self::NormalClose(error) => {
                write!(formatter, "request normal native-window close: {error}")
            }
            Self::InputDelivery(error) => write!(formatter, "deliver native input: {error}"),
            Self::ProcessWindowResidue(count) => {
                write!(formatter, "{count} process window(s) remained after exit")
            }
        }
    }
}

#[cfg(target_os = "windows")]
pub(crate) trait NativePlatformContract: sealed::Sealed {
    type BoundClientArea;

    fn bind_process_client_area(
        &self,
        process_id: u32,
        deadline: Instant,
    ) -> Result<Self::BoundClientArea, NativePlatformFailure>;

    fn observe_bound_client_area(
        &self,
        bound: &Self::BoundClientArea,
    ) -> Result<ProcessBoundNativeClientAreaObservation, NativePlatformFailure>;

    fn await_external_observation_ready(
        &self,
        bound: &Self::BoundClientArea,
        deadline: Instant,
    ) -> Result<ProcessBoundNativeClientAreaObservation, NativePlatformFailure>;

    fn capture_client_area(
        &self,
        bound: &Self::BoundClientArea,
    ) -> Result<NativeClientPixelCapture, NativePlatformFailure>;

    fn deliver_input_reachability_probe(
        &self,
        bound: &Self::BoundClientArea,
        kind: NativeInputProbeKind,
    ) -> Result<NativeInputDeliveryObservation, NativePlatformFailure>;

    fn deliver_pointer_activation(
        &self,
        bound: &Self::BoundClientArea,
        point: NativeClientPixelPoint,
    ) -> Result<NativeInputDeliveryObservation, NativePlatformFailure>;

    fn request_normal_close(
        &self,
        bound: &Self::BoundClientArea,
    ) -> Result<NormalNativeCloseRequestObservation, NativePlatformFailure>;

    fn verify_process_window_released(&self, process_id: u32) -> Result<(), NativePlatformFailure>;
}

#[cfg(target_os = "windows")]
pub(crate) mod sealed {
    pub trait Sealed {}
}
