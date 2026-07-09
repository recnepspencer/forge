use crate::registration::{WorthServerSurfaceFamily, WorthServerSurfaceRegistration};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BinarySurface;

impl BinarySurface {
    pub fn disabled() -> WorthServerSurfaceRegistration {
        WorthServerSurfaceRegistration::disabled(WorthServerSurfaceFamily::Binary)
    }
}
