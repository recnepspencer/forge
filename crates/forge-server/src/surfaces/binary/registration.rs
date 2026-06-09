use crate::registration::{ForgeServerSurfaceFamily, ForgeServerSurfaceRegistration};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BinarySurface;

impl BinarySurface {
    pub fn disabled() -> ForgeServerSurfaceRegistration {
        ForgeServerSurfaceRegistration::disabled(ForgeServerSurfaceFamily::Binary)
    }
}
