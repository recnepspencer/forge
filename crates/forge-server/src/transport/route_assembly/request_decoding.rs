use crate::request_context::DiagnosticRichnessProfile;
use serde_json::{Map, Value};

use super::transport_denial::{ForgeServerTransportDenial, ForgeServerTransportDenialCode};

const MAX_JSON_BODY_BYTES: usize = 1024 * 1024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerRouteBranchTarget {
    Main,
    Branch { branch_id: String },
    Preview { preview_id: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerRouteTransportRequest {
    authenticated_principal_id: String,
    tenant_id: String,
    workspace_id: String,
    branch_target: ForgeServerRouteBranchTarget,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
    headers: Vec<(String, String)>,
    query_pairs: Vec<(String, String)>,
    body_bytes: Vec<u8>,
    body_content_type: Option<String>,
}

impl ForgeServerRouteTransportRequest {
    pub fn new(
        authenticated_principal_id: impl Into<String>,
        tenant_id: impl Into<String>,
        workspace_id: impl Into<String>,
        branch_target: ForgeServerRouteBranchTarget,
    ) -> Self {
        Self {
            authenticated_principal_id: authenticated_principal_id.into(),
            tenant_id: tenant_id.into(),
            workspace_id: workspace_id.into(),
            branch_target,
            diagnostics_profile: None,
            headers: Vec::new(),
            query_pairs: Vec::new(),
            body_bytes: Vec::new(),
            body_content_type: None,
        }
    }

    pub fn with_diagnostics_profile(
        mut self,
        diagnostics_profile: DiagnosticRichnessProfile,
    ) -> Self {
        self.diagnostics_profile = Some(diagnostics_profile);
        self
    }

    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    pub fn with_query_pair(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_pairs.push((name.into(), value.into()));
        self
    }

    pub fn with_json_body(mut self, value: &serde_json::Value) -> Self {
        self.body_bytes = serde_json::to_vec(value).expect("route test body should encode");
        self.body_content_type = Some("application/json".to_string());
        self
    }

    pub(crate) fn with_raw_body(mut self, body_bytes: Vec<u8>, content_type: Option<&str>) -> Self {
        self.body_bytes = body_bytes;
        self.body_content_type = content_type.map(str::to_string);
        self
    }

    pub(crate) fn authenticated_principal_id(&self) -> &str {
        &self.authenticated_principal_id
    }

    pub(crate) fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub(crate) fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    pub(crate) fn branch_target(&self) -> &ForgeServerRouteBranchTarget {
        &self.branch_target
    }

    pub(crate) fn diagnostics_profile(&self) -> Option<DiagnosticRichnessProfile> {
        self.diagnostics_profile
    }

    pub(crate) fn headers(&self) -> &[(String, String)] {
        &self.headers
    }

    pub(crate) fn query_pairs(&self) -> &[(String, String)] {
        &self.query_pairs
    }

    pub(crate) fn body_content_type(&self) -> Option<&str> {
        self.body_content_type.as_deref()
    }

    pub(crate) fn body_present(&self) -> bool {
        !self.body_bytes.is_empty()
    }
}

pub(crate) fn decode_json_body(
    request: &ForgeServerRouteTransportRequest,
    _schema_identity: &str,
) -> Result<Option<serde_json::Value>, ForgeServerTransportDenial> {
    if !request.body_present() {
        return Ok(query_payload_json(request));
    }
    let Some(content_type) = request.body_content_type() else {
        return Err(ForgeServerTransportDenial::new(
            ForgeServerTransportDenialCode::UnsupportedContentType,
            "request body requires an explicit content type",
        ));
    };
    if !content_type.eq_ignore_ascii_case("application/json") {
        return Err(ForgeServerTransportDenial::new(
            ForgeServerTransportDenialCode::UnsupportedContentType,
            format!(
                "content type `{content_type}` is not supported for route-declared JSON payloads"
            ),
        ));
    }
    if request.body_bytes.len() > MAX_JSON_BODY_BYTES {
        return Err(ForgeServerTransportDenial::new(
            ForgeServerTransportDenialCode::OversizedBody,
            format!(
                "request body exceeded the {} byte transport limit",
                MAX_JSON_BODY_BYTES
            ),
        ));
    }
    let json: serde_json::Value = serde_json::from_slice(&request.body_bytes).map_err(|error| {
        ForgeServerTransportDenial::new(
            ForgeServerTransportDenialCode::MalformedJson,
            format!("request body was not valid JSON: {error}"),
        )
    })?;
    Ok(Some(json))
}

fn query_payload_json(request: &ForgeServerRouteTransportRequest) -> Option<Value> {
    let mut payload = Map::new();
    for (name, value) in request.query_pairs() {
        if is_transport_control_query(name) {
            continue;
        }
        payload.insert(name.clone(), Value::String(value.clone()));
    }
    (!payload.is_empty()).then_some(Value::Object(payload))
}

fn is_transport_control_query(name: &str) -> bool {
    name.eq_ignore_ascii_case("basis") || name.eq_ignore_ascii_case("product_session")
}

pub(crate) fn query_value<'a>(
    request: &'a ForgeServerRouteTransportRequest,
    name: &str,
) -> Option<&'a str> {
    request
        .query_pairs()
        .iter()
        .find(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
        .map(|(_, value)| value.as_str())
}

pub(crate) fn header_value<'a>(
    request: &'a ForgeServerRouteTransportRequest,
    name: &str,
) -> Option<&'a str> {
    request
        .headers()
        .iter()
        .find(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
        .map(|(_, value)| value.as_str())
}
