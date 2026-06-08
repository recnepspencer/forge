use forge_proof::{TransitionOutcome, TransitionReadiness};
use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface, SyncSurface},
    ForgeServer, ForgeServerAdmission, ForgeServerConfig, ForgeServerMiddlewareConfig,
    ForgeServerQueryHandoff, ForgeServerQueryHandoffConfig, ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffOutcome, ForgeServerQueryWorkspaceProvider,
    ForgeServerRequestContextConfig, ForgeServerRequestContextInput,
    ForgeServerResolvedRequestContext, ForgeServerSurfaceFamily, ForgeServerTransportClass,
};

pub(crate) fn test_server(
    workspace_provider: impl ForgeServerQueryWorkspaceProvider,
    include_sync_surface: bool,
) -> ForgeServer {
    test_server_with_middleware(
        workspace_provider,
        include_sync_surface,
        ForgeServerMiddlewareConfig::builder()
            .build()
            .expect("middleware config should validate"),
    )
}

pub(crate) fn test_server_with_middleware(
    workspace_provider: impl ForgeServerQueryWorkspaceProvider,
    include_sync_surface: bool,
    middleware_config: ForgeServerMiddlewareConfig,
) -> ForgeServer {
    let mut builder = ForgeServer::builder()
        .with_config(
            ForgeServerConfig::builder()
                .with_bind_address(([127, 0, 0, 1], 8090).into())
                .with_middleware_config(middleware_config)
                .with_request_context_config(
                    ForgeServerRequestContextConfig::builder()
                        .with_default_diagnostics_profile(DiagnosticRichnessProfile::Standard)
                        .build()
                        .expect("request context config should validate"),
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
        .register_surface(ForgeNativeSurface::disabled())
        .register_surface(CompatHttpSurface::disabled());
    if include_sync_surface {
        builder = builder.register_surface(SyncSurface::disabled());
    }
    builder.build().expect("server should build")
}

pub(crate) fn request_input(
    surface_family: ForgeServerSurfaceFamily,
    transport_class: ForgeServerTransportClass,
) -> ForgeServerRequestContextInput {
    ForgeServerRequestContextInput::builder()
        .with_surface_family(surface_family)
        .with_transport_class(transport_class)
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .build()
        .expect("request context input should validate")
}

pub(crate) fn resolve_request_context(
    server: &ForgeServer,
    input: ForgeServerRequestContextInput,
) -> ForgeServerResolvedRequestContext {
    match server.request_contexts().resolve(input) {
        TransitionReadiness::Ready(resolved) => resolved,
        other => panic!("expected resolved request context, got {other:?}"),
    }
}

pub(crate) fn admit_read(
    server: &ForgeServer,
    resolved: ForgeServerResolvedRequestContext,
) -> ForgeServerAdmission {
    match server
        .middleware()
        .admit(forge_server::ForgeServerPipelineInput::new(
            resolved,
            forge_server::ForgeServerPipelineIntent::query_read("users.profile"),
        )) {
        TransitionOutcome::Success(admission) => admission,
        other => panic!("expected admitted pipeline result, got {other:?}"),
    }
}

pub(crate) fn admit_mutation(
    server: &ForgeServer,
    resolved: ForgeServerResolvedRequestContext,
) -> ForgeServerAdmission {
    match server
        .middleware()
        .admit(forge_server::ForgeServerPipelineInput::new(
            resolved,
            forge_server::ForgeServerPipelineIntent::query_mutation("users.rename"),
        )) {
        TransitionOutcome::Success(admission) => admission,
        other => panic!("expected admitted mutation pipeline result, got {other:?}"),
    }
}

pub(crate) fn success(outcome: ForgeServerQueryHandoffOutcome) -> ForgeServerQueryHandoff {
    match outcome {
        TransitionOutcome::Success(handoff) => handoff,
        other => panic!("expected successful query handoff, got {other:?}"),
    }
}

#[allow(dead_code)]
pub(crate) fn denied(outcome: ForgeServerQueryHandoffOutcome) -> ForgeServerQueryHandoffDenial {
    match outcome {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected denied query handoff, got {other:?}"),
    }
}
