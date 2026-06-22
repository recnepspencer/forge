use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    config::ForgeServerConfig,
    diagnostics::ForgeServerCounters,
    middleware::ForgeServerMiddlewareFacade,
    operation_registry::ForgeServerOperationRegistry,
    operator_evidence::ForgeServerOperatorEvidenceFacade,
    product_adapter::ForgeServerProductAdapterRegistry,
    product_operation_contract::ForgeServerStoredProductOperation,
    product_session::{
        default_product_session_clock, ForgeServerProductSessionClock,
        ForgeServerProductSessionRegistry,
    },
    query_handoff::ForgeServerQueryHandoffFacade,
    registration::ForgeServerSurfaceRegistry,
    request_context::ForgeServerRequestContextFacade,
    response::ForgeServerResponseFacade,
    surfaces::{
        compat_http::{ForgeServerStoredBinaryIngress, ForgeServerStoredCompatibilityMutation},
        ForgeServerSurfacesFacade,
    },
    transport::ForgeServerRouteAssembly,
};

#[derive(Debug)]
pub(crate) struct ForgeServerRuntimeAssembly {
    config: ForgeServerConfig,
    surface_registry: ForgeServerSurfaceRegistry,
    operation_registry: ForgeServerOperationRegistry,
    product_adapter_registry: ForgeServerProductAdapterRegistry,
    surfaces_facade: ForgeServerSurfacesFacade,
    middleware_facade: ForgeServerMiddlewareFacade,
    operator_evidence_facade: ForgeServerOperatorEvidenceFacade,
    query_handoff_facade: ForgeServerQueryHandoffFacade,
    response_facade: ForgeServerResponseFacade,
    request_context_facade: ForgeServerRequestContextFacade,
    route_assembly: ForgeServerRouteAssembly,
    counters: Arc<ForgeServerCounters>,
    product_session_registry: ForgeServerProductSessionRegistry,
    compat_http_mutation_replay_store:
        Arc<Mutex<HashMap<String, ForgeServerStoredCompatibilityMutation>>>,
    product_operation_replay_store: Arc<Mutex<HashMap<String, ForgeServerStoredProductOperation>>>,
    compat_http_binary_ingress_store: Arc<Mutex<HashMap<String, ForgeServerStoredBinaryIngress>>>,
}

impl ForgeServerRuntimeAssembly {
    pub(crate) fn new(
        config: ForgeServerConfig,
        surface_registry: ForgeServerSurfaceRegistry,
        operation_registry: ForgeServerOperationRegistry,
        product_adapter_registry: ForgeServerProductAdapterRegistry,
        route_assembly: ForgeServerRouteAssembly,
        counters: Arc<ForgeServerCounters>,
        product_session_clock: Option<Arc<dyn ForgeServerProductSessionClock>>,
    ) -> Self {
        let surfaces_facade = ForgeServerSurfacesFacade::new(&surface_registry);
        let middleware_facade = ForgeServerMiddlewareFacade::new(config.middleware().clone());
        let operator_evidence_facade =
            ForgeServerOperatorEvidenceFacade::new(config.operator_evidence().clone());
        let query_handoff_facade =
            ForgeServerQueryHandoffFacade::new(config.query_handoff().clone());
        let response_facade = ForgeServerResponseFacade::new(config.response().clone());
        let request_context_facade =
            ForgeServerRequestContextFacade::new(config.request_context().clone());
        let product_session_registry = ForgeServerProductSessionRegistry::new(
            counters.clone(),
            product_session_clock.unwrap_or_else(default_product_session_clock),
        );
        Self {
            config,
            surface_registry,
            operation_registry,
            product_adapter_registry,
            surfaces_facade,
            middleware_facade,
            operator_evidence_facade,
            query_handoff_facade,
            response_facade,
            request_context_facade,
            route_assembly,
            counters,
            product_session_registry,
            compat_http_mutation_replay_store: Arc::new(Mutex::new(HashMap::new())),
            product_operation_replay_store: Arc::new(Mutex::new(HashMap::new())),
            compat_http_binary_ingress_store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub(crate) fn config(&self) -> &ForgeServerConfig {
        &self.config
    }

    pub(crate) fn surface_registry(&self) -> &ForgeServerSurfaceRegistry {
        &self.surface_registry
    }

    pub(crate) fn request_context_facade(&self) -> &ForgeServerRequestContextFacade {
        &self.request_context_facade
    }

    pub(crate) fn operation_registry(&self) -> &ForgeServerOperationRegistry {
        &self.operation_registry
    }

    pub(crate) fn surfaces_facade(&self) -> &ForgeServerSurfacesFacade {
        &self.surfaces_facade
    }

    pub(crate) fn product_adapter_registry(&self) -> &ForgeServerProductAdapterRegistry {
        &self.product_adapter_registry
    }

    pub(crate) fn middleware_facade(&self) -> &ForgeServerMiddlewareFacade {
        &self.middleware_facade
    }

    pub(crate) fn operator_evidence_facade(&self) -> &ForgeServerOperatorEvidenceFacade {
        &self.operator_evidence_facade
    }

    pub(crate) fn query_handoff_facade(&self) -> &ForgeServerQueryHandoffFacade {
        &self.query_handoff_facade
    }

    pub(crate) fn response_facade(&self) -> &ForgeServerResponseFacade {
        &self.response_facade
    }

    pub(crate) fn counters(&self) -> &Arc<ForgeServerCounters> {
        &self.counters
    }

    pub(crate) fn route_assembly(&self) -> &ForgeServerRouteAssembly {
        &self.route_assembly
    }

    pub(crate) fn product_session_registry(&self) -> &ForgeServerProductSessionRegistry {
        &self.product_session_registry
    }

    pub(crate) fn compat_http_mutation_replay_store(
        &self,
    ) -> &Arc<Mutex<HashMap<String, ForgeServerStoredCompatibilityMutation>>> {
        &self.compat_http_mutation_replay_store
    }

    pub(crate) fn compat_http_binary_ingress_store(
        &self,
    ) -> &Arc<Mutex<HashMap<String, ForgeServerStoredBinaryIngress>>> {
        &self.compat_http_binary_ingress_store
    }

    pub(crate) fn product_operation_replay_store(
        &self,
    ) -> &Arc<Mutex<HashMap<String, ForgeServerStoredProductOperation>>> {
        &self.product_operation_replay_store
    }
}
