mod replay_errors;

use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

use crate::diagnostics::data::RelationalDiagnosticArtifact;
use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::indexes::data::DerivedIndexGeneration;
use crate::lineage::data::{LineageDecisionRecord, LineageEventRecord, PublishedLineageArtifact};
use crate::publication::data::diff::RelationalPatchRecord;
use crate::schema::data::{
    DescriptorCanonicalizationVersion, DescriptorSemanticsVersion, RelationalSchemaRegistry,
    SchemaContinuationDescriptor, SchemaReconciliationDescriptor, SchemaTransitionArtifact,
    SchemaVersionId,
};
use crate::storage::data::{EntityReadRecord, RelationReadRecord};
use crate::transactions::data::MergedCommitPlan;

pub use replay_errors::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalCommitEnvelope {
    pub commit: CommitReference,
    pub branch_context: BranchId,
    pub authority_kind: CanonicalCommitAuthorityKind,
    pub merge_parent_branches: Vec<BranchId>,
    pub merge_base_commits: Vec<CommitId>,
    pub schema_version: SchemaVersionId,
    pub schema_registry: RelationalSchemaRegistry,
    pub merged_plan: MergedCommitPlan,
    pub patch: RelationalPatchRecord,
    pub diagnostics_summary: RelationalDiagnosticArtifact,
    pub index_generation_ids: Vec<u64>,
    lineage: PublishedLineageArtifact,
    pub index_generations: Vec<DerivedIndexGeneration>,
    pub schema_transition: Option<SchemaTransitionArtifact>,
    pub schema_continuation_descriptor: Option<SchemaContinuationDescriptor>,
    pub schema_reconciliation_descriptor: Option<SchemaReconciliationDescriptor>,
    pub descriptor_semantics_version: DescriptorSemanticsVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanonicalCommitAuthorityKind {
    VersionedTransaction,
    MetadataOnlyLineage,
}

impl CanonicalCommitEnvelope {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        commit: CommitReference,
        branch_context: BranchId,
        authority_kind: CanonicalCommitAuthorityKind,
        merge_parent_branches: Vec<BranchId>,
        merge_base_commits: Vec<CommitId>,
        schema_version: SchemaVersionId,
        schema_registry: RelationalSchemaRegistry,
        merged_plan: MergedCommitPlan,
        patch: RelationalPatchRecord,
        diagnostics_summary: RelationalDiagnosticArtifact,
        index_generation_ids: Vec<u64>,
        lineage: PublishedLineageArtifact,
        index_generations: Vec<DerivedIndexGeneration>,
        schema_transition: Option<SchemaTransitionArtifact>,
        schema_continuation_descriptor: Option<SchemaContinuationDescriptor>,
        schema_reconciliation_descriptor: Option<SchemaReconciliationDescriptor>,
        descriptor_semantics_version: DescriptorSemanticsVersion,
    ) -> Self {
        Self {
            commit,
            branch_context,
            authority_kind,
            merge_parent_branches,
            merge_base_commits,
            schema_version,
            schema_registry,
            merged_plan,
            patch,
            diagnostics_summary,
            index_generation_ids,
            lineage,
            index_generations,
            schema_transition,
            schema_continuation_descriptor,
            schema_reconciliation_descriptor,
            descriptor_semantics_version,
        }
    }

    pub fn lineage_event_ids(&self) -> &[u64] {
        self.lineage.lineage_event_ids()
    }

    pub fn lineage_events(&self) -> &[LineageEventRecord] {
        self.lineage.lineage_events()
    }

    pub fn lineage_decision_log(&self) -> &[LineageDecisionRecord] {
        self.lineage.lineage_decision_log()
    }

    pub fn has_lineage_authority(&self) -> bool {
        self.lineage.has_authority_content()
    }

    pub fn authority_kind(&self) -> CanonicalCommitAuthorityKind {
        self.authority_kind
    }

    pub(crate) fn published_lineage(&self) -> &PublishedLineageArtifact {
        &self.lineage
    }

    #[cfg(test)]
    pub(crate) fn published_lineage_mut_for_test(&mut self) -> &mut PublishedLineageArtifact {
        &mut self.lineage
    }

    pub fn append_index_generations_canonical(&mut self, generations: &[DerivedIndexGeneration]) {
        for generation in generations {
            if let Some(existing) = self
                .index_generations
                .iter_mut()
                .find(|candidate| candidate.generation_id == generation.generation_id)
            {
                *existing = generation.clone();
            } else {
                self.index_generations.push(generation.clone());
            }
        }
        self.index_generations.sort_by_key(|generation| {
            (generation.index_id.0, generation.generation_id.0)
        });
        self.index_generation_ids = self
            .index_generations
            .iter()
            .map(|generation| generation.generation_id.0)
            .collect();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayObservableSurface {
    Snapshot,
    Patch,
    Diagnostics,
    History,
    BranchHead,
    Lineage,
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
pub enum DescriptorAuthorityKind {
    SchemaTransitionArtifact,
    SchemaContinuationDescriptor,
    SchemaReconciliationDescriptor,
    SchemaLineageArtifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplaySurfaceAuthorityKind {
    Patch,
    Diagnostics,
    History,
    Snapshot,
    BranchHead,
    Lineage,
    DerivedIndexes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayAuthorityBasisKind {
    DurableLogCanonical,
    HistoryEnvelopeFallback,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayLineageAuthorityBasis {
    pub kind: ReplayAuthorityBasisKind,
    pub commit_id: CommitId,
    pub lineage_event_count: usize,
    pub lineage_decision_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifiedDescriptorDigest {
    pub kind: DescriptorAuthorityKind,
    pub digest: [u8; 32],
    pub descriptor_semantics_version: DescriptorSemanticsVersion,
    pub canonicalization_version: Option<DescriptorCanonicalizationVersion>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorComparisonBasis {
    pub kind: DescriptorAuthorityKind,
    pub exact_digest: Option<VerifiedDescriptorDigest>,
    pub summary_digest: Option<[u8; 32]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifiedReplaySurfaceDigest {
    pub kind: ReplaySurfaceAuthorityKind,
    pub digest: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaySurfaceComparisonBasis {
    pub kind: ReplaySurfaceAuthorityKind,
    pub exact_digest: Option<VerifiedReplaySurfaceDigest>,
    pub summary_digest: Option<[u8; 32]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DescriptorParityCheck {
    ExactDigestMatch {
        kind: DescriptorAuthorityKind,
    },
    SummaryMatchDigestUnavailable {
        kind: DescriptorAuthorityKind,
    },
    Drift {
        kind: DescriptorAuthorityKind,
        layer: ReplayVerificationLayer,
        mismatch_class: ReplayMismatchClass,
        detail: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplaySurfaceParityCheck {
    ExactDigestMatch {
        kind: ReplaySurfaceAuthorityKind,
    },
    SummaryMatchDigestUnavailable {
        kind: ReplaySurfaceAuthorityKind,
    },
    Drift {
        kind: ReplaySurfaceAuthorityKind,
        layer: ReplayVerificationLayer,
        mismatch_class: ReplayMismatchClass,
        detail: String,
    },
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayMismatchClass {
    PatchDrift,
    DiagnosticsDrift,
    HistoryDrift,
    SnapshotDrift,
    BranchHeadDrift,
    LineageDrift,
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
    pub commit: Option<CommitReference>,
    pub reconstructed_parent_chain: Vec<CommitId>,
    pub snapshot_version: Option<crate::identity::data::VersionId>,
    pub lineage_authority_basis: Option<ReplayLineageAuthorityBasis>,
    pub compared_surfaces: Vec<ReplayObservableSurface>,
    pub mismatches: Vec<ReplayMismatch>,
    pub failure: Option<ReplayFailureClass>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaySnapshotSurface {
    pub version_id: crate::identity::data::VersionId,
    pub entities: Vec<EntityReadRecord>,
    pub relations: Vec<RelationReadRecord>,
}

impl RelationalReplayOutcome {
    pub(crate) fn fail(
        requested: RelationalReplayRequest,
        envelope: Option<&CanonicalCommitEnvelope>,
        chain: Option<&[CommitId]>,
        failure: ReplayFailureClass,
    ) -> Self {
        let commit = envelope.map(|candidate| candidate.commit.clone());
        let reconstructed_parent_chain = chain
            .map(|resolved| resolved.to_vec())
            .or_else(|| envelope.map(|candidate| candidate.commit.parents.clone()))
            .unwrap_or_default();
        let snapshot_version = envelope.map(|candidate| candidate.commit.version_id);
        Self {
            requested,
            commit,
            reconstructed_parent_chain,
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
        self.failure = Some(ReplayFailureClass::ObservableMismatch);
        self
    }
}

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

impl VerifiedDescriptorDigest {
    pub fn new<T: Serialize + ?Sized>(
        kind: DescriptorAuthorityKind,
        descriptor_semantics_version: DescriptorSemanticsVersion,
        canonicalization_version: Option<DescriptorCanonicalizationVersion>,
        value: &T,
    ) -> Self {
        Self {
            kind,
            digest: stable_digest(value),
            descriptor_semantics_version,
            canonicalization_version,
        }
    }
}

impl VerifiedReplaySurfaceDigest {
    pub fn new<T: Serialize + ?Sized>(kind: ReplaySurfaceAuthorityKind, value: &T) -> Self {
        Self {
            kind,
            digest: stable_digest(value),
        }
    }
}

impl DescriptorComparisonBasis {
    pub fn new(
        kind: DescriptorAuthorityKind,
        exact_digest: Option<VerifiedDescriptorDigest>,
        summary_digest: Option<[u8; 32]>,
    ) -> Self {
        Self {
            kind,
            exact_digest,
            summary_digest,
        }
    }

    pub fn compare(
        &self,
        other: &Self,
        mismatch_class: ReplayMismatchClass,
        detail: impl Into<String>,
    ) -> DescriptorParityCheck {
        let detail = detail.into();
        if self.kind != other.kind {
            return DescriptorParityCheck::Drift {
                kind: self.kind,
                layer: ReplayVerificationLayer::DigestParity,
                mismatch_class,
                detail,
            };
        }
        match (&self.exact_digest, &other.exact_digest) {
            (Some(expected), Some(observed)) if expected == observed => {
                return DescriptorParityCheck::ExactDigestMatch { kind: self.kind }
            }
            (None, None) => {}
            _ => {
                return DescriptorParityCheck::Drift {
                    kind: self.kind,
                    layer: ReplayVerificationLayer::DigestParity,
                    mismatch_class,
                    detail,
                }
            }
        }
        match (self.summary_digest, other.summary_digest) {
            (Some(expected), Some(observed)) if expected == observed => {
                DescriptorParityCheck::SummaryMatchDigestUnavailable { kind: self.kind }
            }
            _ => DescriptorParityCheck::Drift {
                kind: self.kind,
                layer: ReplayVerificationLayer::SummaryParity,
                mismatch_class,
                detail,
            },
        }
    }
}

impl ReplaySurfaceComparisonBasis {
    pub fn new(
        kind: ReplaySurfaceAuthorityKind,
        exact_digest: Option<VerifiedReplaySurfaceDigest>,
        summary_digest: Option<[u8; 32]>,
    ) -> Self {
        Self {
            kind,
            exact_digest,
            summary_digest,
        }
    }

    pub fn compare(
        &self,
        other: &Self,
        mismatch_class: ReplayMismatchClass,
        detail: impl Into<String>,
    ) -> ReplaySurfaceParityCheck {
        let detail = detail.into();
        if self.kind != other.kind {
            return ReplaySurfaceParityCheck::Drift {
                kind: self.kind,
                layer: ReplayVerificationLayer::DigestParity,
                mismatch_class,
                detail,
            };
        }
        match (&self.exact_digest, &other.exact_digest) {
            (Some(expected), Some(observed)) if expected == observed => {
                return ReplaySurfaceParityCheck::ExactDigestMatch { kind: self.kind }
            }
            (None, None) => {}
            _ => {
                return ReplaySurfaceParityCheck::Drift {
                    kind: self.kind,
                    layer: ReplayVerificationLayer::DigestParity,
                    mismatch_class,
                    detail,
                };
            }
        }
        match (self.summary_digest, other.summary_digest) {
            (Some(expected), Some(observed)) if expected == observed => {
                ReplaySurfaceParityCheck::SummaryMatchDigestUnavailable { kind: self.kind }
            }
            _ => ReplaySurfaceParityCheck::Drift {
                kind: self.kind,
                layer: ReplayVerificationLayer::SummaryParity,
                mismatch_class,
                detail,
            },
        }
    }
}

pub(crate) fn stable_digest<T: Serialize + ?Sized>(value: &T) -> [u8; 32] {
    let bytes = serde_json::to_vec(value).expect("serializable replay authority input");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut out = [0_u8; 32];
    out.copy_from_slice(&digest);
    out
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaySchemaVersion(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalReplayRecord {
    pub schema_version: ReplaySchemaVersion,
    pub commit_id: crate::history::data::CommitId,
    pub version_id: crate::identity::data::VersionId,
    pub snapshot_id: crate::snapshots::data::SnapshotId,
    pub patch: crate::publication::data::diff::RelationalPatchRecord,
    pub schema_registry: RelationalSchemaRegistry,
}
