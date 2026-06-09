use forge_proof::TransitionOutcome;
use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerCompatibilityExecutionInput, ForgeServerCompatibilityPreparedRequest,
    ForgeServerCompatibilityRequestInput, ForgeServerConfig, ForgeServerQueryHandoffConfig,
    ForgeServerQueryWorkspaceProvider, ForgeServerRequestContextConfig,
};

use crate::forge_native_assertions::forge_native_session;
use crate::forge_native_runtime::forge_native_session_input_builder;
use crate::query_handoff_runtime::TestWorkspaceProvider;

pub(crate) fn build_phase_two_server() -> ForgeServer {
    build_phase_two_server_with_workspace_provider(TestWorkspaceProvider)
}

pub(crate) fn build_phase_two_server_with_workspace_provider(
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

pub(crate) fn prepared_read_request(
    server: &ForgeServer,
    request: ForgeServerCompatibilityRequestInput,
) -> ForgeServerCompatibilityPreparedRequest {
    match server.compat_http().prepare_request(request) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected prepared compat request, got {other:?}"),
    }
}

pub(crate) fn read_input(
    operation_name: &str,
) -> forge_server::ForgeServerCompatibilityRequestInputBuilder {
    ForgeServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .with_branch_id("branch-9")
        .with_route_family(forge_server::ForgeServerCompatHttpRouteFamily::Read)
        .with_method("GET")
        .with_path(format!("/compat/reads/{operation_name}"))
        .with_header("accept", "application/json")
}

pub(crate) fn compat_execution_input(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCompatibilityExecutionInput {
    ForgeServerCompatibilityExecutionInput::new(
        prepared_read_request(
            server,
            read_input(operation_name)
                .build()
                .expect("compat read input should validate"),
        ),
        operation_name,
    )
}

pub(crate) fn branch_head_execution_input(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCompatibilityExecutionInput {
    ForgeServerCompatibilityExecutionInput::new(
        prepared_read_request(
            server,
            read_input(operation_name)
                .with_method("HEAD")
                .build()
                .expect("compat HEAD input should validate"),
        ),
        operation_name,
    )
}

pub(crate) fn forge_native_named_read(
    server: &ForgeServer,
    operation_name: &str,
) -> (
    forge_server::ForgeServerForgeNativeSession,
    forge_server::ForgeServerAdmittedDirectDeclaration,
) {
    let session = forge_native_session(server);
    let declaration = session
        .declarations()
        .read(forge_server::ForgeServerDirectDeclaration::named_read(
            operation_name,
        ))
        .expect("named read should prepare")
        .admit()
        .expect("named read should admit");
    (session, declaration)
}

#[allow(dead_code)]
pub(crate) fn forge_native_branch_session(
    server: &ForgeServer,
    branch_id: &str,
) -> forge_server::ForgeServerForgeNativeSession {
    match server.forge_native().session(
        forge_native_session_input_builder()
            .with_branch_id(branch_id)
            .build()
            .expect("branch session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected forge-native branch session, got {other:?}"),
    }
}
