use crate::registration::{WorthServerSurfaceFamily, WorthServerSurfaceRegistration};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IntegrationSurface;

impl IntegrationSurface {
    pub fn disabled() -> WorthServerSurfaceRegistration {
        WorthServerSurfaceRegistration::disabled(WorthServerSurfaceFamily::Integration)
    }
}
