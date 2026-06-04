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
pub struct RelationalProjectedAspectValueSet {
    values: BTreeMap<AspectKey, AspectValue>,
}

impl RelationalProjectedAspectValueSet {
    pub fn get(&self, aspect_key: &AspectKey) -> Option<&AspectValue> {
        self.values.get(aspect_key)
    }

    pub fn contains_key(&self, aspect_key: &AspectKey) -> bool {
        self.values.contains_key(aspect_key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&AspectKey, &AspectValue)> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn from_projected_values(values: BTreeMap<AspectKey, AspectValue>) -> Self {
        Self { values }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelationalAuthoritativeRowArtifact {
    row_identity: RelationalRowIdentity,
    projected_aspect_values: RelationalProjectedAspectValueSet,
}

impl RelationalAuthoritativeRowArtifact {
    pub fn row_identity(&self) -> &RelationalRowIdentity {
        &self.row_identity
    }

    pub fn projected_aspect_values(&self) -> &RelationalProjectedAspectValueSet {
        &self.projected_aspect_values
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

    let rows =
        rows.into_iter()
            .map(
                |(row_identity, aspect_values)| RelationalAuthoritativeRowArtifact {
                    row_identity,
                    projected_aspect_values:
                        RelationalProjectedAspectValueSet::from_projected_values(aspect_values),
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
    use forge_foundational::facade::{
        AspectKey, AspectValue, FieldKey, InternedString, ScalarAspectType, StructAspectValue,
        Symbol,
    };
    use forge_runtime_bridge::facade::{
        SnapshotReadContract, SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord,
        SnapshotReadRequest, TruthSnapshotIdentity,
    };

    use super::materialize_relational_authoritative_row_set;

    #[test]
    fn relational_row_set_preserves_row_identity_and_aspect_values() {
        let packet = SnapshotReadPacket::new(vec![
            string_read("entity-1", "identity.id"),
            string_read("entity-1", "status.lane"),
            string_read("entity-2", "identity.id"),
            string_read("entity-2", "status.lane"),
        ]);
        let result = SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-a"),
            vec![
                read_record(&packet, 0, AspectValue::String("task-1".into())),
                read_record(&packet, 1, AspectValue::String("todo".into())),
                read_record(&packet, 2, AspectValue::String("task-2".into())),
                read_record(&packet, 3, AspectValue::String("doing".into())),
            ],
        );

        let row_set = materialize_relational_authoritative_row_set(&packet, &result).unwrap();

        assert_eq!(row_set.rows().len(), 2);
        assert_eq!(row_set.rows()[0].row_identity().as_str(), "entity-1");
        assert!(row_set.rows()[0]
            .projected_aspect_values()
            .contains_key(&AspectKey::new("identity.id").unwrap()));
        assert_eq!(
            row_set.rows()[0]
                .projected_aspect_values()
                .get(&AspectKey::new("identity.id").unwrap()),
            Some(&AspectValue::String("task-1".into()))
        );
    }

    #[test]
    fn relational_row_set_rejects_non_scalar_snapshot_values_at_grouped_boundary() {
        let packet = SnapshotReadPacket::new(vec![string_read("entity-1", "identity.id")]);
        let result = SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-a"),
            vec![read_record(
                &packet,
                0,
                StructAspectValue::new([(
                    FieldKey::new("name").expect("valid field key"),
                    AspectValue::String("bad".into()),
                )])
                .expect("valid struct aspect value"),
            )],
        );

        let error = materialize_relational_authoritative_row_set(&packet, &result)
            .expect_err("non-scalar snapshot values must fail grouped truth materialization");

        assert_eq!(
            error,
            super::RelationalGroupedTruthError::AspectValueDecodeFailure {
                request_key: packet.reads()[0].correlation_id().as_str().to_string()
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
        let packet = SnapshotReadPacket::new(vec![string_read("entity-1", "identity.id")]);
        let result = SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-a"),
            vec![read_record(&packet, 0, value)],
        );
        materialize_relational_authoritative_row_set(&packet, &result).unwrap()
    }

    fn read_record(
        packet: &SnapshotReadPacket,
        index: usize,
        value: impl Into<forge_runtime_bridge::facade::SnapshotReadValue>,
    ) -> SnapshotReadRecord {
        SnapshotReadRecord::for_request(&packet.reads()[index], value)
    }

    fn string_read(entity_identity: &str, aspect: &str) -> SnapshotReadRequest {
        SnapshotReadRequest::for_coarse(
            entity_identity,
            SnapshotReadContract::scalar(aspect_key(aspect), ScalarAspectType::String),
        )
    }

    fn aspect_key(value: &str) -> AspectKey {
        AspectKey::new(value).expect("valid test aspect key")
    }
}
