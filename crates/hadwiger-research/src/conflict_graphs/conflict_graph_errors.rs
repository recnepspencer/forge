use crate::domain_artifacts::HadwigerArtifactShapeError;
use crate::domain_declarations::HadwigerResearchDeclarationShapeError;
use crate::mathematical_verification::HadwigerColorabilityError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConflictGraphError {
    EmptyField { field: &'static str },
    EmptyConflictEdges,
    ContactReportDoesNotReject,
    GeneratedReplayHasNoRejectedEvidence,
    GeneratedReplayHasNoExtractableConflictEdges,
    QueryDeclarationNotAdmitted { declaration: &'static str },
    DeletionCheckGraphMismatch { target: String },
    DeletionCheckVerificationMismatch { target: String },
    DeletionCheckColorCountMismatch { target: String },
    MissingDeletionChecks { expected: usize, actual: usize },
    DuplicateDeletionCheck { target: String },
    Artifact(HadwigerArtifactShapeError),
    Declaration(HadwigerResearchDeclarationShapeError),
    Colorability(HadwigerColorabilityError),
}

impl From<HadwigerArtifactShapeError> for ConflictGraphError {
    fn from(error: HadwigerArtifactShapeError) -> Self {
        Self::Artifact(error)
    }
}

impl From<HadwigerResearchDeclarationShapeError> for ConflictGraphError {
    fn from(error: HadwigerResearchDeclarationShapeError) -> Self {
        Self::Declaration(error)
    }
}

impl From<HadwigerColorabilityError> for ConflictGraphError {
    fn from(error: HadwigerColorabilityError) -> Self {
        Self::Colorability(error)
    }
}

pub(crate) fn require_conflict_non_empty(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, ConflictGraphError> {
    let value = value.into();
    if value.trim().is_empty() {
        Err(ConflictGraphError::EmptyField { field })
    } else {
        Ok(value)
    }
}
