use std::sync::Arc;

use crate::{
    config::ForgeServerConfig, diagnostics::ForgeServerCounters,
    registration::ForgeServerSurfaceRegistry,
};

#[derive(Debug)]
pub(crate) struct ForgeServerRuntimeAssembly {
    config: ForgeServerConfig,
    surface_registry: ForgeServerSurfaceRegistry,
    counters: Arc<ForgeServerCounters>,
}

impl ForgeServerRuntimeAssembly {
    pub(crate) fn new(
        config: ForgeServerConfig,
        surface_registry: ForgeServerSurfaceRegistry,
        counters: Arc<ForgeServerCounters>,
    ) -> Self {
        Self {
            config,
            surface_registry,
            counters,
        }
    }

    pub(crate) fn config(&self) -> &ForgeServerConfig {
        &self.config
    }

    pub(crate) fn surface_registry(&self) -> &ForgeServerSurfaceRegistry {
        &self.surface_registry
    }

    pub(crate) fn counters(&self) -> &Arc<ForgeServerCounters> {
        &self.counters
    }
}
