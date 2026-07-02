use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::derived_topology::invalidation_plan::operator_cutover::{
    DerivedInvalidationOperatorCutoverError, DerivedInvalidationOperatorCutoverReceipt,
    DerivedInvalidationProjectionReadStageReceipt, ProjectionReadStageConsumptionScope,
};
use crate::derived_topology::materialized_graph::{
    MaterializedTopologyView, TopologyMaterializationError, TopologyMaterializer,
};
use crate::derived_topology::traversal_views::{
    bootstrap_topology_interpretation, InterpretedTopologyView,
};
use crate::projection::planner_owned_routing::diagnostic_projection_input::derive_topology_validation_report;
use crate::validation::TopologyValidationError;

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

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn consume_derived_invalidation_for_projection_read_stage(
    operator_cutover: &DerivedInvalidationOperatorCutoverReceipt,
) -> Result<DerivedInvalidationProjectionReadStageReceipt, DerivedInvalidationOperatorCutoverError>
{
    DerivedInvalidationProjectionReadStageReceipt::consume_operator_cutover(
        operator_cutover,
        ProjectionReadStageConsumptionScope::CommittedRead,
        0,
    )
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
    let interpreted = bootstrap_topology_interpretation(&materialized);
    let validation = derive_topology_validation_report(&materialized, &interpreted)?;
    Ok(StagedTopologyRead {
        materialized,
        interpreted,
        validation,
    })
}
