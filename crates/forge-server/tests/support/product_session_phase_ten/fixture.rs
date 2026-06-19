use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use forge_proof::TransitionOutcome;
use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerCompatHttpRouteFamily, ForgeServerCompatibilityPreparedRequest,
    ForgeServerCompatibilityRequestInput, ForgeServerConfig, ForgeServerForgeNativeSession,
    ForgeServerForgeNativeSessionInput, ForgeServerMiddlewareConfig,
    ForgeServerProductApplicationAdapter, ForgeServerProductApplicationAdapterRegistration,
    ForgeServerProductOperationBasisKind, ForgeServerProductOperationDeclaration,
    ForgeServerProductOperationErrorMaps, ForgeServerProductOperationPayload,
    ForgeServerProductOperationSuccess, ForgeServerProductOperationSupportSnapshot,
    ForgeServerProductSessionClock, ForgeServerQueryHandoffConfig, ForgeServerRequestContextConfig,
};
use serde_json::json;

mod manual_clock;

pub use manual_clock::ManualProductSessionClock;

pub fn build_server(
    registrations: Vec<ForgeServerProductApplicationAdapterRegistration>,
) -> ForgeServer {
    build_server_with_clock(registrations, None)
}

pub fn build_server_with_clock(
    registrations: Vec<ForgeServerProductApplicationAdapterRegistration>,
    product_session_clock: Option<Arc<dyn ForgeServerProductSessionClock>>,
) -> ForgeServer {
    let mut builder = ForgeServer::builder()
        .with_config(base_config())
        .register_operations(forge_server::ForgeServerOperationRegistration::phase_two_defaults())
        .register_surface(ForgeNativeSurface::enabled())
        .register_surface(CompatHttpSurface::phase_one_enabled())
        .register_product_adapters(registrations);
    if let Some(product_session_clock) = product_session_clock {
        builder = builder.with_product_session_clock(product_session_clock);
    }
    builder
        .build()
        .expect("phase ten product-session server should build")
}

pub fn prepared_session_request(
    server: &ForgeServer,
    workspace_id: &str,
    branch_id: &str,
    operation_name: &str,
) -> ForgeServerCompatibilityPreparedRequest {
    prepared_request(
        server,
        workspace_id,
        branch_id,
        ForgeServerCompatHttpRouteFamily::Mutation,
        "POST",
        &format!("/compat/mutations/{operation_name}"),
        None,
    )
}

pub fn prepared_product_read_request(
    server: &ForgeServer,
    workspace_id: &str,
    branch_id: &str,
    operation_name: &str,
    basis_digest: Option<&str>,
) -> ForgeServerCompatibilityPreparedRequest {
    prepared_request(
        server,
        workspace_id,
        branch_id,
        ForgeServerCompatHttpRouteFamily::Read,
        "GET",
        &format!("/compat/reads/{operation_name}"),
        basis_digest,
    )
}

pub fn prepared_product_mutation_request(
    server: &ForgeServer,
    workspace_id: &str,
    branch_id: &str,
    operation_name: &str,
    basis_digest: Option<&str>,
) -> ForgeServerCompatibilityPreparedRequest {
    prepared_request(
        server,
        workspace_id,
        branch_id,
        ForgeServerCompatHttpRouteFamily::Mutation,
        "POST",
        &format!("/compat/mutations/{operation_name}"),
        basis_digest,
    )
}

fn prepared_request(
    server: &ForgeServer,
    workspace_id: &str,
    branch_id: &str,
    route_family: ForgeServerCompatHttpRouteFamily,
    method: &str,
    path: &str,
    basis_digest: Option<&str>,
) -> ForgeServerCompatibilityPreparedRequest {
    let mut builder = ForgeServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id(workspace_id)
        .with_branch_id(branch_id)
        .with_route_family(route_family)
        .with_method(method)
        .with_path(path)
        .with_header("accept", "application/json");
    if let Some(basis_digest) = basis_digest {
        builder = builder.with_query_pair("basis", basis_digest);
    }
    match server.compat_http().prepare_request(
        builder
            .build()
            .expect("compatibility request input should validate"),
    ) {
        TransitionOutcome::Success(prepared_request) => prepared_request,
        other => panic!("expected prepared compatibility request, got {other:?}"),
    }
}

pub fn direct_session(
    server: &ForgeServer,
    workspace_id: &str,
    branch_id: &str,
) -> ForgeServerForgeNativeSession {
    match server.forge_native().session(
        ForgeServerForgeNativeSessionInput::builder()
            .with_authenticated_principal_id("principal-7")
            .with_tenant_id("tenant-a")
            .with_workspace_id(workspace_id)
            .with_branch_id(branch_id)
            .build()
            .expect("forge-native session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected forge-native session, got {other:?}"),
    }
}

