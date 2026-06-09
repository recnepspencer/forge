use forge_proof::TransitionOutcome;
use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerCompatHttpRouteFamily, ForgeServerCompatibilityPreparedRequest,
    ForgeServerCompatibilityRequestInput, ForgeServerCompatibilityUpload,
    ForgeServerCompatibilityUploadExecutionInput, ForgeServerCompatibilityUploadOutcome,
    ForgeServerConfig, ForgeServerMiddlewareConfig, ForgeServerMultipartUpload,
    ForgeServerPreparedMultipartUpload, ForgeServerQueryHandoffConfig,
    ForgeServerQueryHandoffDenial, ForgeServerQueryWorkspaceProvider,
    ForgeServerRequestContextConfig,
};

pub(crate) fn build_phase_six_server_with_workspace_provider(
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
        .register_surface(ForgeNativeSurface::enabled())
        .register_surface(CompatHttpSurface::phase_one_enabled())
        .build()
        .expect("server should build")
}

pub(crate) fn upload_input(
    tenant_id: &str,
    workspace_id: &str,
    branch_id: &str,
    operation_name: &str,
    boundary: &str,
) -> ForgeServerCompatibilityRequestInput {
    ForgeServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id(tenant_id)
        .with_workspace_id(workspace_id)
        .with_branch_id(branch_id)
        .with_route_family(ForgeServerCompatHttpRouteFamily::Upload)
        .with_method("POST")
        .with_path(format!("/compat/uploads/{operation_name}"))
        .with_header("accept", "application/json")
        .with_body_content_type(format!("multipart/form-data; boundary={boundary}"))
        .with_body_present(true)
        .build()
        .expect("phase six upload input should validate structurally")
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

pub(crate) fn prepared_upload(
    server: &ForgeServer,
    tenant_id: &str,
    workspace_id: &str,
    branch_id: &str,
    operation_name: &str,
    boundary: &str,
    upload: ForgeServerMultipartUpload,
) -> ForgeServerPreparedMultipartUpload {
    match server
        .compat_http()
        .prepare_upload(ForgeServerCompatibilityUploadExecutionInput::new(
            prepared_request(
                server,
                upload_input(tenant_id, workspace_id, branch_id, operation_name, boundary),
            ),
            operation_name,
            upload,
        )) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected prepared upload, got {other:?}"),
    }
}

pub(crate) fn compat_upload_execution_input(
    server: &ForgeServer,
    tenant_id: &str,
    workspace_id: &str,
    branch_id: &str,
    operation_name: &str,
    boundary: &str,
    upload: ForgeServerMultipartUpload,
) -> ForgeServerCompatibilityUploadExecutionInput {
    ForgeServerCompatibilityUploadExecutionInput::new(
        prepared_request(
            server,
            upload_input(tenant_id, workspace_id, branch_id, operation_name, boundary),
        ),
        operation_name,
        upload,
    )
}

pub(crate) fn compat_upload_success(
    outcome: ForgeServerCompatibilityUploadOutcome<ForgeServerCompatibilityUpload>,
) -> ForgeServerCompatibilityUpload {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected compatibility upload success, got {other:?}"),
    }
}

pub(crate) fn compat_upload_denied(
    outcome: ForgeServerCompatibilityUploadOutcome<ForgeServerCompatibilityUpload>,
) -> ForgeServerQueryHandoffDenial {
    match outcome {
        TransitionOutcome::Denied(value) => value,
        other => panic!("expected compatibility upload denial, got {other:?}"),
    }
}
