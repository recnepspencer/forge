use forge_proof::{TransitionOutcome, TransitionReadiness};
use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerAdmission, ForgeServerConfig, ForgeServerDenial, ForgeServerDenialCode,
    ForgeServerDenialPriority, ForgeServerMiddlewareConfig, ForgeServerPipelineInput,
    ForgeServerPipelineIntent, ForgeServerPipelineStep, ForgeServerPreparedQueryHandoffKind,
    ForgeServerRequestContextConfig, ForgeServerRequestContextInput,
    ForgeServerResolvedRequestContext, ForgeServerSurfaceFamily, ForgeServerTransportClass,
};

fn test_server(
    request_context: ForgeServerRequestContextConfig,
    middleware: ForgeServerMiddlewareConfig,
) -> ForgeServer {
    ForgeServer::builder()
        .with_config(
            ForgeServerConfig::builder()
                .with_bind_address(([127, 0, 0, 1], 8080).into())
                .with_request_context_config(request_context)
                .with_middleware_config(middleware)
                .build()
                .expect("server config should validate"),
        )
        .register_surface(ForgeNativeSurface::disabled())
        .register_surface(CompatHttpSurface::disabled())
        .build()
        .expect("server should build")
}

fn request_input_builder() -> forge_server::ForgeServerRequestContextInputBuilder {
    ForgeServerRequestContextInput::builder()
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
}

fn resolve_request_context(
    server: &ForgeServer,
    input: ForgeServerRequestContextInput,
) -> ForgeServerResolvedRequestContext {
    match server.request_contexts().resolve(input) {
        TransitionReadiness::Ready(resolved) => resolved,
        other => panic!("expected resolved request context, got {other:?}"),
    }
}

fn denied(
    outcome: TransitionOutcome<
        ForgeServerAdmission,
        ForgeServerDenial,
        forge_server::ForgeServerMiddlewareDeferred,
        forge_server::ForgeServerMiddlewareStale,
        forge_server::ForgeServerMiddlewareRebindRequired,
        forge_server::ForgeServerMiddlewareFailure,
    >,
) -> ForgeServerDenial {
    match outcome {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected middleware denial, got {other:?}"),
    }
}

#[test]
fn admit_returns_ready_admission_with_prepared_query_handoff_intent() {
    let server = test_server(
        ForgeServerRequestContextConfig::builder()
            .with_preview_targeting_enabled(true)
            .build()
            .expect("request context config should validate"),
        ForgeServerMiddlewareConfig::builder()
            .with_query_mutation_enabled(true)
            .build()
            .expect("middleware config should validate"),
    );
    let resolved_request_context = resolve_request_context(
        &server,
        request_input_builder()
            .with_surface_family(ForgeServerSurfaceFamily::ForgeNative)
            .with_transport_class(ForgeServerTransportClass::ForgeNativeInProcess)
            .with_branch_id("branch-9")
            .build()
            .expect("request context input should validate"),
    );

    let admission = server.middleware().admit(ForgeServerPipelineInput::new(
        resolved_request_context.clone(),
        ForgeServerPipelineIntent::query_read("users.profile"),
    ));

    let admission = match admission {
        TransitionOutcome::Success(admitted) => admitted,
        other => panic!("expected admitted middleware result, got {other:?}"),
    };

    assert_eq!(
        admission.resolved_request_context(),
        &resolved_request_context
    );
    assert_eq!(
        admission.query_handoff_intent().kind(),
        ForgeServerPreparedQueryHandoffKind::QueryRead
    );
    assert_eq!(
        admission.query_handoff_intent().operation_name(),
        "users.profile"
    );
}

#[test]
fn admit_preserves_validation_denial_across_surface_families() {
    let server = test_server(
        ForgeServerRequestContextConfig::builder()
            .build()
            .expect("request context config should validate"),
        ForgeServerMiddlewareConfig::builder()
            .with_query_mutation_enabled(false)
            .build()
            .expect("middleware config should validate"),
    );

    let forge_native = denied(
        server.middleware().admit(ForgeServerPipelineInput::new(
            resolve_request_context(
                &server,
                request_input_builder()
                    .with_surface_family(ForgeServerSurfaceFamily::ForgeNative)
                    .with_transport_class(ForgeServerTransportClass::ForgeNativeInProcess)
                    .build()
                    .expect("request context input should validate"),
            ),
            ForgeServerPipelineIntent::query_mutation("users.rename"),
        )),
    );
    let compat_http = denied(
        server.middleware().admit(ForgeServerPipelineInput::new(
            resolve_request_context(
                &server,
                request_input_builder()
                    .with_surface_family(ForgeServerSurfaceFamily::CompatHttp)
                    .with_transport_class(ForgeServerTransportClass::CompatHttp)
                    .build()
                    .expect("request context input should validate"),
            ),
            ForgeServerPipelineIntent::query_mutation("users.rename"),
        )),
    );

    assert_eq!(forge_native, compat_http);
    assert_eq!(
        forge_native.code(),
        ForgeServerDenialCode::QueryMutationDisabled
    );
    assert_eq!(
        forge_native.priority(),
        ForgeServerDenialPriority::Validation
    );
    assert_eq!(
        forge_native.step(),
        ForgeServerPipelineStep::ValidationPosture
    );
}

