use forge_proof::TransitionReadiness;
use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerBranchTarget, ForgeServerConfig, ForgeServerRequestContext,
    ForgeServerRequestContextConfig, ForgeServerRequestContextDenial,
    ForgeServerRequestContextDenialCode, ForgeServerRequestContextInput,
    ForgeServerResolvedRequestContext, ForgeServerSurfaceFamily, ForgeServerTransportClass,
};

fn test_server(request_context: ForgeServerRequestContextConfig) -> ForgeServer {
    ForgeServer::builder()
        .with_config(
            ForgeServerConfig::builder()
                .with_bind_address(([127, 0, 0, 1], 8080).into())
                .with_request_context_config(request_context)
                .build()
                .expect("server config should validate"),
        )
        .register_surface(ForgeNativeSurface::disabled())
        .register_surface(CompatHttpSurface::disabled())
        .build()
        .expect("server should build")
}

fn input_builder() -> forge_server::ForgeServerRequestContextInputBuilder {
    ForgeServerRequestContextInput::builder()
        .with_surface_family(ForgeServerSurfaceFamily::ForgeNative)
        .with_transport_class(ForgeServerTransportClass::ForgeNativeInProcess)
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
}

fn denied(
    resolution: TransitionReadiness<
        ForgeServerResolvedRequestContext,
        ForgeServerRequestContextDenial,
        forge_server::ForgeServerRequestContextDeferred,
        forge_server::ForgeServerRequestContextStale,
        forge_server::ForgeServerRequestContextRebindRequired,
        forge_server::ForgeServerRequestContextFailure,
    >,
) -> ForgeServerRequestContextDenial {
    match resolution {
        TransitionReadiness::Denied(denial) => denial,
        other => panic!("expected denial, got {other:?}"),
    }
}

fn ready_context(
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
        other => panic!("expected ready context, got {other:?}"),
    }
}

#[test]
fn resolve_canonicalizes_trimmed_input_and_default_diagnostics() {
    let server = test_server(
        ForgeServerRequestContextConfig::builder()
            .with_default_diagnostics_profile(DiagnosticRichnessProfile::Standard)
            .with_maximum_diagnostics_profile(DiagnosticRichnessProfile::Forensic)
            .build()
            .expect("request context config should validate"),
    );

    let trimmed = server.request_contexts().resolve(
        input_builder()
            .with_authenticated_principal_id("principal-7")
            .with_tenant_id("tenant-a")
            .with_workspace_id("workspace-42")
            .with_branch_id("branch-9")
            .build()
            .expect("input should validate"),
    );
    let padded = server.request_contexts().resolve(
        input_builder()
            .with_authenticated_principal_id(" principal-7 ")
            .with_tenant_id(" tenant-a ")
            .with_workspace_id(" workspace-42 ")
            .with_branch_id(" branch-9 ")
            .with_diagnostics_profile(DiagnosticRichnessProfile::Standard)
            .build()
            .expect("input should validate"),
    );

    let trimmed = ready_context(trimmed);
    let padded = ready_context(padded);

    assert_eq!(trimmed, padded);
    let trimmed = trimmed.request_context();
    assert_eq!(
        trimmed.authenticated_principal().principal_id(),
        "principal-7"
    );
    assert_eq!(trimmed.workspace_target().tenant_id(), "tenant-a");
    assert_eq!(trimmed.workspace_target().workspace_id(), "workspace-42");
    assert_eq!(
        trimmed.branch_target(),
        &ForgeServerBranchTarget::Branch {
            branch_id: String::from("branch-9"),
        }
    );
    assert_eq!(
        trimmed.diagnostics_profile(),
        DiagnosticRichnessProfile::Standard
    );
}

#[test]
fn resolve_preserves_semantic_identity_across_surface_families_while_recording_boundary_shape() {
    let server = test_server(
        ForgeServerRequestContextConfig::builder()
            .with_branch_targeting_enabled(true)
            .build()
            .expect("request context config should validate"),
    );

    let forge_native = server.request_contexts().resolve(
        input_builder()
            .with_surface_family(ForgeServerSurfaceFamily::ForgeNative)
            .with_transport_class(ForgeServerTransportClass::ForgeNativeInProcess)
            .with_branch_id("shared-branch")
            .build()
            .expect("input should validate"),
    );
    let compat_http = server.request_contexts().resolve(
        input_builder()
            .with_surface_family(ForgeServerSurfaceFamily::CompatHttp)
            .with_transport_class(ForgeServerTransportClass::CompatHttp)
            .with_branch_id("shared-branch")
            .build()
            .expect("input should validate"),
    );

    let forge_native = ready_context(forge_native);
    let compat_http = ready_context(compat_http);

    assert_eq!(
        forge_native.request_context(),
        compat_http.request_context()
    );
    let semantic_context: &ForgeServerRequestContext = forge_native.request_context();
    assert_eq!(
        semantic_context.branch_target(),
        &ForgeServerBranchTarget::Branch {
            branch_id: String::from("shared-branch"),
        }
    );
    assert_eq!(
        forge_native.surface_family(),
        ForgeServerSurfaceFamily::ForgeNative
    );
    assert_eq!(
        compat_http.surface_family(),
        ForgeServerSurfaceFamily::CompatHttp
    );
    assert_eq!(
        forge_native.transport_class(),
        ForgeServerTransportClass::ForgeNativeInProcess
    );
    assert_eq!(
        compat_http.transport_class(),
        ForgeServerTransportClass::CompatHttp
    );
}

