use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerConfig, ForgeServerMiddlewareConfig, ForgeServerQueryHandoffConfig,
    ForgeServerQueryWorkspaceProvider, ForgeServerRequestContextConfig,
};

pub(crate) fn build_phase_ten_server_with_workspace_provider(
    workspace_provider: impl ForgeServerQueryWorkspaceProvider,
) -> ForgeServer {
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
                .with_middleware_config(
                    ForgeServerMiddlewareConfig::builder()
                        .with_compat_http_maximum_diagnostics_profile(
                            DiagnosticRichnessProfile::Forensic,
                        )
                        .with_query_mutation_enabled(true)
                        .build()
                        .expect("middleware config should validate"),
                )
                .with_query_handoff_config(
                    ForgeServerQueryHandoffConfig::builder()
                        .with_workspace_provider(workspace_provider)
                        .build()
                        .expect("query handoff config should validate"),
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
