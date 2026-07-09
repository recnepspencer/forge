use serde_json::Value;

use crate::WorthServerOperationInputEnvelope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductOperationPayload {
    envelope: WorthServerOperationInputEnvelope,
    body: Value,
}

impl WorthServerProductOperationPayload {
    pub fn json(schema_identity: impl Into<String>, body: Value) -> Self {
        let envelope = WorthServerOperationInputEnvelope::json(schema_identity, &body);
        Self { envelope, body }
    }

    pub fn envelope(&self) -> &WorthServerOperationInputEnvelope {
        &self.envelope
    }

    pub fn body(&self) -> &Value {
        &self.body
    }
}
