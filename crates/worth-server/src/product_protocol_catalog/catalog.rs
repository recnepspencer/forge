use serde::Serialize;
use serde_json::Value;

use super::{WorthServerProductOperationProtocol, WorthServerProductSessionOperationProtocol};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerProductProtocolCatalogError {
    MissingRoute { operation_name: String },
    DuplicateRoute { operation_name: String },
    MissingSessionRoute { operation_name: String },
    UnexpectedSessionRoute { operation_name: String },
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WorthServerProductProtocolCatalog {
    schema_identity: String,
    schema_version: u32,
    catalog_digest: String,
    envelope_schema_identity: String,
    envelope_schema_digest: String,
    envelope_schema: Value,
    operations: Vec<WorthServerProductOperationProtocol>,
    product_session_operations: Vec<WorthServerProductSessionOperationProtocol>,
}

impl WorthServerProductProtocolCatalog {
    pub(crate) fn new(
        catalog_digest: String,
        envelope_schema_identity: String,
        envelope_schema_digest: String,
        envelope_schema: Value,
        operations: Vec<WorthServerProductOperationProtocol>,
        product_session_operations: Vec<WorthServerProductSessionOperationProtocol>,
    ) -> Self {
        Self {
            schema_identity: "worth.server.product-protocol-catalog.v1".to_string(),
            schema_version: 1,
            catalog_digest,
            envelope_schema_identity,
            envelope_schema_digest,
            envelope_schema,
            operations,
            product_session_operations,
        }
    }

    pub fn schema_identity(&self) -> &str {
        &self.schema_identity
    }

    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }

    pub fn envelope_schema_identity(&self) -> &str {
        &self.envelope_schema_identity
    }

    pub fn envelope_schema_digest(&self) -> &str {
        &self.envelope_schema_digest
    }

    pub fn envelope_schema(&self) -> &Value {
        &self.envelope_schema
    }

    pub fn operations(&self) -> &[WorthServerProductOperationProtocol] {
        &self.operations
    }

    pub fn product_session_operations(&self) -> &[WorthServerProductSessionOperationProtocol] {
        &self.product_session_operations
    }
}
