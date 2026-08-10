use super::super::artifacts::{S0ArtifactRowId, S0ArtifactRowStatus};
use super::super::capability::{
    BackendForbiddenClaim, BackendForbiddenClaimKind, Roadmap2SequenceId,
};
use super::super::claims::SemanticPhysicalClaimStatus;
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use super::test_migration_note_row::TestMigrationNoteRow;
use super::validation::{S0TestMigrationBuildRejection, S0TestMigrationParseRejection};

#[derive(serde::Deserialize)]
pub(super) struct RawTestMigrationNotes {
    #[serde(flatten)]
    pub(super) envelope: RawS0ArtifactEnvelope,
    pub(super) rows: Vec<RawTestMigrationNoteRow>,
}

#[derive(serde::Deserialize)]
pub(super) struct RawS0ArtifactEnvelope {
    pub(super) schema_version: String,
    pub(super) artifact_kind: S0ArtifactKind,
    pub(super) source_revision: String,
    pub(super) roadmap_parent_digest: String,
    pub(super) generated_by: String,
    pub(super) deterministic_digest: String,
    pub(super) nondeterministic_metadata: RawS0NondeterministicMetadata,
}

#[derive(serde::Deserialize)]
pub(super) struct RawS0NondeterministicMetadata {
    generated_at_policy: String,
    local_path_hint: Option<String>,
    host_hint: Option<String>,
}

impl RawS0NondeterministicMetadata {
    pub(super) fn into_validated(
        self,
    ) -> Result<super::super::artifacts::S0NondeterministicMetadata, S0TestMigrationParseRejection>
    {
        super::super::artifacts::S0NondeterministicMetadata::excluded(
            self.generated_at_policy,
            self.local_path_hint,
            self.host_hint,
        )
        .map_err(|_| {
            S0TestMigrationParseRejection::RowBuildRejected(
                S0TestMigrationBuildRejection::EmptyRequiredField,
            )
        })
    }
}

#[derive(serde::Deserialize)]
pub(super) struct RawTestMigrationNoteRow {
    row_id: String,
    subject_path_or_symbol: String,
    evidence_refs: Vec<RawS0EvidenceRef>,
    forbidden_claims: Vec<RawBackendForbiddenClaim>,
    deferred_s_sequences: Vec<String>,
    status: S0ArtifactRowStatus,
    notes: String,
    named_suite: String,
    evidence_scope: SemanticPhysicalClaimStatus,
    required_followup_guarantees: Vec<String>,
}

impl RawTestMigrationNoteRow {
    pub(super) fn into_validated(
        self,
    ) -> Result<TestMigrationNoteRow, S0TestMigrationParseRejection> {
        let row_id = S0ArtifactRowId::new(self.row_id).map_err(|_| {
            S0TestMigrationParseRejection::RowBuildRejected(
                S0TestMigrationBuildRejection::EmptyRequiredField,
            )
        })?;
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
                    .map_err(|_| S0TestMigrationParseRejection::InvalidDeferredSequence)
            })
            .collect::<Result<Vec<_>, _>>()?;
        TestMigrationNoteRow::new(
            row_id,
            self.subject_path_or_symbol,
            evidence_refs,
            forbidden_claims,
            deferred_s_sequences,
            self.status,
            self.notes,
            self.named_suite,
            self.evidence_scope,
            self.required_followup_guarantees,
        )
        .map_err(S0TestMigrationParseRejection::RowBuildRejected)
    }
}

#[derive(serde::Deserialize)]
struct RawS0EvidenceRef {
    artifact_kind: S0ArtifactKind,
    digest: String,
}

impl RawS0EvidenceRef {
    fn into_validated(self) -> Result<S0EvidenceRef, S0TestMigrationParseRejection> {
        let digest = S0StableDigest::new(self.digest)
            .map_err(|_| S0TestMigrationParseRejection::InvalidDigest)?;
        Ok(S0EvidenceRef::new(self.artifact_kind, digest))
    }
}

#[derive(serde::Deserialize)]
struct RawBackendForbiddenClaim {
    claim_kind: BackendForbiddenClaimKind,
    deferred_sequence: String,
}

impl RawBackendForbiddenClaim {
    fn into_validated(self) -> Result<BackendForbiddenClaim, S0TestMigrationParseRejection> {
        BackendForbiddenClaim::new(self.claim_kind, self.deferred_sequence)
            .map_err(|_| S0TestMigrationParseRejection::InvalidDeferredSequence)
    }
}