pub fn preview_payload() -> ForgeServerProductOperationPayload {
    ForgeServerProductOperationPayload::json(
        "product-editor.render-preview.v1",
        json!({ "document": "doc-7" }),
    )
}

pub fn apply_payload() -> ForgeServerProductOperationPayload {
    apply_payload_with_title("Rename")
}

pub fn apply_payload_with_title(title: &str) -> ForgeServerProductOperationPayload {
    ForgeServerProductOperationPayload::json("product-editor.apply.v1", json!({ "title": title }))
}

pub fn prepared_product_mutation_request_with_basis_and_header(
    server: &ForgeServer,
    workspace_id: &str,
    branch_id: &str,
    operation_name: &str,
    basis_digest: &str,
    header_name: &str,
    header_value: &str,
) -> ForgeServerCompatibilityPreparedRequest {
    let builder = ForgeServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id(workspace_id)
        .with_branch_id(branch_id)
        .with_route_family(ForgeServerCompatHttpRouteFamily::Mutation)
        .with_method("POST")
        .with_path(&format!("/compat/mutations/{operation_name}"))
        .with_query_pair("basis", basis_digest)
        .with_header("accept", "application/json")
        .with_header(header_name, header_value);
    match server.compat_http().prepare_request(
        builder
            .build()
            .expect("compatibility request input should validate"),
    ) {
        TransitionOutcome::Success(prepared_request) => prepared_request,
        other => panic!("expected prepared compatibility request, got {other:?}"),
    }
}

pub fn session_backed_editor_registration(
    calls: Arc<AtomicUsize>,
) -> ForgeServerProductApplicationAdapterRegistration {
    ForgeServerProductApplicationAdapterRegistration::new(
        "session-backed-editor",
        Arc::new(SessionBackedEditorAdapter { calls }),
    )
    .with_operation(declared(
        ForgeServerProductOperationDeclaration::product_read(
            "product_editor.render_preview",
            "product-editor.render-preview.v1",
            ForgeServerProductOperationBasisKind::ProductSessionDerived,
            ForgeServerProductOperationSupportSnapshot::production_admitted("preview-session"),
        ),
    ))
    .with_operation(declared(
        ForgeServerProductOperationDeclaration::product_mutation(
            "product_editor.apply",
            "product-editor.apply.v1",
            ForgeServerProductOperationBasisKind::ProductSessionDerived,
            ForgeServerProductOperationSupportSnapshot::production_admitted("apply-session"),
            "draft",
        ),
    ))
}

fn base_config() -> ForgeServerConfig {
    ForgeServerConfig::builder()
        .with_bind_address(([127, 0, 0, 1], 8080).into())
        .with_request_context_config(
            ForgeServerRequestContextConfig::builder()
                .with_default_diagnostics_profile(DiagnosticRichnessProfile::Standard)
                .build()
                .expect("request context config should validate"),
        )
        .with_middleware_config(
            ForgeServerMiddlewareConfig::builder()
                .with_query_mutation_enabled(true)
                .build()
                .expect("middleware config should validate"),
        )
        .with_query_handoff_config(
            ForgeServerQueryHandoffConfig::builder()
                .build()
                .expect("query handoff config should validate"),
        )
        .build()
        .expect("server config should validate")
}

fn declared(
    declaration: ForgeServerProductOperationDeclaration,
) -> ForgeServerProductOperationDeclaration {
    declaration.with_error_map(ForgeServerProductOperationErrorMaps::passthrough())
}

#[derive(Debug)]
struct SessionBackedEditorAdapter {
    calls: Arc<AtomicUsize>,
}

impl ForgeServerProductApplicationAdapter for SessionBackedEditorAdapter {
    fn execute(
        &self,
        operation: &forge_server::ForgeServerScheduledProductOperation,
    ) -> Result<
        ForgeServerProductOperationSuccess,
        forge_server::ForgeServerProductAdapterExecutionError,
    > {
        self.calls.fetch_add(1, Ordering::Relaxed);
        Ok(ForgeServerProductOperationSuccess::new(
            operation.plan().declaration().operation_name(),
            format!(
                "{}:{}",
                operation.plan().declaration().operation_name(),
                operation
                    .plan()
                    .operation_admission()
                    .operation_request()
                    .identity()
                    .product_session_identity()
                    .unwrap_or("none")
            ),
        ))
    }
}
