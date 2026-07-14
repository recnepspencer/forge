use crate::{WorthServerOperationFamily, WorthServerResponseTransform, WorthServerSurfaceFamily};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerRouteInventory {
    rows: Vec<WorthServerRouteInventoryRow>,
}

impl WorthServerRouteInventory {
    pub(crate) fn new(rows: Vec<WorthServerRouteInventoryRow>) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &[WorthServerRouteInventoryRow] {
        &self.rows
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerRouteInventoryRow {
    method: String,
    path: String,
    surface_family: WorthServerSurfaceFamily,
    operation_family: Option<WorthServerOperationFamily>,
    operation_name: Option<String>,
    payload_schema_identity: Option<String>,
    support_row: Option<String>,
    diagnostics_policy: String,
    evidence_policy: String,
    response_transform: WorthServerResponseTransform,
    operational_label: Option<String>,
}

impl WorthServerRouteInventoryRow {
    pub(crate) fn semantic(
        method: impl Into<String>,
        path: impl Into<String>,
        operation_family: WorthServerOperationFamily,
        operation_name: impl Into<String>,
        payload_schema_identity: impl Into<String>,
        support_row: impl Into<String>,
        diagnostics_policy: impl Into<String>,
        response_transform: WorthServerResponseTransform,
        evidence_policy: impl Into<String>,
    ) -> Self {
        Self {
            method: method.into(),
            path: path.into(),
            surface_family: WorthServerSurfaceFamily::CompatHttp,
            operation_family: Some(operation_family),
            operation_name: Some(operation_name.into()),
            payload_schema_identity: Some(payload_schema_identity.into()),
            support_row: Some(support_row.into()),
            diagnostics_policy: diagnostics_policy.into(),
            evidence_policy: evidence_policy.into(),
            response_transform,
            operational_label: None,
        }
    }

    pub(crate) fn operational(
        method: impl Into<String>,
        path: impl Into<String>,
        operational_label: impl Into<String>,
    ) -> Self {
        Self {
            method: method.into(),
            path: path.into(),
            surface_family: WorthServerSurfaceFamily::CompatHttp,
            operation_family: None,
            operation_name: None,
            payload_schema_identity: None,
            support_row: None,
            diagnostics_policy: "operational-static".to_string(),
            evidence_policy: "operational".to_string(),
            response_transform: WorthServerResponseTransform::compat_http(),
            operational_label: Some(operational_label.into()),
        }
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn surface_family(&self) -> WorthServerSurfaceFamily {
        self.surface_family
    }

    pub fn operation_family(&self) -> Option<WorthServerOperationFamily> {
        self.operation_family
    }

    pub fn operation_name(&self) -> Option<&str> {
        self.operation_name.as_deref()
    }

    pub fn payload_schema_identity(&self) -> Option<&str> {
        self.payload_schema_identity.as_deref()
    }

    pub fn support_row(&self) -> Option<&str> {
        self.support_row.as_deref()
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

    pub fn operational_label(&self) -> Option<&str> {
        self.operational_label.as_deref()
    }
}
