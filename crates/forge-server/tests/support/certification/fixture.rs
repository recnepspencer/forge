use forge_proof::{TransitionOutcome, TransitionReadiness};
use forge_server::{
    request_context::DiagnosticRichnessProfile, ForgeServer, ForgeServerMiddlewareConfig,
    ForgeServerOperatorEvidenceConfig, ForgeServerPipelineInput, ForgeServerPipelineIntent,
    ForgeServerQueryHandoffInput, ForgeServerQueryHandoffOperation, ForgeServerResponseConfig,
    ForgeServerResponseInput, ForgeServerResponseTransform, ForgeServerSurfaceFamily,
    ForgeServerTransportClass,
};

#[path = "../query_handoff/runtime.rs"]
mod query_handoff_runtime;
#[path = "../response_fixture.rs"]
mod response_fixture;

use query_handoff_runtime::TestWorkspaceProvider;
use response_fixture::{
    operator_evidence_record, query_handoff_durable_denial, resolve_blank_principal_denial,
    resolve_preview_denial, test_server_with_request_context_and_operator_evidence_config,
};

use super::certification_bundle::ForgeServerCertificationBundle;

pub fn certification_server(
    request_context_profile: DiagnosticRichnessProfile,
    response_profile: DiagnosticRichnessProfile,
    operator_profile: DiagnosticRichnessProfile,
) -> ForgeServer {
    test_server_with_request_context_and_operator_evidence_config(
        TestWorkspaceProvider::default(),
        ForgeServerMiddlewareConfig::builder()
            .with_query_mutation_enabled(true)
            .build()
            .expect("middleware config should validate"),
        forge_server::ForgeServerRequestContextConfig::builder()
            .with_default_diagnostics_profile(request_context_profile)
            .with_maximum_diagnostics_profile(DiagnosticRichnessProfile::Forensic)
            .with_preview_targeting_enabled(false)
            .build()
            .expect("request context config should validate"),
        ForgeServerResponseConfig::builder()
            .with_success_minimum_diagnostics_profile(response_profile)
            .with_denial_minimum_diagnostics_profile(response_profile)
            .build()
            .expect("response config should validate"),
        ForgeServerOperatorEvidenceConfig::builder()
            .with_minimum_diagnostics_profile(operator_profile)
            .build()
            .expect("operator evidence config should validate"),
    )
}

pub fn read_success_bundle(
    server: &ForgeServer,
    surface_family: ForgeServerSurfaceFamily,
    transport_class: ForgeServerTransportClass,
    transform: ForgeServerResponseTransform,
) -> ForgeServerCertificationBundle {
    read_success_bundle_for_workspace(
        server,
        surface_family,
        transport_class,
        transform,
        "tenant-a",
        "workspace-42",
    )
}

pub fn read_success_bundle_for_workspace(
    server: &ForgeServer,
    surface_family: ForgeServerSurfaceFamily,
    transport_class: ForgeServerTransportClass,
    transform: ForgeServerResponseTransform,
    tenant_id: &str,
    workspace_id: &str,
) -> ForgeServerCertificationBundle {
    let request_context = resolve_ready_request_context(
        server,
        surface_family,
        transport_class,
        tenant_id,
        workspace_id,
    );
    let admission = admit_query_read(server, request_context);
    let response = server.responses().shape(
        ForgeServerResponseInput::query_handoff_success(prepare_query_handoff(
            server,
            admission,
            ForgeServerQueryHandoffOperation::query_read("users.profile"),
        )),
        transform,
    );
    ForgeServerCertificationBundle::from_response_and_evidence(
        request_context_digest(
            "principal-7",
            tenant_id,
            workspace_id,
            "head",
            DiagnosticRichnessProfile::Standard,
        ),
        response.clone(),
        operator_evidence_record(server, response),
    )
}

pub fn malformed_identity_bundle(server: &ForgeServer) -> ForgeServerCertificationBundle {
    let response = server.responses().shape(
        ForgeServerResponseInput::request_context_denied(resolve_blank_principal_denial(server)),
        ForgeServerResponseTransform::compat_http(),
    );
    ForgeServerCertificationBundle::from_response_and_evidence(
        String::from("request_context:invalid_principal"),
        response.clone(),
        operator_evidence_record(server, response),
    )
}

pub fn preview_branch_denial_bundle(server: &ForgeServer) -> ForgeServerCertificationBundle {
    let response = server.responses().shape(
        ForgeServerResponseInput::request_context_denied(resolve_preview_denial(server)),
        ForgeServerResponseTransform::compat_http(),
    );
    ForgeServerCertificationBundle::from_response_and_evidence(
        String::from("request_context:preview_disabled"),
        response.clone(),
        operator_evidence_record(server, response),
    )
}

