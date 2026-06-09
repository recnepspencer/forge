use crate::candidate_screening::CandidateScreeningError;
use crate::domain_artifacts::HadwigerArtifactShapeError;
use crate::domain_declarations::HadwigerResearchDeclarationShapeError;
use crate::tiling_geometry::TilingGeometryError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GeneratedPatternReplayShapeError {
    EmptyField { field: &'static str },
    DuplicateIdentity { field: &'static str, value: String },
    UnknownLatticeVector { vector_id: String },
    UnknownTile { tile_id: String },
    MissingSourceCell,
    MissingPeriodicQuotientCell,
    MissingReplayCertificate,
    QuotientReferenceMismatch,
}

#[derive(Debug)]
pub enum GeneratedPatternReplayError {
    Shape(GeneratedPatternReplayShapeError),
    Artifact(HadwigerArtifactShapeError),
    Declaration(HadwigerResearchDeclarationShapeError),
    Screening(CandidateScreeningError),
    TilingGeometry(TilingGeometryError),
}

impl From<GeneratedPatternReplayShapeError> for GeneratedPatternReplayError {
    fn from(error: GeneratedPatternReplayShapeError) -> Self {
        Self::Shape(error)
    }
}

impl From<HadwigerArtifactShapeError> for GeneratedPatternReplayError {
    fn from(error: HadwigerArtifactShapeError) -> Self {
        Self::Artifact(error)
    }
}

impl From<HadwigerResearchDeclarationShapeError> for GeneratedPatternReplayError {
    fn from(error: HadwigerResearchDeclarationShapeError) -> Self {
        Self::Declaration(error)
    }
}

impl From<CandidateScreeningError> for GeneratedPatternReplayError {
    fn from(error: CandidateScreeningError) -> Self {
        Self::Screening(error)
    }
}

impl From<TilingGeometryError> for GeneratedPatternReplayError {
    fn from(error: TilingGeometryError) -> Self {
        Self::TilingGeometry(error)
    }
}

pub(crate) fn require_replay_non_empty(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, GeneratedPatternReplayShapeError> {
    let value = value.into();
    if value.trim().is_empty() {
        Err(GeneratedPatternReplayShapeError::EmptyField { field })
    } else {
        Ok(value)
    }
}
