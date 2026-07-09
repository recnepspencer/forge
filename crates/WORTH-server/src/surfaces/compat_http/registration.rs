use crate::registration::{WorthServerSurfaceFamily, WorthServerSurfaceRegistration};

use super::WorthServerCompatHttpRouteFamilies;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CompatHttpSurface;

impl CompatHttpSurface {
    pub fn enabled(
        route_families: WorthServerCompatHttpRouteFamilies,
    ) -> WorthServerSurfaceRegistration {
        WorthServerSurfaceRegistration::enabled_compat_http(
            WorthServerSurfaceFamily::CompatHttp,
            route_families,
        )
    }

    pub fn phase_one_enabled() -> WorthServerSurfaceRegistration {
        Self::enabled(WorthServerCompatHttpRouteFamilies::all_phase_one())
    }

    pub fn disabled() -> WorthServerSurfaceRegistration {
        WorthServerSurfaceRegistration::disabled_compat_http(
            WorthServerSurfaceFamily::CompatHttp,
            WorthServerCompatHttpRouteFamilies::all_phase_one(),
        )
    }
}
