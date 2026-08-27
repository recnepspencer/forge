use serde::{Deserialize, Serialize};

use crate::history::data::{BranchId, CommitId, HistoryDriftClass, RelationalCommitReceipt};
use crate::identity::data::VersionId;
use crate::storage::data::{EntityReadRecord, RelationReadRecord};

use super::{CanonicalCommitEnvelope, ReplayLineageAuthorityBasis};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayObservableSurface {
    Snapshot,
    Patch,
    Diagnostics,
    History,
    BranchHead,
    Lineage,
    Strategy,
    DerivedIndexes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayExecutionMode {
    SerialDeterministic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayVerificationMode {
    NormalRecoveryVerification,
    AuditRecoveryVerification,
    CorruptionDiagnosisReplay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayVerificationLayer {
    DigestParity,
    SummaryParity,
    DeepArtifactParity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayMismatchClass {
    PatchDrift,
    DiagnosticsDrift,
    HistoryDrift,
    SnapshotDrift,
    BranchHeadDrift,
    LineageDrift,
    StrategyArtifactDrift,
    StrategyExecutorUnavailable,
    StrategyExecutionFailure,
    StrategyLoweringDrift,
    DerivedIndexDrift,
    SchemaTransitionDrift,
    SchemaContinuationDescriptorDrift,
    SchemaReconciliationDescriptorDrift,
    DescriptorVersionDrift,
    SchemaLineageDrift,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayMismatch {
    pub class: ReplayMismatchClass,
    pub history_drift_class: Option<HistoryDriftClass>,
    pub surface: ReplayObservableSurface,
    pub verification_layer: ReplayVerificationLayer,
    pub detail: String,
    pub expected: Option<String>,
    pub observed: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalReplayRequest {
    pub commit_id: CommitId,
    pub branch_id: BranchId,
    pub execution_mode: ReplayExecutionMode,
    pub verification_mode: ReplayVerificationMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalReplayOutcome {
    pub requested: RelationalReplayRequest,
    pub commit: Option<RelationalCommitReceipt>,
    pub reconstructed_commit_closure: Vec<CommitId>,
    pub snapshot_version: Option<VersionId>,
    pub lineage_authority_basis: Option<ReplayLineageAuthorityBasis>,
    pub compared_surfaces: Vec<ReplayObservableSurface>,
    pub mismatches: Vec<ReplayMismatch>,
    pub failure: Option<super::ReplayFailureClass>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReplaySnapshotSurface {
    pub version_id: VersionId,
    pub entities: Vec<EntityReadRecord>,
    pub relations: Vec<RelationReadRecord>,
}

impl RelationalReplayOutcome {
    pub(crate) fn fail(
        requested: RelationalReplayRequest,
        envelope: Option<&CanonicalCommitEnvelope>,
        chain: Option<&[CommitId]>,
        failure: super::ReplayFailureClass,
    ) -> Self {
        let commit = envelope.map(|candidate| candidate.commit.clone());
        let reconstructed_commit_closure = chain
            .map(|resolved| resolved.to_vec())
            .or_else(|| envelope.map(|candidate| candidate.commit.ordered_parents().clone_inner()))
            .unwrap_or_default();
        let snapshot_version = envelope.map(|candidate| candidate.commit.version_id);
        Self {
            requested,
            commit,
            reconstructed_commit_closure,
            snapshot_version,
            lineage_authority_basis: None,
            compared_surfaces: Vec::new(),
            mismatches: Vec::new(),
            failure: Some(failure),
        }
    }

    pub(crate) fn with_mismatch(mut self, mismatch: ReplayMismatch) -> Self {
        self.compared_surfaces.push(mismatch.surface);
        self.mismatches.push(mismatch);
        self.failure = Some(super::ReplayFailureClass::ObservableMismatch);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayVerificationPlan {
    Normal(NormalReplayVerificationPlan),
    Audit(AuditReplayVerificationPlan),
    CorruptionDiagnosis(CorruptionDiagnosisReplayPlan),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalReplayVerificationPlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditReplayVerificationPlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorruptionDiagnosisReplayPlan;

impl ReplayVerificationPlan {
    pub fn from_mode(mode: ReplayVerificationMode) -> Self {
        match mode {
            ReplayVerificationMode::NormalRecoveryVerification => {
                Self::Normal(NormalReplayVerificationPlan)
            }
            ReplayVerificationMode::AuditRecoveryVerification => {
                Self::Audit(AuditReplayVerificationPlan)
            }
            ReplayVerificationMode::CorruptionDiagnosisReplay => {
                Self::CorruptionDiagnosis(CorruptionDiagnosisReplayPlan)
            }
        }
    }

    pub fn allows_deep_artifact_parity(&self) -> bool {
        !matches!(self, Self::Normal(_))
    }
}
