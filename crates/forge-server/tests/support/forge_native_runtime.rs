use forge_proof::{TransitionOutcome, TransitionReadiness};
use forge_query::facade::ForgeQueryRuntimeSupportProfile;
use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerConfig, ForgeServerDenialCode, ForgeServerForgeNativeSessionDenial,
    ForgeServerForgeNativeSessionInput, ForgeServerMiddlewareConfig, ForgeServerQueryHandoffConfig,
    ForgeServerQueryWorkspaceBindingError, ForgeServerQueryWorkspaceBindingRequest,
    ForgeServerQueryWorkspaceBindingTarget, ForgeServerQueryWorkspaceProvider,
    ForgeServerRequestContextConfig, ForgeServerRequestContextInput,
    ForgeServerResolvedRequestContext, ForgeServerSurfaceFamily, ForgeServerTransportClass,
};
use std::sync::{atomic::AtomicUsize, Arc, Mutex};

use crate::query_handoff_runtime::{
    ProfiledCountingTestWorkspaceProvider, ProfiledTestWorkspaceProvider, TestWorkspaceProvider,
};

pub(crate) fn build_server(register_forge_native: bool) -> ForgeServer {
    build_server_with_workspace_provider(TestWorkspaceProvider::default(), register_forge_native)
}

pub(crate) fn build_server_with_disabled_forge_native() -> ForgeServer {
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
                        .with_workspace_provider(TestWorkspaceProvider::default())
                        .build()
                        .expect("query handoff config should validate"),
                )
                .build()
                .expect("server config should validate"),
        )
        .register_surface(ForgeNativeSurface::disabled())
        .register_surface(CompatHttpSurface::disabled())
        .build()
        .expect("server should build")
}

pub(crate) fn build_server_with_preview_denial() -> ForgeServer {
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
                        .with_preview_branch_authorization_enabled(false)
                        .with_query_mutation_enabled(true)
                        .build()
                        .expect("middleware config should validate"),
                )
                .with_query_handoff_config(
                    ForgeServerQueryHandoffConfig::builder()
                        .with_workspace_provider(TestWorkspaceProvider::default())
                        .build()
                        .expect("query handoff config should validate"),
                )
                .build()
                .expect("server config should validate"),
        )
        .register_surface(ForgeNativeSurface::enabled())
        .register_surface(CompatHttpSurface::disabled())
        .build()
        .expect("server should build")
}

pub(crate) fn build_server_with_profiled_workspace(
    support_profile: ForgeQueryRuntimeSupportProfile,
) -> ForgeServer {
    build_server_with_workspace_provider(ProfiledTestWorkspaceProvider::new(support_profile), true)
}

pub(crate) fn build_server_with_profiled_counting_workspace(
    support_profile: ForgeQueryRuntimeSupportProfile,
) -> (ForgeServer, Arc<AtomicUsize>) {
    let attempted_writes = Arc::new(AtomicUsize::new(0));
    let server = build_server_with_workspace_provider(
        ProfiledCountingTestWorkspaceProvider::new(support_profile, attempted_writes.clone()),
        true,
    );
    (server, attempted_writes)
}

pub(crate) fn build_server_with_capturing_workspace_provider(
) -> (ForgeServer, CapturingWorkspaceProvider) {
    let workspace_provider = CapturingWorkspaceProvider::default();
    let server = build_server_with_workspace_provider(workspace_provider.clone(), true);
    (server, workspace_provider)
}

pub(crate) fn build_server_with_failing_workspace_provider(
    stage: &'static str,
    message: &'static str,
) -> ForgeServer {
    build_server_with_workspace_provider(FailingWorkspaceProvider { stage, message }, true)
}

pub(crate) fn server_with_request_context_default(
    default_diagnostics_profile: DiagnosticRichnessProfile,
) -> ForgeServer {
    ForgeServer::builder()
        .with_config(
            ForgeServerConfig::builder()
                .with_bind_address(([127, 0, 0, 1], 8080).into())
                .with_request_context_config(
                    ForgeServerRequestContextConfig::builder()
                        .with_preview_targeting_enabled(true)
                        .with_default_diagnostics_profile(default_diagnostics_profile)
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
                        .with_workspace_provider(TestWorkspaceProvider::default())
                        .build()
                        .expect("query handoff config should validate"),
                )
                .build()
                .expect("server config should validate"),
        )
        .register_surface(ForgeNativeSurface::enabled())
        .register_surface(CompatHttpSurface::disabled())
        .build()
        .expect("server should build")
}

