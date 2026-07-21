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
    result_contract_digest: Option<String>,
    durability_contract_digest: Option<String>,
    support_row: Option<String>,
    diagnostics_policy: String,
    evidence_policy: String,
    response_transform: WorthServerResponseTransform,
    operational_label: Option<String>,
}

pub(crate) struct WorthServerSemanticRouteInventoryRowParts {
    pub(crate) method: String,
    pub(crate) path: String,
    pub(crate) operation_family: WorthServerOperationFamily,
    pub(crate) operation_name: String,
    pub(crate) payload_schema_identity: String,
    pub(crate) result_contract_digest: Option<String>,
    pub(crate) durability_contract_digest: Option<String>,
    pub(crate) support_row: String,
    pub(crate) diagnostics_policy: String,
    pub(crate) response_transform: WorthServerResponseTransform,
    pub(crate) evidence_policy: String,
}

impl WorthServerRouteInventoryRow {
    pub(crate) fn semantic(parts: WorthServerSemanticRouteInventoryRowParts) -> Self {
        let WorthServerSemanticRouteInventoryRowParts {
            method,
            path,
            operation_family,
            operation_name,
            payload_schema_identity,
            result_contract_digest,
            durability_contract_digest,
            support_row,
            diagnostics_policy,
            response_transform,
            evidence_policy,
        } = parts;
        Self {
            method,
            path,
            surface_family: WorthServerSurfaceFamily::CompatHttp,
            operation_family: Some(operation_family),
            operation_name: Some(operation_name),
            payload_schema_identity: Some(payload_schema_identity),
            result_contract_digest,
            durability_contract_digest,
            support_row: Some(support_row),
            diagnostics_policy,
            evidence_policy,
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
            result_contract_digest: None,
            durability_contract_digest: None,
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

    pub fn result_contract_digest(&self) -> Option<&str> {
        self.result_contract_digest.as_deref()
    }

    pub fn durability_contract_digest(&self) -> Option<&str> {
        self.durability_contract_digest.as_deref()
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
