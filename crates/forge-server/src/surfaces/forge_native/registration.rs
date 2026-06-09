use crate::registration::{ForgeServerSurfaceFamily, ForgeServerSurfaceRegistration};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ForgeNativeSurface;

impl ForgeNativeSurface {
    pub fn enabled() -> ForgeServerSurfaceRegistration {
        ForgeServerSurfaceRegistration::enabled(ForgeServerSurfaceFamily::ForgeNative)
    }

    pub fn disabled() -> ForgeServerSurfaceRegistration {
        ForgeServerSurfaceRegistration::disabled(ForgeServerSurfaceFamily::ForgeNative)
    }
}
