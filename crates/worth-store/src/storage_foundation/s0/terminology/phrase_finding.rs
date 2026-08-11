use super::super::artifacts::{S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind};
use super::super::capability::Roadmap2SequenceId;
use super::super::evidence::S0EvidenceRef;
use super::phrase_policy::{
    required_qualifier, TerminologyAllowedUse, TerminologyRequiredQualifier,
};
use super::validation::TerminologyCleanupRejection;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TerminologyPhraseFinding {
    pub(super) row_id: S0ArtifactRowId,
    pub(super) subject_kind: S0ArtifactSubjectKind,
    pub(super) subject_path_or_symbol: String,
    pub(super) classification: String,
    pub(super) evidence_refs: Vec<S0EvidenceRef>,
    pub(super) forbidden_claims: Vec<super::super::capability::BackendForbiddenClaim>,
    pub(super) deferred_s_sequences: Vec<Roadmap2SequenceId>,
    pub(super) status: S0ArtifactRowStatus,
    pub(super) notes: String,
    pub(super) phrase: String,
    pub(super) line_number: u64,
    pub(super) line_excerpt: String,
    pub(super) allowed_use: TerminologyAllowedUse,
    pub(super) required_qualifier: TerminologyRequiredQualifier,
}

impl TerminologyPhraseFinding {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        row_id: S0ArtifactRowId,
        subject_path_or_symbol: impl Into<String>,
        evidence_refs: Vec<S0EvidenceRef>,
        deferred_s_sequences: Vec<Roadmap2SequenceId>,
        status: S0ArtifactRowStatus,
        notes: impl Into<String>,
        phrase: impl Into<String>,
        line_number: u64,
        line_excerpt: impl Into<String>,
        allowed_use: TerminologyAllowedUse,
    ) -> Result<Self, TerminologyCleanupRejection> {
        let subject_path_or_symbol = subject_path_or_symbol.into();
        let notes = notes.into();
        let phrase = phrase.into().to_ascii_lowercase();
        let line_excerpt = line_excerpt.into();
        if subject_path_or_symbol.trim().is_empty()
            || notes.trim().is_empty()
            || phrase.trim().is_empty()
            || line_excerpt.trim().is_empty()
        {
            return Err(TerminologyCleanupRejection::EmptyRequiredField);
        }
        if line_number == 0 {
            return Err(TerminologyCleanupRejection::InvalidLineNumber);
        }
        if evidence_refs.is_empty() {
            return Err(TerminologyCleanupRejection::MissingEvidenceRef);
        }
        let required_qualifier = required_qualifier(&allowed_use);
        if matches!(
            allowed_use,
            TerminologyAllowedUse::QualifiedPhysicalDebt { .. }
        ) && deferred_s_sequences.is_empty()
        {
            return Err(TerminologyCleanupRejection::QualifiedPhysicalDebtMissingSequence);
        }
        Ok(Self {
            row_id,
            subject_kind: S0ArtifactSubjectKind::ClaimSurface,
            subject_path_or_symbol,
            classification: "terminology-risk".to_string(),
            evidence_refs,
            forbidden_claims: Vec::new(),
            deferred_s_sequences,
            status,
            notes,
            phrase,
            line_number,
            line_excerpt,
            allowed_use,
            required_qualifier,
        })
    }

    pub fn row_id(&self) -> &S0ArtifactRowId {
        &self.row_id
    }

    pub fn allowed_use(&self) -> &TerminologyAllowedUse {
        &self.allowed_use
    }

    pub fn evidence_refs(&self) -> &[S0EvidenceRef] {
        &self.evidence_refs
    }

    pub fn subject_path_or_symbol(&self) -> &str {
        &self.subject_path_or_symbol
    }

    pub fn line_number(&self) -> u64 {
        self.line_number
    }
}
