use serde_json::Value;

use crate::ForgeServerOperationInputEnvelope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductOperationPayload {
    envelope: ForgeServerOperationInputEnvelope,
    body: Value,
}

impl ForgeServerProductOperationPayload {
    pub fn json(schema_identity: impl Into<String>, body: Value) -> Self {
        let envelope = ForgeServerOperationInputEnvelope::json(schema_identity, &body);
        Self { envelope, body }
    }

    pub fn envelope(&self) -> &ForgeServerOperationInputEnvelope {
        &self.envelope
    }

    pub fn body(&self) -> &Value {
        &self.body
    }
}