#[test]
fn resolve_denies_incompatible_surface_transport_binding_before_context_construction() {
    let server = test_server(
        ForgeServerRequestContextConfig::builder()
            .build()
            .expect("request context config should validate"),
    );

    let denial = denied(
        server.request_contexts().resolve(
            input_builder()
                .with_surface_family(ForgeServerSurfaceFamily::ForgeNative)
                .with_transport_class(ForgeServerTransportClass::CompatHttp)
                .build()
                .expect("input should validate"),
        ),
    );

    assert_eq!(
        denial.code(),
        ForgeServerRequestContextDenialCode::IncompatibleSurfaceTransportBinding
    );
    assert_eq!(
        denial.detail(),
        "surface family ForgeNative cannot resolve transport class CompatHttp"
    );
}

#[test]
fn resolve_denies_blank_principal_before_context_construction() {
    let server = test_server(
        ForgeServerRequestContextConfig::builder()
            .build()
            .expect("request context config should validate"),
    );

    let denial = denied(
        server.request_contexts().resolve(
            input_builder()
                .with_authenticated_principal_id("   ")
                .build()
                .expect("input should validate"),
        ),
    );

    assert_eq!(
        denial.code(),
        ForgeServerRequestContextDenialCode::InvalidAuthenticatedPrincipal
    );
    assert_eq!(
        denial.detail(),
        "authenticated principal id must not be empty"
    );
}

#[test]
fn resolve_denies_blank_workspace_target_before_context_construction() {
    let server = test_server(
        ForgeServerRequestContextConfig::builder()
            .build()
            .expect("request context config should validate"),
    );

    let denial = denied(
        server.request_contexts().resolve(
            input_builder()
                .with_tenant_id("tenant-a")
                .with_workspace_id("   ")
                .build()
                .expect("input should validate"),
        ),
    );

    assert_eq!(
        denial.code(),
        ForgeServerRequestContextDenialCode::InvalidWorkspaceTarget
    );
    assert_eq!(denial.detail(), "workspace id must not be empty");
}

#[test]
fn resolve_denies_disabled_branch_targeting_with_exact_code_and_detail() {
    let server = test_server(
        ForgeServerRequestContextConfig::builder()
            .with_branch_targeting_enabled(false)
            .build()
            .expect("request context config should validate"),
    );

    let denial = denied(
        server.request_contexts().resolve(
            input_builder()
                .with_branch_id("branch-1")
                .build()
                .expect("input should validate"),
        ),
    );

    assert_eq!(
        denial.code(),
        ForgeServerRequestContextDenialCode::BranchTargetingDisabled
    );
    assert_eq!(
        denial.detail(),
        "branch targeting is disabled by server configuration"
    );
}

#[test]
fn resolve_denies_blank_branch_identifier_before_context_construction() {
    let server = test_server(
        ForgeServerRequestContextConfig::builder()
            .with_branch_targeting_enabled(true)
            .build()
            .expect("request context config should validate"),
    );

    let denial = denied(
        server.request_contexts().resolve(
            input_builder()
                .with_branch_id("   ")
                .build()
                .expect("input should validate"),
        ),
    );

    assert_eq!(
        denial.code(),
        ForgeServerRequestContextDenialCode::InvalidBranchTarget
    );
    assert_eq!(denial.detail(), "branch id must not be empty");
}

#[test]
fn resolve_denies_preview_targeting_and_overspecified_diagnostics_strictly() {
    let preview_denial_server = test_server(
        ForgeServerRequestContextConfig::builder()
            .with_preview_targeting_enabled(false)
            .build()
            .expect("request context config should validate"),
    );
    let diagnostics_denial_server = test_server(
        ForgeServerRequestContextConfig::builder()
            .with_default_diagnostics_profile(DiagnosticRichnessProfile::Standard)
            .with_maximum_diagnostics_profile(DiagnosticRichnessProfile::Standard)
            .build()
            .expect("request context config should validate"),
    );

    let preview_denial = denied(
        preview_denial_server.request_contexts().resolve(
            input_builder()
                .with_preview_id("preview-unsafe")
                .build()
                .expect("input should validate"),
        ),
    );
    let diagnostics_denial = denied(
        diagnostics_denial_server.request_contexts().resolve(
            input_builder()
                .with_diagnostics_profile(DiagnosticRichnessProfile::Forensic)
                .build()
                .expect("input should validate"),
        ),
    );

    assert_eq!(
        preview_denial.code(),
        ForgeServerRequestContextDenialCode::PreviewTargetingDisabled
    );
    assert_eq!(
        preview_denial.detail(),
        "preview targeting is disabled by server configuration"
    );
    assert_eq!(
        diagnostics_denial.code(),
        ForgeServerRequestContextDenialCode::DiagnosticsProfileExceedsMaximum
    );
    assert_eq!(
        diagnostics_denial.detail(),
        "requested diagnostics profile Forensic exceeds configured maximum Standard"
    );
}
