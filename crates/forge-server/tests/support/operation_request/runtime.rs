#![allow(dead_code)]

use forge_proof::{TransitionOutcome, TransitionReadiness};
use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerCompatHttpRouteFamily, ForgeServerCompatibilityPreparedRequest,
    ForgeServerCompatibilityRequestInput, ForgeServerConfig, ForgeServerMiddlewareConfig,
    ForgeServerOperationRegistration, ForgeServerPipelineInput, ForgeServerPipelineIntent,
    ForgeServerRequestContextConfig, ForgeServerRequestContextInput,
    ForgeServerResolvedRequestContext, ForgeServerSurfaceFamily, ForgeServerTransportClass,
};

pub(crate) fn operation_request_test_server() -> ForgeServer {
    operation_request_test_server_with_operations(
        ForgeServerOperationRegistration::phase_two_defaults(),
    )
}

pub(crate) fn operation_request_test_server_with_operations(
    registrations: impl IntoIterator<Item = ForgeServerOperationRegistration>,
) -> ForgeServer {
    ForgeServer::builder()
        .with_config(
            ForgeServerConfig::builder()
                .with_bind_address(([127, 0, 0, 1], 8080).into())
                .with_request_context_config(
                    ForgeServerRequestContextConfig::builder()
                        .with_default_diagnostics_profile(DiagnosticRichnessProfile::Standard)
                        .with_preview_targeting_enabled(true)
                        .build()
                        .expect("request context config should validate"),
                )
                .with_middleware_config(
                    ForgeServerMiddlewareConfig::builder()
                        .with_query_mutation_enabled(true)
                        .build()
                        .expect("middleware config should validate"),
                )
                .build()
                .expect("server config should validate"),
        )
        .register_operations(registrations)
        .register_surface(ForgeNativeSurface::enabled())
        .register_surface(CompatHttpSurface::phase_one_enabled())
        .build()
        .expect("server should build")
}

pub(crate) fn compat_prepared_request(
    server: &ForgeServer,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
    route_family: ForgeServerCompatHttpRouteFamily,
    method: &str,
    path: &str,
    basis_digest: Option<&str>,
    idempotency_key: Option<&str>,
) -> ForgeServerCompatibilityPreparedRequest {
    let mut builder = compat_request_input(route_family, method, path);
    if let Some(diagnostics_profile) = diagnostics_profile {
        builder = builder.with_diagnostics_profile(diagnostics_profile);
    }
    if let Some(basis_digest) = basis_digest {
        builder = builder.with_query_pair("basis", basis_digest);
    }
    if let Some(idempotency_key) = idempotency_key {
        builder = builder.with_header("idempotency-key", idempotency_key);
    }
    match server.compat_http().prepare_request(
        builder
            .build()
            .expect("compat request should validate structurally"),
    ) {
        TransitionOutcome::Success(prepared_request) => prepared_request,
        other => panic!("expected prepared compatibility request, got {other:?}"),
    }
}

pub(crate) fn compat_request_input(
    route_family: ForgeServerCompatHttpRouteFamily,
    method: &str,
    path: &str,
) -> forge_server::ForgeServerCompatibilityRequestInputBuilder {
    ForgeServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .with_branch_id("branch-9")
        .with_route_family(route_family)
        .with_method(method)
        .with_path(path)
}

pub(crate) fn forge_native_resolved_context(
    server: &ForgeServer,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
) -> ForgeServerResolvedRequestContext {
    forge_native_resolved_context_for_principal(server, diagnostics_profile, "principal-7")
}

pub(crate) fn forge_native_resolved_context_for_principal(
    server: &ForgeServer,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
    principal_id: &str,
) -> ForgeServerResolvedRequestContext {
    let mut builder = ForgeServerRequestContextInput::builder()
        .with_surface_family(ForgeServerSurfaceFamily::ForgeNative)
        .with_transport_class(ForgeServerTransportClass::ForgeNativeInProcess)
        .with_authenticated_principal_id(principal_id)
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .with_branch_id("branch-9");
    if let Some(diagnostics_profile) = diagnostics_profile {
        builder = builder.with_diagnostics_profile(diagnostics_profile);
    }
    match server.request_contexts().resolve(
        builder
            .build()
            .expect("forge-native request context should validate"),
    ) {
        TransitionReadiness::Ready(resolved) => resolved,
        other => panic!("expected resolved request context, got {other:?}"),
    }
}

pub(crate) fn forge_native_admission(
    server: &ForgeServer,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
    operation_name: &str,
) -> forge_server::ForgeServerAdmission {
    match server.middleware().admit(ForgeServerPipelineInput::new(
        forge_native_resolved_context(server, diagnostics_profile),
        ForgeServerPipelineIntent::query_mutation(operation_name),
    )) {
        TransitionOutcome::Success(admission) => admission,
        other => panic!("expected admitted forge-native operation, got {other:?}"),
    }
}
