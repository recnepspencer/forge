use crate::{
    WorthServerCompatHttpRouteFamily, WorthServerOperationFamily, WorthServerResponseTransform,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerDeclaredRoute {
    method: String,
    path: String,
    route_family: WorthServerCompatHttpRouteFamily,
    operation_family: WorthServerOperationFamily,
    operation_name: String,
    payload_schema_identity: String,
    support_row: String,
    diagnostics_policy: String,
    evidence_policy: String,
    response_transform: WorthServerResponseTransform,
}

impl WorthServerDeclaredRoute {
    pub(crate) fn new(
        method: impl Into<String>,
        path: impl Into<String>,
        route_family: WorthServerCompatHttpRouteFamily,
        operation_family: WorthServerOperationFamily,
        operation_name: impl Into<String>,
        payload_schema_identity: impl Into<String>,
        support_row: impl Into<String>,
    ) -> Self {
        Self {
            method: method.into(),
            path: path.into(),
            route_family,
            operation_family,
            operation_name: operation_name.into(),
            payload_schema_identity: payload_schema_identity.into(),
            support_row: support_row.into(),
            diagnostics_policy: "request-context-resolved".to_string(),
            evidence_policy: "runtime-derived".to_string(),
            response_transform: WorthServerResponseTransform::compat_http(),
        }
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn route_family(&self) -> WorthServerCompatHttpRouteFamily {
        self.route_family
    }

    pub fn operation_family(&self) -> WorthServerOperationFamily {
        self.operation_family
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn payload_schema_identity(&self) -> &str {
        &self.payload_schema_identity
    }

    pub fn support_row(&self) -> &str {
        &self.support_row
    }

    pub fn diagnostics_policy(&self) -> &str {
        &self.diagnostics_policy
    }

    pub fn evidence_policy(&self) -> &str {
        &self.evidence_policy
    }

    pub fn response_transform(&self) -> WorthServerResponseTransform {
        self.response_transform
    }
}
