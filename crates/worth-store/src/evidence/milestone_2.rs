use crate::{
    authority::AuthoritativeExportBundle,
    evidence::StoreCounterSnapshot,
    failure::{StoreError, StoreErrorKind},
    AbsentModeSemanticEvidence, EmbeddedCheckpointClassification,
    EmbeddedCheckpointPersistenceReceipt,
};
use worth_relational::facade::history::CommitId;
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum OperatingModeLane {
    Durable,
    Embedded,
    Absent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PersistedModeLaneEvidence {
    pub lane: OperatingModeLane,
    pub artifact_digest: String,
    pub export_digest: String,
    pub counter_snapshot: StoreCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AbsentModeLaneEvidence {
    pub lane: OperatingModeLane,
    pub semantic_digest: String,
    pub latest_commit_id: Option<CommitId>,
    pub counter_snapshot: StoreCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OperatingModeCounterSnapshot {
    pub durable_lane: StoreCounterSnapshot,
    pub embedded_lane: StoreCounterSnapshot,
    pub absent_lane: StoreCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CheckpointAuthorityReport {
    pub checkpoint_id: String,
    pub classification: EmbeddedCheckpointClassification,
    pub authoritative_artifact_digest_before: String,
    pub authoritative_artifact_digest_after: String,
    pub persisted_checkpoint_digest: String,
    pub contained_commit_ids: Vec<CommitId>,
    pub checkpoint_changed_authority: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ObservedModeFailure {
    pub kind: StoreErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OperatingModeContractMatrix {
    pub durable_lane: PersistedModeLaneEvidence,
    pub embedded_lane: PersistedModeLaneEvidence,
    pub absent_lane: AbsentModeLaneEvidence,
    pub durable_embedded_artifact_parity: bool,
    pub absent_mode_is_no_store: bool,
    pub zero_forbidden_cross_mode_work: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone2CertificationBundle {
    pub artifact_digest: String,
    pub diagnostics_digest: String,
    pub mode_contract_matrix: OperatingModeContractMatrix,
    pub failure_digest: String,
    pub checkpoint_authority_report: CheckpointAuthorityReport,
    pub counter_snapshot: OperatingModeCounterSnapshot,
}

impl PersistedModeLaneEvidence {
    pub(crate) fn from_export(
        lane: OperatingModeLane,
        bundle: &AuthoritativeExportBundle,
        counter_snapshot: StoreCounterSnapshot,
    ) -> Self {
        let export_digest = stable_digest(bundle);
        let canonical = bundle.clone().into_canonicalized();
        let artifact_digest = stable_digest(&canonical.authoritative_artifact_digests);

        Self {
            lane,
            artifact_digest,
            export_digest,
            counter_snapshot,
        }
    }
}

impl AbsentModeLaneEvidence {
    pub fn from_semantic_evidence(evidence: &AbsentModeSemanticEvidence) -> Self {
        let counter_snapshot = StoreCounterSnapshot {
            absent_mode_selection_count: 1,
            ..StoreCounterSnapshot::default()
        };

        let semantic_digest = stable_digest(&AbsentSemanticDigestBasis {
            latest_commit_id: evidence.latest_commit_id(),
            latest_commit_envelope: evidence.latest_commit_envelope(),
        });

        Self {
            lane: OperatingModeLane::Absent,
            semantic_digest,
            latest_commit_id: evidence.latest_commit_id(),
            counter_snapshot,
        }
    }
}

impl CheckpointAuthorityReport {
    pub fn from_checkpoint(
        authoritative_artifact_digest_before: String,
        authoritative_artifact_digest_after: String,
        receipt: &EmbeddedCheckpointPersistenceReceipt,
    ) -> Self {
        let persisted_checkpoint_digest = stable_digest(&CheckpointDigestBasis {
            checkpoint_id: receipt.checkpoint_id(),
            classification: receipt.classification(),
            contained_commit_ids: receipt.contained_commit_ids(),
        });

        Self {
            checkpoint_id: receipt.checkpoint_id().to_string(),
            classification: *receipt.classification(),
            authoritative_artifact_digest_before,
            authoritative_artifact_digest_after,
            persisted_checkpoint_digest,
            contained_commit_ids: receipt.contained_commit_ids().to_vec(),
            checkpoint_changed_authority: false,
        }
    }
}

impl ObservedModeFailure {
    pub fn from_error(error: &StoreError) -> Self {
        Self {
            kind: error.kind().clone(),
            message: error.message().to_string(),
        }
    }
}

impl Milestone2CertificationBundle {
    pub fn new(
        durable_lane: PersistedModeLaneEvidence,
        embedded_lane: PersistedModeLaneEvidence,
        absent_lane: AbsentModeLaneEvidence,
        checkpoint_authority_report: CheckpointAuthorityReport,
        failures: &[ObservedModeFailure],
    ) -> Self {
        let counter_snapshot = OperatingModeCounterSnapshot {
            durable_lane: durable_lane.counter_snapshot,
            embedded_lane: embedded_lane.counter_snapshot,
            absent_lane: absent_lane.counter_snapshot,
        };

        let mode_contract_matrix = OperatingModeContractMatrix {
            durable_embedded_artifact_parity: durable_lane.artifact_digest
                == embedded_lane.artifact_digest,
            absent_mode_is_no_store: absent_lane.counter_snapshot.absent_mode_selection_count == 1
                && absent_lane.counter_snapshot.absent_mode_store_touch_count == 0
                && absent_lane.counter_snapshot.durable_mode_selection_count == 0
                && absent_lane.counter_snapshot.embedded_mode_selection_count == 0
                && absent_lane.counter_snapshot.hosted_runtime_start_count == 0
                && absent_lane.counter_snapshot.hosted_runtime_stop_count == 0
                && absent_lane.counter_snapshot.external_commit_intake_count == 0
                && absent_lane
                    .counter_snapshot
                    .external_checkpoint_intake_count
                    == 0,
            zero_forbidden_cross_mode_work: durable_lane
                .counter_snapshot
                .external_commit_intake_count
                == 0
                && durable_lane
                    .counter_snapshot
                    .external_checkpoint_intake_count
                    == 0
                && embedded_lane.counter_snapshot.hosted_runtime_start_count == 0
                && embedded_lane.counter_snapshot.hosted_runtime_stop_count == 0
                && absent_lane.counter_snapshot.absent_mode_store_touch_count == 0,
            durable_lane,
            embedded_lane,
            absent_lane,
        };

        let artifact_digest = stable_digest(&ArtifactDigestBasis {
            durable_artifact_digest: &mode_contract_matrix.durable_lane.artifact_digest,
            embedded_artifact_digest: &mode_contract_matrix.embedded_lane.artifact_digest,
            absent_semantic_digest: &mode_contract_matrix.absent_lane.semantic_digest,
            durable_embedded_artifact_parity: mode_contract_matrix.durable_embedded_artifact_parity,
        });
        let failure_digest = stable_digest(failures);
        let diagnostics_digest = stable_digest(&DiagnosticsDigestBasis {
            mode_contract_matrix: &mode_contract_matrix,
            checkpoint_authority_report: &checkpoint_authority_report,
            failure_digest: &failure_digest,
            counter_snapshot: &counter_snapshot,
        });

        Self {
            artifact_digest,
            diagnostics_digest,
            mode_contract_matrix,
            failure_digest,
            checkpoint_authority_report,
            counter_snapshot,
        }
    }

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).expect("milestone 2 certification serialization")
    }
}

#[derive(Serialize)]
struct AbsentSemanticDigestBasis<'a> {
    latest_commit_id: Option<CommitId>,
    latest_commit_envelope: Option<&'a worth_relational::facade::replay::CanonicalCommitEnvelope>,
}

#[derive(Serialize)]
struct CheckpointDigestBasis<'a> {
    checkpoint_id: &'a str,
    classification: &'a EmbeddedCheckpointClassification,
    contained_commit_ids: &'a [CommitId],
}

#[derive(Serialize)]
struct ArtifactDigestBasis<'a> {
    durable_artifact_digest: &'a str,
    embedded_artifact_digest: &'a str,
    absent_semantic_digest: &'a str,
    durable_embedded_artifact_parity: bool,
}

#[derive(Serialize)]
struct DiagnosticsDigestBasis<'a> {
    mode_contract_matrix: &'a OperatingModeContractMatrix,
    checkpoint_authority_report: &'a CheckpointAuthorityReport,
    failure_digest: &'a str,
    counter_snapshot: &'a OperatingModeCounterSnapshot,
}

fn stable_digest<T: Serialize + ?Sized>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("milestone 2 digest serialization");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