#[test]
fn admit_uses_canonical_denial_priority_under_overlapping_failures() {
    let server = test_server(
        ForgeServerRequestContextConfig::builder()
            .with_preview_targeting_enabled(true)
            .with_default_diagnostics_profile(DiagnosticRichnessProfile::Forensic)
            .with_maximum_diagnostics_profile(DiagnosticRichnessProfile::Forensic)
            .build()
            .expect("request context config should validate"),
        ForgeServerMiddlewareConfig::builder()
            .with_compat_http_maximum_diagnostics_profile(
                DiagnosticRichnessProfile::OperationalMinimal,
            )
            .with_preview_branch_authorization_enabled(false)
            .with_query_mutation_enabled(false)
            .build()
            .expect("middleware config should validate"),
    );

    let overlapping_inputs = [
        ForgeServerPipelineInput::new(
            resolve_request_context(
                &server,
                request_input_builder()
                    .with_surface_family(ForgeServerSurfaceFamily::CompatHttp)
                    .with_transport_class(ForgeServerTransportClass::CompatHttp)
                    .with_preview_id("preview-1")
                    .with_diagnostics_profile(DiagnosticRichnessProfile::Forensic)
                    .build()
                    .expect("request context input should validate"),
            ),
            ForgeServerPipelineIntent::query_mutation("users.rename"),
        ),
        ForgeServerPipelineInput::new(
            resolve_request_context(
                &server,
                request_input_builder()
                    .with_surface_family(ForgeServerSurfaceFamily::ForgeNative)
                    .with_transport_class(ForgeServerTransportClass::ForgeNativeInProcess)
                    .with_preview_id("preview-2")
                    .with_diagnostics_profile(DiagnosticRichnessProfile::Forensic)
                    .build()
                    .expect("request context input should validate"),
            ),
            ForgeServerPipelineIntent::query_mutation("users.rename"),
        ),
        ForgeServerPipelineInput::new(
            resolve_request_context(
                &server,
                request_input_builder()
                    .with_surface_family(ForgeServerSurfaceFamily::CompatHttp)
                    .with_transport_class(ForgeServerTransportClass::CompatHttp)
                    .with_preview_id("preview-3")
                    .with_diagnostics_profile(DiagnosticRichnessProfile::Standard)
                    .build()
                    .expect("request context input should validate"),
            ),
            ForgeServerPipelineIntent::query_mutation("users.rename"),
        ),
    ];

    for overlapping_input in overlapping_inputs {
        let denial = denied(server.middleware().admit(overlapping_input));
        assert_eq!(
            denial.code(),
            ForgeServerDenialCode::PreviewBranchAccessDenied
        );
        assert_eq!(denial.priority(), ForgeServerDenialPriority::Authorization);
        assert_eq!(denial.step(), ForgeServerPipelineStep::AuthorizationPosture);
        assert_eq!(
            denial.detail(),
            "preview branch access is denied by middleware authorization posture"
        );
    }
}

#[test]
fn admit_denies_compat_http_diagnostics_budget_before_validation_when_authorization_allows() {
    let server = test_server(
        ForgeServerRequestContextConfig::builder()
            .with_default_diagnostics_profile(DiagnosticRichnessProfile::Forensic)
            .with_maximum_diagnostics_profile(DiagnosticRichnessProfile::Forensic)
            .build()
            .expect("request context config should validate"),
        ForgeServerMiddlewareConfig::builder()
            .with_compat_http_maximum_diagnostics_profile(
                DiagnosticRichnessProfile::OperationalMinimal,
            )
            .with_preview_branch_authorization_enabled(true)
            .with_query_mutation_enabled(false)
            .build()
            .expect("middleware config should validate"),
    );

    let denial = denied(
        server.middleware().admit(ForgeServerPipelineInput::new(
            resolve_request_context(
                &server,
                request_input_builder()
                    .with_surface_family(ForgeServerSurfaceFamily::CompatHttp)
                    .with_transport_class(ForgeServerTransportClass::CompatHttp)
                    .with_diagnostics_profile(DiagnosticRichnessProfile::Forensic)
                    .build()
                    .expect("request context input should validate"),
            ),
            ForgeServerPipelineIntent::query_mutation("users.rename"),
        )),
    );

    assert_eq!(
        denial.code(),
        ForgeServerDenialCode::CompatHttpDiagnosticsBudgetExceeded
    );
    assert_eq!(denial.priority(), ForgeServerDenialPriority::Budget);
    assert_eq!(denial.step(), ForgeServerPipelineStep::BudgetPosture);
}