pub fn durable_resume_denial_bundle(server: &ForgeServer) -> ForgeServerCertificationBundle {
    let response = server.responses().shape(
        ForgeServerResponseInput::query_handoff_denied(query_handoff_durable_denial(server)),
        ForgeServerResponseTransform::compat_http(),
    );
    ForgeServerCertificationBundle::from_response_and_evidence(
        request_context_digest(
            "principal-7",
            "tenant-a",
            "workspace-42",
            "head",
            DiagnosticRichnessProfile::Standard,
        ),
        response.clone(),
        operator_evidence_record(server, response),
    )
}

pub fn middleware_preview_authorization_denial_bundle() -> ForgeServerCertificationBundle {
    let server = test_server_with_request_context_and_operator_evidence_config(
        TestWorkspaceProvider::default(),
        ForgeServerMiddlewareConfig::builder()
            .with_preview_branch_authorization_enabled(false)
            .with_query_mutation_enabled(true)
            .build()
            .expect("middleware config should validate"),
        forge_server::ForgeServerRequestContextConfig::builder()
            .with_default_diagnostics_profile(DiagnosticRichnessProfile::Standard)
            .with_preview_targeting_enabled(true)
            .build()
            .expect("request context config should validate"),
        ForgeServerResponseConfig::builder()
            .build()
            .expect("response config should validate"),
        ForgeServerOperatorEvidenceConfig::builder()
            .build()
            .expect("operator evidence config should validate"),
    );

    let response = server.responses().shape(
        ForgeServerResponseInput::middleware_denied(preview_authorization_denial(&server)),
        ForgeServerResponseTransform::compat_http(),
    );
    ForgeServerCertificationBundle::from_response_and_evidence(
        request_context_digest(
            "principal-7",
            "tenant-a",
            "workspace-42",
            "preview-9",
            DiagnosticRichnessProfile::Standard,
        ),
        response.clone(),
        operator_evidence_record(&server, response),
    )
}

fn request_context_digest(
    principal_id: &str,
    tenant_id: &str,
    workspace_id: &str,
    branch_target: &str,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> String {
    format!(
        "principal={principal_id};tenant={tenant_id};workspace={workspace_id};branch={branch_target};diagnostics={diagnostics_profile:?}"
    )
}

fn resolve_ready_request_context(
    server: &ForgeServer,
    surface_family: ForgeServerSurfaceFamily,
    transport_class: ForgeServerTransportClass,
    tenant_id: &str,
    workspace_id: &str,
) -> forge_server::ForgeServerResolvedRequestContext {
    match server.request_contexts().resolve(
        forge_server::ForgeServerRequestContextInput::builder()
            .with_surface_family(surface_family)
            .with_transport_class(transport_class)
            .with_authenticated_principal_id("principal-7")
            .with_tenant_id(tenant_id)
            .with_workspace_id(workspace_id)
            .build()
            .expect("request input should validate"),
    ) {
        TransitionReadiness::Ready(resolved) => resolved,
        other => panic!("expected resolved request context, got {other:?}"),
    }
}

fn admit_query_read(
    server: &ForgeServer,
    request_context: forge_server::ForgeServerResolvedRequestContext,
) -> forge_server::ForgeServerAdmission {
    match server.middleware().admit(ForgeServerPipelineInput::new(
        request_context,
        ForgeServerPipelineIntent::query_read("users.profile"),
    )) {
        TransitionOutcome::Success(admission) => admission,
        other => panic!("expected admitted read pipeline, got {other:?}"),
    }
}

fn prepare_query_handoff(
    server: &ForgeServer,
    admission: forge_server::ForgeServerAdmission,
    operation: ForgeServerQueryHandoffOperation,
) -> forge_server::ForgeServerQueryHandoff {
    match server
        .query_handoff()
        .prepare(ForgeServerQueryHandoffInput::new(admission, operation))
    {
        TransitionOutcome::Success(handoff) => handoff,
        other => panic!("expected query handoff success, got {other:?}"),
    }
}

fn preview_authorization_denial(server: &ForgeServer) -> forge_server::ForgeServerDenial {
    let request_context = match server.request_contexts().resolve(
        forge_server::ForgeServerRequestContextInput::builder()
            .with_surface_family(ForgeServerSurfaceFamily::CompatHttp)
            .with_transport_class(ForgeServerTransportClass::CompatHttp)
            .with_authenticated_principal_id("principal-7")
            .with_tenant_id("tenant-a")
            .with_workspace_id("workspace-42")
            .with_preview_id("preview-9")
            .build()
            .expect("preview request should validate"),
    ) {
        TransitionReadiness::Ready(resolved) => resolved,
        other => panic!("expected preview request context, got {other:?}"),
    };
    match server.middleware().admit(ForgeServerPipelineInput::new(
        request_context,
        ForgeServerPipelineIntent::query_read("users.profile"),
    )) {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected preview authorization denial, got {other:?}"),
    }
}
