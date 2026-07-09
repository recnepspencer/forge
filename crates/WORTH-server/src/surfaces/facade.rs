use crate::registration::WorthServerSurfaceRegistry;

use super::{
    binary::BinarySurfaceRoot, compat_http::CompatHttpSurfaceRoot,
    worth_native::WorthNativeSurfaceRoot, integration::IntegrationSurfaceRoot,
    lease::LeaseSurfaceRoot, sync::SyncSurfaceRoot,
};

#[derive(Clone, Debug)]
pub struct WorthServerSurfacesFacade {
    worth_native: WorthNativeSurfaceRoot,
    compat_http: CompatHttpSurfaceRoot,
    sync: SyncSurfaceRoot,
    lease: LeaseSurfaceRoot,
    binary: BinarySurfaceRoot,
    integration: IntegrationSurfaceRoot,
}

impl WorthServerSurfacesFacade {
    pub(crate) fn new(surface_registry: &WorthServerSurfaceRegistry) -> Self {
        Self {
            worth_native: WorthNativeSurfaceRoot::new(surface_registry),
            compat_http: CompatHttpSurfaceRoot::new(surface_registry),
            sync: SyncSurfaceRoot::new(surface_registry),
            lease: LeaseSurfaceRoot::new(surface_registry),
            binary: BinarySurfaceRoot::new(surface_registry),
            integration: IntegrationSurfaceRoot::new(surface_registry),
        }
    }

    pub fn worth_native(&self) -> WorthNativeSurfaceRoot {
        self.worth_native
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
