use crate::registration::ForgeServerSurfaceRegistry;

use super::{
    binary::BinarySurfaceRoot, compat_http::CompatHttpSurfaceRoot,
    forge_native::ForgeNativeSurfaceRoot, integration::IntegrationSurfaceRoot,
    lease::LeaseSurfaceRoot, sync::SyncSurfaceRoot,
};

#[derive(Clone, Debug)]
pub struct ForgeServerSurfacesFacade {
    forge_native: ForgeNativeSurfaceRoot,
    compat_http: CompatHttpSurfaceRoot,
    sync: SyncSurfaceRoot,
    lease: LeaseSurfaceRoot,
    binary: BinarySurfaceRoot,
    integration: IntegrationSurfaceRoot,
}

impl ForgeServerSurfacesFacade {
    pub(crate) fn new(surface_registry: &ForgeServerSurfaceRegistry) -> Self {
        Self {
            forge_native: ForgeNativeSurfaceRoot::new(surface_registry),
            compat_http: CompatHttpSurfaceRoot::new(surface_registry),
            sync: SyncSurfaceRoot::new(surface_registry),
            lease: LeaseSurfaceRoot::new(surface_registry),
            binary: BinarySurfaceRoot::new(surface_registry),
            integration: IntegrationSurfaceRoot::new(surface_registry),
        }
    }

    pub fn forge_native(&self) -> ForgeNativeSurfaceRoot {
        self.forge_native
    }

    pub fn compat_http(&self) -> CompatHttpSurfaceRoot {
        self.compat_http.clone()
    }

    pub fn sync(&self) -> SyncSurfaceRoot {
        self.sync
    }

    pub fn lease(&self) -> LeaseSurfaceRoot {
        self.lease
    }

    pub fn binary(&self) -> BinarySurfaceRoot {
        self.binary
    }

    pub fn integration(&self) -> IntegrationSurfaceRoot {
        self.integration
    }
}
