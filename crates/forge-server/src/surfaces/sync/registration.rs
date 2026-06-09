use crate::registration::{ForgeServerSurfaceFamily, ForgeServerSurfaceRegistration};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SyncSurface;

impl SyncSurface {
    pub fn disabled() -> ForgeServerSurfaceRegistration {
        ForgeServerSurfaceRegistration::disabled(ForgeServerSurfaceFamily::Sync)
    }
}
