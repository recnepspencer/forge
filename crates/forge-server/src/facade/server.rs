use std::io;

use crate::{
    declaration_intake::ForgeServerDirectDeclarationIntakeFacade,
    diagnostics::ForgeServerCounterSnapshot,
    forge_native::ForgeServerForgeNativeFacade,
    middleware::ForgeServerMiddlewareFacade,
    operation_admission::ForgeServerOperationAdmissionFacade,
    operation_planning::ForgeServerOperationPlanner,
    operation_readiness::ForgeServerOperationReadinessFacade,
    operation_registry::{ForgeServerOperationInventory, ForgeServerOperationRegistry},
    operation_request::ForgeServerOperationRequestFacade,
    operation_runtime_certification::ForgeServerProductOperationRuntimeCertificationFacade,
    operation_scheduler::ForgeServerOperationScheduler,
    operator_evidence::ForgeServerOperatorEvidenceFacade,
    product_adapter::{
        ForgeServerProductAdapterRegistrationReceipt, ForgeServerProductOperationRuntime,
    },
    product_session::ForgeServerProductSessionRegistry,
    query_dependency_audit::ForgeServerQueryDependencyAuditFacade,
    query_handoff::ForgeServerQueryHandoffFacade,
    registration::ForgeServerSurfaceInventory,
    request_context::ForgeServerRequestContextFacade,
    response::ForgeServerResponseFacade,
    runtime::ForgeServerRuntime,
    surfaces::compat_http::ForgeServerCompatibilityFacade,
    surfaces::ForgeServerSurfacesFacade,
    transport::serve_runtime,
    transport::{
        ForgeServerOperationRouter, ForgeServerProjectedRouter, ForgeServerRouteInventory,
    },
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

    pub fn operation_inventory(&self) -> ForgeServerOperationInventory {
        self.runtime.assembly().operation_registry().inventory()
    }

    pub fn route_inventory(&self) -> ForgeServerRouteInventory {
        self.runtime.assembly().route_assembly().inventory().clone()
    }

    pub fn operation_registry(&self) -> ForgeServerOperationRegistry {
        self.runtime.assembly().operation_registry().clone()
    }

    pub fn operation_requests(&self) -> ForgeServerOperationRequestFacade {
        ForgeServerOperationRequestFacade::new(self.runtime.assembly().operation_registry().clone())
    }

    pub fn operation_admissions(&self) -> ForgeServerOperationAdmissionFacade {
        ForgeServerOperationAdmissionFacade::with_operation_registry(
            self.runtime.assembly().operation_registry().clone(),
        )
    }

    pub fn operation_readiness(&self) -> ForgeServerOperationReadinessFacade {
        ForgeServerOperationReadinessFacade::with_operation_registry(
            self.runtime.assembly().operation_registry().clone(),
        )
    }

    pub fn operation_planner(&self) -> ForgeServerOperationPlanner {
        ForgeServerOperationPlanner::with_operation_registry(
            self.runtime.assembly().config().query_handoff().clone(),
            self.runtime.assembly().operation_registry().clone(),
        )
    }

    pub fn operation_scheduler(&self) -> ForgeServerOperationScheduler {
        ForgeServerOperationScheduler::new(self.responses())
    }

    pub fn product_adapter_inventory(&self) -> &[ForgeServerProductAdapterRegistrationReceipt] {
        self.runtime
            .assembly()
            .product_adapter_registry()
            .receipts()
    }

    pub fn product_operation_runtime(&self) -> ForgeServerProductOperationRuntime {
        ForgeServerProductOperationRuntime::new(
            self.runtime.assembly().operation_registry().clone(),
            self.runtime.assembly().product_adapter_registry().clone(),
            self.runtime.assembly().config().query_handoff().clone(),
            self.runtime.assembly().product_session_registry().clone(),
            self.runtime
                .assembly()
                .product_operation_replay_store()
                .clone(),
        )
    }

    pub fn product_session_registry(&self) -> ForgeServerProductSessionRegistry {
        self.runtime.assembly().product_session_registry().clone()
    }

    pub fn operation_runtime_certification(
        &self,
    ) -> ForgeServerProductOperationRuntimeCertificationFacade {
        ForgeServerProductOperationRuntimeCertificationFacade::new(
            self.query_dependency_audit(),
            self.operation_inventory(),
            self.route_inventory(),
            self.product_adapter_inventory().to_vec(),
        )
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

    pub fn query_dependency_audit(&self) -> ForgeServerQueryDependencyAuditFacade {
        ForgeServerQueryDependencyAuditFacade::new(
            self.request_contexts(),
            self.runtime.assembly().config().query_handoff().clone(),
        )
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
            self.runtime.assembly().operation_registry().clone(),
            self.runtime.assembly().product_adapter_registry().clone(),
            self.runtime.assembly().product_session_registry().clone(),
            self.runtime
                .assembly()
                .product_operation_replay_store()
                .clone(),
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
            self.runtime.assembly().operation_registry().clone(),
            self.runtime.assembly().product_adapter_registry().clone(),
            self.runtime.assembly().product_session_registry().clone(),
            self.request_contexts(),
            self.middleware(),
            ForgeServerDirectDeclarationIntakeFacade::new(
                self.runtime.assembly().config().query_handoff().clone(),
            ),
            self.query_handoff(),
            self.responses(),
            self.operator_evidence(),
            self.runtime
                .assembly()
                .compat_http_mutation_replay_store()
                .clone(),
            self.runtime
                .assembly()
                .product_operation_replay_store()
                .clone(),
            self.runtime
                .assembly()
                .compat_http_binary_ingress_store()
                .clone(),
        )
    }

    pub fn operation_router(&self) -> ForgeServerOperationRouter {
        ForgeServerOperationRouter::new(
            self.runtime.assembly().route_assembly().clone(),
            self.compat_http(),
        )
    }

    pub fn projected_router(&self) -> ForgeServerProjectedRouter {
        ForgeServerProjectedRouter::new(crate::transport::project_axum_router(
            self.runtime.assembly().route_assembly(),
            self.operation_router(),
        ))
    }

    pub async fn serve(self) -> io::Result<()> {
        serve_runtime(self.runtime).await
    }
}
