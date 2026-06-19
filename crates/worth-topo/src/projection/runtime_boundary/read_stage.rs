#[cfg(test)]
use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
#[cfg(test)]
use schema::facade::topology_authoring::DerivedTopologyReadBasis;

#[cfg(not(test))]
use crate::derived_topology::materialized_graph::TopologyMaterializationError;
#[cfg(test)]
use crate::derived_topology::materialized_graph::{
    MaterializedTopologyView, TopologyMaterializationError, TopologyMaterializer,
};
#[cfg(test)]
use crate::derived_topology::traversal_views::{interpret_topology_view, InterpretedTopologyView};
#[cfg(test)]
use crate::projection::diagnostic_surfaces::derived_read_diagnostics::derive_topology_validation_report;
#[cfg(not(test))]
use crate::validation::TopologyValidationError;
#[cfg(test)]
use crate::validation::TopologyValidationError;

#[derive(Debug)]
pub(crate) enum TopologyReadStageError {
    #[cfg(test)]
    ReadView(String),
    Materialization(TopologyMaterializationError),
    Validation(TopologyValidationError),
}

impl std::fmt::Display for TopologyReadStageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(test)]
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

#[cfg(test)]
#[derive(Debug, Clone)]
pub(crate) struct StagedTopologyRead {
    materialized: MaterializedTopologyView,
    interpreted: InterpretedTopologyView,
    validation: crate::validation::DerivedTopologyValidationReport,
}

#[cfg(test)]
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

#[cfg(test)]
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

#[cfg(test)]
pub(crate) fn stage_topology_read_from_view(
    read_view: &RelationalReadView,
) -> Result<StagedTopologyRead, TopologyReadStageError> {
    let materialized = TopologyMaterializer::materialize_from_truth(read_view)?;
    let interpreted = interpret_topology_view(&materialized);
    let validation = derive_topology_validation_report(&materialized, &interpreted)?;
    Ok(StagedTopologyRead {
        materialized,
        interpreted,
        validation,
    })
}
