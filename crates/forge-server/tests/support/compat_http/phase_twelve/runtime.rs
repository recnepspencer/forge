#![allow(dead_code)]

use forge_foundational::facade::DiagnosticRichnessProfile;
use forge_proof::TransitionOutcome;
use forge_server::{
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerBinaryDownloadExecutionInput, ForgeServerBinaryDownloadRequest,
    ForgeServerCompatHttpRouteFamily, ForgeServerCompatibilityPreparedRequest,
    ForgeServerCompatibilityRequestInput, ForgeServerCompatibilityUploadExecutionInput,
    ForgeServerConfig, ForgeServerMiddlewareConfig, ForgeServerMultipartUpload,
    ForgeServerQueryHandoffConfig, ForgeServerRequestContextConfig,
};

use crate::query_handoff_runtime::TestWorkspaceProvider;

pub(crate) fn build_phase_twelve_server() -> ForgeServer {
    build_server_with_max_diagnostics(DiagnosticRichnessProfile::Forensic)
}

pub(crate) fn build_phase_twelve_budget_limited_server() -> ForgeServer {
    build_server_with_max_diagnostics(DiagnosticRichnessProfile::Standard)
}

fn build_server_with_max_diagnostics(
    maximum_diagnostics_profile: DiagnosticRichnessProfile,
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
                        .with_compat_http_maximum_diagnostics_profile(maximum_diagnostics_profile)
                        .with_query_mutation_enabled(true)
                        .build()
                        .expect("middleware config should validate"),
                )
                .with_query_handoff_config(
                    ForgeServerQueryHandoffConfig::builder()
                        .with_workspace_provider(TestWorkspaceProvider)
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

pub(crate) fn prepared_request(
    server: &ForgeServer,
    request: ForgeServerCompatibilityRequestInput,
) -> ForgeServerCompatibilityPreparedRequest {
    match server.compat_http().prepare_request(request) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected prepared compatibility request, got {other:?}"),
    }
}

pub(crate) fn read_request(
    diagnostics_profile: DiagnosticRichnessProfile,
) -> ForgeServerCompatibilityRequestInput {
    ForgeServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-12")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .with_branch_id("branch-9")
        .with_route_family(ForgeServerCompatHttpRouteFamily::Read)
        .with_method("GET")
        .with_path("/compat/reads/tasks")
        .with_header("accept", "application/json")
        .with_diagnostics_profile(diagnostics_profile)
        .build()
        .expect("phase twelve read input should validate")
}

pub(crate) fn head_read_request() -> ForgeServerCompatibilityRequestInput {
    ForgeServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-12")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .with_branch_id("branch-9")
        .with_route_family(ForgeServerCompatHttpRouteFamily::Read)
        .with_method("HEAD")
        .with_path("/compat/reads/tasks")
        .with_header("accept", "application/json")
        .build()
        .expect("phase twelve head read input should validate")
}

pub(crate) fn upload_input(
    server: &ForgeServer,
    upload: ForgeServerMultipartUpload,
) -> ForgeServerCompatibilityUploadExecutionInput {
    let request = ForgeServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-12")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .with_branch_id("branch-9")
        .with_route_family(ForgeServerCompatHttpRouteFamily::Upload)
        .with_method("POST")
        .with_path("/compat/uploads/blob")
        .with_header("accept", "application/json")
        .with_body_content_type("multipart/form-data; boundary=phase-twelve")
        .with_body_present(true)
        .build()
        .expect("phase twelve upload input should validate");
    ForgeServerCompatibilityUploadExecutionInput::new(
        prepared_request(server, request),
        "blob",
        upload,
    )
}

pub(crate) fn download_input(
    server: &ForgeServer,
    request: ForgeServerBinaryDownloadRequest,
) -> ForgeServerBinaryDownloadExecutionInput {
    let compat_request = ForgeServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-12")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .with_branch_id("branch-9")
        .with_route_family(ForgeServerCompatHttpRouteFamily::Download)
        .with_method("GET")
        .with_path("/compat/downloads/blob")
        .with_header("accept", "application/octet-stream")
        .build()
        .expect("phase twelve download input should validate");
    ForgeServerBinaryDownloadExecutionInput::new(
        prepared_request(server, compat_request),
        "blob",
        request,
    )
}

pub(crate) fn drip_fed_upload() -> ForgeServerMultipartUpload {
    use forge_server::{
        ForgeServerUploadChunk, ForgeServerUploadManifest, ForgeServerUploadPart,
        ForgeServerUploadTransferMode,
    };
    use serde_json::json;

    let part = (0..33).fold(
        ForgeServerUploadPart::file("blob")
            .with_content_type("application/octet-stream")
            .with_declared_length(33)
            .with_transfer_mode(ForgeServerUploadTransferMode::UnknownLength)
            .with_body_bytes(vec![b'a'; 33]),
        |part, _| part.with_wire_chunk(ForgeServerUploadChunk::new(vec![1])),
    );
    ForgeServerMultipartUpload::new(
        ForgeServerUploadManifest::new(json!({
            "command": {
                "family": "insert",
                "collection": "Task",
                "aspects": {
                    "identity.id": "drip",
                    "title.value": "Drip Feed"
                }
            }
        }))
        .with_file_part("blob"),
    )
    .with_part(part)
}
