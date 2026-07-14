use crate::registration::{WorthServerSurfaceFamily, WorthServerSurfaceRegistration};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthNativeSurface;

impl WorthNativeSurface {
    pub fn enabled() -> WorthServerSurfaceRegistration {
        WorthServerSurfaceRegistration::enabled(WorthServerSurfaceFamily::WorthNative)
    }

    pub fn disabled() -> WorthServerSurfaceRegistration {
        WorthServerSurfaceRegistration::disabled(WorthServerSurfaceFamily::WorthNative)
    }
}
