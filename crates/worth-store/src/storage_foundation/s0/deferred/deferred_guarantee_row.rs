use super::super::artifacts::{S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind};
use super::super::capability::{BackendForbiddenClaim, Roadmap2SequenceId};
use super::super::evidence::S0EvidenceRef;
use super::super::milestones::S0PhysicalStatus;
use super::deferred_category_policy::DeferredPhysicalGuaranteeCategory;
use super::deferred_validation::{require_non_empty, S0DeferredGuaranteeBuildRejection};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct DeferredPhysicalGuaranteeRow {
    pub(super) row_id: S0ArtifactRowId,
    pub(super) subject_kind: S0ArtifactSubjectKind,
    pub(super) subject_path_or_symbol: String,
    pub(super) classification: String,
    pub(super) evidence_refs: Vec<S0EvidenceRef>,
    pub(super) forbidden_claims: Vec<BackendForbiddenClaim>,
    pub(super) deferred_s_sequences: Vec<Roadmap2SequenceId>,
    pub(super) status: S0ArtifactRowStatus,
    pub(super) notes: String,
    pub(super) guarantee_category: DeferredPhysicalGuaranteeCategory,
    pub(super) current_evidence_status: S0PhysicalStatus,
    pub(super) missing_proof: String,
    pub(super) dependent_named_suite: String,
    pub(super) dependent_evidence_lanes: Vec<String>,
}

impl DeferredPhysicalGuaranteeRow {
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
        guarantee_category: DeferredPhysicalGuaranteeCategory,
        current_evidence_status: S0PhysicalStatus,
        missing_proof: impl Into<String>,
        dependent_named_suite: impl Into<String>,
        dependent_evidence_lanes: Vec<String>,
    ) -> Result<Self, S0DeferredGuaranteeBuildRejection> {
        let text = validate_row_text(DeferredPhysicalGuaranteeRowTextInput {
            subject_path_or_symbol: subject_path_or_symbol.into(),
            classification: classification.into(),
            notes: notes.into(),
            missing_proof: missing_proof.into(),
            dependent_named_suite: dependent_named_suite.into(),
        })?;
        validate_required_row_collections(
            &evidence_refs,
            &dependent_evidence_lanes,
            &deferred_s_sequences,
        )?;
        validate_guarantee_status(
            current_evidence_status,
            guarantee_category,
            &deferred_s_sequences,
        )?;
        Ok(construct_deferred_guarantee_row(
            DeferredPhysicalGuaranteeRowConstruction {
                row_id,
                subject_kind,
                text,
                evidence_refs,
                forbidden_claims,
                deferred_s_sequences,
                status,
                guarantee_category,
                current_evidence_status,
                dependent_evidence_lanes,
            },
        ))
    }

    pub fn row_id(&self) -> &S0ArtifactRowId {
        &self.row_id
    }
}

struct DeferredPhysicalGuaranteeRowTextInput<Subject, Classification, Notes, Proof, Suite> {
    subject_path_or_symbol: Subject,
    classification: Classification,
    notes: Notes,
    missing_proof: Proof,
    dependent_named_suite: Suite,
}

struct DeferredPhysicalGuaranteeRowText {
    subject_path_or_symbol: String,
    classification: String,
    notes: String,
    missing_proof: String,
    dependent_named_suite: String,
}

fn validate_row_text<Subject, Classification, Notes, Proof, Suite>(
    input: DeferredPhysicalGuaranteeRowTextInput<Subject, Classification, Notes, Proof, Suite>,
) -> Result<DeferredPhysicalGuaranteeRowText, S0DeferredGuaranteeBuildRejection>
where
    Subject: Into<String>,
    Classification: Into<String>,
    Notes: Into<String>,
    Proof: Into<String>,
    Suite: Into<String>,
{
    Ok(DeferredPhysicalGuaranteeRowText {
        subject_path_or_symbol: require_non_empty(input.subject_path_or_symbol)?,
        classification: require_non_empty(input.classification)?,
        notes: require_non_empty(input.notes)?,
        missing_proof: require_non_empty(input.missing_proof)?,
        dependent_named_suite: require_non_empty(input.dependent_named_suite)?,
    })
}

fn validate_required_row_collections(
    evidence_refs: &[S0EvidenceRef],
    dependent_evidence_lanes: &[String],
    deferred_s_sequences: &[Roadmap2SequenceId],
) -> Result<(), S0DeferredGuaranteeBuildRejection> {
    if evidence_refs.is_empty() {
        return Err(S0DeferredGuaranteeBuildRejection::MissingEvidenceRef);
    }
    if dependent_evidence_lanes.is_empty() {
        return Err(S0DeferredGuaranteeBuildRejection::MissingEvidenceLane);
    }
    if deferred_s_sequences.is_empty() {
        return Err(S0DeferredGuaranteeBuildRejection::DeferredSequenceMissing);
    }
    Ok(())
}

fn validate_guarantee_status(
    current_evidence_status: S0PhysicalStatus,
    guarantee_category: DeferredPhysicalGuaranteeCategory,
    deferred_s_sequences: &[Roadmap2SequenceId],
) -> Result<(), S0DeferredGuaranteeBuildRejection> {
    if matches!(
        current_evidence_status,
        S0PhysicalStatus::FoundationBacked
            | S0PhysicalStatus::PlatformGrade
            | S0PhysicalStatus::NotApplicable
    ) {
        return Err(S0DeferredGuaranteeBuildRejection::GuaranteeAlreadySatisfied);
    }
    if !guarantee_category
        .minimum_required_sequences()
        .iter()
        .any(|required| {
            deferred_s_sequences
                .iter()
                .any(|sequence| sequence.as_str() == *required)
        })
    {
        return Err(S0DeferredGuaranteeBuildRejection::GuaranteeCategorySequenceMismatch);
    }
    Ok(())
}

struct DeferredPhysicalGuaranteeRowConstruction {
    row_id: S0ArtifactRowId,
    subject_kind: S0ArtifactSubjectKind,
    text: DeferredPhysicalGuaranteeRowText,
    evidence_refs: Vec<S0EvidenceRef>,
    forbidden_claims: Vec<BackendForbiddenClaim>,
    deferred_s_sequences: Vec<Roadmap2SequenceId>,
    status: S0ArtifactRowStatus,
    guarantee_category: DeferredPhysicalGuaranteeCategory,
    current_evidence_status: S0PhysicalStatus,
    dependent_evidence_lanes: Vec<String>,
}

fn construct_deferred_guarantee_row(
    input: DeferredPhysicalGuaranteeRowConstruction,
) -> DeferredPhysicalGuaranteeRow {
    let DeferredPhysicalGuaranteeRowConstruction {
        row_id,
        subject_kind,
        text:
            DeferredPhysicalGuaranteeRowText {
                subject_path_or_symbol,
                classification,
                notes,
                missing_proof,
                dependent_named_suite,
            },
        evidence_refs,
        forbidden_claims,
        deferred_s_sequences,
        status,
        guarantee_category,
        current_evidence_status,
        dependent_evidence_lanes,
    } = input;
    DeferredPhysicalGuaranteeRow {
        row_id,
        subject_kind,
        subject_path_or_symbol,
        classification,
        evidence_refs,
        forbidden_claims,
        deferred_s_sequences,
        status,
        notes,
        guarantee_category,
        current_evidence_status,
        missing_proof,
        dependent_named_suite,
        dependent_evidence_lanes,
    }
}
