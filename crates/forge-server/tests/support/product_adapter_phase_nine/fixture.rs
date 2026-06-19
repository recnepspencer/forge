#![allow(dead_code)]

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use forge_proof::TransitionOutcome;
use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerCompatHttpRouteFamily, ForgeServerCompatibilityPreparedRequest,
    ForgeServerCompatibilityRequestInput, ForgeServerConfig, ForgeServerForgeNativeSession,
    ForgeServerForgeNativeSessionInput, ForgeServerMiddlewareConfig,
    ForgeServerProductAdapterExecutionError, ForgeServerProductApplicationAdapter,
    ForgeServerProductApplicationAdapterRegistration, ForgeServerProductOperationBasisKind,
    ForgeServerProductOperationDeclaration, ForgeServerProductOperationDenial,
    ForgeServerProductOperationErrorMaps, ForgeServerProductOperationPayload,
    ForgeServerProductOperationSuccess, ForgeServerProductOperationSupportSnapshot,
    ForgeServerProductPayloadSchemaValidator, ForgeServerProductSession,
    ForgeServerProductSessionCreationRequest, ForgeServerQueryHandoffConfig,
    ForgeServerQueryWorkspaceProvider, ForgeServerRequestContextConfig,
};
use serde_json::Value;

#[path = "../query_handoff/runtime.rs"]
mod query_handoff_runtime;

pub fn build_server(
    registrations: Vec<ForgeServerProductApplicationAdapterRegistration>,
) -> ForgeServer {
    build_server_with_query_workspace_provider(
        registrations,
        Arc::new(query_handoff_runtime::TestWorkspaceProvider),
    )
}

pub fn build_server_with_query_workspace_provider(
    registrations: Vec<ForgeServerProductApplicationAdapterRegistration>,
    workspace_provider: Arc<dyn ForgeServerQueryWorkspaceProvider>,
) -> ForgeServer {
    ForgeServer::builder()
        .with_config(base_config_with_workspace_provider(workspace_provider))
        .register_operations(forge_server::ForgeServerOperationRegistration::phase_two_defaults())
        .register_surface(ForgeNativeSurface::enabled())
        .register_surface(CompatHttpSurface::phase_one_enabled())
        .register_product_adapters(registrations)
        .build()
        .expect("phase nine product server should build")
}

pub fn direct_session(server: &ForgeServer) -> ForgeServerForgeNativeSession {
    match server.forge_native().session(
        ForgeServerForgeNativeSessionInput::builder()
            .with_authenticated_principal_id("principal-7")
            .with_tenant_id("tenant-a")
            .with_workspace_id("workspace-42")
            .with_branch_id("branch-9")
            .build()
            .expect("forge-native session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected forge-native session, got {other:?}"),
    }
}

pub fn prepared_read_request(
    server: &ForgeServer,
    operation_name: &str,
    basis_digest: Option<&str>,
) -> ForgeServerCompatibilityPreparedRequest {
    let mut builder = ForgeServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .with_branch_id("branch-9")
        .with_route_family(ForgeServerCompatHttpRouteFamily::Read)
        .with_method("GET")
        .with_path(format!("/compat/reads/{operation_name}"))
        .with_header("accept", "application/json");
    if let Some(basis_digest) = basis_digest {
        builder = builder.with_query_pair("basis", basis_digest);
    }
    match server.compat_http().prepare_request(
        builder
            .build()
            .expect("compat read request should validate"),
    ) {
        TransitionOutcome::Success(request) => request,
        other => panic!("expected prepared read request, got {other:?}"),
    }
}

pub fn prepared_mutation_request(
    server: &ForgeServer,
    operation_name: &str,
    basis_digest: Option<&str>,
) -> ForgeServerCompatibilityPreparedRequest {
    let mut builder = ForgeServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .with_branch_id("branch-9")
        .with_route_family(ForgeServerCompatHttpRouteFamily::Mutation)
        .with_method("POST")
        .with_path(format!("/compat/mutations/{operation_name}"))
        .with_header("accept", "application/json")
        .with_body_content_type("application/json")
        .with_body_present(true);
    if let Some(basis_digest) = basis_digest {
        builder = builder.with_query_pair("basis", basis_digest);
    }
    match server.compat_http().prepare_request(
        builder
            .build()
            .expect("compat mutation request should validate"),
    ) {
        TransitionOutcome::Success(request) => request,
        other => panic!("expected prepared mutation request, got {other:?}"),
    }
}

pub fn completed(
    result: Result<
        forge_server::ForgeServerCompletedProductOperation,
        forge_server::ForgeServerProductOperationSurfaceDenial,
    >,
) -> forge_server::ForgeServerCompletedProductOperation {
    result.expect("product operation should complete through the server boundary")
}

pub fn open_mutation_product_session(
    session: &ForgeServerForgeNativeSession,
    operation_name: &str,
    basis_digest: &str,
) -> ForgeServerProductSession {
    session
        .product_sessions()
        .open_mutation(
            ForgeServerProductSessionCreationRequest::for_operation(operation_name)
                .with_basis_digest(basis_digest)
                .with_expiry_seconds(300),
        )
        .expect("mutation product session should open through the server boundary")
}

