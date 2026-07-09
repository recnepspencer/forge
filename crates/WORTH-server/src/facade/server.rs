use std::io;

use crate::{
    declaration_intake::WorthServerDirectDeclarationIntakeFacade,
    diagnostics::WorthServerCounterSnapshot,
    worth_native::WorthServerWorthNativeFacade,
    middleware::WorthServerMiddlewareFacade,
    operation_admission::WorthServerOperationAdmissionFacade,
    operation_planning::WorthServerOperationPlanner,
    operation_readiness::WorthServerOperationReadinessFacade,
    operation_registry::{WorthServerOperationInventory, WorthServerOperationRegistry},
    operation_request::WorthServerOperationRequestFacade,
    operation_runtime_certification::WorthServerProductOperationRuntimeCertificationFacade,
    operation_scheduler::WorthServerOperationScheduler,
    operator_evidence::WorthServerOperatorEvidenceFacade,
    product_adapter::{
        WorthServerProductAdapterRegistrationReceipt, WorthServerProductOperationRuntime,
    },
    product_session::WorthServerProductSessionRegistry,
    query_dependency_audit::WorthServerQueryDependencyAuditFacade,
    query_handoff::WorthServerQueryHandoffFacade,
    registration::WorthServerSurfaceInventory,
    request_context::WorthServerRequestContextFacade,
    response::WorthServerResponseFacade,
    runtime::WorthServerRuntime,
    surfaces::compat_http::WorthServerCompatibilityFacade,
    surfaces::WorthServerSurfacesFacade,
    transport::serve_runtime,
    transport::{
        WorthServerOperationRouter, WorthServerProjectedRouter, WorthServerRouteInventory,
    },
};

use super::builder::WorthServerBuilder;

#[derive(Debug)]
pub struct WorthServer {
    runtime: WorthServerRuntime,
}

impl WorthServer {
    pub fn builder() -> WorthServerBuilder {
        WorthServerBuilder::default()
    }

    pub(crate) fn new(runtime: WorthServerRuntime) -> Self {
        Self { runtime }
    }

    pub fn surface_inventory(&self) -> WorthServerSurfaceInventory {
        self.runtime.assembly().surface_registry().inventory()
    }

    pub fn counters(&self) -> WorthServerCounterSnapshot {
        self.runtime.assembly().counters().snapshot()
    }

    pub fn operation_inventory(&self) -> WorthServerOperationInventory {
        self.runtime.assembly().operation_registry().inventory()
    }

    pub fn route_inventory(&self) -> WorthServerRouteInventory {
        self.runtime.assembly().route_assembly().inventory().clone()
    }

    pub fn operation_registry(&self) -> WorthServerOperationRegistry {
        self.runtime.assembly().operation_registry().clone()
    }

    pub fn operation_requests(&self) -> WorthServerOperationRequestFacade {
        WorthServerOperationRequestFacade::new(self.runtime.assembly().operation_registry().clone())
    }

    pub fn operation_admissions(&self) -> WorthServerOperationAdmissionFacade {
        WorthServerOperationAdmissionFacade::with_operation_registry(
            self.runtime.assembly().operation_registry().clone(),
        )
    }

    pub fn operation_readiness(&self) -> WorthServerOperationReadinessFacade {
        WorthServerOperationReadinessFacade::with_operation_registry(
            self.runtime.assembly().operation_registry().clone(),
        )
    }

    pub fn operation_planner(&self) -> WorthServerOperationPlanner {
        WorthServerOperationPlanner::with_operation_registry(
            self.runtime.assembly().config().query_handoff().clone(),
            self.runtime.assembly().operation_registry().clone(),
        )
    }

    pub fn operation_scheduler(&self) -> WorthServerOperationScheduler {
        WorthServerOperationScheduler::new(self.responses())
    }

    pub fn product_adapter_inventory(&self) -> &[WorthServerProductAdapterRegistrationReceipt] {
        self.runtime
            .assembly()
            .product_adapter_registry()
            .receipts()
    }

    pub fn product_operation_runtime(&self) -> WorthServerProductOperationRuntime {
        WorthServerProductOperationRuntime::new(
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

    pub fn product_session_registry(&self) -> WorthServerProductSessionRegistry {
        self.runtime.assembly().product_session_registry().clone()
    }

    pub fn operation_runtime_certification(
        &self,
    ) -> WorthServerProductOperationRuntimeCertificationFacade {
        WorthServerProductOperationRuntimeCertificationFacade::new(
            self.query_dependency_audit(),
            self.operation_inventory(),
            self.route_inventory(),
            self.product_adapter_inventory().to_vec(),
        )
    }

    pub fn request_contexts(&self) -> WorthServerRequestContextFacade {
        self.runtime.assembly().request_context_facade().clone()
    }

    pub fn middleware(&self) -> WorthServerMiddlewareFacade {
        self.runtime.assembly().middleware_facade().clone()
    }

    pub fn query_handoff(&self) -> WorthServerQueryHandoffFacade {
        self.runtime.assembly().query_handoff_facade().clone()
    }

    pub fn query_dependency_audit(&self) -> WorthServerQueryDependencyAuditFacade {
        WorthServerQueryDependencyAuditFacade::new(
            self.request_contexts(),
            self.runtime.assembly().config().query_handoff().clone(),
        )
    }

    pub fn operator_evidence(&self) -> WorthServerOperatorEvidenceFacade {
        self.runtime.assembly().operator_evidence_facade().clone()
    }

    pub fn responses(&self) -> WorthServerResponseFacade {
        self.runtime.assembly().response_facade().clone()
    }

    pub fn surfaces(&self) -> WorthServerSurfacesFacade {
        self.runtime.assembly().surfaces_facade().clone()
    }

    pub fn worth_native(&self) -> WorthServerWorthNativeFacade {
        WorthServerWorthNativeFacade::new(
            self.runtime.assembly().surfaces_facade().worth_native(),
            self.runtime.assembly().operation_registry().clone(),
            self.runtime.assembly().product_adapter_registry().clone(),
            self.runtime.assembly().product_session_registry().clone(),
            self.runtime
                .assembly()
                .product_operation_replay_store()
                .clone(),
            self.request_contexts(),
            self.middleware(),
            WorthServerDirectDeclarationIntakeFacade::new(
                self.runtime.assembly().config().query_handoff().clone(),
            ),
            self.query_handoff(),
            self.responses(),
        )
    }

    pub fn compat_http(&self) -> WorthServerCompatibilityFacade {
        WorthServerCompatibilityFacade::new(
            self.runtime.assembly().surfaces_facade().compat_http(),
            self.runtime.assembly().operation_registry().clone(),
            self.runtime.assembly().product_adapter_registry().clone(),
            self.runtime.assembly().product_session_registry().clone(),
            self.request_contexts(),
            self.middleware(),
            WorthServerDirectDeclarationIntakeFacade::new(
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

    pub fn operation_router(&self) -> WorthServerOperationRouter {
        WorthServerOperationRouter::new(
            self.runtime.assembly().route_assembly().clone(),
            self.compat_http(),
        )
    }

    pub fn projected_router(&self) -> WorthServerProjectedRouter {
        WorthServerProjectedRouter::new(crate::transport::project_axum_router(
            self.runtime.assembly().route_assembly(),
            self.operation_router(),
        ))
    }

    pub async fn serve(self) -> io::Result<()> {
        serve_runtime(self.runtime).await
    }
}
