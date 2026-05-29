use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::facade::{
    interpret_topology_view, validate_interpreted_topology, InterpretedTopologyView,
    MaterializedTopologyView, TopologyMaterializationError, TopologyMaterializer,
    TopologyValidationError,
};

#[derive(Debug)]
pub(crate) enum TopologyReadStageError {
    ReadView(String),
    Materialization(TopologyMaterializationError),
    Validation(TopologyValidationError),
}

impl std::fmt::Display for TopologyReadStageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadView(error) => write!(f, "read view: {error}"),
            Self::Materialization(error) => write!(f, "materialization: {error}"),
            Self::Validation(error) => write!(f, "validation: {error}"),
        }
    }
}

impl std::error::Error for TopologyReadStageError {}

impl From<TopologyMaterializationError> for TopologyReadStageError {
    fn from(value: TopologyMaterializationError) -> Self {
        Self::Materialization(value)
    }
}

impl From<TopologyValidationError> for TopologyReadStageError {
    fn from(value: TopologyValidationError) -> Self {
        Self::Validation(value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StagedTopologyRead {
    materialized: MaterializedTopologyView,
    interpreted: InterpretedTopologyView,
    validation: crate::validation::DerivedTopologyValidationReport,
}

impl StagedTopologyRead {
    pub(crate) fn materialized(&self) -> &MaterializedTopologyView {
        &self.materialized
    }

    pub(crate) fn interpreted(&self) -> &InterpretedTopologyView {
        &self.interpreted
    }

    pub(crate) fn validation(&self) -> &crate::validation::DerivedTopologyValidationReport {
        &self.validation
    }
}

pub(crate) fn open_topology_read_view(
    runtime: &RelationalRuntime,
    basis: &DerivedTopologyReadBasis,
) -> Result<RelationalReadView, TopologyReadStageError> {
    runtime
        .read_truth()
        .read_snapshot(basis.snapshot())
        .ok_or_else(|| {
            TopologyReadStageError::ReadView(format!(
                " topology staging could not open snapshot {:?}",
                basis.snapshot()
            ))
        })
}

pub(crate) fn stage_topology_read_from_view(
    read_view: &RelationalReadView,
) -> Result<StagedTopologyRead, TopologyReadStageError> {
    let materialized = TopologyMaterializer::materialize_from_truth(read_view)?;
    let interpreted = interpret_topology_view(&materialized);
    let validation = validate_interpreted_topology(&materialized, &interpreted)?;
    Ok(StagedTopologyRead {
        materialized,
        interpreted,
        validation,
    })
}




