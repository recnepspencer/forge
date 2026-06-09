use crate::candidate_screening::CandidateScreeningError;
use crate::domain_artifacts::HadwigerArtifactShapeError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TilingGeometryError {
    ArtifactShape(HadwigerArtifactShapeError),
    Screening(CandidateScreeningError),
    DuplicateTile { tile_id: String },
    MissingTile { tile_id: String },
    MissingBoundaryOwnership { tile_id: String },
    AmbiguousBoundaryOwnership { tile_id: String },
    SameTileContact { tile_id: String },
    QueryCellDeclarationNotAdmitted,
    QueryContactDeclarationNotAdmitted,
    RationalConversion,
}

impl From<HadwigerArtifactShapeError> for TilingGeometryError {
    fn from(value: HadwigerArtifactShapeError) -> Self {
        Self::ArtifactShape(value)
    }
}

impl From<CandidateScreeningError> for TilingGeometryError {
    fn from(value: CandidateScreeningError) -> Self {
        Self::Screening(value)
    }
}
