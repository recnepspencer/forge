use forge_proof::{TransitionOutcome, TransitionReadiness};
use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface, SyncSurface},
    ForgeServer, ForgeServerConfig, ForgeServerMiddlewareConfig, ForgeServerPipelineInput,
    ForgeServerPipelineIntent, ForgeServerRequestContextConfig, ForgeServerRequestContextInput,
    ForgeServerResolvedRequestContext, ForgeServerSurfaceFamily, ForgeServerTransportClass,
};

fn test_server_with_surfaces(registered_families: &[ForgeServerSurfaceFamily]) -> ForgeServer {
    let mut builder = ForgeServer::builder().with_config(
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
                    .with_query_mutation_enabled(false)
                    .build()
                    .expect("middleware config should validate"),
            )
            .build()
            .expect("server config should validate"),
    );

    for family in registered_families {
        builder = match family {
            ForgeServerSurfaceFamily::ForgeNative => {
                builder.register_surface(ForgeNativeSurface::disabled())
            }
            ForgeServerSurfaceFamily::CompatHttp => {
                builder.register_surface(CompatHttpSurface::disabled())
            }
            ForgeServerSurfaceFamily::Sync => builder.register_surface(SyncSurface::disabled()),
            unsupported => panic!("unexpected unsupported test family {unsupported:?}"),
        };
    }

    builder.build().expect("server should build")
}

fn resolve_forge_native_request_context(server: &ForgeServer) -> ForgeServerResolvedRequestContext {
    match server.request_contexts().resolve(
        ForgeServerRequestContextInput::builder()
            .with_authenticated_principal_id("principal-7")
            .with_tenant_id("tenant-a")
            .with_workspace_id("workspace-42")
            .with_branch_id("branch-9")
            .with_surface_family(ForgeServerSurfaceFamily::ForgeNative)
            .with_transport_class(ForgeServerTransportClass::ForgeNativeInProcess)
            .build()
            .expect("request context input should validate"),
    ) {
        TransitionReadiness::Ready(resolved) => resolved,
        other => panic!("expected resolved request context, got {other:?}"),
    }
}

#[test]
fn surface_families_are_independently_inspectable_through_surface_roots() {
    let forge_native_only = test_server_with_surfaces(&[ForgeServerSurfaceFamily::ForgeNative]);
    let surfaces = forge_native_only.surfaces();

    assert!(surfaces.forge_native().capabilities().is_registered());
    assert!(surfaces.forge_native().capabilities().is_disabled());
    assert!(!surfaces.forge_native().capabilities().is_absent());

    assert!(!surfaces.compat_http().capabilities().is_registered());
    assert!(surfaces.compat_http().capabilities().is_absent());
    assert!(!surfaces.compat_http().capabilities().is_disabled());

    assert!(surfaces.sync().capabilities().is_absent());
    assert!(surfaces.lease().capabilities().is_absent());
    assert!(surfaces.binary().capabilities().is_absent());
    assert!(surfaces.integration().capabilities().is_absent());
}

#[test]
fn sibling_surface_membership_does_not_change_shared_pipeline_truth() {
    let forge_native_only = test_server_with_surfaces(&[ForgeServerSurfaceFamily::ForgeNative]);
    let with_compat_http = test_server_with_surfaces(&[
        ForgeServerSurfaceFamily::ForgeNative,
        ForgeServerSurfaceFamily::CompatHttp,
    ]);

    let forge_native_only_context = resolve_forge_native_request_context(&forge_native_only);
    let with_compat_http_context = resolve_forge_native_request_context(&with_compat_http);

    assert_eq!(forge_native_only_context, with_compat_http_context);

    let forge_native_only_admission =
        forge_native_only
            .middleware()
            .admit(ForgeServerPipelineInput::new(
                forge_native_only_context,
                ForgeServerPipelineIntent::query_read("users.profile"),
            ));
    let with_compat_http_admission =
        with_compat_http
            .middleware()
            .admit(ForgeServerPipelineInput::new(
                with_compat_http_context,
                ForgeServerPipelineIntent::query_read("users.profile"),
            ));

    assert_eq!(forge_native_only_admission, with_compat_http_admission);
    assert!(matches!(
        forge_native_only_admission,
        TransitionOutcome::Success(_)
    ));
}

#[test]
fn future_surface_placeholder_registration_is_independent_of_present_family_truth() {
    let with_sync_placeholder = test_server_with_surfaces(&[
        ForgeServerSurfaceFamily::ForgeNative,
        ForgeServerSurfaceFamily::Sync,
    ]);

    let surfaces = with_sync_placeholder.surfaces();
    assert!(surfaces.forge_native().capabilities().is_registered());
    assert!(surfaces.sync().capabilities().is_registered());
    assert!(surfaces.sync().capabilities().is_disabled());
    assert!(!surfaces.sync().capabilities().is_absent());
    assert!(surfaces.compat_http().capabilities().is_absent());

    let resolved_request_context = resolve_forge_native_request_context(&with_sync_placeholder);
    let admission = with_sync_placeholder
        .middleware()
        .admit(ForgeServerPipelineInput::new(
            resolved_request_context,
            ForgeServerPipelineIntent::query_read("users.profile"),
        ));

    assert!(matches!(admission, TransitionOutcome::Success(_)));
}
