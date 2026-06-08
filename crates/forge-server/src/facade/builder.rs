use std::sync::Arc;

use crate::{
    config::ForgeServerConfig,
    diagnostics::ForgeServerCounters,
    registration::{
        ForgeServerSurfaceRegistration, ForgeServerSurfaceRegistry, ForgeServerSurfaceRegistryError,
    },
    runtime::{ForgeServerRuntime, ForgeServerRuntimeAssembly},
};

use super::server::ForgeServer;

#[derive(Debug, Default)]
pub struct ForgeServerBuilder {
    config: Option<ForgeServerConfig>,
    surface_registrations: Vec<ForgeServerSurfaceRegistration>,
}

impl ForgeServerBuilder {
    pub fn with_config(mut self, config: ForgeServerConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn register_surface(mut self, registration: ForgeServerSurfaceRegistration) -> Self {
        self.surface_registrations.push(registration);
        self
    }

    pub fn build(self) -> Result<ForgeServer, ForgeServerBuildError> {
        let config = self.config.ok_or(ForgeServerBuildError::MissingConfig)?;
        let counters = Arc::new(ForgeServerCounters::default());
        let surface_registry =
            ForgeServerSurfaceRegistry::build(self.surface_registrations, counters.as_ref())
                .map_err(ForgeServerBuildError::InvalidSurfaceRegistry)?;
        let assembly = ForgeServerRuntimeAssembly::new(config, surface_registry, counters);
        let runtime = ForgeServerRuntime::from_assembly(assembly);
        Ok(ForgeServer::new(runtime))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerBuildError {
    MissingConfig,
    InvalidSurfaceRegistry(ForgeServerSurfaceRegistryError),
}
