use std::collections::BTreeMap;
use std::sync::Arc;

use forge_foundational::facade::AspectValue;
use sha2::{Digest, Sha256};

use super::aspect_values::canonical_aspect_value_text;
use super::grouped_contract::{
    GroupedProjectionContract, GroupedProjectionMemberSource, GroupedProjectionSource,
};
use super::row_set::{
    BridgeMaterializedFieldValue, BridgeMaterializedRowSetArtifact, BridgeRowIdentity,
};

#[derive(Clone, Debug, PartialEq)]
pub struct BridgeGroupedLaneValue {
    grouping_aspect: Arc<str>,
    value: AspectValue,
}

impl BridgeGroupedLaneValue {
    pub fn grouping_aspect(&self) -> &str {
        self.grouping_aspect.as_ref()
    }

    pub fn value(&self) -> &AspectValue {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BridgeGroupedMemberRow {
    row_identity: BridgeRowIdentity,
    identity_value: AspectValue,
    lane: BridgeGroupedLaneValue,
}

impl BridgeGroupedMemberRow {
    pub fn row_identity(&self) -> &BridgeRowIdentity {
        &self.row_identity
    }

    pub fn identity_value(&self) -> &AspectValue {
        &self.identity_value
    }

    pub fn lane(&self) -> &BridgeGroupedLaneValue {
        &self.lane
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeGroupedTruthViewDigest(Arc<str>);

impl BridgeGroupedTruthViewDigest {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    fn new(parts: &[String]) -> Self {
        let canonical = parts.join("|");
        let digest = Sha256::digest(canonical.as_bytes());
        Self(Arc::from(format!(
            "bridge-grouped-truth-view:sha256:{digest:x}"
        )))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BridgeGroupedTruthViewArtifact {
    truth_view_digest: Arc<str>,
    basis_snapshot_identity: crate::snapshot::TruthSnapshotIdentity,
    contract: GroupedProjectionContract,
    members: Vec<BridgeGroupedMemberRow>,
    digest: BridgeGroupedTruthViewDigest,
}

impl BridgeGroupedTruthViewArtifact {
    pub fn truth_view_digest(&self) -> &str {
        self.truth_view_digest.as_ref()
    }

    pub fn basis_snapshot_identity(&self) -> &crate::snapshot::TruthSnapshotIdentity {
        &self.basis_snapshot_identity
    }

    pub fn contract(&self) -> &GroupedProjectionContract {
        &self.contract
    }

    pub fn members(&self) -> &[BridgeGroupedMemberRow] {
        &self.members
    }

    pub fn digest(&self) -> &BridgeGroupedTruthViewDigest {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BridgeGroupedTruthViewError {
    BasisSnapshotMismatch {
        row_set_snapshot: String,
        projection_snapshot: String,
    },
    RowCountMismatch {
        row_set_count: usize,
        projection_count: usize,
    },
    MissingProjectionRow {
        row_identity: String,
    },
    MissingIdentityAspect {
        row_identity: String,
        aspect_key: String,
    },
    MissingGroupingAspect {
        row_identity: String,
        aspect_key: String,
    },
    IdentityParityMismatch {
        row_identity: String,
        aspect_key: String,
    },
    GroupingParityMismatch {
        row_identity: String,
        aspect_key: String,
    },
}

pub fn materialize_bridge_grouped_truth_view_from_projection(
    row_set: &BridgeMaterializedRowSetArtifact,
    projection: &impl GroupedProjectionSource,
) -> Result<BridgeGroupedTruthViewArtifact, BridgeGroupedTruthViewError> {
    if row_set.basis_snapshot_identity() != projection.basis_snapshot_identity() {
        return Err(BridgeGroupedTruthViewError::BasisSnapshotMismatch {
            row_set_snapshot: row_set.basis_snapshot_identity().as_str().to_string(),
            projection_snapshot: projection.basis_snapshot_identity().as_str().to_string(),
        });
    }
    if row_set.rows().len() != projection.members().len() {
        return Err(BridgeGroupedTruthViewError::RowCountMismatch {
            row_set_count: row_set.rows().len(),
            projection_count: projection.members().len(),
        });
    }

    let contract = GroupedProjectionContract::from_source(projection);
    let row_index = row_set
        .rows()
        .iter()
        .map(|row| (row.row_identity().as_str(), row))
        .collect::<BTreeMap<_, _>>();

    let mut members = Vec::with_capacity(projection.members().len());
    for member in projection.members() {
        let Some(row) = row_index.get(member.row_identity()) else {
            return Err(BridgeGroupedTruthViewError::MissingProjectionRow {
                row_identity: member.row_identity().to_string(),
            });
        };
        let identity_value = value_for(row.fields().get(contract.identity_binding().aspect_key()))
            .ok_or_else(|| BridgeGroupedTruthViewError::MissingIdentityAspect {
                row_identity: member.row_identity().to_string(),
                aspect_key: contract.identity_binding().aspect_key().to_string(),
            })?;
        if &identity_value != member.identity_value() {
            return Err(BridgeGroupedTruthViewError::IdentityParityMismatch {
                row_identity: member.row_identity().to_string(),
                aspect_key: contract.identity_binding().aspect_key().to_string(),
            });
        }
        let grouping_value = value_for(row.fields().get(contract.grouping_binding().aspect_key()))
            .ok_or_else(|| BridgeGroupedTruthViewError::MissingGroupingAspect {
                row_identity: member.row_identity().to_string(),
                aspect_key: contract.grouping_binding().aspect_key().to_string(),
            })?;
        if &grouping_value != member.grouping_value() {
            return Err(BridgeGroupedTruthViewError::GroupingParityMismatch {
                row_identity: member.row_identity().to_string(),
                aspect_key: contract.grouping_binding().aspect_key().to_string(),
            });
        }

        members.push(BridgeGroupedMemberRow {
            row_identity: row.row_identity().clone(),
            identity_value,
            lane: BridgeGroupedLaneValue {
                grouping_aspect: Arc::from(contract.grouping_aspect().to_string()),
                value: grouping_value,
            },
        });
    }

    let mut digest_parts = vec![
        format!("truth_view:{}", row_set.truth_view_digest()),
        format!("snapshot:{}", row_set.basis_snapshot_identity().as_str()),
        format!("grouping:{}", contract.grouping_aspect()),
        format!(
            "identity_binding:{}",
            contract.identity_binding().aspect_key()
        ),
        format!(
            "grouping_binding:{}",
            contract.grouping_binding().aspect_key()
        ),
    ];
    for member in &members {
        digest_parts.push(format!(
            "member:{}|id={}|lane={}",
            member.row_identity().as_str(),
            canonical_aspect_value_text(member.identity_value()),
            canonical_aspect_value_text(member.lane().value())
        ));
    }

    Ok(BridgeGroupedTruthViewArtifact {
        truth_view_digest: Arc::from(row_set.truth_view_digest().to_string()),
        basis_snapshot_identity: row_set.basis_snapshot_identity().clone(),
        contract,
        members,
        digest: BridgeGroupedTruthViewDigest::new(&digest_parts),
    })
}

fn value_for(field: Option<&BridgeMaterializedFieldValue>) -> Option<AspectValue> {
    field.map(|value| value.value().clone())
}

#[cfg(test)]
#[path = "grouped_truth_view_tests.rs"]
mod tests;
