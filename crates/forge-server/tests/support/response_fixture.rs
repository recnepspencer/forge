#![allow(dead_code)]

use forge_proof::{TransitionOutcome, TransitionReadiness};
use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerConfig, ForgeServerDirectDeliveryClass, ForgeServerDirectFreshnessMode,
    ForgeServerMiddlewareConfig, ForgeServerOperatorEvidenceConfig, ForgeServerQueryHandoffConfig,
    ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffInput, ForgeServerQueryHandoffOperation,
    ForgeServerQueryHandoffOutcome, ForgeServerQueryWorkspaceProvider,
    ForgeServerRequestContextConfig, ForgeServerRequestContextDenial,
    ForgeServerRequestContextInput, ForgeServerResolvedRequestContext, ForgeServerResponseConfig,
    ForgeServerResponseInput, ForgeServerResponseTransform, ForgeServerSurfaceFamily,
    ForgeServerTransportClass,
};

pub(crate) fn test_server(
    workspace_provider: impl ForgeServerQueryWorkspaceProvider,
    middleware_config: ForgeServerMiddlewareConfig,
) -> ForgeServer {
    test_server_with_response_config(
        workspace_provider,
        middleware_config,
        ForgeServerResponseConfig::builder()
            .build()
            .expect("response config should validate"),
    )
}