pub(crate) fn forge_native_session_input_builder(
) -> forge_server::ForgeServerForgeNativeSessionInputBuilder {
    ForgeServerForgeNativeSessionInput::builder()
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
}

pub(crate) fn resolve_forge_native_request_context(
    server: &ForgeServer,
) -> ForgeServerResolvedRequestContext {
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

pub(crate) fn denied_prepared_session(
    outcome: forge_proof::TransitionOutcome<
        forge_server::ForgeServerForgeNativePreparedSession,
        ForgeServerForgeNativeSessionDenial,
        forge_server::ForgeServerForgeNativeDeferred,
        forge_server::ForgeServerForgeNativeStale,
        forge_server::ForgeServerForgeNativeRebindRequired,
        forge_server::ForgeServerForgeNativeFailure,
    >,
) -> ForgeServerForgeNativeSessionDenial {
    match outcome {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected forge-native denial, got {other:?}"),
    }
}

pub(crate) fn denied_session(
    outcome: forge_proof::TransitionOutcome<
        forge_server::ForgeServerForgeNativeSession,
        ForgeServerForgeNativeSessionDenial,
        forge_server::ForgeServerForgeNativeDeferred,
        forge_server::ForgeServerForgeNativeStale,
        forge_server::ForgeServerForgeNativeRebindRequired,
        forge_server::ForgeServerForgeNativeFailure,
    >,
) -> ForgeServerForgeNativeSessionDenial {
    match outcome {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected forge-native denial, got {other:?}"),
    }
}

pub(crate) fn expect_preview_access_denial(denial: &ForgeServerForgeNativeSessionDenial) {
    assert_eq!(
        denial
            .middleware_denial()
            .expect("middleware denial should be preserved")
            .code(),
        ForgeServerDenialCode::PreviewBranchAccessDenied
    );
}

pub(crate) fn build_server_with_workspace_provider(
    workspace_provider: impl ForgeServerQueryWorkspaceProvider,
    register_forge_native: bool,
) -> ForgeServer {
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
    );

    if register_forge_native {
        builder = builder.register_surface(ForgeNativeSurface::enabled());
    }

    builder
        .register_surface(CompatHttpSurface::disabled())
        .build()
        .expect("server should build")
}

#[derive(Clone, Debug, Default)]
pub(crate) struct CapturingWorkspaceProvider {
    captured_targets: Arc<Mutex<Vec<ForgeServerQueryWorkspaceBindingTarget>>>,
}

impl CapturingWorkspaceProvider {
    pub(crate) fn take_captured_targets(&self) -> Vec<ForgeServerQueryWorkspaceBindingTarget> {
        let mut captured_targets = self
            .captured_targets
            .lock()
            .expect("capturing workspace provider mutex should not be poisoned");
        std::mem::take(&mut *captured_targets)
    }
}

impl ForgeServerQueryWorkspaceProvider for CapturingWorkspaceProvider {
    fn provider_name(&self) -> &'static str {
        "capturing-test-workspace-provider"
    }

    fn bind_workspace(
        &self,
        request: &ForgeServerQueryWorkspaceBindingRequest,
    ) -> Result<forge_query::facade::ForgeQueryWorkspace, ForgeServerQueryWorkspaceBindingError>
    {
        self.captured_targets
            .lock()
            .expect("capturing workspace provider mutex should not be poisoned")
            .push(request.target().clone());

        TestWorkspaceProvider.bind_workspace(request)
    }
}

#[derive(Clone, Debug)]
struct FailingWorkspaceProvider {
    stage: &'static str,
    message: &'static str,
}

impl ForgeServerQueryWorkspaceProvider for FailingWorkspaceProvider {
    fn provider_name(&self) -> &'static str {
        "failing-test-workspace-provider"
    }

    fn bind_workspace(
        &self,
        _request: &ForgeServerQueryWorkspaceBindingRequest,
    ) -> Result<forge_query::facade::ForgeQueryWorkspace, ForgeServerQueryWorkspaceBindingError>
    {
        Err(ForgeServerQueryWorkspaceBindingError::new(
            self.stage,
            self.message,
        ))
    }
}
