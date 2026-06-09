use crate::registration::{ForgeServerSurfaceFamily, ForgeServerSurfaceRegistration};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IntegrationSurface;

impl IntegrationSurface {
    pub fn disabled() -> ForgeServerSurfaceRegistration {
        ForgeServerSurfaceRegistration::disabled(ForgeServerSurfaceFamily::Integration)
    }
}
