use std::collections::BTreeMap;

use forge_foundational::facade::{AspectKey, AspectValue};
use forge_runtime_bridge::facade::{
    SnapshotReadPacket, SnapshotReadPacketResult, TruthSnapshotIdentity,
};

use super::canonical_digest::row_set_digest;
use super::grouped_projection::RelationalGroupedTruthError;
use super::snapshot_aspect_reads::decode_snapshot_aspect_read_value;

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalRowSetDigest(String);

impl RelationalRowSetDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn from_canonical_bytes(bytes: &[u8]) -> Self {
        Self(super::canonical_digest::digest_with_prefix(
            "relational-row-set",
            bytes,
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelationalAuthoritativeRowArtifact {
    row_identity: RelationalRowIdentity,
    aspect_values: BTreeMap<AspectKey, AspectValue>,
}

impl RelationalAuthoritativeRowArtifact {
    pub fn row_identity(&self) -> &RelationalRowIdentity {
        &self.row_identity
    }

    pub fn aspect_values(&self) -> &BTreeMap<AspectKey, AspectValue> {
        &self.aspect_values
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

    let mut rows: BTreeMap<String, BTreeMap<AspectKey, AspectValue>> = BTreeMap::new();
    for (read, record) in packet.reads().iter().zip(result.records().iter()) {
        let aspect_read = decode_snapshot_aspect_read_value(record)?;
        let aspect_key = parse_snapshot_request_aspect_key(read.aspect_label())?;
        rows.entry(read.entity_identity().to_string())
            .or_default()
            .insert(aspect_key, aspect_read.value().clone());
    }

    let rows = rows
        .into_iter()
        .map(
            |(row_identity, aspect_values)| RelationalAuthoritativeRowArtifact {
                row_identity: RelationalRowIdentity::new(row_identity),
                aspect_values,
            },
        )
        .collect::<Vec<_>>();

    let digest = row_set_digest(result.snapshot_identity(), &rows)?;

    Ok(RelationalAuthoritativeRowSetArtifact {
        snapshot_identity: result.snapshot_identity().clone(),
        rows,
        digest,
    })
}

fn parse_snapshot_request_aspect_key(
    value: impl Into<String>,
) -> Result<AspectKey, RelationalGroupedTruthError> {
    let value = value.into();
    AspectKey::new(value.clone())
        .ok_or(RelationalGroupedTruthError::InvalidAspectBindingKey { aspect_key: value })
}

#[cfg(test)]
mod tests {
    use forge_foundational::facade::{AspectKey, AspectValue, InternedString, Symbol};
    use forge_runtime_bridge::facade::{
        SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadRequest,
        TruthSnapshotIdentity,
    };

    use super::materialize_relational_authoritative_row_set;

    #[test]
    fn relational_row_set_preserves_row_identity_and_aspect_values() {
        let packet = SnapshotReadPacket::new(vec![
            SnapshotReadRequest::for_coarse("entity-1", "identity.id"),
            SnapshotReadRequest::for_coarse("entity-1", "status.lane"),
            SnapshotReadRequest::for_coarse("entity-2", "identity.id"),
            SnapshotReadRequest::for_coarse("entity-2", "status.lane"),
        ]);
        let result = SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-a"),
            vec![
                SnapshotReadRecord::new(
                    "entity-1:identity.id",
                    aspect_bytes(AspectValue::String("task-1".into())),
                ),
                SnapshotReadRecord::new(
                    "entity-1:status.lane",
                    aspect_bytes(AspectValue::String("todo".into())),
                ),
                SnapshotReadRecord::new(
                    "entity-2:identity.id",
                    aspect_bytes(AspectValue::String("task-2".into())),
                ),
                SnapshotReadRecord::new(
                    "entity-2:status.lane",
                    aspect_bytes(AspectValue::String("doing".into())),
                ),
            ],
        );

        let row_set = materialize_relational_authoritative_row_set(&packet, &result).unwrap();

        assert_eq!(row_set.rows().len(), 2);
        assert_eq!(row_set.rows()[0].row_identity().as_str(), "entity-1");
        assert!(row_set.rows()[0]
            .aspect_values()
            .contains_key(&AspectKey::new("identity.id").unwrap()));
        assert_eq!(
            row_set.rows()[0]
                .aspect_values()
                .get(&AspectKey::new("identity.id").unwrap()),
            Some(&AspectValue::String("task-1".into()))
        );
    }

    #[test]
    fn relational_row_set_rejects_invalid_snapshot_request_aspect_label() {
        let packet = SnapshotReadPacket::new(vec![SnapshotReadRequest::for_coarse(
            "entity-1",
            "not a field key",
        )]);
        let result = SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-a"),
            vec![SnapshotReadRecord::new(
                "entity-1:not a field key",
                aspect_bytes(AspectValue::String("task-1".into())),
            )],
        );

        let error = materialize_relational_authoritative_row_set(&packet, &result)
            .expect_err("invalid external snapshot aspect labels must be denied");

        assert_eq!(
            error,
            super::RelationalGroupedTruthError::InvalidAspectBindingKey {
                aspect_key: "not a field key".to_string()
            }
        );
    }

    #[test]
    fn relational_row_set_rejects_malformed_aspect_bytes_at_snapshot_boundary() {
        let packet = SnapshotReadPacket::new(vec![SnapshotReadRequest::for_coarse(
            "entity-1",
            "identity.id",
        )]);
        let result = SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-a"),
            vec![SnapshotReadRecord::new(
                "entity-1:identity.id",
                b"bad".to_vec(),
            )],
        );

        let error = materialize_relational_authoritative_row_set(&packet, &result)
            .expect_err("malformed snapshot aspect bytes must fail grouped truth materialization");

        assert_eq!(
            error,
            super::RelationalGroupedTruthError::AspectValueDecodeFailure {
                request_key: "entity-1:identity.id".to_string()
            }
        );
    }

    #[test]
    fn relational_row_set_digest_preserves_interned_string_family() {
        let raw = row_set_with_identity_value(AspectValue::String(InternedString::Raw(
            "symbol:7".to_string(),
        )));
        let symbol =
            row_set_with_identity_value(AspectValue::String(InternedString::Symbol(Symbol(7))));

        assert_ne!(raw.digest(), symbol.digest());
    }

    fn row_set_with_identity_value(
        value: AspectValue,
    ) -> super::RelationalAuthoritativeRowSetArtifact {
        let packet = SnapshotReadPacket::new(vec![SnapshotReadRequest::for_coarse(
            "entity-1",
            "identity.id",
        )]);
        let result = SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-a"),
            vec![SnapshotReadRecord::new(
                "entity-1:identity.id",
                aspect_bytes(value),
            )],
        );
        materialize_relational_authoritative_row_set(&packet, &result).unwrap()
    }

    fn aspect_bytes(value: AspectValue) -> Vec<u8> {
        crate::aspect_wire::encode_aspect_value(&value).expect("test aspect value bytes")
    }
}
