use std::sync::Arc;

use crate::{
    config::ForgeServerConfig,
    diagnostics::ForgeServerCounters,
    operation_registry::{
        ForgeServerOperationRegistration, ForgeServerOperationRegistry,
        ForgeServerOperationRegistryError,
    },
    product_adapter::{
        ForgeServerProductAdapterRegistry, ForgeServerProductAdapterRegistryError,
        ForgeServerProductApplicationAdapterRegistration,
    },
    product_session::ForgeServerProductSessionClock,
    registration::{
        ForgeServerSurfaceRegistration, ForgeServerSurfaceRegistry, ForgeServerSurfaceRegistryError,
    },
    runtime::{ForgeServerRuntime, ForgeServerRuntimeAssembly},
    transport::ForgeServerRouteAssemblyError,
};

use super::server::ForgeServer;

#[derive(Debug, Default)]
pub struct ForgeServerBuilder {
    config: Option<ForgeServerConfig>,
    surface_registrations: Vec<ForgeServerSurfaceRegistration>,
    operation_registrations: Vec<ForgeServerOperationRegistration>,
    product_adapter_registrations: Vec<ForgeServerProductApplicationAdapterRegistration>,
    product_session_clock: Option<Arc<dyn ForgeServerProductSessionClock>>,
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

    pub fn register_operation(mut self, registration: ForgeServerOperationRegistration) -> Self {
        self.operation_registrations.push(registration);
        self
    }

    pub fn register_operations(
        mut self,
        registrations: impl IntoIterator<Item = ForgeServerOperationRegistration>,
    ) -> Self {
        self.operation_registrations.extend(registrations);
        self
    }

    pub fn register_product_adapter(
        mut self,
        registration: ForgeServerProductApplicationAdapterRegistration,
    ) -> Self {
        self.product_adapter_registrations.push(registration);
        self
    }

    pub fn register_product_adapters(
        mut self,
        registrations: impl IntoIterator<Item = ForgeServerProductApplicationAdapterRegistration>,
    ) -> Self {
        self.product_adapter_registrations.extend(registrations);
        self
    }

    pub fn with_product_session_clock(
        mut self,
        product_session_clock: Arc<dyn ForgeServerProductSessionClock>,
    ) -> Self {
        self.product_session_clock = Some(product_session_clock);
        self
    }

    pub fn build(self) -> Result<ForgeServer, ForgeServerBuildError> {
        let config = self.config.ok_or(ForgeServerBuildError::MissingConfig)?;
        let counters = Arc::new(ForgeServerCounters::default());
        let surface_registry =
            ForgeServerSurfaceRegistry::build(self.surface_registrations, counters.as_ref())
                .map_err(ForgeServerBuildError::InvalidSurfaceRegistry)?;
        let operation_registry =
            ForgeServerOperationRegistry::build(self.operation_registrations, counters.as_ref())
                .map_err(ForgeServerBuildError::InvalidOperationRegistry)?;
        let product_adapter_registry =
            ForgeServerProductAdapterRegistry::build(self.product_adapter_registrations)
                .map_err(ForgeServerBuildError::InvalidProductAdapterRegistry)?;
        let route_assembly = crate::transport::ForgeServerRouteAssembly::assemble(
            &crate::surfaces::CompatHttpSurfaceRoot::new(&surface_registry),
            &operation_registry,
            &product_adapter_registry,
        )
        .map_err(ForgeServerBuildError::InvalidRouteAssembly)?;
        let assembly = ForgeServerRuntimeAssembly::new(
            config,
            surface_registry,
            operation_registry,
            product_adapter_registry,
            route_assembly,
            counters,
            self.product_session_clock,
        );
        let runtime = ForgeServerRuntime::from_assembly(assembly);
        Ok(ForgeServer::new(runtime))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerBuildError {
    MissingConfig,
    InvalidSurfaceRegistry(ForgeServerSurfaceRegistryError),
    InvalidOperationRegistry(ForgeServerOperationRegistryError),
    InvalidProductAdapterRegistry(ForgeServerProductAdapterRegistryError),
    InvalidRouteAssembly(ForgeServerRouteAssemblyError),
}
