use crate::{
    ForgeServerCompatibilityAdmittedProductMutationCommand, ForgeServerCompatibilityFacade,
    ForgeServerCompatibilityPreparedRequest, ForgeServerCompatibilityRequestInput,
    ForgeServerCompletedProductOperation, ForgeServerCompletedProductSessionCoordination,
    ForgeServerProductOperationInput, ForgeServerProductSessionCreationRequest,
};
use forge_proof::TransitionOutcome;

use super::{
    declared_route::ForgeServerDeclaredRoute,
    operational_route::ForgeServerOperationalRoute,
    request_decoding::{
        decode_json_body, header_value, query_value, ForgeServerRouteBranchTarget,
        ForgeServerRouteTransportRequest,
    },
    transport_denial::{ForgeServerTransportDenial, ForgeServerTransportDenialCode},
};

#[derive(Clone, Debug)]
pub struct ForgeServerRouteExecutionBridge {
    route: ForgeServerRouteBridgeTarget,
    compat_http: ForgeServerCompatibilityFacade,
}

impl ForgeServerRouteExecutionBridge {
    pub(crate) fn semantic(
        route: ForgeServerDeclaredRoute,
        compat_http: ForgeServerCompatibilityFacade,
    ) -> Self {
        Self {
            route: ForgeServerRouteBridgeTarget::Semantic(route),
            compat_http,
        }
    }

    pub(crate) fn operational(
        route: ForgeServerOperationalRoute,
        compat_http: ForgeServerCompatibilityFacade,
    ) -> Self {
        Self {
            route: ForgeServerRouteBridgeTarget::Operational(route),
            compat_http,
        }
    }

    pub fn method(&self) -> &str {
        match &self.route {
            ForgeServerRouteBridgeTarget::Semantic(route) => route.method(),
            ForgeServerRouteBridgeTarget::Operational(route) => route.method(),
        }
    }

    pub fn path(&self) -> &str {
        match &self.route {
            ForgeServerRouteBridgeTarget::Semantic(route) => route.path(),
            ForgeServerRouteBridgeTarget::Operational(route) => route.path(),
        }
    }

    pub fn execute(
        &self,
        request: ForgeServerRouteTransportRequest,
    ) -> Result<ForgeServerRouteExecutionOutcome, ForgeServerTransportDenial> {
        match &self.route {
            ForgeServerRouteBridgeTarget::Semantic(route) => {
                let prepared_request = self.prepare_request(&request, route)?;
                execute_semantic_route(&self.compat_http, route, &request, &prepared_request)
            }
            ForgeServerRouteBridgeTarget::Operational(route) => {
                Ok(ForgeServerRouteExecutionOutcome::Operational(
                    ForgeServerOperationalRouteOutcome::new(route.kind(), route.path().to_string()),
                ))
            }
        }
    }

