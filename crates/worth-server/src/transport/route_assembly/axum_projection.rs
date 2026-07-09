use axum::{
    body::Bytes,
    extract::{Query, State},
    http::{HeaderMap, Method, Uri},
    response::Response,
    routing::{get, options, post},
    Router,
};
use std::collections::HashMap;

use super::{
    execution_bridge::WorthServerRouteExecutionOutcome,
    request_decoding::{WorthServerRouteBranchTarget, WorthServerRouteTransportRequest},
    response_projection::{semantic_response, transport_denial_response},
    WorthServerOperationRouter, WorthServerRouteAssembly, WorthServerTransportDenial,
};

pub(crate) fn project_axum_router(
    assembly: &WorthServerRouteAssembly,
    operation_router: WorthServerOperationRouter,
) -> Router {
    let mut router = Router::new();
    for route in assembly.declared_routes() {
        router = match route.method() {
            "GET" => router.route(route.path(), get(semantic_route_handler)),
            "POST" => router.route(route.path(), post(semantic_route_handler)),
            _ => router,
        };
    }
    for route in assembly.operational_routes() {
        router = match route.method() {
            "GET" => router.route(route.path(), get(operational_route_handler)),
            "OPTIONS" => router.route(route.path(), options(operational_route_handler)),
            _ => router,
        };
    }
    router
        .fallback(unknown_route_handler)
        .with_state(operation_router)
}

async fn semantic_route_handler(
    method: Method,
    uri: Uri,
    State(operation_router): State<WorthServerOperationRouter>,
    headers: HeaderMap,
    Query(query): Query<HashMap<String, String>>,
    body: Bytes,
) -> Response {
    let Some(bridge) = operation_router.bridge_for(method.as_str(), uri.path()) else {
        return unknown_route_handler().await;
    };
    let transport_request = lower_transport_request(&headers, &query, body.to_vec());
    match transport_request.and_then(|request| bridge.execute(request)) {
        Ok(outcome) => semantic_response(outcome),
        Err(denial) => transport_denial_response(denial),
    }
}

async fn operational_route_handler(
    method: Method,
    uri: Uri,
    State(operation_router): State<WorthServerOperationRouter>,
) -> Response {
    let Some(bridge) = operation_router.bridge_for(method.as_str(), uri.path()) else {
        return unknown_route_handler().await;
    };
    match bridge.execute(WorthServerRouteTransportRequest::new(
        "operational",
        "operational",
        "operational",
        WorthServerRouteBranchTarget::Main,
    )) {
        Ok(WorthServerRouteExecutionOutcome::Operational(outcome)) => {
            semantic_response(WorthServerRouteExecutionOutcome::Operational(outcome))
        }
        Ok(other) => semantic_response(other),
        Err(denial) => transport_denial_response(denial),
    }
}

async fn unknown_route_handler() -> Response {
    transport_denial_response(WorthServerTransportDenial::new(
        super::WorthServerTransportDenialCode::UnknownRoute,
        "no declared or operational route matched the request",
    ))
}

fn lower_transport_request(
    headers: &HeaderMap,
    query: &HashMap<String, String>,
    body: Vec<u8>,
) -> Result<WorthServerRouteTransportRequest, WorthServerTransportDenial> {
    let authenticated_principal_id =
        required_header(headers, "x-principal-id", "authenticated principal id")?;
    let tenant_id = required_header(headers, "x-tenant-id", "tenant id")?;
    let workspace_id = required_header(headers, "x-workspace-id", "workspace id")?;
    let branch_target = if let Some(preview_id) = optional_header(headers, "x-preview-id") {
        WorthServerRouteBranchTarget::Preview {
            preview_id: preview_id.to_string(),
        }
    } else if let Some(branch_id) = optional_header(headers, "x-branch-id") {
        WorthServerRouteBranchTarget::Branch {
            branch_id: branch_id.to_string(),
        }
    } else {
        WorthServerRouteBranchTarget::Main
    };
    let mut request = WorthServerRouteTransportRequest::new(
        authenticated_principal_id,
        tenant_id,
        workspace_id,
        branch_target,
    );
    for (name, value) in headers {
        if let Ok(value) = value.to_str() {
            request = request.with_header(name.as_str(), value);
        }
    }
    for (name, value) in query {
        request = request.with_query_pair(name, value);
    }
    if !body.is_empty() {
        request = request.with_raw_body(body, optional_header(headers, "content-type"));
    }
    Ok(request)
}

fn required_header<'a>(
    headers: &'a HeaderMap,
    name: &str,
    label: &str,
) -> Result<&'a str, WorthServerTransportDenial> {
    optional_header(headers, name).ok_or_else(|| {
        WorthServerTransportDenial::new(
            match name {
                "x-principal-id" => {
                    super::WorthServerTransportDenialCode::MissingAuthenticatedPrincipalId
                }
                "x-tenant-id" => super::WorthServerTransportDenialCode::MissingTenantId,
                _ => super::WorthServerTransportDenialCode::MissingWorkspaceId,
            },
            format!("compat route requests require `{name}` header for {label}"),
        )
    })
}

fn optional_header<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers.get(name).and_then(|value| value.to_str().ok())
}

#[cfg(test)]
mod tests {
    use super::lower_transport_request;
    use axum::http::{HeaderMap, HeaderValue};
    use std::collections::HashMap;

    #[test]
    fn lower_transport_request_preserves_original_content_type() {
        let mut headers = HeaderMap::new();
        headers.insert("x-principal-id", HeaderValue::from_static("principal-7"));
        headers.insert("x-tenant-id", HeaderValue::from_static("tenant-a"));
        headers.insert("x-workspace-id", HeaderValue::from_static("workspace-42"));
        headers.insert("content-type", HeaderValue::from_static("text/plain"));

        let request =
            lower_transport_request(&headers, &HashMap::new(), br#"{"title":"Rename"}"#.to_vec())
                .expect("transport request should lower");

        assert_eq!(request.body_content_type(), Some("text/plain"));
    }
}
