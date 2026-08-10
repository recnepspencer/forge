use super::super::capability::{
    BackendForbiddenClaim, Roadmap2SequenceId, StoreBackendCapabilityTier,
};
use super::super::evidence::{S0EvidenceRef, S0StableDigest};
use super::artifact_envelope::{S0ArtifactEnvelopeMetadata, S0NondeterministicMetadata};
use super::artifact_validation::{
    backend_capability_matrix_digest, reject_duplicate_rows, reject_missing_first_audit_rows,
    require_non_empty, S0ArtifactBuildRejection,
};
use super::first_audit_baseline::first_audit_baseline_rows;
use super::row_identity::{S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct BackendCapabilityMatrixRow {
    row_id: S0ArtifactRowId,
    subject_kind: S0ArtifactSubjectKind,
    subject_path_or_symbol: String,
    classification: String,
    evidence_refs: Vec<S0EvidenceRef>,
    forbidden_claims: Vec<BackendForbiddenClaim>,
    deferred_s_sequences: Vec<Roadmap2SequenceId>,
    status: S0ArtifactRowStatus,
    notes: String,
    capability_tier: StoreBackendCapabilityTier,
    valid_use: String,
    required_evidence_before_promotion: Vec<String>,
    known_semantic_guarantees: Vec<String>,
    known_physical_gaps: Vec<String>,
}

impl BackendCapabilityMatrixRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        row_id: S0ArtifactRowId,
        subject_kind: S0ArtifactSubjectKind,
        subject_path_or_symbol: impl Into<String>,
        classification: impl Into<String>,
        evidence_refs: Vec<S0EvidenceRef>,
        forbidden_claims: Vec<BackendForbiddenClaim>,
        deferred_s_sequences: Vec<Roadmap2SequenceId>,
        status: S0ArtifactRowStatus,
        notes: impl Into<String>,
        capability_tier: StoreBackendCapabilityTier,
        valid_use: impl Into<String>,
        required_evidence_before_promotion: Vec<String>,
        known_semantic_guarantees: Vec<String>,
        known_physical_gaps: Vec<String>,
    ) -> Result<Self, S0ArtifactBuildRejection> {
        let subject_path_or_symbol =
            require_non_empty("subject_path_or_symbol", subject_path_or_symbol)?;
        let classification = require_non_empty("classification", classification)?;
        let notes = require_non_empty("notes", notes)?;
        let valid_use = require_non_empty("valid_use", valid_use)?;
        if evidence_refs.is_empty() {
            return Err(S0ArtifactBuildRejection::MissingEvidenceRef);
        }
        if status != S0ArtifactRowStatus::Admitted && forbidden_claims.is_empty() {
            return Err(S0ArtifactBuildRejection::ForbiddenClaimsMissing);
        }
        if !known_physical_gaps.is_empty() && deferred_s_sequences.is_empty() {
            return Err(S0ArtifactBuildRejection::DeferredSequenceMissing);
        }
        Ok(Self {
            row_id,
            subject_kind,
            subject_path_or_symbol,
            classification,
            evidence_refs,
            forbidden_claims,
            deferred_s_sequences,
            status,
            notes,
            capability_tier,
            valid_use,
            required_evidence_before_promotion,
            known_semantic_guarantees,
            known_physical_gaps,
        })
    }

    pub fn row_id(&self) -> &S0ArtifactRowId {
        &self.row_id
    }

    pub fn subject_path_or_symbol(&self) -> &str {
        &self.subject_path_or_symbol
    }

    pub fn evidence_refs(&self) -> &[S0EvidenceRef] {
        &self.evidence_refs
    }

    pub fn forbidden_claims(&self) -> &[BackendForbiddenClaim] {
        &self.forbidden_claims
    }

    pub fn deferred_s_sequences(&self) -> &[Roadmap2SequenceId] {
        &self.deferred_s_sequences
    }

    pub fn capability_tier(&self) -> StoreBackendCapabilityTier {
        self.capability_tier
    }

    pub fn status(&self) -> S0ArtifactRowStatus {
        self.status
    }

    pub fn known_physical_gaps(&self) -> &[String] {
        &self.known_physical_gaps
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct BackendCapabilityMatrix {
    #[serde(flatten)]
    envelope: S0ArtifactEnvelopeMetadata,
    rows: Vec<BackendCapabilityMatrixRow>,
}

impl BackendCapabilityMatrix {
    pub fn new(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        mut rows: Vec<BackendCapabilityMatrixRow>,
    ) -> Result<Self, S0ArtifactBuildRejection> {
        let source_revision = require_non_empty("source_revision", source_revision)?;
        let generated_by = require_non_empty("generated_by", generated_by)?;
        rows.sort_by(|left, right| left.row_id.cmp(&right.row_id));
        reject_duplicate_rows(&rows)?;
        reject_missing_first_audit_rows(&rows)?;
        let deterministic_digest = backend_capability_matrix_digest(
            &source_revision,
            &roadmap_parent_digest,
            &generated_by,
            &rows,
        )?;
        let envelope = S0ArtifactEnvelopeMetadata::new(
            super::super::evidence::S0ArtifactKind::BackendCapabilityMatrix,
            source_revision,
            roadmap_parent_digest,
            generated_by,
            deterministic_digest,
            nondeterministic_metadata,
        );
        Ok(Self { envelope, rows })
    }

    pub fn first_audit_baseline(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
    ) -> Result<Self, S0ArtifactBuildRejection> {
        Self::new(
            source_revision,
            roadmap_parent_digest,
            generated_by,
            nondeterministic_metadata,
            first_audit_baseline_rows(),
        )
    }

    pub fn envelope(&self) -> &S0ArtifactEnvelopeMetadata {
        &self.envelope
    }

    pub fn rows(&self) -> &[BackendCapabilityMatrixRow] {
        &self.rows
    }
}
