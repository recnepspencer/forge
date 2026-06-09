use crate::{
    registration::ForgeServerSurfaceRegistry,
    surfaces::{
        ForgeServerSurfaceCapabilities, ForgeServerSurfaceFamilyMarker, ForgeServerSurfaceRoot,
        ForgeServerTypedSurfaceRoot,
    },
    ForgeServerSurfaceFamily,
};

use super::ForgeServerCompatHttpRouteFamilies;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompatHttpSurfaceFamilyMarker;

impl ForgeServerSurfaceFamilyMarker for CompatHttpSurfaceFamilyMarker {
    const FAMILY: ForgeServerSurfaceFamily = ForgeServerSurfaceFamily::CompatHttp;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompatHttpSurfaceRoot {
    root: ForgeServerTypedSurfaceRoot<CompatHttpSurfaceFamilyMarker>,
    route_families: ForgeServerCompatHttpRouteFamilies,
}

impl CompatHttpSurfaceRoot {
    pub(crate) fn new(surface_registry: &ForgeServerSurfaceRegistry) -> Self {
        Self {
            root: ForgeServerTypedSurfaceRoot::new(surface_registry),
            route_families: surface_registry.compat_http_route_families(),
        }
    }

    pub fn capabilities(&self) -> &ForgeServerSurfaceCapabilities {
        self.root.capabilities()
    }

    pub fn route_families(&self) -> &ForgeServerCompatHttpRouteFamilies {
        &self.route_families
    }
}

impl ForgeServerSurfaceRoot for CompatHttpSurfaceRoot {
    fn family(&self) -> ForgeServerSurfaceFamily {
        ForgeServerSurfaceFamily::CompatHttp
    }

    fn capabilities(&self) -> &ForgeServerSurfaceCapabilities {
        self.root.capabilities()
    }
}
