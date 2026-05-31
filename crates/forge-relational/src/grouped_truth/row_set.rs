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

    let mut rows: BTreeMap<RelationalRowIdentity, BTreeMap<AspectKey, AspectValue>> =
        BTreeMap::new();
    for (read, record) in packet.reads().iter().zip(result.records().iter()) {
        let aspect_read = decode_snapshot_aspect_read_value(record)?;
        rows.entry(RelationalRowIdentity::new(read.entity_identity()))
            .or_default()
            .insert(read.aspect_key().clone(), aspect_read.value().clone());
    }

    let rows = rows
        .into_iter()
        .map(
            |(row_identity, aspect_values)| RelationalAuthoritativeRowArtifact {
                row_identity,
                aspect_values,
            },
        )
        .collect::<Vec<_>>();

    let digest = row_set_digest(result.snapshot_identity(), &rows);

    Ok(RelationalAuthoritativeRowSetArtifact {
        snapshot_identity: result.snapshot_identity().clone(),
        rows,
        digest,
    })
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
            SnapshotReadRequest::for_coarse("entity-1", aspect_key("identity.id")),
            SnapshotReadRequest::for_coarse("entity-1", aspect_key("status.lane")),
            SnapshotReadRequest::for_coarse("entity-2", aspect_key("identity.id")),
            SnapshotReadRequest::for_coarse("entity-2", aspect_key("status.lane")),
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
    fn relational_row_set_rejects_malformed_aspect_bytes_at_snapshot_boundary() {
        let packet = SnapshotReadPacket::new(vec![SnapshotReadRequest::for_coarse(
            "entity-1",
            aspect_key("identity.id"),
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
            aspect_key("identity.id"),
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
        crate::aspect_wire::encode_aspect_value(&value)
    }

    fn aspect_key(value: &str) -> AspectKey {
        AspectKey::new(value).expect("valid test aspect key")
    }
}
