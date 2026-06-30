use crate::derived_topology::materialized_graph::TopologyMaterializationError;
use crate::projection::runtime_boundary::declared_query_surfaces::TopologyQuerySurfaceError;
use crate::projection::runtime_boundary::read_stage::TopologyReadStageError;
use crate::test_support::schema_topology_authoring_boundary::SchemaPrimitiveAuthoringError;
use crate::validation::TopologyValidationError;

#[derive(Debug)]
pub enum MilestoneOneCertificationError {
    Authoring(SchemaPrimitiveAuthoringError),
    Query(String),
    ReadView(String),
    Materialization(TopologyMaterializationError),
    Validation(TopologyValidationError),
}

pub type TopologyCertificationError = MilestoneOneCertificationError;

impl std::fmt::Display for MilestoneOneCertificationError {
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

impl std::error::Error for MilestoneOneCertificationError {}

impl From<TopologyMaterializationError> for MilestoneOneCertificationError {
    fn from(value: TopologyMaterializationError) -> Self {
        Self::Materialization(value)
    }
}

impl From<TopologyValidationError> for MilestoneOneCertificationError {
    fn from(value: TopologyValidationError) -> Self {
        Self::Validation(value)
    }
}

impl From<SchemaPrimitiveAuthoringError> for MilestoneOneCertificationError {
    fn from(value: SchemaPrimitiveAuthoringError) -> Self {
        Self::Authoring(value)
    }
}

impl From<TopologyQuerySurfaceError> for MilestoneOneCertificationError {
    fn from(value: TopologyQuerySurfaceError) -> Self {
        Self::Query(value.to_string())
    }
}

impl From<TopologyReadStageError> for MilestoneOneCertificationError {
    fn from(value: TopologyReadStageError) -> Self {
        match value {
            TopologyReadStageError::ReadView(error) => Self::ReadView(error),
            TopologyReadStageError::Materialization(error) => Self::Materialization(error),
            TopologyReadStageError::Validation(error) => Self::Validation(error),
        }
    }
}
