use crate::registration::{ForgeServerSurfaceFamily, ForgeServerSurfaceRegistration};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LeaseSurface;

impl LeaseSurface {
    pub fn disabled() -> ForgeServerSurfaceRegistration {
        ForgeServerSurfaceRegistration::disabled(ForgeServerSurfaceFamily::Lease)
    }
}
