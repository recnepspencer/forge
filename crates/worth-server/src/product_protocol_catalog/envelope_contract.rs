use serde_json::{json, Value};

pub(super) const ENVELOPE_SCHEMA_IDENTITY: &str = "worth.server.product-operation-envelope.v1";

pub(super) fn envelope_schema() -> Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": ENVELOPE_SCHEMA_IDENTITY,
        "type": "object",
        "required": [
            "route_kind",
            "operation_name",
            "envelope_kind",
            "canonical_digest",
            "plan_digest",
            "result",
            "denial",
            "failure",
            "durable_completion"
        ],
        "properties": {
            "route_kind": { "const": "product_operation" },
            "operation_name": { "type": "string", "minLength": 1 },
            "envelope_kind": { "enum": ["Success", "Denial", "Failure"] },
            "canonical_digest": { "type": "string", "minLength": 1 },
            "plan_digest": { "type": ["string", "null"] },
            "result": {
                "oneOf": [
                    { "$ref": "#/$defs/result" },
                    { "type": "null" }
                ]
            },
            "denial": {
                "oneOf": [
                    { "$ref": "#/$defs/denial" },
                    { "type": "null" }
                ]
            },
            "failure": {
                "oneOf": [
                    { "$ref": "#/$defs/failure" },
                    { "type": "null" }
                ]
            },
            "durable_completion": {
                "oneOf": [
                    { "$ref": "#/$defs/durableCompletion" },
                    { "type": "null" }
                ]
            }
        },
        "oneOf": [
            {
                "properties": {
                    "envelope_kind": { "const": "Success" },
                    "result": { "$ref": "#/$defs/result" },
                    "denial": { "type": "null" },
                    "failure": { "type": "null" }
                }
            },
            {
                "properties": {
                    "envelope_kind": { "const": "Denial" },
                    "result": { "type": "null" },
                    "denial": { "$ref": "#/$defs/denial" },
                    "failure": { "type": "null" }
                }
            },
            {
                "properties": {
                    "envelope_kind": { "const": "Failure" },
                    "result": { "type": "null" },
                    "denial": { "type": "null" },
                    "failure": { "$ref": "#/$defs/failure" }
                }
            }
        ],
        "$defs": {
            "result": {
                "type": "object",
                "required": [
                    "result_key",
                    "schema_identity",
                    "schema_version",
                    "encoding",
                    "canonicalization",
                    "body",
                    "body_digest",
                    "artifact_digest"
                ],
                "properties": {
                    "result_key": { "type": "string", "minLength": 1 },
                    "schema_identity": { "type": "string", "minLength": 1 },
                    "schema_version": { "type": "integer", "minimum": 1 },
                    "encoding": { "type": "string", "minLength": 1 },
                    "canonicalization": { "type": "string", "minLength": 1 },
                    "body": true,
                    "body_digest": { "type": "string", "minLength": 1 },
                    "artifact_digest": { "type": "string", "minLength": 1 }
                },
                "additionalProperties": true
            },
            "denial": {
                "type": "object",
                "required": [
                    "reason_key",
                    "detail",
                    "code",
                    "expected_basis_digest",
                    "observed_basis_digest"
                ],
                "properties": {
                    "reason_key": { "type": "string", "minLength": 1 },
                    "detail": { "type": "string" },
                    "code": { "type": ["string", "null"] },
                    "expected_basis_digest": { "type": ["string", "null"] },
                    "observed_basis_digest": { "type": ["string", "null"] }
                },
                "additionalProperties": true
            },
            "failure": {
                "type": "object",
                "required": ["reason_key", "detail"],
                "properties": {
                    "reason_key": { "type": "string", "minLength": 1 },
                    "detail": { "type": "string" }
                },
                "additionalProperties": true
            },
            "durableCompletion": {
                "type": "object",
                "required": [
                    "disposition",
                    "request_digest",
                    "completion_digest",
                    "next_basis",
                    "product_commit_digest"
                ],
                "properties": {
                    "disposition": { "type": "string", "minLength": 1 },
                    "request_digest": { "type": "string", "minLength": 1 },
                    "completion_digest": { "type": "string", "minLength": 1 },
                    "next_basis": { "type": "string", "minLength": 1 },
                    "product_commit_digest": { "type": "string", "minLength": 1 }
                },
                "additionalProperties": true
            }
        },
        "additionalProperties": true
    })
}

pub(super) fn envelope_schema_digest() -> String {
    digest_envelope_schema(&envelope_schema())
}

fn digest_envelope_schema(schema: &Value) -> String {
    let schema = serde_json::to_string(schema)
        .expect("the product-operation envelope schema must serialize");
    crate::canonical_digest::WorthServerCanonicalDigestBuilder::new(
        "worth-server-product-operation-envelope-schema-v1",
    )
    .field("identity", ENVELOPE_SCHEMA_IDENTITY)
    .field("schema", &schema)
    .finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelope_digest_changes_with_emitted_schema_drift() {
        let base = envelope_schema();
        let mut changed = base.clone();
        changed["additionalProperties"] = Value::Bool(false);

        assert_ne!(
            digest_envelope_schema(&base),
            digest_envelope_schema(&changed),
        );
    }
}
