#[path = "support/compat_http/phase_three_runtime.rs"]
mod compat_http_phase_three_runtime;
#[path = "support/forge_native/assertions.rs"]
mod forge_native_assertions;
#[path = "support/forge_native/runtime.rs"]
mod forge_native_runtime;
#[path = "support/query_handoff/runtime.rs"]
mod query_handoff_runtime;

use forge_proof::TransitionOutcome;
use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerCompatHttpRouteFamily, ForgeServerCompatibilityMutationExecutionInput,
    ForgeServerCompatibilityRequestInput, ForgeServerConfig, ForgeServerMiddlewareConfig,
    ForgeServerOperationAuthorityDeclaration, ForgeServerOperationAuthorizationPolicy,
    ForgeServerOperationFamily, ForgeServerOperationRegistration, ForgeServerQueryHandoffConfig,
    ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffDenialCode, ForgeServerQueryOperation,
    ForgeServerRequestContextConfig, ForgeServerSuccessKind, ForgeServerSurfaceFamily,
};

use compat_http_phase_three_runtime::{compat_mutation_success, insert_task, single_insert_body};
use forge_native_assertions::forge_native_session;
use query_handoff_runtime::RealMutationWorkspaceProvider;

#[test]
fn successful_mutation_boundaries_expose_the_canonical_operation_request() {
    let server = build_operation_name_guarded_server();
    let compat = compat_mutation_success(server.compat_http().mutate(
        ForgeServerCompatibilityMutationExecutionInput::new(
            prepared_mutation_request(&server, "Tasks.Insert"),
            "Tasks.Insert",
            single_insert_body("task-1"),
        ),
    ));
    let direct = direct_mutation_success(forge_native_session(&server).direct().mutate(
        &ForgeServerQueryOperation::single_mutation("Tasks.Insert", insert_task("task-1")),
    ));

    assert_eq!(
        compat
            .envelope()
            .response_envelope()
            .success()
            .expect("compat response should succeed")
            .payload()
            .kind(),
        ForgeServerSuccessKind::QueryMutation
    );
    assert_eq!(
        compat.operation_request().identity().operation_name(),
        "tasks.insert"
    );
    assert_eq!(
        direct.operation_request().identity().operation_name(),
        "tasks.insert"
    );
}

#[test]
fn compat_mutation_denial_reports_unknown_operation_name_as_structured_facts() {
    let server = build_operation_name_guarded_server();
    let denial = compat_mutation_denied(server.compat_http().mutate(
        ForgeServerCompatibilityMutationExecutionInput::new(
            prepared_mutation_request(&server, "tasks.unknown"),
            "tasks.unknown",
            single_insert_body("task-1"),
        ),
    ));

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::UnknownOperationName
    );
    assert_eq!(
        denial
            .facts()
            .and_then(|facts| facts.rejected_operation_name()),
        Some("tasks.unknown")
    );
}

#[test]
fn direct_mutation_denial_reports_unknown_operation_name_as_structured_facts() {
    let server = build_operation_name_guarded_server();
    let denial = direct_mutation_denied(forge_native_session(&server).direct().mutate(
        &ForgeServerQueryOperation::single_mutation("tasks.unknown", insert_task("task-1")),
    ));

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::UnknownOperationName
    );
    assert_eq!(
        denial
            .facts()
            .and_then(|facts| facts.rejected_operation_name()),
        Some("tasks.unknown")
    );
}

fn build_operation_name_guarded_server() -> ForgeServer {
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
                        .with_workspace_provider(RealMutationWorkspaceProvider)
                        .build()
                        .expect("query handoff config should validate"),
                )
                .build()
                .expect("server config should validate"),
        )
        .register_operations(operation_name_guarded_registrations())
        .register_surface(ForgeNativeSurface::enabled())
        .register_surface(CompatHttpSurface::phase_one_enabled())
        .build()
        .expect("operation-name guarded server should build")
}

fn operation_name_guarded_registrations() -> Vec<ForgeServerOperationRegistration> {
    ForgeServerOperationRegistration::phase_two_defaults()
        .into_iter()
        .filter(|registration| {
            registration.family() != ForgeServerOperationFamily::QueryDirectSubmission
        })
        .chain([ForgeServerOperationRegistration::enabled(
            ForgeServerOperationFamily::QueryDirectSubmission,
        )
        .exposed_on([
            ForgeServerSurfaceFamily::ForgeNative,
            ForgeServerSurfaceFamily::CompatHttp,
        ])
        .admit_operation_names(["tasks.insert"])
        .with_authority_declaration(
            ForgeServerOperationAuthorityDeclaration::deterministic_submission(
                "query-write",
                "query-write-review",
                "derive-from-request",
                "derive-from-request",
            ),
        )
        .with_authorization_policy(ForgeServerOperationAuthorizationPolicy::allow_authenticated())])
        .collect()
}

fn prepared_mutation_request(
    server: &ForgeServer,
    operation_name: &str,
) -> forge_server::ForgeServerCompatibilityPreparedRequest {
    match server.compat_http().prepare_request(
        ForgeServerCompatibilityRequestInput::builder()
            .with_authenticated_principal_id("principal-7")
            .with_tenant_id("tenant-a")
            .with_workspace_id("workspace-42")
            .with_branch_id("branch-9")
            .with_route_family(ForgeServerCompatHttpRouteFamily::Mutation)
            .with_method("POST")
            .with_path(format!("/compat/mutations/{operation_name}"))
            .with_header("accept", "application/json")
            .with_body_content_type("application/json")
            .with_body_present(true)
            .build()
            .expect("compat mutation request should validate"),
    ) {
        TransitionOutcome::Success(prepared) => prepared,
        other => panic!("expected prepared compatibility mutation request, got {other:?}"),
    }
}

fn compat_mutation_denied(
    outcome: forge_server::ForgeServerCompatibilityMutationOutcome<
        forge_server::ForgeServerCompatibilityMutation,
    >,
) -> ForgeServerQueryHandoffDenial {
    match outcome {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected denied compat mutation, got {other:?}"),
    }
}

fn direct_mutation_success(
    outcome: forge_server::ForgeServerDirectMutationOutcome,
) -> forge_server::ForgeServerDirectMutation {
    match outcome {
        TransitionOutcome::Success(mutation) => mutation,
        other => panic!("expected direct mutation success, got {other:?}"),
    }
}

fn direct_mutation_denied(
    outcome: forge_server::ForgeServerDirectMutationOutcome,
) -> ForgeServerQueryHandoffDenial {
    match outcome {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected denied direct mutation, got {other:?}"),
    }
}
