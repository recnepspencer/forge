use super::super::artifacts::{S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind};
use super::super::claims::SemanticPhysicalClaimStatus;
use super::super::evidence::S0EvidenceRef;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TestMigrationNoteRow {
    row_id: S0ArtifactRowId,
    subject_kind: S0ArtifactSubjectKind,
    subject_path_or_symbol: String,
    classification: String,
    evidence_refs: Vec<S0EvidenceRef>,
    forbidden_claims: Vec<super::super::capability::BackendForbiddenClaim>,
    deferred_s_sequences: Vec<super::super::capability::Roadmap2SequenceId>,
    status: S0ArtifactRowStatus,
    notes: String,
    named_suite: String,
    evidence_scope: SemanticPhysicalClaimStatus,
    required_followup_guarantees: Vec<String>,
}

impl TestMigrationNoteRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        row_id: S0ArtifactRowId,
        subject_path_or_symbol: impl Into<String>,
        evidence_refs: Vec<S0EvidenceRef>,
        forbidden_claims: Vec<super::super::capability::BackendForbiddenClaim>,
        deferred_s_sequences: Vec<super::super::capability::Roadmap2SequenceId>,
        status: S0ArtifactRowStatus,
        notes: impl Into<String>,
        named_suite: impl Into<String>,
        evidence_scope: SemanticPhysicalClaimStatus,
        required_followup_guarantees: Vec<String>,
    ) -> Result<Self, super::validation::S0TestMigrationBuildRejection> {
        let subject_path_or_symbol = super::validation::require_non_empty(subject_path_or_symbol)?;
        let notes = super::validation::require_non_empty(notes)?;
        let named_suite = super::validation::require_non_empty(named_suite)?;
        if evidence_refs.is_empty() {
            return Err(super::validation::S0TestMigrationBuildRejection::MissingEvidenceRef);
        }
        if required_followup_guarantees.is_empty()
            && evidence_scope != SemanticPhysicalClaimStatus::PlatformGrade
        {
            return Err(
                super::validation::S0TestMigrationBuildRejection::MissingRequiredFollowupGuarantee,
            );
        }
        Ok(Self {
            row_id,
            subject_kind: S0ArtifactSubjectKind::TestSuite,
            subject_path_or_symbol,
            classification: "test-migration-note".to_string(),
            evidence_refs,
            forbidden_claims,
            deferred_s_sequences,
            status,
            notes,
            named_suite,
            evidence_scope,
            required_followup_guarantees,
        })
    }

    pub fn row_id(&self) -> &S0ArtifactRowId {
        &self.row_id
    }

    pub fn evidence_scope(&self) -> SemanticPhysicalClaimStatus {
        self.evidence_scope
    }
}
