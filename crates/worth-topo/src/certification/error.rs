use crate::materialization::WorthTopologyMaterializationError;
use crate::query::{WorthTopologyQueryImportError, WorthTopologyQuerySurfaceError};
use crate::reader::WorthTopologyReadError;
use crate::validators::WorthTopologyValidationError;
use worth_schema::facade::WorthMilestoneOnePrimitiveAuthoringError;

#[derive(Debug)]
pub enum WorthMilestoneOneCertificationError {
    Authoring(WorthMilestoneOnePrimitiveAuthoringError),
    Query(String),
    ReadView(String),
    Materialization(WorthTopologyMaterializationError),
    Validation(WorthTopologyValidationError),
}

impl std::fmt::Display for WorthMilestoneOneCertificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authoring(error) => write!(f, "authoring: {error}"),
            Self::Query(error) => write!(f, "query: {error}"),
            Self::ReadView(error) => write!(f, "read view: {error}"),
            Self::Materialization(error) => write!(f, "materialization: {error}"),
            Self::Validation(error) => write!(f, "validation: {error}"),
        }
    }
}

impl std::error::Error for WorthMilestoneOneCertificationError {}

impl From<WorthTopologyMaterializationError> for WorthMilestoneOneCertificationError {
    fn from(value: WorthTopologyMaterializationError) -> Self {
        Self::Materialization(value)
    }
}

impl From<WorthTopologyValidationError> for WorthMilestoneOneCertificationError {
    fn from(value: WorthTopologyValidationError) -> Self {
        Self::Validation(value)
    }
}

impl From<WorthTopologyReadError> for WorthMilestoneOneCertificationError {
    fn from(value: WorthTopologyReadError) -> Self {
        match value {
            WorthTopologyReadError::ReadView(error) => Self::ReadView(error),
            WorthTopologyReadError::Materialization(error) => Self::Materialization(error),
            WorthTopologyReadError::Validation(error) => Self::Validation(error),
        }
    }
}

impl From<WorthMilestoneOnePrimitiveAuthoringError> for WorthMilestoneOneCertificationError {
    fn from(value: WorthMilestoneOnePrimitiveAuthoringError) -> Self {
        Self::Authoring(value)
    }
}

impl From<WorthTopologyQueryImportError> for WorthMilestoneOneCertificationError {
    fn from(value: WorthTopologyQueryImportError) -> Self {
        Self::Query(value.to_string())
    }
}

impl From<WorthTopologyQuerySurfaceError> for WorthMilestoneOneCertificationError {
    fn from(value: WorthTopologyQuerySurfaceError) -> Self {
        Self::Query(value.to_string())
    }
}
