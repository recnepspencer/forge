#![allow(dead_code)]

use forge_proof::TransitionOutcome;
use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerBinaryDownload, ForgeServerBinaryDownloadExecutionInput,
    ForgeServerBinaryDownloadOutcome, ForgeServerBinaryDownloadRequest,
    ForgeServerCompatHttpRouteFamily, ForgeServerCompatibilityPreparedRequest,
    ForgeServerCompatibilityRequestInput, ForgeServerConfig, ForgeServerMiddlewareConfig,
    ForgeServerQueryHandoffConfig, ForgeServerQueryHandoffDenial,
    ForgeServerQueryWorkspaceProvider, ForgeServerRequestContextConfig,
};

pub(crate) fn build_phase_seven_server_with_workspace_provider(
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

pub(crate) fn download_input(
    tenant_id: &str,
    workspace_id: &str,
    branch_id: &str,
    operation_name: &str,
) -> forge_server::ForgeServerCompatibilityRequestInputBuilder {
    ForgeServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id(tenant_id)
        .with_workspace_id(workspace_id)
        .with_branch_id(branch_id)
        .with_route_family(ForgeServerCompatHttpRouteFamily::Download)
        .with_method("GET")
        .with_path(format!("/compat/downloads/{operation_name}"))
        .with_header("accept", "application/octet-stream")
}

pub(crate) fn prepared_request(
    server: &ForgeServer,
    request: ForgeServerCompatibilityRequestInput,
) -> ForgeServerCompatibilityPreparedRequest {
    match server.compat_http().prepare_request(request) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected prepared compatibility request, got {other:?}"),
    }
}

pub(crate) fn compat_download_execution_input(
    server: &ForgeServer,
    tenant_id: &str,
    workspace_id: &str,
    branch_id: &str,
    operation_name: &str,
    request: ForgeServerBinaryDownloadRequest,
) -> ForgeServerBinaryDownloadExecutionInput {
    ForgeServerBinaryDownloadExecutionInput::new(
        prepared_request(
            server,
            download_input(tenant_id, workspace_id, branch_id, operation_name)
                .build()
                .expect("download input should validate structurally"),
        ),
        operation_name,
        request,
    )
}

pub(crate) fn compat_download_success(
    outcome: ForgeServerBinaryDownloadOutcome<ForgeServerBinaryDownload>,
) -> ForgeServerBinaryDownload {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected binary download success, got {other:?}"),
    }
}

pub(crate) fn compat_download_denied(
    outcome: ForgeServerBinaryDownloadOutcome<ForgeServerBinaryDownload>,
) -> ForgeServerQueryHandoffDenial {
    match outcome {
        TransitionOutcome::Denied(value) => value,
        other => panic!("expected binary download denial, got {other:?}"),
    }
}
