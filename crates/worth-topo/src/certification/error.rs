use crate::derived_topology::materialized_graph::TopologyMaterializationError;
use crate::projection::TopologyQuerySurfaceError;
use crate::validation::TopologyValidationError;
use schema::facade::topology_authoring::MilestoneOnePrimitiveAuthoringError;

#[derive(Debug)]
pub enum MilestoneOneCertificationError {
    Authoring(MilestoneOnePrimitiveAuthoringError),
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

impl From<MilestoneOnePrimitiveAuthoringError> for MilestoneOneCertificationError {
    fn from(value: MilestoneOnePrimitiveAuthoringError) -> Self {
        Self::Authoring(value)
    }
}

impl From<TopologyQuerySurfaceError> for MilestoneOneCertificationError {
    fn from(value: TopologyQuerySurfaceError) -> Self {
        Self::Query(value.to_string())
    }
}
