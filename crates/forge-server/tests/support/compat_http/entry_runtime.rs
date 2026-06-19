use forge_proof::{TransitionOutcome, TransitionReadiness};
use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerCompatibilityRequestInput, ForgeServerConfig,
    ForgeServerRequestContextConfig, ForgeServerRequestContextDenial,
    ForgeServerRequestContextInput, ForgeServerResolvedRequestContext, ForgeServerSurfaceFamily,
    ForgeServerTransportClass,
};

pub(crate) fn compat_http_entry_test_server() -> ForgeServer {
    ForgeServer::builder()
        .with_config(
            ForgeServerConfig::builder()
                .with_bind_address(([127, 0, 0, 1], 8080).into())
                .with_request_context_config(
                    ForgeServerRequestContextConfig::builder()
                        .with_preview_targeting_enabled(true)
                        .with_default_diagnostics_profile(DiagnosticRichnessProfile::Standard)
                        .build()
                        .expect("request context config should validate"),
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

pub(crate) fn compat_http_prepared_request(
    server: &ForgeServer,
    input: ForgeServerCompatibilityRequestInput,
) -> forge_server::ForgeServerCompatibilityPreparedRequest {
    match server.compat_http().prepare_request(input) {
        TransitionOutcome::Success(request) => request,
        other => panic!("expected admitted compat request, got {other:?}"),
    }
}

pub(crate) fn ready_context(
    resolution: TransitionReadiness<
        ForgeServerResolvedRequestContext,
        ForgeServerRequestContextDenial,
        forge_server::ForgeServerRequestContextDeferred,
        forge_server::ForgeServerRequestContextStale,
        forge_server::ForgeServerRequestContextRebindRequired,
        forge_server::ForgeServerRequestContextFailure,
    >,
) -> ForgeServerResolvedRequestContext {
    match resolution {
        TransitionReadiness::Ready(context) => context,
        other => panic!("expected ready request context, got {other:?}"),
    }
}

pub(crate) fn forge_native_request_context_input() -> ForgeServerRequestContextInput {
    ForgeServerRequestContextInput::builder()
        .with_surface_family(ForgeServerSurfaceFamily::ForgeNative)
        .with_transport_class(ForgeServerTransportClass::ForgeNativeInProcess)
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .with_branch_id("branch-9")
        .build()
        .expect("forge-native input should validate")
}
