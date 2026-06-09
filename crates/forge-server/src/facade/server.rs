use std::io;

use crate::{
    declaration_intake::ForgeServerDirectDeclarationIntakeFacade,
    diagnostics::ForgeServerCounterSnapshot, forge_native::ForgeServerForgeNativeFacade,
    middleware::ForgeServerMiddlewareFacade, operator_evidence::ForgeServerOperatorEvidenceFacade,
    query_handoff::ForgeServerQueryHandoffFacade, registration::ForgeServerSurfaceInventory,
    request_context::ForgeServerRequestContextFacade, response::ForgeServerResponseFacade,
    runtime::ForgeServerRuntime, surfaces::compat_http::ForgeServerCompatibilityFacade,
    surfaces::ForgeServerSurfacesFacade, transport::serve_runtime,
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

    pub fn forge_native(&self) -> ForgeServerForgeNativeFacade {
        ForgeServerForgeNativeFacade::new(
            self.runtime.assembly().surfaces_facade().forge_native(),
            self.request_contexts(),
            self.middleware(),
            ForgeServerDirectDeclarationIntakeFacade::new(
                self.runtime.assembly().config().query_handoff().clone(),
            ),
            self.query_handoff(),
            self.responses(),
        )
    }

    pub fn compat_http(&self) -> ForgeServerCompatibilityFacade {
        ForgeServerCompatibilityFacade::new(
            self.runtime.assembly().surfaces_facade().compat_http(),
            self.request_contexts(),
            self.middleware(),
            ForgeServerDirectDeclarationIntakeFacade::new(
                self.runtime.assembly().config().query_handoff().clone(),
            ),
            self.query_handoff(),
            self.responses(),
            self.runtime
                .assembly()
                .compat_http_mutation_replay_store()
                .clone(),
        )
    }

    pub async fn serve(self) -> io::Result<()> {
        serve_runtime(self.runtime).await
    }
}
