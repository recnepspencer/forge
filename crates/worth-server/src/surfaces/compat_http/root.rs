use crate::{
    registration::WorthServerSurfaceRegistry,
    surfaces::{
        WorthServerSurfaceCapabilities, WorthServerSurfaceFamilyMarker, WorthServerSurfaceRoot,
        WorthServerTypedSurfaceRoot,
    },
    WorthServerSurfaceFamily,
};

use super::WorthServerCompatHttpRouteFamilies;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompatHttpSurfaceFamilyMarker;

impl WorthServerSurfaceFamilyMarker for CompatHttpSurfaceFamilyMarker {
    const FAMILY: WorthServerSurfaceFamily = WorthServerSurfaceFamily::CompatHttp;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompatHttpSurfaceRoot {
    root: WorthServerTypedSurfaceRoot<CompatHttpSurfaceFamilyMarker>,
    route_families: WorthServerCompatHttpRouteFamilies,
}

impl CompatHttpSurfaceRoot {
    pub(crate) fn new(surface_registry: &WorthServerSurfaceRegistry) -> Self {
        Self {
            root: WorthServerTypedSurfaceRoot::new(surface_registry),
            route_families: surface_registry.compat_http_route_families(),
        }
    }

    pub fn capabilities(&self) -> &WorthServerSurfaceCapabilities {
        self.root.capabilities()
    }

    pub fn route_families(&self) -> &WorthServerCompatHttpRouteFamilies {
        &self.route_families
    }
}

impl WorthServerSurfaceRoot for CompatHttpSurfaceRoot {
    fn family(&self) -> WorthServerSurfaceFamily {
        WorthServerSurfaceFamily::CompatHttp
    }

    fn capabilities(&self) -> &WorthServerSurfaceCapabilities {
        self.root.capabilities()
    }
}