    fn prepare_request(
        &self,
        request: &ForgeServerRouteTransportRequest,
        route: &ForgeServerDeclaredRoute,
    ) -> Result<ForgeServerCompatibilityPreparedRequest, ForgeServerTransportDenial> {
        let mut builder = ForgeServerCompatibilityRequestInput::builder()
            .with_authenticated_principal_id(require_non_blank(
                request.authenticated_principal_id(),
                ForgeServerTransportDenialCode::MissingAuthenticatedPrincipalId,
                "authenticated principal id",
            )?)
            .with_tenant_id(require_non_blank(
                request.tenant_id(),
                ForgeServerTransportDenialCode::MissingTenantId,
                "tenant id",
            )?)
            .with_workspace_id(require_non_blank(
                request.workspace_id(),
                ForgeServerTransportDenialCode::MissingWorkspaceId,
                "workspace id",
            )?)
            .with_route_family(route.route_family())
            .with_method(route.method())
            .with_path(route.path());
        builder = match request.branch_target() {
            ForgeServerRouteBranchTarget::Main => builder.with_main_branch(),
            ForgeServerRouteBranchTarget::Branch { branch_id } => builder.with_branch_id(branch_id),
            ForgeServerRouteBranchTarget::Preview { preview_id } => {
                builder.with_preview_id(preview_id)
            }
        };
        if let Some(diagnostics_profile) = request.diagnostics_profile() {
            builder = builder.with_diagnostics_profile(diagnostics_profile);
        }
        for (name, value) in request.headers() {
            builder = builder.with_header(name, value);
        }
        for (name, value) in request.query_pairs() {
            builder = builder.with_query_pair(name, value);
        }
        if let Some(content_type) = request.body_content_type() {
            builder = builder.with_body_content_type(content_type);
        }
        builder = builder.with_body_present(request.body_present());
        let input = builder.build().map_err(|error| {
            ForgeServerTransportDenial::new(
                ForgeServerTransportDenialCode::MissingBranchTarget,
                format!("failed to build compatibility request input: {error:?}"),
            )
        })?;
        match self.compat_http.prepare_request(input) {
            TransitionOutcome::Success(prepared) => Ok(prepared),
            TransitionOutcome::Denied(denial) => Err(ForgeServerTransportDenial::new(
                ForgeServerTransportDenialCode::UnknownRoute,
                denial.detail().to_string(),
            )),
            other => Err(ForgeServerTransportDenial::new(
                ForgeServerTransportDenialCode::UnknownRoute,
                format!("compatibility request could not be prepared: {other:?}"),
            )),
        }
    }
}

#[derive(Clone, Debug)]
enum ForgeServerRouteBridgeTarget {
    Semantic(ForgeServerDeclaredRoute),
    Operational(ForgeServerOperationalRoute),
}

#[derive(Clone, Debug)]
pub enum ForgeServerRouteExecutionOutcome {
    ProductOperation(ForgeServerCompletedProductOperation),
    ProductSession(ForgeServerCompletedProductSessionCoordination),
    Operational(ForgeServerOperationalRouteOutcome),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationalRouteOutcome {
    kind: super::ForgeServerOperationalRouteKind,
    path: String,
}

impl ForgeServerOperationalRouteOutcome {
    fn new(kind: super::ForgeServerOperationalRouteKind, path: String) -> Self {
        Self { kind, path }
    }