pub fn editor_registration(
    calls: Option<Arc<AtomicUsize>>,
    validator: Option<Arc<dyn ForgeServerProductPayloadSchemaValidator>>,
) -> ForgeServerProductApplicationAdapterRegistration {
    let adapter = Arc::new(EditorAdapter {
        calls: calls.unwrap_or_else(|| Arc::new(AtomicUsize::new(0))),
    });
    let render = declared(ForgeServerProductOperationDeclaration::product_read(
        "product_editor.render",
        "product-editor.render.v1",
        ForgeServerProductOperationBasisKind::DurableProductDerived,
        ForgeServerProductOperationSupportSnapshot::production_admitted("render-supported"),
    ));
    let apply = declared(ForgeServerProductOperationDeclaration::product_mutation(
        "product_editor.apply",
        "product-editor.apply.v1",
        ForgeServerProductOperationBasisKind::DurableProductDerived,
        ForgeServerProductOperationSupportSnapshot::production_admitted("apply-supported"),
        "draft",
    ));
    let apply = match validator {
        Some(validator) => apply.with_payload_validator(validator),
        None => apply,
    };
    ForgeServerProductApplicationAdapterRegistration::new("editor-adapter", adapter)
        .with_operation(render)
        .with_operation(declared(
            ForgeServerProductOperationDeclaration::product_read(
                "product_editor.select",
                "product-editor.select.v1",
                ForgeServerProductOperationBasisKind::DurableProductDerived,
                ForgeServerProductOperationSupportSnapshot::production_admitted("select-supported"),
            ),
        ))
        .with_operation(declared(
            ForgeServerProductOperationDeclaration::product_read(
                "product_editor.available_actions",
                "product-editor.actions.v1",
                ForgeServerProductOperationBasisKind::DurableProductDerived,
                ForgeServerProductOperationSupportSnapshot::production_admitted(
                    "actions-supported",
                ),
            ),
        ))
        .with_operation(apply)
        .with_operation(declared(
            ForgeServerProductOperationDeclaration::product_mutation(
                "product_editor.finalize",
                "product-editor.finalize.v1",
                ForgeServerProductOperationBasisKind::DurableProductDerived,
                ForgeServerProductOperationSupportSnapshot::production_admitted(
                    "finalize-supported",
                ),
                "draft",
            ),
        ))
}

pub fn query_derived_editor_registration(
    calls: Option<Arc<AtomicUsize>>,
) -> ForgeServerProductApplicationAdapterRegistration {
    let adapter = Arc::new(EditorAdapter {
        calls: calls.unwrap_or_else(|| Arc::new(AtomicUsize::new(0))),
    });
    ForgeServerProductApplicationAdapterRegistration::new("query-derived-editor", adapter)
        .with_operation(declared(
            ForgeServerProductOperationDeclaration::product_read(
                "product_editor.render",
                "product-editor.render.v1",
                ForgeServerProductOperationBasisKind::QueryDerived,
                ForgeServerProductOperationSupportSnapshot::production_admitted(
                    "render-query-derived",
                ),
            ),
        ))
        .with_operation(declared(
            ForgeServerProductOperationDeclaration::product_mutation(
                "product_editor.apply",
                "product-editor.apply.v1",
                ForgeServerProductOperationBasisKind::QueryDerived,
                ForgeServerProductOperationSupportSnapshot::production_admitted(
                    "apply-query-derived",
                ),
                "draft",
            ),
        ))
}

pub fn base_config() -> ForgeServerConfig {
    base_config_with_workspace_provider(Arc::new(query_handoff_runtime::TestWorkspaceProvider))
}

pub fn base_config_with_workspace_provider(
    workspace_provider: Arc<dyn ForgeServerQueryWorkspaceProvider>,
) -> ForgeServerConfig {
    ForgeServerConfig::builder()
        .with_bind_address(([127, 0, 0, 1], 8080).into())
        .with_request_context_config(
            ForgeServerRequestContextConfig::builder()
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
                .with_workspace_provider_arc(workspace_provider)
                .build()
                .expect("query handoff config should validate"),
        )
        .build()
        .expect("server config should validate")
}

fn declared(
    declaration: ForgeServerProductOperationDeclaration,
) -> ForgeServerProductOperationDeclaration {
    declaration.with_error_map(ForgeServerProductOperationErrorMaps::passthrough())
}

#[derive(Debug, Default)]
pub struct EditorAdapter {
    calls: Arc<AtomicUsize>,
}

impl ForgeServerProductApplicationAdapter for EditorAdapter {
    fn execute(
        &self,
        operation: &forge_server::ForgeServerScheduledProductOperation,
    ) -> Result<ForgeServerProductOperationSuccess, ForgeServerProductAdapterExecutionError> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        let plan = operation.plan();
        if let Some(reason_key) = plan
            .payload()
            .body()
            .get("deny_reason")
            .and_then(Value::as_str)
        {
            return Err(ForgeServerProductAdapterExecutionError::denied(
                ForgeServerProductOperationDenial::new(reason_key, "product-owned refusal"),
            ));
        }
        Ok(ForgeServerProductOperationSuccess::new(
            plan.declaration().operation_name(),
            format!(
                "{}:{}:{}",
                plan.declaration().operation_name(),
                operation.scheduler_admission().canonical_digest(),
                plan.operation_admission()
                    .operation_request()
                    .identity()
                    .basis_digest()
                    .unwrap_or("none")
            ),
        ))
    }
}

#[derive(Debug)]
pub struct RequireTitleValidator;

impl ForgeServerProductPayloadSchemaValidator for RequireTitleValidator {
    fn validate(
        &self,
        payload: &ForgeServerProductOperationPayload,
    ) -> Result<(), ForgeServerProductOperationDenial> {
        if payload
            .body()
            .get("title")
            .and_then(Value::as_str)
            .is_some()
        {
            Ok(())
        } else {
            Err(ForgeServerProductOperationDenial::new(
                "missing_title",
                "payload must include a title string",
            ))
        }
    }
}
