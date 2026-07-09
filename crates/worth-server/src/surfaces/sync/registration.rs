use crate::registration::{WorthServerSurfaceFamily, WorthServerSurfaceRegistration};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SyncSurface;

impl SyncSurface {
    pub fn disabled() -> WorthServerSurfaceRegistration {
        WorthServerSurfaceRegistration::disabled(WorthServerSurfaceFamily::Sync)
    }
}
