use forge_proof::TransitionOutcome;
use forge_server::{
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerBuildError, ForgeServerCompatHttpRouteFamily,
    ForgeServerCompatibilityExecutionInput, ForgeServerCompatibilityRequestInput,
    ForgeServerConfig, ForgeServerOperationDenial, ForgeServerOperationFamily,
    ForgeServerOperationRegistration, ForgeServerQueryHandoffConfig,
    ForgeServerQueryWorkspaceBindingError, ForgeServerQueryWorkspaceBindingRequest,
    ForgeServerQueryWorkspaceProvider, ForgeServerRequestContextConfig,
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

#[test]
fn operation_family_inventory_is_distinct_from_surface_inventory() {
    let server = build_server(
        CountingWorkspaceProvider::default(),
        ForgeServerOperationRegistration::phase_two_defaults(),
    )
    .expect("server should build");

    let surface_inventory = server.surface_inventory();
    let operation_inventory = server.operation_inventory();

    assert_eq!(surface_inventory.registered_families.len(), 2);
    assert!(operation_inventory.registered_families().len() >= 8);
    assert!(operation_inventory
        .registered_families()
        .contains(&ForgeServerOperationFamily::QueryDirectRead));
    assert!(operation_inventory
        .registered_families()
        .contains(&ForgeServerOperationFamily::SyncLease));
    assert_eq!(
        server.operation_registry().admit(
            forge_server::ForgeServerSurfaceFamily::CompatHttp,
            ForgeServerOperationFamily::QueryDirectProjection,
        ),
        Err(ForgeServerOperationDenial::SurfaceFamilyNotExposed {
            family: ForgeServerOperationFamily::QueryDirectProjection,
            surface_family: forge_server::ForgeServerSurfaceFamily::CompatHttp,
        })
    );
}

#[test]
fn unregistered_operation_family_denies_before_execution() {
    let provider = CountingWorkspaceProvider::default();
    let binds = provider.bind_count();
    let server = build_server(
        provider,
        vec![ForgeServerOperationRegistration::enabled(
            ForgeServerOperationFamily::QueryDirectSubmission,
        )
        .exposed_on([
            forge_server::ForgeServerSurfaceFamily::ForgeNative,
            forge_server::ForgeServerSurfaceFamily::CompatHttp,
        ])],
    )
    .expect("server should build");

    let prepared_request = prepared_read_request(&server);
    let outcome = server
        .compat_http()
        .read(ForgeServerCompatibilityExecutionInput::new(
            prepared_request,
            "users.profile",
        ));

    let denial = match outcome {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected operation-family denial, got {other:?}"),
    };
    assert_eq!(
        denial.code(),
        forge_server::ForgeServerQueryHandoffDenialCode::OperationFamilyNotRegistered
    );
    assert_eq!(binds.load(Ordering::Relaxed), 0);
}

#[test]
fn disabled_operation_family_denies_before_execution() {
    let provider = CountingWorkspaceProvider::default();
    let binds = provider.bind_count();
    let server = build_server(
        provider,
        vec![ForgeServerOperationRegistration::disabled(
            ForgeServerOperationFamily::QueryDirectRead,
        )
        .exposed_on([
            forge_server::ForgeServerSurfaceFamily::ForgeNative,
            forge_server::ForgeServerSurfaceFamily::CompatHttp,
        ])],
    )
    .expect("server should build");

    let prepared_request = prepared_read_request(&server);
    let outcome = server
        .compat_http()
        .read(ForgeServerCompatibilityExecutionInput::new(
            prepared_request,
            "users.profile",
        ));

    let denial = match outcome {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected operation-family denial, got {other:?}"),
    };
    assert_eq!(
        denial.code(),
        forge_server::ForgeServerQueryHandoffDenialCode::OperationFamilyDisabled
    );
    assert_eq!(binds.load(Ordering::Relaxed), 0);
}

#[test]
fn operation_family_surface_exposure_denies_before_execution() {
    let provider = CountingWorkspaceProvider::default();
    let binds = provider.bind_count();
    let server = build_server(
        provider,
        vec![ForgeServerOperationRegistration::enabled(
            ForgeServerOperationFamily::QueryDirectRead,
        )
        .exposed_on([forge_server::ForgeServerSurfaceFamily::ForgeNative])],
    )
    .expect("server should build");

    let prepared_request = prepared_read_request(&server);
    let outcome = server
        .compat_http()
        .read(ForgeServerCompatibilityExecutionInput::new(
            prepared_request,
            "users.profile",
        ));

    let denial = match outcome {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected operation-family denial, got {other:?}"),
    };
    assert_eq!(
        denial.code(),
        forge_server::ForgeServerQueryHandoffDenialCode::OperationFamilyNotExposedOnSurface
    );
    assert_eq!(binds.load(Ordering::Relaxed), 0);
}

#[test]
fn duplicate_operation_family_registration_fails_server_build() {
    let build_error = build_server(
        CountingWorkspaceProvider::default(),
        vec![
            ForgeServerOperationRegistration::enabled(ForgeServerOperationFamily::QueryDirectRead),
            ForgeServerOperationRegistration::disabled(ForgeServerOperationFamily::QueryDirectRead),
        ],
    )
    .expect_err("duplicate operation registrations must fail build");

    assert_eq!(
        build_error,
        ForgeServerBuildError::InvalidOperationRegistry(
            forge_server::ForgeServerOperationRegistryError::DuplicateOperationFamily {
                family: ForgeServerOperationFamily::QueryDirectRead,
            },
        )
    );
}

fn build_server(
    workspace_provider: impl ForgeServerQueryWorkspaceProvider,
    operations: Vec<ForgeServerOperationRegistration>,
) -> Result<ForgeServer, ForgeServerBuildError> {
    ForgeServer::builder()
        .with_config(
            ForgeServerConfig::builder()
                .with_bind_address(([127, 0, 0, 1], 8080).into())
                .with_request_context_config(
                    ForgeServerRequestContextConfig::builder()
                        .with_preview_targeting_enabled(true)
                        .with_default_diagnostics_profile(
                            forge_server::request_context::DiagnosticRichnessProfile::Standard,
                        )
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
        .register_operations(operations)
        .register_surface(ForgeNativeSurface::enabled())
        .register_surface(CompatHttpSurface::phase_one_enabled())
        .build()
}

fn prepared_read_request(
    server: &ForgeServer,
) -> forge_server::ForgeServerCompatibilityPreparedRequest {
    match server.compat_http().prepare_request(
        ForgeServerCompatibilityRequestInput::builder()
            .with_authenticated_principal_id("principal-7")
            .with_tenant_id("tenant-a")
            .with_workspace_id("workspace-42")
            .with_branch_id("branch-9")
            .with_route_family(ForgeServerCompatHttpRouteFamily::Read)
            .with_method("GET")
            .with_path("/compat/reads/users.profile")
            .with_header("accept", "application/json")
            .build()
            .expect("compat request should validate"),
    ) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected prepared compatibility request, got {other:?}"),
    }
}

#[derive(Clone, Debug, Default)]
struct CountingWorkspaceProvider {
    bind_count: Arc<AtomicUsize>,
}

impl CountingWorkspaceProvider {
    fn bind_count(&self) -> Arc<AtomicUsize> {
        self.bind_count.clone()
    }
}

impl ForgeServerQueryWorkspaceProvider for CountingWorkspaceProvider {
    fn provider_name(&self) -> &'static str {
        "counting-workspace-provider"
    }

    fn bind_workspace(
        &self,
        _request: &ForgeServerQueryWorkspaceBindingRequest,
    ) -> Result<forge_query::facade::ForgeQueryWorkspace, ForgeServerQueryWorkspaceBindingError>
    {
        self.bind_count.fetch_add(1, Ordering::Relaxed);
        Err(ForgeServerQueryWorkspaceBindingError::new(
            "bind_workspace",
            "operation-family guard should deny before workspace binding",
        ))
    }
}
