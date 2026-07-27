use std::fmt;
use std::time::Instant;

#[cfg(target_os = "windows")]
use crate::external_observation::{
    NativeClientPixelCapture, NormalNativeCloseRequestObservation,
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
    WindowEnumeration(String),
    WindowLookupDeadline,
    AmbiguousProcessWindows(usize),
    CaptureWindowMissing,
    CaptureWindowAmbiguous(usize),
    ClientCapture(String),
    InvalidCaptureWindowBounds,
    BoundWindowMissing,
    BoundWindowOwnerChanged,
    InvalidClientCapture {
        image_width: u32,
        image_height: u32,
        outer: crate::external_observation::NativeClientAreaBounds,
        client: crate::external_observation::NativeClientAreaBounds,
    },
    NormalClose(String),
    ProcessWindowResidue(usize),
}

impl fmt::Display for NativePlatformFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DpiAwareness(error) => {
                write!(formatter, "establish process DPI awareness: {error}")
            }
            Self::WindowEnumeration(error) => {
                write!(formatter, "enumerate process windows: {error}")
            }
            Self::WindowLookupDeadline => {
                formatter.write_str("process-bound native window lookup deadline elapsed")
            }
            Self::AmbiguousProcessWindows(count) => {
                write!(formatter, "found {count} visible process windows")
            }
            Self::CaptureWindowMissing => {
                formatter.write_str("process-bound native capture window is missing")
            }
            Self::CaptureWindowAmbiguous(count) => {
                write!(formatter, "found {count} process-bound native capture windows")
            }
            Self::ClientCapture(error) => write!(formatter, "capture native client area: {error}"),
            Self::InvalidCaptureWindowBounds => {
                formatter.write_str("native capture window reported invalid bounds")
            }
            Self::BoundWindowMissing => {
                formatter.write_str("the process-bound native window no longer exists")
            }
            Self::BoundWindowOwnerChanged => {
                formatter.write_str("the bound native window no longer belongs to the child")
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

    fn capture_client_area(
        &self,
        bound: &Self::BoundClientArea,
    ) -> Result<NativeClientPixelCapture, NativePlatformFailure>;

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
