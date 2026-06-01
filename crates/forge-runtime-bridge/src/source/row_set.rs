use std::collections::BTreeMap;
use std::sync::Arc;

use forge_foundational::facade::AspectValue;
use sha2::{Digest, Sha256};

use super::aspect_values::{canonical_aspect_value_text, decode_snapshot_aspect_bytes};
use crate::snapshot::MaterializedTruthViewObservation;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BridgeRowIdentity(Arc<str>);

impl BridgeRowIdentity {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    fn new(value: impl Into<Arc<str>>) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BridgeMaterializedFieldValue(AspectValue);

impl BridgeMaterializedFieldValue {
    pub fn value(&self) -> &AspectValue {
        &self.0
    }

    fn new(value: AspectValue) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BridgeMaterializedRowArtifact {
    row_identity: BridgeRowIdentity,
    fields: BTreeMap<Arc<str>, BridgeMaterializedFieldValue>,
}

impl BridgeMaterializedRowArtifact {
    pub fn row_identity(&self) -> &BridgeRowIdentity {
        &self.row_identity
    }

    pub fn fields(&self) -> &BTreeMap<Arc<str>, BridgeMaterializedFieldValue> {
        &self.fields
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeMaterializedRowSetDigest(Arc<str>);

impl BridgeMaterializedRowSetDigest {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    fn new(parts: &[String]) -> Self {
        let canonical = parts.join("|");
        let digest = Sha256::digest(canonical.as_bytes());
        Self(Arc::from(format!("bridge-row-set:sha256:{digest:x}")))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BridgeMaterializedRowSetArtifact {
    truth_view_digest: Arc<str>,
    basis_snapshot_identity: crate::snapshot::TruthSnapshotIdentity,
    rows: Vec<BridgeMaterializedRowArtifact>,
    digest: BridgeMaterializedRowSetDigest,
}

impl BridgeMaterializedRowSetArtifact {
    pub fn truth_view_digest(&self) -> &str {
        self.truth_view_digest.as_ref()
    }

    pub fn basis_snapshot_identity(&self) -> &crate::snapshot::TruthSnapshotIdentity {
        &self.basis_snapshot_identity
    }

    pub fn rows(&self) -> &[BridgeMaterializedRowArtifact] {
        &self.rows
    }

    pub fn digest(&self) -> &BridgeMaterializedRowSetDigest {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BridgeRowSetMaterializationError {
    AspectBytesDecodeFailure { request_key: String },
}

pub fn materialize_bridge_row_set(
    observation: &MaterializedTruthViewObservation,
) -> Result<BridgeMaterializedRowSetArtifact, BridgeRowSetMaterializationError> {
    let result = observation
        .read_planned_packet()
        .expect("validated read packet should remain valid during materialization");
    let mut rows: BTreeMap<Arc<str>, BTreeMap<Arc<str>, BridgeMaterializedFieldValue>> =
        BTreeMap::new();

    for (read, record) in observation
        .read_packet()
        .reads()
        .iter()
        .zip(result.records().iter())
    {
        let value = decode_snapshot_aspect_bytes(record.aspect_bytes()).map_err(|_| {
            BridgeRowSetMaterializationError::AspectBytesDecodeFailure {
                request_key: record.request_key().to_string(),
            }
        })?;
        rows.entry(Arc::from(read.entity_identity()))
            .or_default()
            .insert(
                Arc::from(read.aspect_label()),
                BridgeMaterializedFieldValue::new(value),
            );
    }

    let rows = rows
        .into_iter()
        .map(|(row_identity, fields)| BridgeMaterializedRowArtifact {
            row_identity: BridgeRowIdentity::new(row_identity),
            fields,
        })
        .collect::<Vec<_>>();

    let mut digest_parts = vec![
        format!("planned:{}", observation.planned().digest()),
        format!("snapshot:{}", result.snapshot_identity().as_str()),
    ];
    for row in &rows {
        digest_parts.push(format!("row:{}", row.row_identity().as_str()));
        for (field, value) in row.fields() {
            digest_parts.push(format!(
                "field:{}={}",
                field,
                canonical_aspect_value_text(value.value())
            ));
        }
    }

    Ok(BridgeMaterializedRowSetArtifact {
        truth_view_digest: Arc::from(observation.planned().digest().to_string()),
        basis_snapshot_identity: result.snapshot_identity().clone(),
        rows,
        digest: BridgeMaterializedRowSetDigest::new(&digest_parts),
    })
}

#[cfg(test)]
mod tests {
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

    use super::materialize_bridge_row_set;

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
                    let aspect_bytes = match (read.entity_identity(), read.aspect_label()) {
                        ("entity-1", "identity.id") => b"task-1".to_vec(),
                        ("entity-1", "status") => b"todo".to_vec(),
                        ("entity-2", "identity.id") => b"task-2".to_vec(),
                        ("entity-2", "status") => b"doing".to_vec(),
                        _ => b"unknown".to_vec(),
                    };
                    crate::snapshot::SnapshotReadRecord::new(read.request_key(), aspect_bytes)
                })
                .collect();
            Ok(SnapshotReadPacketResult::new(
                TruthSnapshotIdentity::new("snapshot-a"),
                records,
            ))
        }
    }

    fn observation() -> crate::snapshot::MaterializedTruthViewObservation {
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
                SnapshotReadRequest::for_coarse(
                    "entity-1",
                    forge_foundational::facade::AspectKey::new("identity.id")
                        .expect("valid snapshot aspect key"),
                ),
                SnapshotReadRequest::for_coarse(
                    "entity-1",
                    forge_foundational::facade::AspectKey::new("status")
                        .expect("valid snapshot aspect key"),
                ),
                SnapshotReadRequest::for_coarse(
                    "entity-2",
                    forge_foundational::facade::AspectKey::new("identity.id")
                        .expect("valid snapshot aspect key"),
                ),
                SnapshotReadRequest::for_coarse(
                    "entity-2",
                    forge_foundational::facade::AspectKey::new("status")
                        .expect("valid snapshot aspect key"),
                ),
            ]),
        );
        let snapshot =
            BridgeSnapshotContext::bind(Box::new(FixtureReader) as Box<dyn TruthSnapshotReader>);
        let admitted =
            AdmittedSnapshotContext::admit_for(snapshot, &TruthSnapshotIdentity::new("snapshot-a"))
                .expect("snapshot should admit");
        crate::snapshot::MaterializedTruthViewObservation::new(
            packet,
            BridgeSnapshotToken::issued(TruthSnapshotIdentity::new("snapshot-a"), "row-set-test"),
            BridgeHistoricalMaterializationPath::CommitEnvelopeSnapshot,
            admitted,
        )
    }

    #[test]
    fn bridge_row_set_preserves_multi_row_truth() {
        let row_set = materialize_bridge_row_set(&observation()).expect("row set");

        assert_eq!(row_set.rows().len(), 2);
        assert_eq!(row_set.rows()[0].row_identity().as_str(), "entity-1");
        assert_eq!(row_set.rows()[1].row_identity().as_str(), "entity-2");
    }
}
