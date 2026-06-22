use forge_proof::TransitionOutcome;
use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerCompatibilityDenial, ForgeServerCompatibilityRequest,
    ForgeServerCompatibilityRequestInput, ForgeServerConfig, ForgeServerMiddlewareConfig,
    ForgeServerRequestContextConfig,
};

pub(crate) fn compat_http_admission_test_server() -> ForgeServer {
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
        .register_operations(forge_server::ForgeServerOperationRegistration::phase_two_defaults())
        .register_surface(ForgeNativeSurface::enabled())
        .register_surface(CompatHttpSurface::phase_one_enabled())
        .build()
        .expect("server should build")
}

pub(crate) fn compat_http_denial(
    server: &ForgeServer,
    input: ForgeServerCompatibilityRequestInput,
) -> ForgeServerCompatibilityDenial {
    match server.compat_http().request(input) {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected denied compat request, got {other:?}"),
    }
}

pub(crate) fn compat_http_request(
    server: &ForgeServer,
    input: ForgeServerCompatibilityRequestInput,
) -> ForgeServerCompatibilityRequest {
    match server.compat_http().request(input) {
        TransitionOutcome::Success(request) => request,
        other => panic!("expected successful compat request, got {other:?}"),
    }
}