    pub fn kind(&self) -> super::ForgeServerOperationalRouteKind {
        self.kind
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

fn execute_semantic_route(
    compat_http: &ForgeServerCompatibilityFacade,
    route: &ForgeServerDeclaredRoute,
    request: &ForgeServerRouteTransportRequest,
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
) -> Result<ForgeServerRouteExecutionOutcome, ForgeServerTransportDenial> {
    let payload_json = decode_json_body(request, route.payload_schema_identity())?;
    match route.operation_family() {
        crate::ForgeServerOperationFamily::ProductApplicationRead => {
            let mut input = ForgeServerProductOperationInput::new(
                route.operation_name(),
                crate::ForgeServerProductOperationPayload::json(
                    route.payload_schema_identity(),
                    payload_json
                        .clone()
                        .unwrap_or_else(|| serde_json::json!({})),
                ),
            );
            if let Some(product_session_identity) = header_value(request, "x-product-session-id")
                .or_else(|| query_value(request, "product_session"))
            {
                input = input.with_product_session_identity(product_session_identity);
            }
            let operation = compat_http
                .product_operations()
                .execute(prepared_request, input)
                .map_err(|denial| {
                    ForgeServerTransportDenial::new(
                        ForgeServerTransportDenialCode::UnknownRoute,
                        denial.detail().to_string(),
                    )
                })?;
            Ok(ForgeServerRouteExecutionOutcome::ProductOperation(
                operation,
            ))
        }
        crate::ForgeServerOperationFamily::ProductApplicationMutation => {
            let product_session_identity =
                header_value(request, "x-product-session-id")
                    .or_else(|| query_value(request, "product_session"))
                    .ok_or_else(|| {
                        ForgeServerTransportDenial::new(
                            ForgeServerTransportDenialCode::MissingProductSessionIdentity,
                            "product mutation routes require a product session identity in `x-product-session-id` header or `product_session` query parameter",
                        )
                    })?;
            let body = payload_json
                .clone()
                .unwrap_or_else(|| serde_json::json!({}));
            let operation = compat_http
                .product_operations()
                .execute_admitted_mutation(
                    prepared_request,
                    ForgeServerCompatibilityAdmittedProductMutationCommand::new(
                        route.operation_name(),
                        crate::ForgeServerProductOperationPayload::json(
                            route.payload_schema_identity(),
                            body,
                        ),
                    )
                    .with_product_session_identity(product_session_identity),
                )
                .map_err(|denial| {
                    ForgeServerTransportDenial::new(
                        ForgeServerTransportDenialCode::UnknownRoute,
                        denial.detail().to_string(),
                    )
                })?;
            Ok(ForgeServerRouteExecutionOutcome::ProductOperation(
                operation,
            ))
        }
        crate::ForgeServerOperationFamily::ProductSessionCoordination => {
            let body = payload_json.unwrap_or_else(|| serde_json::json!({}));
            let completed = match route.operation_name() {
                "product_session.open_preview" => compat_http
                    .product_sessions()
                    .open_preview_with_proof(
                        prepared_request,
                        decode_session_creation_request(route.operation_name(), &body),
                    ),
                "product_session.open_mutation" => compat_http
                    .product_sessions()
                    .open_mutation_with_proof(
                        prepared_request,
                        decode_session_creation_request(route.operation_name(), &body),
                    ),
                "product_session.close" => {
                    let product_session_identity =
                        header_value(request, "x-product-session-id")
                            .or_else(|| query_value(request, "product_session"))
                            .ok_or_else(|| {
                                ForgeServerTransportDenial::new(
                                    ForgeServerTransportDenialCode::MissingProductSessionIdentity,
                                    "product session close routes require a product session identity in `x-product-session-id` header or `product_session` query parameter",
                                )
                            })?;
                    compat_http.product_sessions().close_with_proof(
                        prepared_request,
                        &crate::ForgeServerProductSessionIdentity::new(product_session_identity),
                    )
                }
                operation_name => {
                    return Err(ForgeServerTransportDenial::new(
                        ForgeServerTransportDenialCode::UnknownRoute,
                        format!(
                            "route-declared session operation `{operation_name}` is unsupported"
                        ),
                    ))
                }
            }
            .map_err(|denial| {
                ForgeServerTransportDenial::new(
                    ForgeServerTransportDenialCode::UnknownRoute,
                    denial.detail().to_string(),
                )
            })?;
            Ok(ForgeServerRouteExecutionOutcome::ProductSession(completed))
        }
        family => Err(ForgeServerTransportDenial::new(
            ForgeServerTransportDenialCode::UnknownRoute,
            format!(
                "route execution does not yet support operation family `{}`",
                family.as_str()
            ),
        )),
    }
}

fn decode_session_creation_request(
    operation_name: &str,
    body: &serde_json::Value,
) -> ForgeServerProductSessionCreationRequest {
    let default_operation_name = match operation_name {
        "product_session.open_preview" => "product_editor.render_preview",
        "product_session.open_mutation" => "product_editor.apply",
        other => other,
    };
    let mut request = ForgeServerProductSessionCreationRequest::for_operation(
        body.get("operation_name")
            .and_then(|value| value.as_str())
            .unwrap_or(default_operation_name),
    );
    if let Some(basis_digest) = body.get("basis_digest").and_then(|value| value.as_str()) {
        request = request.with_basis_digest(basis_digest);
    }
    if let Some(expiry_seconds) = body.get("expiry_seconds").and_then(|value| value.as_u64()) {
        request = request.with_expiry_seconds(expiry_seconds);
    }
    request
}

fn require_non_blank<'a>(
    value: &'a str,
    code: ForgeServerTransportDenialCode,
    label: &str,
) -> Result<&'a str, ForgeServerTransportDenial> {
    if value.trim().is_empty() {
        return Err(ForgeServerTransportDenial::new(
            code,
            format!("route transport request requires a non-blank {label}"),
        ));
    }
    Ok(value)
}
