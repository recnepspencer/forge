use std::sync::Arc;

use crate::{
    config::ForgeServerConfig, diagnostics::ForgeServerCounters,
    middleware::ForgeServerMiddlewareFacade, operator_evidence::ForgeServerOperatorEvidenceFacade,
    query_handoff::ForgeServerQueryHandoffFacade, registration::ForgeServerSurfaceRegistry,
    request_context::ForgeServerRequestContextFacade, response::ForgeServerResponseFacade,
    surfaces::ForgeServerSurfacesFacade,
};

#[derive(Debug)]
pub(crate) struct ForgeServerRuntimeAssembly {
    config: ForgeServerConfig,
    surface_registry: ForgeServerSurfaceRegistry,
    surfaces_facade: ForgeServerSurfacesFacade,
    middleware_facade: ForgeServerMiddlewareFacade,
    operator_evidence_facade: ForgeServerOperatorEvidenceFacade,
    query_handoff_facade: ForgeServerQueryHandoffFacade,
    response_facade: ForgeServerResponseFacade,
    request_context_facade: ForgeServerRequestContextFacade,
    counters: Arc<ForgeServerCounters>,
}

impl ForgeServerRuntimeAssembly {
    pub(crate) fn new(
        config: ForgeServerConfig,
        surface_registry: ForgeServerSurfaceRegistry,
        counters: Arc<ForgeServerCounters>,
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
        Self {
            config,
            surface_registry,
            surfaces_facade,
            middleware_facade,
            operator_evidence_facade,
            query_handoff_facade,
            response_facade,
            request_context_facade,
            counters,
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

    pub(crate) fn surfaces_facade(&self) -> &ForgeServerSurfacesFacade {
        &self.surfaces_facade
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
}
