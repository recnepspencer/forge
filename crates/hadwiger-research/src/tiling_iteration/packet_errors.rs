use crate::domain_artifacts::HadwigerArtifactShapeError;
use crate::domain_declarations::HadwigerResearchDeclarationShapeError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TilingIterationError {
    EmptyField { field: &'static str },
    MissingCockpitSession,
    MissingEvidenceBasis,
    MissingRequiredCheckerLane,
    MissingExpectedInformationGain,
    MissingReactivationObligation,
    QueryDeclarationNotAdmitted { declaration: &'static str },
    ReplayDigestMismatch,
    Shape(HadwigerArtifactShapeError),
    Declaration(HadwigerResearchDeclarationShapeError),
}

impl From<HadwigerArtifactShapeError> for TilingIterationError {
    fn from(error: HadwigerArtifactShapeError) -> Self {
        Self::Shape(error)
    }
}

impl From<HadwigerResearchDeclarationShapeError> for TilingIterationError {
    fn from(error: HadwigerResearchDeclarationShapeError) -> Self {
        Self::Declaration(error)
    }
}

pub(crate) fn require_iteration_non_empty(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, TilingIterationError> {
    let value = value.into();
    if value.trim().is_empty() {
        Err(TilingIterationError::EmptyField { field })
    } else {
        Ok(value)
    }
}
