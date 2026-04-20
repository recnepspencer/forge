use std::collections::BTreeMap;
use std::sync::Arc;

use serde_json::Value;
use sha2::{Digest, Sha256};

use super::grouped_contract::{
    GroupedProjectionContract, GroupedProjectionMemberSource, GroupedProjectionSource,
};
use super::row_set::{
    BridgeMaterializedFieldValue, BridgeMaterializedRowSetArtifact, BridgeRowIdentity,
};

#[derive(Clone, Debug, PartialEq)]
pub struct BridgeGroupedLaneValue {
    grouping_aspect: Arc<str>,
    value: Value,
}

impl BridgeGroupedLaneValue {
    pub fn grouping_aspect(&self) -> &str {
        self.grouping_aspect.as_ref()
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BridgeGroupedMemberRow {
    row_identity: BridgeRowIdentity,
    identity_value: Value,
    lane: BridgeGroupedLaneValue,
}

impl BridgeGroupedMemberRow {
    pub fn row_identity(&self) -> &BridgeRowIdentity {
        &self.row_identity
    }

    pub fn identity_value(&self) -> &Value {
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
        Self(Arc::from(format!("bridge-grouped-truth-view:sha256:{digest:x}")))
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
    MissingIdentityField {
        row_identity: String,
        field_key: String,
    },
    MissingGroupingField {
        row_identity: String,
        field_key: String,
    },
    IdentityParityMismatch {
        row_identity: String,
        field_key: String,
    },
    GroupingParityMismatch {
        row_identity: String,
        field_key: String,
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
        let identity_value = value_for(row.fields().get(contract.identity_binding().field_key()))
            .ok_or_else(|| BridgeGroupedTruthViewError::MissingIdentityField {
                row_identity: member.row_identity().to_string(),
                field_key: contract.identity_binding().field_key().to_string(),
            })?;
        if &identity_value != member.identity_value() {
            return Err(BridgeGroupedTruthViewError::IdentityParityMismatch {
                row_identity: member.row_identity().to_string(),
                field_key: contract.identity_binding().field_key().to_string(),
            });
        }
        let grouping_value = value_for(row.fields().get(contract.grouping_binding().field_key()))
            .ok_or_else(|| BridgeGroupedTruthViewError::MissingGroupingField {
                row_identity: member.row_identity().to_string(),
                field_key: contract.grouping_binding().field_key().to_string(),
            })?;
        if &grouping_value != member.grouping_value() {
            return Err(BridgeGroupedTruthViewError::GroupingParityMismatch {
                row_identity: member.row_identity().to_string(),
                field_key: contract.grouping_binding().field_key().to_string(),
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
        format!("identity_binding:{}", contract.identity_binding().field_key()),
        format!("grouping_binding:{}", contract.grouping_binding().field_key()),
    ];
    for member in &members {
        digest_parts.push(format!(
            "member:{}|id={}|lane={}",
            member.row_identity().as_str(),
            canonical_json(member.identity_value()),
            canonical_json(member.lane().value())
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

fn value_for(field: Option<&BridgeMaterializedFieldValue>) -> Option<Value> {
    field.map(|value| value.value().clone())
}

fn canonical_json(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "<invalid-json>".to_string())
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::diagnostics::BridgeHistoricalMaterializationPath;
    use crate::input::envelope::{TruthBranchIdentity, TruthCommitIdentity};
    use crate::policy::BridgeDiagnosticsTier;
    use crate::snapshot::{
        AdmittedSnapshotContext, BridgeDeliveryIntent, BridgeReplayMode, BridgeSnapshotContext,
        BridgeSnapshotToken, BridgeTruthViewAuthorityBasis, BridgeTruthViewSelector,
        HistoricalEvaluationDeclaration, PlannedTruthViewPacket, ResolvedTruthViewPolicy,
        SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRequest, TruthSnapshotIdentity,
        TruthSnapshotReader, TruthViewReplayCompatibility, TruthViewRetentionAdmission,
        TruthViewSourceCapability,
    };

    use super::materialize_bridge_grouped_truth_view_from_projection;
    use crate::source::materialize_bridge_row_set;
    use crate::source::{
        GroupedProjectionMemberSource, GroupedProjectionSource,
    };

    #[derive(Debug)]
    struct FixtureReader;

    impl TruthSnapshotReader for FixtureReader {
        fn snapshot_identity(&self) -> TruthSnapshotIdentity {
            TruthSnapshotIdentity::new("snapshot-a")
        }

        fn read_packet(
            &self,
            request: &SnapshotReadPacket,
        ) -> Result<SnapshotReadPacketResult, crate::snapshot::BridgeSnapshotReadError> {
            let records = request
                .reads()
                .iter()
                .map(|read| {
                    let payload = match (read.entity_identity(), read.aspect_label()) {
                        ("entity-1", "identity.id") => b"task-1".to_vec(),
                        ("entity-1", "status.lane") => b"todo".to_vec(),
                        ("entity-2", "identity.id") => b"task-2".to_vec(),
                        ("entity-2", "status.lane") => b"doing".to_vec(),
                        _ => b"unknown".to_vec(),
                    };
                    crate::snapshot::SnapshotReadRecord::new(read.request_key(), payload)
                })
                .collect();
            Ok(SnapshotReadPacketResult::new(
                TruthSnapshotIdentity::new("snapshot-a"),
                records,
            ))
        }
    }

    #[derive(Clone)]
    struct TestProjectionMember {
        row_identity: String,
        identity_value: Value,
        grouping_value: Value,
    }

    impl GroupedProjectionMemberSource for TestProjectionMember {
        fn row_identity(&self) -> &str {
            &self.row_identity
        }

        fn identity_value(&self) -> &Value {
            &self.identity_value
        }

        fn grouping_value(&self) -> &Value {
            &self.grouping_value
        }
    }

    struct TestProjection {
        snapshot_identity: TruthSnapshotIdentity,
        grouping_aspect: String,
        identity_binding_field_key: String,
        grouping_binding_field_key: String,
        members: Vec<TestProjectionMember>,
    }

    impl GroupedProjectionSource for TestProjection {
        type Member = TestProjectionMember;

        fn basis_snapshot_identity(&self) -> &TruthSnapshotIdentity {
            &self.snapshot_identity
        }

        fn grouping_aspect(&self) -> &str {
            &self.grouping_aspect
        }

        fn identity_binding_field_key(&self) -> &str {
            &self.identity_binding_field_key
        }

        fn grouping_binding_field_key(&self) -> &str {
            &self.grouping_binding_field_key
        }

        fn members(&self) -> &[Self::Member] {
            &self.members
        }
    }

    fn row_set() -> crate::source::BridgeMaterializedRowSetArtifact {
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                TruthCommitIdentity::new("commit-a"),
            ),
            BridgeReplayMode::Disabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        let packet = PlannedTruthViewPacket::new(
            declaration.clone(),
            ResolvedTruthViewPolicy::admitted(
                &declaration,
                TruthViewRetentionAdmission::HistoricalLookupRequired,
                TruthViewSourceCapability::HistoricalLookupAndSnapshotRead,
                TruthViewReplayCompatibility::ReplayPermitted,
            ),
            BridgeTruthViewAuthorityBasis::from_resolved_envelope(
                declaration.selector(),
                TruthCommitIdentity::new("commit-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            SnapshotReadPacket::new(vec![
                SnapshotReadRequest::for_coarse("entity-1", "identity.id"),
                SnapshotReadRequest::for_coarse("entity-1", "status.lane"),
                SnapshotReadRequest::for_coarse("entity-2", "identity.id"),
                SnapshotReadRequest::for_coarse("entity-2", "status.lane"),
            ]),
        );
        let snapshot =
            BridgeSnapshotContext::bind(Box::new(FixtureReader) as Box<dyn TruthSnapshotReader>);
        let admitted =
            AdmittedSnapshotContext::admit_for(snapshot, &TruthSnapshotIdentity::new("snapshot-a"))
                .expect("snapshot should admit");
        let observation = crate::snapshot::MaterializedTruthViewObservation::new(
            packet,
            BridgeSnapshotToken::issued(
                TruthSnapshotIdentity::new("snapshot-a"),
                "grouped-truth-test",
            ),
            BridgeHistoricalMaterializationPath::CommitEnvelopeSnapshot,
            admitted,
        );
        materialize_bridge_row_set(&observation).expect("row set")
    }

    #[test]
    fn grouped_truth_view_preserves_row_and_lane_pairing() {
        let grouped = materialize_bridge_grouped_truth_view_from_projection(
            &row_set(),
            &TestProjection {
                snapshot_identity: TruthSnapshotIdentity::new("snapshot-a"),
                grouping_aspect: "status".to_string(),
                identity_binding_field_key: "identity.id".to_string(),
                grouping_binding_field_key: "status.lane".to_string(),
                members: vec![
                    TestProjectionMember {
                        row_identity: "entity-1".to_string(),
                        identity_value: Value::String("task-1".to_string()),
                        grouping_value: Value::String("todo".to_string()),
                    },
                    TestProjectionMember {
                        row_identity: "entity-2".to_string(),
                        identity_value: Value::String("task-2".to_string()),
                        grouping_value: Value::String("doing".to_string()),
                    },
                ],
            },
        )
        .expect("grouped truth view");

        assert_eq!(grouped.members().len(), 2);
        assert_eq!(
            grouped.members()[0].lane().value(),
            &Value::String("todo".to_string())
        );
    }

    fn projection(
        snapshot_identity: &str,
        identity_binding_field_key: &str,
        grouping_binding_field_key: &str,
        members: Vec<TestProjectionMember>,
    ) -> TestProjection {
        TestProjection {
            snapshot_identity: TruthSnapshotIdentity::new(snapshot_identity),
            grouping_aspect: "status".to_string(),
            identity_binding_field_key: identity_binding_field_key.to_string(),
            grouping_binding_field_key: grouping_binding_field_key.to_string(),
            members,
        }
    }

    fn standard_members() -> Vec<TestProjectionMember> {
        vec![
            TestProjectionMember {
                row_identity: "entity-1".to_string(),
                identity_value: Value::String("task-1".to_string()),
                grouping_value: Value::String("todo".to_string()),
            },
            TestProjectionMember {
                row_identity: "entity-2".to_string(),
                identity_value: Value::String("task-2".to_string()),
                grouping_value: Value::String("doing".to_string()),
            },
        ]
    }

    #[test]
    fn grouped_truth_view_rejects_basis_snapshot_mismatch() {
        let error = materialize_bridge_grouped_truth_view_from_projection(
            &row_set(),
            &projection(
                "snapshot-b",
                "identity.id",
                "status.lane",
                standard_members(),
            ),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            super::BridgeGroupedTruthViewError::BasisSnapshotMismatch { .. }
        ));
    }

    #[test]
    fn grouped_truth_view_rejects_projection_row_count_mismatch() {
        let error = materialize_bridge_grouped_truth_view_from_projection(
            &row_set(),
            &projection(
                "snapshot-a",
                "identity.id",
                "status.lane",
                vec![standard_members()[0].clone()],
            ),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            super::BridgeGroupedTruthViewError::RowCountMismatch { .. }
        ));
    }

    #[test]
    fn grouped_truth_view_rejects_missing_projection_row() {
        let error = materialize_bridge_grouped_truth_view_from_projection(
            &row_set(),
            &projection(
                "snapshot-a",
                "identity.id",
                "status.lane",
                vec![
                    TestProjectionMember {
                        row_identity: "entity-1".to_string(),
                        identity_value: Value::String("task-1".to_string()),
                        grouping_value: Value::String("todo".to_string()),
                    },
                    TestProjectionMember {
                        row_identity: "entity-404".to_string(),
                        identity_value: Value::String("task-2".to_string()),
                        grouping_value: Value::String("doing".to_string()),
                    },
                ],
            ),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            super::BridgeGroupedTruthViewError::MissingProjectionRow { .. }
        ));
    }

    #[test]
    fn grouped_truth_view_rejects_identity_and_grouping_parity_mismatches() {
        let identity_error = materialize_bridge_grouped_truth_view_from_projection(
            &row_set(),
            &projection(
                "snapshot-a",
                "identity.id",
                "status.lane",
                vec![
                    TestProjectionMember {
                        row_identity: "entity-1".to_string(),
                        identity_value: Value::String("wrong-task".to_string()),
                        grouping_value: Value::String("todo".to_string()),
                    },
                    TestProjectionMember {
                        row_identity: "entity-2".to_string(),
                        identity_value: Value::String("task-2".to_string()),
                        grouping_value: Value::String("doing".to_string()),
                    },
                ],
            ),
        )
        .unwrap_err();
        assert!(matches!(
            identity_error,
            super::BridgeGroupedTruthViewError::IdentityParityMismatch { .. }
        ));

        let grouping_error = materialize_bridge_grouped_truth_view_from_projection(
            &row_set(),
            &projection(
                "snapshot-a",
                "identity.id",
                "status.lane",
                vec![
                    TestProjectionMember {
                        row_identity: "entity-1".to_string(),
                        identity_value: Value::String("task-1".to_string()),
                        grouping_value: Value::String("done".to_string()),
                    },
                    TestProjectionMember {
                        row_identity: "entity-2".to_string(),
                        identity_value: Value::String("task-2".to_string()),
                        grouping_value: Value::String("doing".to_string()),
                    },
                ],
            ),
        )
        .unwrap_err();
        assert!(matches!(
            grouping_error,
            super::BridgeGroupedTruthViewError::GroupingParityMismatch { .. }
        ));
    }

    #[test]
    fn grouped_truth_view_rejects_missing_identity_and_grouping_fields() {
        let identity_error = materialize_bridge_grouped_truth_view_from_projection(
            &row_set(),
            &projection(
                "snapshot-a",
                "identity.missing",
                "status.lane",
                standard_members(),
            ),
        )
        .unwrap_err();
        assert!(matches!(
            identity_error,
            super::BridgeGroupedTruthViewError::MissingIdentityField { .. }
        ));

        let grouping_error = materialize_bridge_grouped_truth_view_from_projection(
            &row_set(),
            &projection(
                "snapshot-a",
                "identity.id",
                "status.missing",
                standard_members(),
            ),
        )
        .unwrap_err();
        assert!(matches!(
            grouping_error,
            super::BridgeGroupedTruthViewError::MissingGroupingField { .. }
        ));
    }
}
