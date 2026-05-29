use forge_foundational::facade::AspectValue;

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
use crate::source::aspect_values::aspect_value_to_json;
use crate::source::materialize_bridge_row_set;
use crate::source::{GroupedProjectionMemberSource, GroupedProjectionSource};

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
                let snapshot_bytes = match (read.entity_identity(), read.aspect_label()) {
                    ("entity-1", "identity.id") => b"task-1".to_vec(),
                    ("entity-1", "status.lane") => b"todo".to_vec(),
                    ("entity-2", "identity.id") => b"task-2".to_vec(),
                    ("entity-2", "status.lane") => b"doing".to_vec(),
                    _ => b"unknown".to_vec(),
                };
                crate::snapshot::SnapshotReadRecord::new(read.request_key(), snapshot_bytes)
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
    identity_value: AspectValue,
    grouping_value: AspectValue,
}

impl GroupedProjectionMemberSource for TestProjectionMember {
    fn row_identity(&self) -> &str {
        &self.row_identity
    }

    fn identity_value(&self) -> &AspectValue {
        &self.identity_value
    }

    fn grouping_value(&self) -> &AspectValue {
        &self.grouping_value
    }
}

struct TestProjection {
    snapshot_identity: TruthSnapshotIdentity,
    grouping_aspect: String,
    identity_binding_aspect_key: String,
    grouping_binding_aspect_key: String,
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

    fn identity_binding_aspect_key(&self) -> &str {
        &self.identity_binding_aspect_key
    }

    fn grouping_binding_aspect_key(&self) -> &str {
        &self.grouping_binding_aspect_key
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
            identity_binding_aspect_key: "identity.id".to_string(),
            grouping_binding_aspect_key: "status.lane".to_string(),
            members: vec![
                TestProjectionMember {
                    row_identity: "entity-1".to_string(),
                    identity_value: AspectValue::String("task-1".into()),
                    grouping_value: AspectValue::String("todo".into()),
                },
                TestProjectionMember {
                    row_identity: "entity-2".to_string(),
                    identity_value: AspectValue::String("task-2".into()),
                    grouping_value: AspectValue::String("doing".into()),
                },
            ],
        },
    )
    .expect("grouped truth view");

    assert_eq!(grouped.members().len(), 2);
    assert_eq!(
        aspect_value_to_json(grouped.members()[0].lane().value()),
        serde_json::Value::String("todo".to_string())
    );
}

fn projection(
    snapshot_identity: &str,
    identity_binding_aspect_key: &str,
    grouping_binding_aspect_key: &str,
    members: Vec<TestProjectionMember>,
) -> TestProjection {
    TestProjection {
        snapshot_identity: TruthSnapshotIdentity::new(snapshot_identity),
        grouping_aspect: "status".to_string(),
        identity_binding_aspect_key: identity_binding_aspect_key.to_string(),
        grouping_binding_aspect_key: grouping_binding_aspect_key.to_string(),
        members,
    }
}

fn standard_members() -> Vec<TestProjectionMember> {
    vec![
        TestProjectionMember {
            row_identity: "entity-1".to_string(),
            identity_value: AspectValue::String("task-1".into()),
            grouping_value: AspectValue::String("todo".into()),
        },
        TestProjectionMember {
            row_identity: "entity-2".to_string(),
            identity_value: AspectValue::String("task-2".into()),
            grouping_value: AspectValue::String("doing".into()),
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
                    identity_value: AspectValue::String("task-1".into()),
                    grouping_value: AspectValue::String("todo".into()),
                },
                TestProjectionMember {
                    row_identity: "entity-404".to_string(),
                    identity_value: AspectValue::String("task-2".into()),
                    grouping_value: AspectValue::String("doing".into()),
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
                    identity_value: AspectValue::String("wrong-task".into()),
                    grouping_value: AspectValue::String("todo".into()),
                },
                TestProjectionMember {
                    row_identity: "entity-2".to_string(),
                    identity_value: AspectValue::String("task-2".into()),
                    grouping_value: AspectValue::String("doing".into()),
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
                    identity_value: AspectValue::String("task-1".into()),
                    grouping_value: AspectValue::String("done".into()),
                },
                TestProjectionMember {
                    row_identity: "entity-2".to_string(),
                    identity_value: AspectValue::String("task-2".into()),
                    grouping_value: AspectValue::String("doing".into()),
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
fn grouped_truth_view_rejects_missing_identity_and_grouping_aspects() {
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
        super::BridgeGroupedTruthViewError::MissingIdentityAspect { .. }
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
        super::BridgeGroupedTruthViewError::MissingGroupingAspect { .. }
    ));
}
