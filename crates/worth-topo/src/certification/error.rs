use crate::materialization::WorthTopologyMaterializationError;
use crate::validators::WorthTopologyValidationError;

#[derive(Debug)]
pub enum WorthMilestoneOneCertificationError {
    Materialization(WorthTopologyMaterializationError),
    Validation(WorthTopologyValidationError),
}

impl std::fmt::Display for WorthMilestoneOneCertificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
