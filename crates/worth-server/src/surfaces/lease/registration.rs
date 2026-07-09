use crate::registration::{WorthServerSurfaceFamily, WorthServerSurfaceRegistration};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LeaseSurface;

impl LeaseSurface {
    pub fn disabled() -> WorthServerSurfaceRegistration {
        WorthServerSurfaceRegistration::disabled(WorthServerSurfaceFamily::Lease)
    }
}
