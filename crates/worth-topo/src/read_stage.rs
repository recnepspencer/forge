use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
use worth_schema::facade::DerivedTopologyReadBasis;

use crate::facade::{
    interpret_topology_view, validate_interpreted_topology, InterpretedTopologyView,
    MaterializedTopologyView, WorthTopologyMaterializationError, WorthTopologyMaterializer,
    WorthTopologyValidationError,
};

#[derive(Debug)]
pub(crate) enum WorthTopologyReadStageError {
    ReadView(String),
    Materialization(WorthTopologyMaterializationError),
    Validation(WorthTopologyValidationError),
}

impl std::fmt::Display for WorthTopologyReadStageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadView(error) => write!(f, "read view: {error}"),
            Self::Materialization(error) => write!(f, "materialization: {error}"),
            Self::Validation(error) => write!(f, "validation: {error}"),
        }
    }
}

impl std::error::Error for WorthTopologyReadStageError {}

impl From<WorthTopologyMaterializationError> for WorthTopologyReadStageError {
    fn from(value: WorthTopologyMaterializationError) -> Self {
        Self::Materialization(value)
    }
}

impl From<WorthTopologyValidationError> for WorthTopologyReadStageError {
    fn from(value: WorthTopologyValidationError) -> Self {
        Self::Validation(value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StagedWorthTopologyRead {
    materialized: MaterializedTopologyView,
    interpreted: InterpretedTopologyView,
    validation: crate::validators::DerivedTopologyValidationReport,
}

impl StagedWorthTopologyRead {
    pub(crate) fn materialized(&self) -> &MaterializedTopologyView {
        &self.materialized
    }

    pub(crate) fn interpreted(&self) -> &InterpretedTopologyView {
        &self.interpreted
    }

    pub(crate) fn validation(&self) -> &crate::validators::DerivedTopologyValidationReport {
        &self.validation
    }
}

pub(crate) fn open_topology_read_view(
    runtime: &RelationalRuntime,
    basis: &DerivedTopologyReadBasis,
) -> Result<RelationalReadView, WorthTopologyReadStageError> {
    runtime
        .read_truth()
        .read_snapshot(basis.snapshot())
        .ok_or_else(|| {
            WorthTopologyReadStageError::ReadView(format!(
                "worth topology staging could not open snapshot {:?}",
                basis.snapshot()
            ))
        })
}

pub(crate) fn stage_topology_read_from_view(
    read_view: &RelationalReadView,
) -> Result<StagedWorthTopologyRead, WorthTopologyReadStageError> {
    let materialized = WorthTopologyMaterializer::materialize_from_truth(read_view)?;
    let interpreted = interpret_topology_view(&materialized);
    let validation = validate_interpreted_topology(&materialized, &interpreted)?;
    Ok(StagedWorthTopologyRead {
        materialized,
        interpreted,
        validation,
    })
}
