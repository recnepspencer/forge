use std::collections::BTreeMap;

use forge_runtime_bridge::facade::{
    SnapshotReadPacket, SnapshotReadPacketResult, TruthSnapshotIdentity,
};
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::grouped_projection::RelationalGroupedTruthError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RelationalRowIdentity(String);

impl RelationalRowIdentity {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RelationalFieldBindingKey(String);

impl RelationalFieldBindingKey {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelationalFieldValue(Value);

impl RelationalFieldValue {
    pub fn value(&self) -> &Value {
        &self.0
    }

    pub(crate) fn new(value: Value) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalRowSetDigest(String);

impl RelationalRowSetDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn new(parts: &[String]) -> Self {
        let canonical = parts.join("|");
        let digest = Sha256::digest(canonical.as_bytes());
        Self(format!("relational-row-set:sha256:{digest:x}"))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelationalAuthoritativeRowArtifact {
    row_identity: RelationalRowIdentity,
    fields: BTreeMap<RelationalFieldBindingKey, RelationalFieldValue>,
}

impl RelationalAuthoritativeRowArtifact {
    pub fn row_identity(&self) -> &RelationalRowIdentity {
        &self.row_identity
    }

    pub fn fields(&self) -> &BTreeMap<RelationalFieldBindingKey, RelationalFieldValue> {
        &self.fields
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelationalAuthoritativeRowSetArtifact {
    snapshot_identity: TruthSnapshotIdentity,
    rows: Vec<RelationalAuthoritativeRowArtifact>,
    digest: RelationalRowSetDigest,
}

impl RelationalAuthoritativeRowSetArtifact {
    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn rows(&self) -> &[RelationalAuthoritativeRowArtifact] {
        &self.rows
    }

    pub fn digest(&self) -> &RelationalRowSetDigest {
        &self.digest
    }
}

pub fn materialize_relational_authoritative_row_set(
    packet: &SnapshotReadPacket,
    result: &SnapshotReadPacketResult,
) -> Result<RelationalAuthoritativeRowSetArtifact, RelationalGroupedTruthError> {
    if packet.reads().len() != result.records().len() {
        return Err(RelationalGroupedTruthError::PacketResultShapeMismatch);
    }

    let mut rows: BTreeMap<String, BTreeMap<RelationalFieldBindingKey, RelationalFieldValue>> =
        BTreeMap::new();
    for (read, record) in packet.reads().iter().zip(result.records().iter()) {
        let value = decode_record_value(record.request_key(), record.payload())?;
        rows.entry(read.entity_identity().to_string())
            .or_default()
            .insert(
                RelationalFieldBindingKey::new(read.aspect_label()),
                RelationalFieldValue::new(value),
            );
    }

    let rows = rows
        .into_iter()
        .map(|(row_identity, fields)| RelationalAuthoritativeRowArtifact {
            row_identity: RelationalRowIdentity::new(row_identity),
            fields,
        })
        .collect::<Vec<_>>();

    let mut digest_parts = vec![format!("snapshot:{}", result.snapshot_identity().as_str())];
    for row in &rows {
        digest_parts.push(format!("row:{}", row.row_identity().as_str()));
        for (field, value) in row.fields() {
            digest_parts.push(format!(
                "field:{}={}",
                field.as_str(),
                canonical_json(value.value())
            ));
        }
    }

    Ok(RelationalAuthoritativeRowSetArtifact {
        snapshot_identity: result.snapshot_identity().clone(),
        rows,
        digest: RelationalRowSetDigest::new(&digest_parts),
    })
}

fn decode_record_value(
    request_key: &str,
    payload: &[u8],
) -> Result<Value, RelationalGroupedTruthError> {
    if let Ok(value) = serde_json::from_slice::<Value>(payload) {
        return Ok(value);
    }
    let text =
        std::str::from_utf8(payload).map_err(|_| RelationalGroupedTruthError::PayloadDecodeFailure {
            request_key: request_key.to_string(),
        })?;
    Ok(Value::String(text.to_string()))
}

fn canonical_json(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "<invalid-json>".to_string())
}

#[cfg(test)]
mod tests {
    use forge_runtime_bridge::facade::{
        SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadRequest,
        TruthSnapshotIdentity,
    };

    use super::materialize_relational_authoritative_row_set;

    #[test]
    fn relational_row_set_preserves_row_identity_and_fields() {
        let packet = SnapshotReadPacket::new(vec![
            SnapshotReadRequest::for_coarse("entity-1", "identity.id"),
            SnapshotReadRequest::for_coarse("entity-1", "status.lane"),
            SnapshotReadRequest::for_coarse("entity-2", "identity.id"),
            SnapshotReadRequest::for_coarse("entity-2", "status.lane"),
        ]);
        let result = SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-a"),
            vec![
                SnapshotReadRecord::new("entity-1:identity.id", b"task-1".to_vec()),
                SnapshotReadRecord::new("entity-1:status.lane", b"todo".to_vec()),
                SnapshotReadRecord::new("entity-2:identity.id", b"task-2".to_vec()),
                SnapshotReadRecord::new("entity-2:status.lane", b"doing".to_vec()),
            ],
        );

        let row_set = materialize_relational_authoritative_row_set(&packet, &result).unwrap();

        assert_eq!(row_set.rows().len(), 2);
        assert_eq!(row_set.rows()[0].row_identity().as_str(), "entity-1");
        assert!(row_set.rows()[0].fields().contains_key(&super::RelationalFieldBindingKey::new("identity.id")));
    }
}
