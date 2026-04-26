use forge_runtime_bridge::facade::{
    GroupedProjectionMemberSource, GroupedProjectionSource, TruthSnapshotIdentity,
};
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::row_set::{
    RelationalAuthoritativeRowSetArtifact, RelationalFieldBindingKey, RelationalRowIdentity,
    RelationalRowSetDigest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedProjectionContract {
    grouping_aspect: String,
    identity_binding_field_key: String,
    grouping_binding_field_key: String,
}

impl GroupedProjectionContract {
    pub fn new(
        grouping_aspect: impl Into<String>,
        identity_binding_field_key: impl Into<String>,
        grouping_binding_field_key: impl Into<String>,
    ) -> Self {
        Self {
            grouping_aspect: grouping_aspect.into(),
            identity_binding_field_key: identity_binding_field_key.into(),
            grouping_binding_field_key: grouping_binding_field_key.into(),
        }
    }

    pub fn grouping_aspect(&self) -> &str {
        &self.grouping_aspect
    }

    pub fn identity_binding_field_key(&self) -> &str {
        &self.identity_binding_field_key
    }

    pub fn grouping_binding_field_key(&self) -> &str {
        &self.grouping_binding_field_key
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelationalGroupingValue(Value);

impl RelationalGroupingValue {
    pub fn value(&self) -> &Value {
        &self.0
    }

    pub(crate) fn new(value: Value) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelationalGroupedMemberRow {
    row_identity: RelationalRowIdentity,
    identity_value: Value,
    grouping_value: RelationalGroupingValue,
}

impl RelationalGroupedMemberRow {
    pub fn row_identity(&self) -> &RelationalRowIdentity {
        &self.row_identity
    }

    pub fn identity_value(&self) -> &Value {
        &self.identity_value
    }

    pub fn grouping_value(&self) -> &RelationalGroupingValue {
        &self.grouping_value
    }
}

impl GroupedProjectionMemberSource for RelationalGroupedMemberRow {
    fn row_identity(&self) -> &str {
        self.row_identity.as_str()
    }

    fn identity_value(&self) -> &Value {
        &self.identity_value
    }

    fn grouping_value(&self) -> &Value {
        self.grouping_value.value()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalGroupedProjectionDigest(String);

impl RelationalGroupedProjectionDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn new(parts: &[String]) -> Self {
        let canonical = parts.join("|");
        let digest = Sha256::digest(canonical.as_bytes());
        Self(format!("relational-grouped-projection:sha256:{digest:x}"))
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
        self.contract.grouping_aspect()
    }

    fn identity_binding_field_key(&self) -> &str {
        self.contract.identity_binding_field_key()
    }

    fn grouping_binding_field_key(&self) -> &str {
        self.contract.grouping_binding_field_key()
    }

    fn members(&self) -> &[Self::Member] {
        &self.members
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelationalGroupedTruthError {
    PacketResultShapeMismatch,
    PayloadDecodeFailure {
        request_key: String,
    },
    MissingIdentityField {
        row_identity: String,
        field_key: String,
    },
    MissingGroupingField {
        row_identity: String,
        field_key: String,
    },
}

pub fn project_relational_grouped_truth(
    row_set: &RelationalAuthoritativeRowSetArtifact,
    contract: GroupedProjectionContract,
) -> Result<RelationalGroupedProjectionArtifact, RelationalGroupedTruthError> {
    let identity_field = RelationalFieldBindingKey::new(contract.identity_binding_field_key());
    let grouping_field = RelationalFieldBindingKey::new(contract.grouping_binding_field_key());

    let mut members = Vec::with_capacity(row_set.rows().len());
    for row in row_set.rows() {
        let Some(identity_value) = row.fields().get(&identity_field).cloned() else {
            return Err(RelationalGroupedTruthError::MissingIdentityField {
                row_identity: row.row_identity().as_str().to_string(),
                field_key: contract.identity_binding_field_key().to_string(),
            });
        };
        let Some(grouping_value) = row.fields().get(&grouping_field).cloned() else {
            return Err(RelationalGroupedTruthError::MissingGroupingField {
                row_identity: row.row_identity().as_str().to_string(),
                field_key: contract.grouping_binding_field_key().to_string(),
            });
        };

        members.push(RelationalGroupedMemberRow {
            row_identity: row.row_identity().clone(),
            identity_value: identity_value.value().clone(),
            grouping_value: RelationalGroupingValue::new(grouping_value.value().clone()),
        });
    }

    let mut digest_parts = vec![
        format!("row_set:{}", row_set.digest().as_str()),
        format!("snapshot:{}", row_set.snapshot_identity().as_str()),
        format!("grouping:{}", contract.grouping_aspect()),
        format!("identity_binding:{}", contract.identity_binding_field_key()),
        format!("grouping_binding:{}", contract.grouping_binding_field_key()),
    ];
    for member in &members {
        digest_parts.push(format!(
            "member:{}|id={}|lane={}",
            member.row_identity().as_str(),
            canonical_json(member.identity_value()),
            canonical_json(member.grouping_value().value())
        ));
    }

    Ok(RelationalGroupedProjectionArtifact {
        row_set_digest: row_set.digest().clone(),
        snapshot_identity: row_set.snapshot_identity().clone(),
        contract,
        members,
        digest: RelationalGroupedProjectionDigest::new(&digest_parts),
    })
}

fn canonical_json(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "<invalid-json>".to_string())
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use forge_runtime_bridge::facade::{
        SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadRequest,
        TruthSnapshotIdentity,
    };

    use super::{project_relational_grouped_truth, GroupedProjectionContract};
    use crate::grouped_truth::materialize_relational_authoritative_row_set;

    #[test]
    fn relational_grouped_projection_preserves_member_and_grouping_pairing() {
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

        let grouped = project_relational_grouped_truth(
            &row_set,
            GroupedProjectionContract::new("status", "identity.id", "status.lane"),
        )
        .unwrap();

        assert_eq!(grouped.members().len(), 2);
        assert_eq!(grouped.members()[0].row_identity().as_str(), "entity-1");
        assert_eq!(
            grouped.members()[0].identity_value(),
            &Value::String("task-1".to_string())
        );
    }
}