pub(crate) fn test_server_with_response_config(
    workspace_provider: impl ForgeServerQueryWorkspaceProvider,
    middleware_config: ForgeServerMiddlewareConfig,
    response_config: ForgeServerResponseConfig,
) -> ForgeServer {
    ForgeServer::builder()
        .with_config(
            ForgeServerConfig::builder()
                .with_bind_address(([127, 0, 0, 1], 8091).into())
                .with_middleware_config(middleware_config)
                .with_request_context_config(
                    ForgeServerRequestContextConfig::builder()
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
                .with_response_config(response_config)
                .with_operator_evidence_config(
                    ForgeServerOperatorEvidenceConfig::builder()
                        .build()
                        .expect("operator evidence config should validate"),
                )
                .build()
                .expect("server config should validate"),
        )
        .register_surface(ForgeNativeSurface::disabled())
        .register_surface(CompatHttpSurface::disabled())
        .build()
        .expect("server should build")
}

pub(crate) fn test_server_with_response_and_operator_evidence_config(
    workspace_provider: impl ForgeServerQueryWorkspaceProvider,
    middleware_config: ForgeServerMiddlewareConfig,
    response_config: ForgeServerResponseConfig,
    operator_evidence_config: ForgeServerOperatorEvidenceConfig,
) -> ForgeServer {
    test_server_with_request_context_and_operator_evidence_config(
        workspace_provider,
        middleware_config,
        ForgeServerRequestContextConfig::builder()
            .with_default_diagnostics_profile(DiagnosticRichnessProfile::Standard)
            .build()
            .expect("request context config should validate"),
        response_config,
        operator_evidence_config,
    )
}

pub(crate) fn test_server_with_request_context_and_operator_evidence_config(
    workspace_provider: impl ForgeServerQueryWorkspaceProvider,
    middleware_config: ForgeServerMiddlewareConfig,
    request_context_config: ForgeServerRequestContextConfig,
    response_config: ForgeServerResponseConfig,
    operator_evidence_config: ForgeServerOperatorEvidenceConfig,
) -> ForgeServer {
    ForgeServer::builder()
        .with_config(
            ForgeServerConfig::builder()
                .with_bind_address(([127, 0, 0, 1], 8091).into())
                .with_middleware_config(middleware_config)
                .with_request_context_config(request_context_config)
                .with_query_handoff_config(
                    ForgeServerQueryHandoffConfig::builder()
                        .with_workspace_provider(workspace_provider)
                        .build()
                        .expect("query handoff config should validate"),
                )
                .with_response_config(response_config)
                .with_operator_evidence_config(operator_evidence_config)
                .build()
                .expect("server config should validate"),
        )
        .register_surface(ForgeNativeSurface::disabled())
        .register_surface(CompatHttpSurface::disabled())
        .build()
        .expect("server should build")
}

pub(crate) fn request_input(
    surface_family: ForgeServerSurfaceFamily,
    transport_class: ForgeServerTransportClass,
) -> ForgeServerRequestContextInput {
    ForgeServerRequestContextInput::builder()
        .with_surface_family(surface_family)
        .with_transport_class(transport_class)
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .build()
        .expect("request input should validate")
}

pub(crate) fn resolve_ready(
    server: &ForgeServer,
    input: ForgeServerRequestContextInput,
) -> ForgeServerResolvedRequestContext {
    match server.request_contexts().resolve(input) {
        TransitionReadiness::Ready(resolved) => resolved,
        other => panic!("expected resolved request context, got {other:?}"),
    }
}

pub(crate) fn resolve_preview_denial(server: &ForgeServer) -> ForgeServerRequestContextDenial {
    let input = ForgeServerRequestContextInput::builder()
        .with_surface_family(ForgeServerSurfaceFamily::CompatHttp)
        .with_transport_class(ForgeServerTransportClass::CompatHttp)
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .with_preview_id("preview-9")
        .build()
        .expect("preview request should validate");
    match server.request_contexts().resolve(input) {
        TransitionReadiness::Denied(denial) => denial,
        other => panic!("expected request context denial, got {other:?}"),
    }
}

pub(crate) fn resolve_blank_principal_denial(
    server: &ForgeServer,
) -> ForgeServerRequestContextDenial {
    let input = ForgeServerRequestContextInput::builder()
        .with_surface_family(ForgeServerSurfaceFamily::CompatHttp)
        .with_transport_class(ForgeServerTransportClass::CompatHttp)
        .with_authenticated_principal_id("   ")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .build()
        .expect("blank principal request should validate");
    match server.request_contexts().resolve(input) {
        TransitionReadiness::Denied(denial) => denial,
        other => panic!("expected request context denial, got {other:?}"),
    }
}

pub(crate) fn resolve_blank_workspace_denial(
    server: &ForgeServer,
) -> ForgeServerRequestContextDenial {
    let input = ForgeServerRequestContextInput::builder()
        .with_surface_family(ForgeServerSurfaceFamily::CompatHttp)
        .with_transport_class(ForgeServerTransportClass::CompatHttp)
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("   ")
        .build()
        .expect("blank workspace request should validate");
    match server.request_contexts().resolve(input) {
        TransitionReadiness::Denied(denial) => denial,
        other => panic!("expected request context denial, got {other:?}"),
    }
}

pub(crate) fn admit_read(
    server: &ForgeServer,
    resolved: ForgeServerResolvedRequestContext,
) -> forge_server::ForgeServerAdmission {
    match server
        .middleware()
        .admit(forge_server::ForgeServerPipelineInput::new(
            resolved,
            forge_server::ForgeServerPipelineIntent::query_read("users.profile"),
        )) {
        TransitionOutcome::Success(admission) => admission,
        other => panic!("expected admitted read, got {other:?}"),
    }
}

pub(crate) fn admit_mutation(
    server: &ForgeServer,
    resolved: ForgeServerResolvedRequestContext,
) -> forge_server::ForgeServerAdmission {
    match server
        .middleware()
        .admit(forge_server::ForgeServerPipelineInput::new(
            resolved,
            forge_server::ForgeServerPipelineIntent::query_mutation("users.rename"),
        )) {
        TransitionOutcome::Success(admission) => admission,
        other => panic!("expected admitted mutation, got {other:?}"),
    }
}

pub(crate) fn middleware_mutation_denial(server: &ForgeServer) -> forge_server::ForgeServerDenial {
    match server
        .middleware()
        .admit(forge_server::ForgeServerPipelineInput::new(
            resolve_ready(
                server,
                request_input(
                    ForgeServerSurfaceFamily::CompatHttp,
                    ForgeServerTransportClass::CompatHttp,
                ),
            ),
            forge_server::ForgeServerPipelineIntent::query_mutation("users.rename"),
        )) {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected middleware denial, got {other:?}"),
    }
}

pub(crate) fn query_handoff_success(
    server: &ForgeServer,
    surface_family: ForgeServerSurfaceFamily,
    transport_class: ForgeServerTransportClass,
    operation: ForgeServerQueryHandoffOperation,
) -> forge_server::ForgeServerQueryHandoff {
    let resolved = resolve_ready(server, request_input(surface_family, transport_class));
    let admission = match &operation {
        ForgeServerQueryHandoffOperation::QueryRead { .. }
        | ForgeServerQueryHandoffOperation::DownstreamDelivery { .. } => {
            admit_read(server, resolved)
        }
        ForgeServerQueryHandoffOperation::QueryMutation { .. } => admit_mutation(server, resolved),
        ForgeServerQueryHandoffOperation::DirectRead { .. }
        | ForgeServerQueryHandoffOperation::DirectState { .. }
        | ForgeServerQueryHandoffOperation::DirectInspection { .. }
        | ForgeServerQueryHandoffOperation::DirectProjection { .. }
        | ForgeServerQueryHandoffOperation::DirectMutation { .. } => {
            panic!("response fixture does not construct direct forge-native query handoffs")
        }
    };
    match server
        .query_handoff()
        .prepare(ForgeServerQueryHandoffInput::new(admission, operation))
    {
        TransitionOutcome::Success(handoff) => handoff,
        other => panic!("expected query handoff success, got {other:?}"),
    }
}

pub(crate) fn query_handoff_durable_denial(server: &ForgeServer) -> ForgeServerQueryHandoffDenial {
    let outcome: ForgeServerQueryHandoffOutcome =
        server
            .query_handoff()
            .prepare(ForgeServerQueryHandoffInput::new(
                admit_read(
                    server,
                    resolve_ready(
                        server,
                        request_input(
                            ForgeServerSurfaceFamily::CompatHttp,
                            ForgeServerTransportClass::CompatHttp,
                        ),
                    ),
                ),
                ForgeServerQueryHandoffOperation::downstream_delivery(
                    "users.profile",
                    ForgeServerDirectFreshnessMode::LiveStrict,
                    ForgeServerDirectDeliveryClass::AuthoritativeOrdered,
                    forge_server::ForgeServerQueryRequestedResume::durable(),
                ),
            ));
    match outcome {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected durable resume denial, got {other:?}"),
    }
}

pub(crate) fn shape_success(
    server: &ForgeServer,
    handoff: forge_server::ForgeServerQueryHandoff,
    transform: ForgeServerResponseTransform,
) -> forge_server::ForgeServerResponseEnvelope {
    server.responses().shape(
        ForgeServerResponseInput::query_handoff_success(handoff),
        transform,
    )
}

pub(crate) fn operator_evidence_record(
    server: &ForgeServer,
    response: forge_server::ForgeServerResponseEnvelope,
) -> forge_server::ForgeServerOperatorEvidenceRecord {
    server
        .operator_evidence()
        .record(
            forge_server::ForgeServerEvidenceInput::response_envelope(response),
            forge_server::ForgeServerEvidenceTransform::operator_default(),
        )
        .expect("operator evidence record should materialize")
}
