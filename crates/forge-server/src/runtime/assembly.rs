use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    config::ForgeServerConfig,
    diagnostics::ForgeServerCounters,
    middleware::ForgeServerMiddlewareFacade,
    operator_evidence::ForgeServerOperatorEvidenceFacade,
    query_handoff::ForgeServerQueryHandoffFacade,
    registration::ForgeServerSurfaceRegistry,
    request_context::ForgeServerRequestContextFacade,
    response::ForgeServerResponseFacade,
    surfaces::{
        compat_http::{ForgeServerStoredBinaryIngress, ForgeServerStoredCompatibilityMutation},
        ForgeServerSurfacesFacade,
    },
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
    compat_http_mutation_replay_store:
        Arc<Mutex<HashMap<String, ForgeServerStoredCompatibilityMutation>>>,
    compat_http_binary_ingress_store: Arc<Mutex<HashMap<String, ForgeServerStoredBinaryIngress>>>,
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
            compat_http_mutation_replay_store: Arc::new(Mutex::new(HashMap::new())),
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
}
