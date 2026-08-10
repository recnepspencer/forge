use super::super::capability::{
    BackendForbiddenClaim, BackendForbiddenClaimKind, Roadmap2SequenceId,
    S0ClaimPromotionRejection, StoreBackendCapabilityTier,
};
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use super::artifact_envelope::{S0NondeterministicMetadata, S0_ARTIFACT_SCHEMA_VERSION};
use super::artifact_validation::{
    S0ArtifactParseRejection, S0ValidatedBackendCapabilityMatrixArtifact,
};
use super::capability_matrix::{BackendCapabilityMatrix, BackendCapabilityMatrixRow};
use super::row_identity::{S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind};
use serde::Deserialize;

impl BackendCapabilityMatrix {
    pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, S0ArtifactParseRejection> {
        serde_json::to_vec_pretty(self).map_err(|_| S0ArtifactParseRejection::SerializationFailed)
    }

    pub fn validate_canonical_json_bytes(
        bytes: &[u8],
    ) -> Result<S0ValidatedBackendCapabilityMatrixArtifact, S0ArtifactParseRejection> {
        let raw = serde_json::from_slice::<RawBackendCapabilityMatrix>(bytes)
            .map_err(|_| S0ArtifactParseRejection::NonParseable)?;
        if raw.envelope.schema_version != S0_ARTIFACT_SCHEMA_VERSION {
            return Err(S0ArtifactParseRejection::SchemaVersionMismatch);
        }
        if raw.envelope.artifact_kind != S0ArtifactKind::BackendCapabilityMatrix {
            return Err(S0ArtifactParseRejection::ArtifactKindMismatch);
        }

        let roadmap_parent_digest = S0StableDigest::new(raw.envelope.roadmap_parent_digest)
            .map_err(|_| S0ArtifactParseRejection::InvalidDigest)?;
        let expected_digest = S0StableDigest::new(raw.envelope.deterministic_digest)
            .map_err(|_| S0ArtifactParseRejection::InvalidDigest)?;
        let rows = raw
            .rows
            .into_iter()
            .map(RawBackendCapabilityMatrixRow::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let matrix = Self::new(
            raw.envelope.source_revision,
            roadmap_parent_digest,
            raw.envelope.generated_by,
            raw.envelope.nondeterministic_metadata.into_validated()?,
            rows,
        )?;
        if matrix.envelope().deterministic_digest() != &expected_digest {
            return Err(S0ArtifactParseRejection::DeterministicDigestMismatch);
        }
        let canonicalized_row_byte_count = serde_json::to_vec(matrix.rows())
            .map_err(|_| S0ArtifactParseRejection::SerializationFailed)?
            .len() as u64;
        let validation_cost = super::artifact_envelope::S0ArtifactValidationCostSurface::new(
            bytes.len() as u64,
            matrix.rows().len() as u64,
            canonicalized_row_byte_count,
            matrix.rows().len() as u64,
        );
        Ok(S0ValidatedBackendCapabilityMatrixArtifact {
            matrix,
            validation_cost,
        })
    }
}

#[derive(Deserialize)]
struct RawBackendCapabilityMatrix {
    #[serde(flatten)]
    envelope: RawS0ArtifactEnvelope,
    rows: Vec<RawBackendCapabilityMatrixRow>,
}

#[derive(Deserialize)]
struct RawS0ArtifactEnvelope {
    schema_version: String,
    artifact_kind: S0ArtifactKind,
    source_revision: String,
    roadmap_parent_digest: String,
    generated_by: String,
    deterministic_digest: String,
    nondeterministic_metadata: RawS0NondeterministicMetadata,
}

#[derive(Deserialize)]
struct RawS0NondeterministicMetadata {
    generated_at_policy: String,
    local_path_hint: Option<String>,
    host_hint: Option<String>,
}

impl RawS0NondeterministicMetadata {
    fn into_validated(self) -> Result<S0NondeterministicMetadata, S0ArtifactParseRejection> {
        S0NondeterministicMetadata::excluded(
            self.generated_at_policy,
            self.local_path_hint,
            self.host_hint,
        )
        .map_err(S0ArtifactParseRejection::RowBuildRejected)
    }
}

#[derive(Deserialize)]
struct RawBackendCapabilityMatrixRow {
    row_id: String,
    subject_kind: S0ArtifactSubjectKind,
    subject_path_or_symbol: String,
    classification: String,
    evidence_refs: Vec<RawS0EvidenceRef>,
    forbidden_claims: Vec<RawBackendForbiddenClaim>,
    deferred_s_sequences: Vec<String>,
    status: S0ArtifactRowStatus,
    notes: String,
    capability_tier: StoreBackendCapabilityTier,
    valid_use: String,
    required_evidence_before_promotion: Vec<String>,
    known_semantic_guarantees: Vec<String>,
    known_physical_gaps: Vec<String>,
}

impl RawBackendCapabilityMatrixRow {
    fn into_validated(self) -> Result<BackendCapabilityMatrixRow, S0ArtifactParseRejection> {
        let row_id = S0ArtifactRowId::new(self.row_id)
            .map_err(S0ArtifactParseRejection::RowBuildRejected)?;
        let evidence_refs = self
            .evidence_refs
            .into_iter()
            .map(RawS0EvidenceRef::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let forbidden_claims = self
            .forbidden_claims
            .into_iter()
            .map(RawBackendForbiddenClaim::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let deferred_s_sequences = self
            .deferred_s_sequences
            .into_iter()
            .map(|sequence| {
                Roadmap2SequenceId::new(sequence)
                    .map_err(|_| S0ArtifactParseRejection::InvalidDeferredSequence)
            })
            .collect::<Result<Vec<_>, _>>()?;
        BackendCapabilityMatrixRow::new(
            row_id,
            self.subject_kind,
            self.subject_path_or_symbol,
            self.classification,
            evidence_refs,
            forbidden_claims,
            deferred_s_sequences,
            self.status,
            self.notes,
            self.capability_tier,
            self.valid_use,
            self.required_evidence_before_promotion,
            self.known_semantic_guarantees,
            self.known_physical_gaps,
        )
        .map_err(S0ArtifactParseRejection::RowBuildRejected)
    }
}

#[derive(Deserialize)]
struct RawS0EvidenceRef {
    artifact_kind: S0ArtifactKind,
    digest: String,
}

impl RawS0EvidenceRef {
    fn into_validated(self) -> Result<S0EvidenceRef, S0ArtifactParseRejection> {
        let digest = S0StableDigest::new(self.digest)
            .map_err(|_| S0ArtifactParseRejection::InvalidDigest)?;
        Ok(S0EvidenceRef::new(self.artifact_kind, digest))
    }
}

#[derive(Deserialize)]
struct RawBackendForbiddenClaim {
    claim_kind: BackendForbiddenClaimKind,
    deferred_sequence: String,
}

impl RawBackendForbiddenClaim {
    fn into_validated(self) -> Result<BackendForbiddenClaim, S0ArtifactParseRejection> {
        BackendForbiddenClaim::new(self.claim_kind, self.deferred_sequence).map_err(|error| {
            if error == S0ClaimPromotionRejection::MissingSequenceMapping {
                S0ArtifactParseRejection::InvalidDeferredSequence
            } else {
                S0ArtifactParseRejection::InvalidForbiddenClaim
            }
        })
    }
}
