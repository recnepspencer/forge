use forge_foundational::facade::{AspectKey, AspectValue};
use forge_runtime_bridge::facade::{
    GroupedProjectionMemberSource, GroupedProjectionSource, TruthSnapshotIdentity,
};

use super::canonical_digest::grouped_projection_digest;

use super::row_set::{
    RelationalAuthoritativeRowSetArtifact, RelationalRowIdentity, RelationalRowSetDigest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedProjectionContract {
    grouping_aspect: AspectKey,
    identity_binding_aspect_key: AspectKey,
    grouping_binding_aspect_key: AspectKey,
}

impl GroupedProjectionContract {
    pub fn new(
        grouping_aspect: AspectKey,
        identity_binding_aspect_key: AspectKey,
        grouping_binding_aspect_key: AspectKey,
    ) -> Self {
        Self {
            grouping_aspect,
            identity_binding_aspect_key,
            grouping_binding_aspect_key,
        }
    }

    pub fn grouping_aspect(&self) -> &AspectKey {
        &self.grouping_aspect
    }

    pub fn identity_binding_aspect_key(&self) -> &AspectKey {
        &self.identity_binding_aspect_key
    }

    pub fn grouping_binding_aspect_key(&self) -> &AspectKey {
        &self.grouping_binding_aspect_key
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelationalGroupedMemberRow {
    row_identity: RelationalRowIdentity,
    identity_value: AspectValue,
    grouping_value: AspectValue,
}

impl RelationalGroupedMemberRow {
    pub fn row_identity(&self) -> &RelationalRowIdentity {
        &self.row_identity
    }

    pub fn identity_value(&self) -> &AspectValue {
        &self.identity_value
    }

    pub fn grouping_value(&self) -> &AspectValue {
        &self.grouping_value
    }
}

impl GroupedProjectionMemberSource for RelationalGroupedMemberRow {
    fn row_identity(&self) -> &str {
        self.row_identity.as_str()
    }

    fn identity_value(&self) -> &AspectValue {
        &self.identity_value
    }

    fn grouping_value(&self) -> &AspectValue {
        &self.grouping_value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalGroupedProjectionDigest(String);

impl RelationalGroupedProjectionDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn from_canonical_bytes(bytes: &[u8]) -> Self {
        Self(super::canonical_digest::digest_with_prefix(
            "relational-grouped-projection",
            bytes,
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelationalGroupedProjectionArtifact {
    row_set_digest: RelationalRowSetDigest,
    snapshot_identity: TruthSnapshotIdentity,
    contract: GroupedProjectionContract,
    members: Vec<RelationalGroupedMemberRow>,
    digest: RelationalGroupedProjectionDigest,
}

impl RelationalGroupedProjectionArtifact {
    pub fn row_set_digest(&self) -> &RelationalRowSetDigest {
        &self.row_set_digest
    }

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn contract(&self) -> &GroupedProjectionContract {
        &self.contract
    }

    pub fn members(&self) -> &[RelationalGroupedMemberRow] {
        &self.members
    }

    pub fn digest(&self) -> &RelationalGroupedProjectionDigest {
        &self.digest
    }
}

impl GroupedProjectionSource for RelationalGroupedProjectionArtifact {
    type Member = RelationalGroupedMemberRow;

    fn basis_snapshot_identity(&self) -> &TruthSnapshotIdentity {
        &self.snapshot_identity
    }

    fn grouping_aspect(&self) -> &str {
        self.contract.grouping_aspect().as_str()
    }

    fn identity_binding_aspect_key(&self) -> &str {
        self.contract.identity_binding_aspect_key().as_str()
    }

    fn grouping_binding_aspect_key(&self) -> &str {
        self.contract.grouping_binding_aspect_key().as_str()
    }

    fn members(&self) -> &[Self::Member] {
        &self.members
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelationalGroupedTruthError {
    PacketResultShapeMismatch,
    AspectValueDecodeFailure {
        request_key: String,
    },
    MissingIdentityAspect {
        row_identity: RelationalRowIdentity,
        aspect_key: AspectKey,
    },
    MissingGroupingAspect {
        row_identity: RelationalRowIdentity,
        aspect_key: AspectKey,
    },
}

pub fn project_relational_grouped_truth(
    row_set: &RelationalAuthoritativeRowSetArtifact,
    contract: GroupedProjectionContract,
) -> Result<RelationalGroupedProjectionArtifact, RelationalGroupedTruthError> {
    let identity_aspect = contract.identity_binding_aspect_key();
    let grouping_aspect = contract.grouping_binding_aspect_key();

    let mut members = Vec::with_capacity(row_set.rows().len());
    for row in row_set.rows() {
        let Some(identity_value) = row.aspect_values().get(&identity_aspect).cloned() else {
            return Err(RelationalGroupedTruthError::MissingIdentityAspect {
                row_identity: row.row_identity().clone(),
                aspect_key: contract.identity_binding_aspect_key().clone(),
            });
        };
        let Some(grouping_value) = row.aspect_values().get(&grouping_aspect).cloned() else {
            return Err(RelationalGroupedTruthError::MissingGroupingAspect {
                row_identity: row.row_identity().clone(),
                aspect_key: contract.grouping_binding_aspect_key().clone(),
            });
        };

        members.push(RelationalGroupedMemberRow {
            row_identity: row.row_identity().clone(),
            identity_value: identity_value.clone(),
            grouping_value: grouping_value.clone(),
        });
    }

    let digest = grouped_projection_digest(
        row_set.digest(),
        row_set.snapshot_identity(),
        &contract,
        &members,
    );

    Ok(RelationalGroupedProjectionArtifact {
        row_set_digest: row_set.digest().clone(),
        snapshot_identity: row_set.snapshot_identity().clone(),
        contract,
        members,
        digest,
    })
}

#[cfg(test)]
mod tests {
    use forge_foundational::facade::{AspectKey, AspectValue};
    use forge_runtime_bridge::facade::{
        SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadRequest,
        TruthSnapshotIdentity,
    };

    use super::{project_relational_grouped_truth, GroupedProjectionContract};
    use crate::grouped_truth::materialize_relational_authoritative_row_set;

    #[test]
    fn relational_grouped_projection_preserves_member_and_grouping_pairing() {
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

        let grouped = project_relational_grouped_truth(
            &row_set,
            GroupedProjectionContract::new(
                AspectKey::new("status").unwrap(),
                AspectKey::new("identity.id").unwrap(),
                AspectKey::new("status.lane").unwrap(),
            ),
        )
        .unwrap();

        assert_eq!(grouped.members().len(), 2);
        assert_eq!(grouped.members()[0].row_identity().as_str(), "entity-1");
        assert_eq!(
            grouped.members()[0].identity_value(),
            &AspectValue::String("task-1".into())
        );
    }

    #[test]
    fn relational_grouped_projection_missing_identity_error_carries_typed_row_identity() {
        let packet = SnapshotReadPacket::new(vec![SnapshotReadRequest::for_coarse(
            "entity-1",
            aspect_key("status.lane"),
        )]);
        let result = SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-a"),
            vec![SnapshotReadRecord::new(
                "entity-1:status.lane",
                aspect_bytes(AspectValue::String("todo".into())),
            )],
        );
        let row_set = materialize_relational_authoritative_row_set(&packet, &result).unwrap();

        let error = project_relational_grouped_truth(
            &row_set,
            GroupedProjectionContract::new(
                AspectKey::new("status").unwrap(),
                AspectKey::new("identity.id").unwrap(),
                AspectKey::new("status.lane").unwrap(),
            ),
        )
        .expect_err("missing identity aspect should be denied");

        match error {
            super::RelationalGroupedTruthError::MissingIdentityAspect {
                row_identity,
                aspect_key,
            } => {
                assert_eq!(row_identity.as_str(), "entity-1");
                assert_eq!(aspect_key, AspectKey::new("identity.id").unwrap());
            }
            other => panic!("expected missing identity aspect error, got {other:?}"),
        }
    }

    fn aspect_bytes(value: AspectValue) -> Vec<u8> {
        crate::aspect_wire::encode_aspect_value(&value)
    }

    fn aspect_key(value: &str) -> AspectKey {
        AspectKey::new(value).expect("valid test aspect key")
    }
}
