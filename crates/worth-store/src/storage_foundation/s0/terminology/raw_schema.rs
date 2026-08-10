use super::super::artifacts::{S0ArtifactRowId, S0ArtifactRowStatus};
use super::super::capability::Roadmap2SequenceId;
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use super::phrase_finding::TerminologyPhraseFinding;
use super::phrase_policy::TerminologyAllowedUse;
use super::validation::TerminologyCleanupRejection;

#[derive(serde::Deserialize)]
pub(super) struct RawTerminologyRiskReport {
    #[serde(flatten)]
    pub(super) envelope: RawS0ArtifactEnvelope,
    pub(super) rows: Vec<RawTerminologyPhraseFinding>,
    pub(super) scan_digest: String,
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
    ) -> Result<super::super::artifacts::S0NondeterministicMetadata, TerminologyCleanupRejection>
    {
        super::super::artifacts::S0NondeterministicMetadata::excluded(
            self.generated_at_policy,
            self.local_path_hint,
            self.host_hint,
        )
        .map_err(|_| TerminologyCleanupRejection::EmptyRequiredField)
    }
}

#[derive(serde::Deserialize)]
pub(super) struct RawTerminologyPhraseFinding {
    row_id: String,
    subject_path_or_symbol: String,
    evidence_refs: Vec<RawS0EvidenceRef>,
    deferred_s_sequences: Vec<String>,
    status: S0ArtifactRowStatus,
    notes: String,
    phrase: String,
    line_number: u64,
    line_excerpt: String,
    allowed_use: RawTerminologyAllowedUse,
}

impl RawTerminologyPhraseFinding {
    pub(super) fn into_validated(
        self,
    ) -> Result<TerminologyPhraseFinding, TerminologyCleanupRejection> {
        let row_id = S0ArtifactRowId::new(self.row_id)
            .map_err(|_| TerminologyCleanupRejection::EmptyRequiredField)?;
        let evidence_refs = self
            .evidence_refs
            .into_iter()
            .map(RawS0EvidenceRef::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let deferred_s_sequences = self
            .deferred_s_sequences
            .into_iter()
            .map(|sequence| {
                Roadmap2SequenceId::new(sequence)
                    .map_err(|_| TerminologyCleanupRejection::InvalidDeferredSequence)
            })
            .collect::<Result<Vec<_>, _>>()?;
        TerminologyPhraseFinding::new(
            row_id,
            self.subject_path_or_symbol,
            evidence_refs,
            deferred_s_sequences,
            self.status,
            self.notes,
            self.phrase,
            self.line_number,
            self.line_excerpt,
            self.allowed_use.into_validated()?,
        )
    }
}

#[derive(serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum RawTerminologyAllowedUse {
    AllowedSemanticUse,
    QualifiedPhysicalDebt {
        deferred_sequence: String,
    },
    ClosedFoundationEvidence {
        artifact_kind: S0ArtifactKind,
        digest: String,
    },
    OverclaimedPhysicalPosture,
}

impl RawTerminologyAllowedUse {
    fn into_validated(self) -> Result<TerminologyAllowedUse, TerminologyCleanupRejection> {
        match self {
            Self::AllowedSemanticUse => Ok(TerminologyAllowedUse::AllowedSemanticUse),
            Self::QualifiedPhysicalDebt { deferred_sequence } => {
                Ok(TerminologyAllowedUse::QualifiedPhysicalDebt {
                    deferred_sequence: Roadmap2SequenceId::new(deferred_sequence)
                        .map_err(|_| TerminologyCleanupRejection::InvalidDeferredSequence)?,
                })
            }
            Self::ClosedFoundationEvidence {
                artifact_kind,
                digest,
            } => Ok(TerminologyAllowedUse::ClosedFoundationEvidence {
                evidence_ref: S0EvidenceRef::new(
                    artifact_kind,
                    S0StableDigest::new(digest)
                        .map_err(|_| TerminologyCleanupRejection::InvalidDigest)?,
                ),
            }),
            Self::OverclaimedPhysicalPosture => {
                Ok(TerminologyAllowedUse::OverclaimedPhysicalPosture)
            }
        }
    }
}

#[derive(serde::Deserialize)]
struct RawS0EvidenceRef {
    artifact_kind: S0ArtifactKind,
    digest: String,
}

impl RawS0EvidenceRef {
    fn into_validated(self) -> Result<S0EvidenceRef, TerminologyCleanupRejection> {
        let digest = S0StableDigest::new(self.digest)
            .map_err(|_| TerminologyCleanupRejection::InvalidDigest)?;
        Ok(S0EvidenceRef::new(self.artifact_kind, digest))
    }
}
