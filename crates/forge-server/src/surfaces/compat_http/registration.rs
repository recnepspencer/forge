use crate::registration::{ForgeServerSurfaceFamily, ForgeServerSurfaceRegistration};

use super::ForgeServerCompatHttpRouteFamilies;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CompatHttpSurface;

impl CompatHttpSurface {
    pub fn enabled(
        route_families: ForgeServerCompatHttpRouteFamilies,
    ) -> ForgeServerSurfaceRegistration {
        ForgeServerSurfaceRegistration::enabled_compat_http(
            ForgeServerSurfaceFamily::CompatHttp,
            route_families,
        )
    }

    pub fn phase_one_enabled() -> ForgeServerSurfaceRegistration {
        Self::enabled(ForgeServerCompatHttpRouteFamilies::all_phase_one())
    }

    pub fn disabled() -> ForgeServerSurfaceRegistration {
        ForgeServerSurfaceRegistration::disabled_compat_http(
            ForgeServerSurfaceFamily::CompatHttp,
            ForgeServerCompatHttpRouteFamilies::all_phase_one(),
        )
    }
}
