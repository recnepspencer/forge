use std::io;

use tokio::net::TcpListener;

use crate::{
    declaration_intake::ForgeServerDirectDeclarationIntakeFacade,
    runtime::ForgeServerRuntime,
    surfaces::compat_http::ForgeServerCompatibilityFacade,
    transport::{route_assembly::project_axum_router, ForgeServerOperationRouter},
};

pub(crate) async fn serve_runtime(runtime: ForgeServerRuntime) -> io::Result<()> {
    let listener =
        TcpListener::bind(runtime.assembly().config().bind_address().socket_addr()).await?;
    runtime.assembly().counters().increment_serve_start_count();
    let _surface_inventory = runtime.assembly().surface_registry().inventory();
    let compat_http = ForgeServerCompatibilityFacade::new(
        runtime.assembly().surfaces_facade().compat_http(),
        runtime.assembly().operation_registry().clone(),
        runtime.assembly().product_adapter_registry().clone(),
        runtime.assembly().product_session_registry().clone(),
        runtime.assembly().request_context_facade().clone(),
        runtime.assembly().middleware_facade().clone(),
        ForgeServerDirectDeclarationIntakeFacade::new(
            runtime.assembly().config().query_handoff().clone(),
        ),
        runtime.assembly().query_handoff_facade().clone(),
        runtime.assembly().response_facade().clone(),
        runtime.assembly().operator_evidence_facade().clone(),
        runtime
            .assembly()
            .compat_http_mutation_replay_store()
            .clone(),
        runtime.assembly().product_operation_replay_store().clone(),
        runtime
            .assembly()
            .compat_http_binary_ingress_store()
            .clone(),
    );
    let router = project_axum_router(
        runtime.assembly().route_assembly(),
        ForgeServerOperationRouter::new(runtime.assembly().route_assembly().clone(), compat_http),
    );
    axum::serve(listener, router).await
}
