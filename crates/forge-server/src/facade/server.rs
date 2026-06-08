use std::io;

use crate::{
    diagnostics::ForgeServerCounterSnapshot, middleware::ForgeServerMiddlewareFacade,
    operator_evidence::ForgeServerOperatorEvidenceFacade,
    query_handoff::ForgeServerQueryHandoffFacade, registration::ForgeServerSurfaceInventory,
    request_context::ForgeServerRequestContextFacade, response::ForgeServerResponseFacade,
    runtime::ForgeServerRuntime, surfaces::ForgeServerSurfacesFacade, transport::serve_runtime,
};

use super::builder::ForgeServerBuilder;

#[derive(Debug)]
pub struct ForgeServer {
    runtime: ForgeServerRuntime,
}

impl ForgeServer {
    pub fn builder() -> ForgeServerBuilder {
        ForgeServerBuilder::default()
    }

    pub(crate) fn new(runtime: ForgeServerRuntime) -> Self {
        Self { runtime }
    }

    pub fn surface_inventory(&self) -> ForgeServerSurfaceInventory {
        self.runtime.assembly().surface_registry().inventory()
    }

    pub fn counters(&self) -> ForgeServerCounterSnapshot {
        self.runtime.assembly().counters().snapshot()
    }

    pub fn request_contexts(&self) -> ForgeServerRequestContextFacade {
        self.runtime.assembly().request_context_facade().clone()
    }

    pub fn middleware(&self) -> ForgeServerMiddlewareFacade {
        self.runtime.assembly().middleware_facade().clone()
    }

    pub fn query_handoff(&self) -> ForgeServerQueryHandoffFacade {
        self.runtime.assembly().query_handoff_facade().clone()
    }

    pub fn operator_evidence(&self) -> ForgeServerOperatorEvidenceFacade {
        self.runtime.assembly().operator_evidence_facade().clone()
    }

    pub fn responses(&self) -> ForgeServerResponseFacade {
        self.runtime.assembly().response_facade().clone()
    }

    pub fn surfaces(&self) -> ForgeServerSurfacesFacade {
        self.runtime.assembly().surfaces_facade().clone()
    }

    pub async fn serve(self) -> io::Result<()> {
        serve_runtime(self.runtime).await
    }
}
